//
// stdf_file.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Wed Nov 02 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use crate::stdf_error::StdfError;
use crate::stdf_types::*;
use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use std::io::{self, BufReader, SeekFrom}; // struct or enum
use std::io::{BufRead, Read, Seek};
use std::{fs, path::Path}; // trait

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
///     .map(|x| x.unwrap())
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
pub struct StdfReader<R> {
    endianness: ByteOrder,
    stream: StdfStream<R>,
}

pub struct RecordIter<'a, R> {
    inner: &'a mut StdfReader<R>,
}

pub struct RawDataIter<'a, R> {
    offset: u64,
    inner: &'a mut StdfReader<R>,
}

// implementations

impl StdfReader<BufReader<fs::File>> {
    /// Open the given file and return a StdfReader, if successful
    #[inline(always)]
    pub fn new<P>(path: P) -> Result<Self, StdfError>
    where
        P: AsRef<Path>,
    {
        // determine the compress type by file extension
        let path_string = path.as_ref().display().to_string();
        let compress_type = if path_string.ends_with(".gz") {
            CompressType::GzipCompressed
        } else if path_string.ends_with(".bz2") {
            CompressType::BzipCompressed
        } else if path_string.ends_with(".zip") {
            CompressType::ZipCompressed
        } else {
            CompressType::Uncompressed
        };
        let fp = fs::OpenOptions::new().read(true).open(path)?;
        let br = BufReader::with_capacity(2 << 20, fp);
        StdfReader::from(br, &compress_type)
    }
}

impl<R: BufRead + Seek> StdfReader<R> {
    /// Consume a input stream and generate a StdfReader, if successful
    #[inline(always)]
    pub fn from(in_stream: R, compress_type: &CompressType) -> Result<Self, StdfError> {
        let mut stream = match compress_type {
            CompressType::GzipCompressed => StdfStream::Gz(GzDecoder::new(in_stream)),
            CompressType::BzipCompressed => StdfStream::Bz(BzDecoder::new(in_stream)),
            _ => StdfStream::Binary(in_stream),
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

        Ok(StdfReader { endianness, stream })
    }

    #[inline(always)]
    fn read_header(&mut self) -> Result<RecordHeader, StdfError> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf)?;
        RecordHeader::new().read_from_bytes(&buf, &self.endianness)
    }

    /// return an iterator for StdfRecord
    ///
    /// Only the records after the current file position
    /// can be read.
    #[inline(always)]
    pub fn get_record_iter(&mut self) -> RecordIter<R> {
        RecordIter { inner: self }
    }

    /// return an iterator for unprocessed STDF bytes
    ///
    /// beware that internal `offset` counter is starting
    /// from the current position.
    #[inline(always)]
    pub fn get_rawdata_iter(&mut self) -> RawDataIter<R> {
        RawDataIter {
            offset: 0,
            inner: self,
        }
    }
}

impl<R: BufRead> StdfStream<R> {
    #[inline(always)]
    pub(crate) fn read_until(&mut self, delim: u8, buf: &mut Vec<u8>) -> io::Result<usize> {
        match self {
            StdfStream::Binary(bstream) => bstream.read_until(delim, buf),
            StdfStream::Gz(gzstream) => general_read_until(gzstream, delim, buf),
            StdfStream::Bz(bzstream) => general_read_until(bzstream, delim, buf),
        }
    }
}

impl<R: BufRead> Read for StdfStream<R> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            StdfStream::Gz(gzstream) => gzstream.read(buf),
            StdfStream::Bz(bzstream) => bzstream.read(buf),
            StdfStream::Binary(bstream) => bstream.read(buf),
        }
    }
}

// impl<R: Seek> Seek for StdfStream<R> {
//     fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
//         match self {
//             StdfStream::Binary(bstream) => bstream.seek(pos),
//             // arm that does not support seek
//             _ => Ok(0),
//         }
//     }
// }

impl<R: BufRead + Seek> Iterator for RecordIter<'_, R> {
    type Item = Result<StdfRecord, StdfError>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let header = match self.inner.read_header() {
            Ok(h) => h,
            Err(e) => {
                return match e.code {
                    // only 2 error will be returned by `read_header`
                    // code = 4, indicates normal EOF
                    4 => None,
                    // code = 5, indicates unexpected EOF
                    _ => Some(Err(e)),
                };
            }
        };
        // create a buffer to store record raw data
        let mut buffer = vec![0u8; header.len as usize];
        if let Err(io_e) = self.inner.stream.read_exact(&mut buffer) {
            return Some(Err(StdfError {
                code: 3,
                msg: io_e.to_string(),
            }));
        }

        let mut rec = StdfRecord::new_from_header(header);
        rec.read_from_bytes(&buffer, &self.inner.endianness);
        Some(Ok(rec))
    }
}

impl<R: BufRead + Seek> Iterator for RawDataIter<'_, R> {
    type Item = Result<RawDataElement, StdfError>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let header = match self.inner.read_header() {
            Ok(h) => h,
            Err(e) => {
                return match e.code {
                    // code = 4, indicates normal EOF
                    4 => None,
                    // code = 5, indicates unexpected EOF
                    _ => Some(Err(e)),
                };
            }
        };
        // advance position by 4 after reading a header successfully
        self.offset += 4;
        let data_offset = self.offset;
        // create a buffer to store record raw data
        let mut buffer = vec![0u8; header.len as usize];
        if let Err(io_e) = self.inner.stream.read_exact(&mut buffer) {
            return Some(Err(StdfError {
                code: 3,
                msg: io_e.to_string(),
            }));
        }
        self.offset += header.len as u64;

        Some(Ok(RawDataElement {
            offset: data_offset,
            header,
            raw_data: buffer,
            byte_order: self.inner.endianness,
        }))
    }
}

// help functions

#[inline(always)]
pub(crate) fn rewind_stream_position<R: BufRead + Seek>(
    old_stream: StdfStream<R>,
) -> Result<StdfStream<R>, StdfError> {
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

#[inline(always)]
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
