//
// stdf_file.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Thu Oct 06 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use crate::stdf_error::StdfError;
use crate::stdf_types::*;
use flate2::bufread::GzDecoder;
use std::fs;
use std::io::{self, BufReader, SeekFrom}; // struct or enum
use std::io::{BufRead, Read, Seek}; // trait

type StreamT = StdfStream<BufReader<fs::File>>;

#[derive(Debug)]
enum StdfStream<R> {
    BinaryStream(R),
    GzStream(GzDecoder<R>),
}

/// STDF Reader
/// 
/// This reader can process STDF datalogs of Version V4 and V4-2007
/// 
/// Supported compression:
///  - Uncompressed
///  - Gzip (.gz)
/// 
/// # Example
/// 
/// ```
/// use rust_stdf::*;
/// 
/// fn main() {
///     let stdf_path = "demo_file.stdf";
///     let mut reader = match StdfReader::new(&stdf_path) {
///         Ok(r) => r,
///         Err(e) => {
///             println!("{}", e);
///             return;
///         }
///     };
/// 
///     // we will count total DUT# in the file
///     // and put test result of PTR named 
///     // "continuity test" in a vector.
///     let mut dut_count: u64 = 0;
///     let mut continuity_rlt = vec![];
/// 
///     // use type filter to work on certain types,
///     // use `|` to combine multiple typs
///     let rec_types = REC_PIR | REC_PTR;
///     // iterator starts from current file position,
///     // if file hits EOF, it will NOT redirect to 0.
///     for rec in reader
///         .get_record_iter()
///         .filter(|x| x.is_type(rec_types)) 
///     {
///         match rec {
///             StdfRecord::PIR(_) => {dut_count += 1;}
///             StdfRecord::PTR(ref ptr_rec) => {
///                 if ptr_rec.test_txt == "continuity test" {
///                     continuity_rlt.push(ptr_rec.result);
///                 }
///             }
///             _ => {}
///         }
///     }
///     println!("Total duts {} \n continuity result {:?}", 
///             dut_count, 
///             continuity_rlt);
/// }
/// 
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
            CompressType::GzipCompressed => StdfStream::GzStream(GzDecoder::new(br)),
            _ => StdfStream::BinaryStream(br),
        };

        // read FAR header from file
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf)?;
        // parse header assuming little endian
        let far_header = RecordHeader::new().from_bytes(&buf, &ByteOrder::LittleEndian)?;
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
        stream = StdfReader::rewind_stream_position(stream)?;

        Ok(StdfReader {
            file_path: String::from(path),
            endianness,
            stream,
        })
    }

    fn rewind_stream_position(old_stream: StreamT) -> Result<StreamT, StdfError> {
        let new_stream = match old_stream {
            StdfStream::BinaryStream(mut br) => {
                br.seek(SeekFrom::Start(0))?;
                StdfStream::BinaryStream(br)
            }
            StdfStream::GzStream(gzr) => {
                // get the inner handle and create a new stream after seek
                let mut fp = gzr.into_inner();
                fp.seek(SeekFrom::Start(0))?;
                StdfStream::GzStream(GzDecoder::new(fp))
            }
        };
        Ok(new_stream)
    }

    fn read_header(&mut self) -> Result<RecordHeader, StdfError> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf)?;
        // parse header assuming little endian
        RecordHeader::new().from_bytes(&buf, &self.endianness)
    }

    pub fn get_record_iter(&mut self) -> RecordIter {
        RecordIter { inner: self }
    }
}

impl<R: BufRead> Read for StdfStream<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            StdfStream::GzStream(gzstream) => gzstream.read(buf),
            StdfStream::BinaryStream(bstream) => bstream.read(buf),
        }
    }
}

impl<R: Seek> Seek for StdfStream<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match self {
            // flate2 does not support seek
            StdfStream::GzStream(_gzstream) => Ok(0),
            StdfStream::BinaryStream(bstream) => bstream.seek(pos),
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
        Some(StdfRecord::new(header.type_code).from_bytes(&buffer, &self.inner.endianness))
    }
}
