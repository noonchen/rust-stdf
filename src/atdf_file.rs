//
// atdf_file.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 6th 2022
// -----
// Last Modified: Fri Oct 07 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//


use crate::stdf_file::{StdfStream, StreamT, rewind_stream_position};
use crate::stdf_types::{CompressType, StdfRecord};
use crate::stdf_error::StdfError;
use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use std::{fs, str};
use std::io::{self, BufReader, SeekFrom}; // struct or enum
use std::io::{BufRead, Read, Seek}; // trait


pub struct AtdfReader {
    pub file_path: String,
    delimiter: char,
    scale_flag: bool,
    stream: StreamT,
}

pub struct AtdfRecordIter<'a> {
    inner: &'a mut AtdfReader,
    // ATDF record might be divided 
    // into multiple line.
    incomplete_rec: String,
}


// impl

impl AtdfReader {
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

        let mut far_bytes = vec![];
        stream.read_until(b'\n', &mut far_bytes)?;
        let far_str = std::str::from_utf8(&far_bytes)?;
        if !far_str.starts_with("FAR:A") || far_bytes.len() < 9 {
            return Err(StdfError {code: 6, msg: format!("FAR record pattern 'FAR:A' not detected, found {}", far_str)});
        }
        // according to atdf spec, delimiter is the byte after 'A'
        let delimiter = far_bytes[5] as char;
        // parametric scale flag, default is false
        let scale_flag = {
            let far_str_vec: Vec<_> = far_str.split(delimiter).collect();
            if far_str_vec.len() > 3 {
                far_str_vec[3] == "S"
            } else {
                false
            }
        };
        // reset file position
        stream = rewind_stream_position(stream)?;

        Ok(AtdfReader {
            file_path: String::from(path),
            delimiter,
            scale_flag,
            stream: stream,
        })
    }

    pub fn get_record_iter(&mut self) -> AtdfRecordIter {
        AtdfRecordIter { 
            inner: self,
            incomplete_rec: String::new(),
        }
    }
    
}


fn str_trim(input: &str) -> &str {
    let no_pre_space = input
                            .strip_prefix(" ")
                            .unwrap_or(input);
    no_pre_space
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(no_pre_space)
}

impl Iterator for AtdfRecordIter<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // if next_rec is empty, means 
        // the previous rec is not completed yet
        loop {
            // read a line
            let mut tmp_buf = Vec::with_capacity(255);
            if let Err(e) = self.inner.stream.read_until(b'\n', &mut tmp_buf) {
                println!("{}", e);
                return None;
            }

            let data_len = tmp_buf.len();
            if data_len > 0 && tmp_buf[0] == b' ' {
                // starts with space, means it belongs to incomplete_rec
                // remove prefix space and suffix \n
                if let Ok(tmp_str) = str::from_utf8(&tmp_buf) {
                    self.incomplete_rec.push_str(str_trim(tmp_str));
                } else {
                    println!("error: non ascii found");
                    return None;
                }
            } else {
                // possibly a new rec
                // do some checks here
                if data_len < 4 || tmp_buf[3] != b':' {
                    println!("invalid line, {}, {:?}", str::from_utf8(&tmp_buf).unwrap(), tmp_buf);
                    return None;
                }
                // get the completed rec
                let complete = self.incomplete_rec.clone();
                // save fresh data
                if let Ok(tmp_str) = str::from_utf8(&tmp_buf) {
                    self.incomplete_rec = String::from(str_trim(tmp_str));
                    return Some(complete)
                } else {
                    println!("error: non ascii found");
                    return None;
                }
            }
        }
    }
}
