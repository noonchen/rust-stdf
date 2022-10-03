//
// stdf_file.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Tue Oct 04 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//


use crate::stdf_error::StdfError;
use crate::stdf_types::*;
use std::fs;
use std::io::{BufReader, Read, SeekFrom, Seek};
// use std::iter::Iterator;
// use flate2::{self, Compress};



#[derive(Debug)]
pub struct StdfReader {
    pub file_path: String,
    pub compress_type: CompressType,
    pub endianness: ByteOrder,
    pub reader: BufReader<fs::File>,
}

impl StdfReader {
    pub fn new(path: &str) -> Result<Self, StdfError> {
        let fp = fs::OpenOptions::new().read(true).open(path)?;
        let mut reader = BufReader::with_capacity(2<<20, fp);

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
        
        // read FAR header from file
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        // parse header assuming little endian
        let far_header = RecordHeader::from_bytes(&buf, &ByteOrder::LittleEndian)?;
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
        reader.seek(SeekFrom::Start(0))?;
        // return
        Ok(StdfReader{
            file_path: String::from(path), 
            compress_type,
            endianness,
            reader})
    }
}


