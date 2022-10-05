//
// stdf_file.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Wed Oct 05 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//


use crate::stdf_error::StdfError;
use crate::stdf_types::*;
use std::fs;
use std::io::{self, BufReader, SeekFrom};     // struct or enum
use std::io::{Read, Seek, BufRead};     // trait
use flate2::{bufread::GzDecoder, };



#[derive(Debug)]
pub enum StdfStream<R> {
    BinaryStream(R),
    GzStream(GzDecoder<R>),
}

pub struct StdfReader {
    file_path: String,
    compress_type: CompressType,
    endianness: ByteOrder,
    stream: StdfStream<BufReader<fs::File>>,
}

pub struct RecordIter<'a> { 
    inner: &'a mut StdfReader
}

// implementations

impl StdfReader {
    pub fn new(path: &str) -> Result<Self, StdfError> {
        // determine the compress type by file extension
        let compress_type = 
            if path.ends_with(".gz") {
                CompressType::GzipCompressed
            } else if path.ends_with(".bz2") {
                CompressType::BzipCompressed
            } else if path.ends_with(".zip") {
                CompressType::ZipCompressed
            } else {
                CompressType::Uncompressed
            };
        
        let fp = fs::OpenOptions::new().read(true).open(path)?;
        let br = BufReader::with_capacity(2<<20, fp);
    
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
            _ => Err(StdfError {code: 1, msg: String::from("Cannot determine endianness")})
        }?;
        // check if it's FAR
        if (far_header.typ, far_header.sub) != (0, 10) {
            return Err(StdfError {code: 1, msg: format!("FAR header (0, 10) expected, but {:?} is found", (far_header.typ, far_header.sub)) })
        }
        // restore file position
        stream.seek(SeekFrom::Start(0))?;
        // current flate2 does not support seek, we need to manually skip FAR data
        match compress_type {
            CompressType::Uncompressed => (),
            _ => {
                let mut far_data = [0u8; 2];
                stream.read_exact(&mut far_data)?;
            }
        };
        // return
        Ok(StdfReader{
            file_path: String::from(path), 
            compress_type,
            endianness,
            stream})
    }

    fn read_header(&mut self) -> Result<RecordHeader, StdfError> {
        let mut buf = [0u8; 4];
        self.stream.read_exact(&mut buf)?;
        // parse header assuming little endian
        Ok(RecordHeader::new().from_bytes(&buf, &self.endianness)?)
    }

    pub fn get_record_iter(&mut self) -> RecordIter {
        RecordIter { inner: self }
    }

    pub fn read_all_records(&mut self) -> Result<Vec<StdfRecord>, StdfError> {
        // restore file position
        self.stream.seek(SeekFrom::Start(0))?;
        Ok(self.get_record_iter().collect())
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
        if let Err(_) = self.inner.stream.read_exact(&mut buffer) {
            return None;
        }
        Some(StdfRecord::new(&header.code).from_bytes(&buffer, &self.inner.endianness))
    }
}