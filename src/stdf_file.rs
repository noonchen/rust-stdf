//
// stdf_file.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Fri Oct 07 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use crate::stdf_error::StdfError;
use crate::stdf_types::*;
use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use std::fs;
use std::io::{self, BufReader, SeekFrom}; // struct or enum
use std::io::{BufRead, Read, Seek}; // trait

pub(crate) type StreamT = StdfStream<BufReader<fs::File>>;

pub(crate) enum StdfStream<R> {
    Binary(R),
    Gz(GzDecoder<R>),
    Bz(BzDecoder<R>),
}

/// STDF Reader
///
/// This reader can process STDF datalogs of Version V4 and V4-2007
///
/// Supported compression:
///  - Uncompressed
///  - Gzip (.gz)
///  - Bzip (.bz2)
///
/// # Example
///
/// ```
/// use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};
///
/// let stdf_path = "demo_file.stdf";
/// let mut reader = match StdfReader::new(&stdf_path) {
///     Ok(r) => r,
///     Err(e) => {
///         println!("{}", e);
///         return;
///     }
/// };
///
/// // we will count total DUT# in the file
/// // and put test result of PTR named
/// // "continuity test" in a vector.
/// let mut dut_count: u64 = 0;
/// let mut continuity_rlt = vec![];
///
/// // use type filter to work on certain types,
/// // use `|` to combine multiple typs
/// let rec_types = REC_PIR | REC_PTR;
/// // iterator starts from current file position,
/// // if file hits EOF, it will NOT redirect to 0.
/// for rec in reader
///     .get_record_iter()
///     .filter(|x| x.is_type(rec_types))
/// {
///     match rec {
///         StdfRecord::PIR(_) => {dut_count += 1;}
///         StdfRecord::PTR(ref ptr_rec) => {
///             if ptr_rec.test_txt == "continuity test" {
///                 continuity_rlt.push(ptr_rec.result);
///             }
///         }
///         _ => {}
///     }
/// }
/// println!("Total duts {} \n continuity result {:?}",
///         dut_count,
///         continuity_rlt);
/// ```
pub struct StdfReader {
    pub file_path: String,
    endianness: ByteOrder,
    stream: StreamT,
}

pub struct RecordIter<'a> {
    inner: &'a mut StdfReader,
}

// implementations

pub(crate) fn rewind_stream_position(old_stream: StreamT) -> Result<StreamT, StdfError> {
    let new_stream = match old_stream {
        StdfStream::Binary(mut br) => {
            br.seek(SeekFrom::Start(0))?;
            StdfStream::Binary(br)
        }
        StdfStream::Gz(gzr) => {
            // get the inner handle and create a new stream after seek
            let mut fp = gzr.into_inner();
            fp.seek(SeekFrom::Start(0))?;
            StdfStream::Gz(GzDecoder::new(fp))
        }
        StdfStream::Bz(bzr) => {
            // get the inner handle and create a new stream after seek
            let mut fp = bzr.into_inner();
            fp.seek(SeekFrom::Start(0))?;
            StdfStream::Bz(BzDecoder::new(fp))
        }
    };
    Ok(new_stream)
}

impl StdfReader {
    pub fn new(path: &str) -> Result<Self, StdfError> {
        // determine the compress type by file extension
        let compress_type = if path.ends_with(".gz") {
            CompressType::GzipCompressed
        } else if path.ends_with(".bz2") {
            CompressType::BzipCompressed
        } else if path.ends_with(".zip") {
            CompressType::ZipCompressed
        } else {
            CompressType::Uncompressed
        };

        let fp = fs::OpenOptions::new().read(true).open(path)?;
        let br = BufReader::with_capacity(2 << 20, fp);

        let mut stream = match compress_type {
            CompressType::GzipCompressed => StdfStream::Gz(GzDecoder::new(br)),
            CompressType::BzipCompressed => StdfStream::Bz(BzDecoder::new(br)),
            _ => StdfStream::Binary(br),
        };

        // read FAR header from file
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf)?;
        // parse header assuming little endian
        let far_header = RecordHeader::new().read_from_bytes(&buf, &ByteOrder::LittleEndian)?;
        let endianness = match far_header.len {
            2 => Ok(ByteOrder::LittleEndian),
            512 => Ok(ByteOrder::BigEndian),
            _ => Err(StdfError {
                code: 1,
                msg: String::from("Cannot determine endianness"),
            }),
        }?;
        // check if it's FAR
        if (far_header.typ, far_header.sub) != (0, 10) {
            return Err(StdfError {
                code: 1,
                msg: format!(
                    "FAR header (0, 10) expected, but {:?} is found",
                    (far_header.typ, far_header.sub)
                ),
            });
        }
        // restore file position
        // current flate2 does not support fseek, we need to consume
        // old stream and create a new one.
        // If seek is supported, this function can be replaced by:
        //
        // stream.seek(SeekFrom::Start(0))?;
        //
        stream = rewind_stream_position(stream)?;

        Ok(StdfReader {
            file_path: String::from(path),
            endianness,
            stream,
        })
    }

    fn read_header(&mut self) -> Result<RecordHeader, StdfError> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf)?;
        // parse header assuming little endian
        RecordHeader::new().read_from_bytes(&buf, &self.endianness)
    }

    pub fn get_record_iter(&mut self) -> RecordIter {
        RecordIter { inner: self }
    }
}

fn general_read_until<T: Read>(r: &mut T, delim: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
    let mut one_byte = [0u8; 1];
    let mut n: usize = 0;
    loop {
        // read one byte at a time
        match r.read(&mut one_byte) {
            Ok(num) => {
                if num == 0 {
                    // EOF reached
                    break;
                }
            }
            Err(e) => return Err(e),
        };
        buf.extend_from_slice(&one_byte);
        n += 1;
        // break at delimiter
        if delim == one_byte[0] {
            break;
        }
    }
    Ok(n)
}

impl<R: BufRead> StdfStream<R> {
    pub(crate) fn read_until(&mut self, delim: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self {
            StdfStream::Binary(bstream) => bstream.read_until(delim, buf),
            StdfStream::Gz(gzstream) => general_read_until(gzstream, delim, buf),
            StdfStream::Bz(bzstream) => general_read_until(bzstream, delim, buf),
        }
    }
}

impl<R: BufRead> Read for StdfStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            StdfStream::Gz(gzstream) => gzstream.read(buf),
            StdfStream::Bz(bzstream) => bzstream.read(buf),
            StdfStream::Binary(bstream) => bstream.read(buf),
        }
    }
}

impl<R: Seek> Seek for StdfStream<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match self {
            StdfStream::Binary(bstream) => bstream.seek(pos),
            // arm that does not support seek
            _ => Ok(0),
        }
    }
}

impl Iterator for RecordIter<'_> {
    type Item = StdfRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let header = match self.inner.read_header() {
            Ok(h) => h,
            Err(_) => {
                return None;
            }
        };
        // create a buffer to store record raw data
        let mut buffer = vec![0u8; header.len as usize];
        if self.inner.stream.read_exact(&mut buffer).is_err() {
            return None;
        }
        Some(StdfRecord::new(header.type_code).read_from_bytes(&buffer, &self.inner.endianness))
    }
}
