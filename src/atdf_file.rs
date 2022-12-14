//
// atdf_file.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 6th 2022
// -----
// Last Modified: Mon Nov 14 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use crate::atdf_types::AtdfRecord;
use crate::stdf_error::StdfError;
use crate::stdf_file::{rewind_stream_position, StdfStream};
use crate::stdf_types::{bytes_to_string, CompressType};
#[cfg(feature = "bzip")]
use bzip2::bufread::BzDecoder;
#[cfg(feature = "gzip")]
use flate2::bufread::GzDecoder;
use std::io::{BufRead, BufReader, Seek};
use std::{fs, mem, path::Path, str};

pub struct AtdfReader<R> {
    delimiter: char,
    scale_flag: bool,
    stream: StdfStream<R>,
}

pub struct AtdfRecordIter<'a, R> {
    inner: &'a mut AtdfReader<R>,
    // ATDF record might be divided
    // into multiple lines.
    incomplete_rec: String,
}

// impl

impl AtdfReader<BufReader<fs::File>> {
    #[inline(always)]
    pub fn new<P>(path: P) -> Result<Self, StdfError>
    where
        P: AsRef<Path>,
    {
        // determine the compress type by file extension
        let path_string = path.as_ref().display().to_string();
        let file_ext = path_string.rsplit('.').next();
        let compress_type = match file_ext {
            Some(ext) => match ext {
                #[cfg(feature = "gzip")]
                "gz" => CompressType::GzipCompressed,
                #[cfg(feature = "bzip")]
                "bz2" => CompressType::BzipCompressed,
                _ => CompressType::Uncompressed,
            },
            None => CompressType::Uncompressed,
        };

        let fp = fs::OpenOptions::new().read(true).open(path)?;
        let br = BufReader::with_capacity(2 << 20, fp);
        AtdfReader::from(br, &compress_type)
    }
}

impl<R: BufRead + Seek> AtdfReader<R> {
    #[inline(always)]
    pub fn from(in_stream: R, compress_type: &CompressType) -> Result<Self, StdfError> {
        let mut stream = match compress_type {
            #[cfg(feature = "gzip")]
            CompressType::GzipCompressed => StdfStream::Gz(GzDecoder::new(in_stream)),
            #[cfg(feature = "bzip")]
            CompressType::BzipCompressed => StdfStream::Bz(BzDecoder::new(in_stream)),
            _ => StdfStream::Binary(in_stream),
        };

        let mut far_bytes = vec![];
        stream.read_until(b'\n', &mut far_bytes)?;
        let far_str = bytes_to_string(&far_bytes);
        if !far_str.starts_with("FAR:A") || far_bytes.len() < 9 {
            return Err(StdfError {
                code: 6,
                msg: format!(
                    "FAR record pattern 'FAR:A' not detected or required fields missing, found {}",
                    far_str
                ),
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
            delimiter,
            scale_flag,
            stream,
        })
    }

    #[inline(always)]
    pub fn get_record_iter(&mut self) -> AtdfRecordIter<R> {
        AtdfRecordIter {
            inner: self,
            incomplete_rec: String::new(),
        }
    }
}

// implement of ATDF iterator

impl<R: BufRead + Seek> Iterator for AtdfRecordIter<'_, R> {
    type Item = AtdfRecord;

    #[inline(always)]
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
            // store clean_line to the completed_rec then swap with incomplete_rec
            let mut complete_rec = String::from(clean_line);
            mem::swap(&mut self.incomplete_rec, &mut complete_rec);
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

#[inline(always)]
pub(crate) fn str_trim(input: &str) -> &str {
    let no_pre_space = input.strip_prefix(' ').unwrap_or(input);
    no_pre_space
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap_or(no_pre_space)
}
