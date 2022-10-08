//
// atdf_file.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 6th 2022
// -----
// Last Modified: Sat Oct 08 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use crate::stdf_error::StdfError;
use crate::stdf_file::{rewind_stream_position, StdfStream, StreamT};
use crate::stdf_types::{AtdfRecord, CompressType};
use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use std::io::BufReader;
use std::{fs, str};

pub struct AtdfReader {
    pub file_path: String,
    delimiter: char,
    scale_flag: bool,
    stream: StreamT,
}

pub struct AtdfRecordIter<'a> {
    inner: &'a mut AtdfReader,
    // ATDF record might be divided
    // into multiple lines.
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
            return Err(StdfError {
                code: 6,
                msg: format!("FAR record pattern 'FAR:A' not detected, found {}", far_str),
            });
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
            stream,
        })
    }

    pub fn get_record_iter(&mut self) -> AtdfRecordIter {
        AtdfRecordIter {
            inner: self,
            incomplete_rec: String::new(),
        }
    }
}

// implement of ATDF iterator

impl Iterator for AtdfRecordIter<'_> {
    type Item = AtdfRecord;

    fn next(&mut self) -> Option<Self::Item> {
        // if next_rec is empty, means
        // the previous rec is not completed yet
        loop {
            // read a line
            let mut tmp_buf = Vec::with_capacity(127);
            let eof = match self.inner.stream.read_until(b'\n', &mut tmp_buf) {
                Ok(n) => n == 0,
                Err(e) => {
                    println!("Error when reading ATDF file => {}", e);
                    return None;
                }
            };

            let tmp_line = match str::from_utf8(&tmp_buf) {
                Ok(s) => s,
                Err(_) => {
                    println!("String error: ATDF should only contains ascii symbols, ");
                    return None;
                }
            };

            if !tmp_line.is_empty() && tmp_line.starts_with(' ') {
                // starts with space, means it belongs to incomplete_rec
                // remove prefix space and suffix \n
                self.incomplete_rec.push_str(str_trim(tmp_line));
                // directly goes to the next loop iteration
                continue;
            }

            // not starts with space, trim \r\n first
            let clean_line = str_trim(tmp_line);
            // if current line is empty, but eof is not reach
            // skip this empty line...
            if !eof && clean_line.is_empty() {
                continue;
            }

            // a possible new rec found! or EOF reached
            // clone the completed_rec for processing
            let complete_rec = self.incomplete_rec.clone();
            // store clean_line to incomplete_rec
            self.incomplete_rec = String::from(clean_line);
            // if previous incomplete_rec is empty && EOF, we should stop
            if eof && complete_rec.is_empty() {
                return None;
            } else if complete_rec.is_empty() {
                // not eof, but not content in complete_rec
                // happens in the beginning
                continue;
            }

            // send...
            return match AtdfRecord::from_atdf_string(
                &complete_rec,
                self.inner.delimiter,
                self.inner.scale_flag,
            ) {
                Ok(atdf_rec) => Some(atdf_rec),
                Err(e) => {
                    println!("{}", e);
                    None
                }
            };
        }
    }
}

pub(crate) fn str_trim(input: &str) -> &str {
    let no_pre_space = input.strip_prefix(' ').unwrap_or(input);
    no_pre_space
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap_or(no_pre_space)
}
