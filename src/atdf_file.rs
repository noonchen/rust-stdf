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
use crate::stdf_record_type::*;
use crate::stdf_types::{CompressType, StdfRecord};
use crate::stdf_error::StdfError;
use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use std::collections::hash_map::HashMap;
use std::{fs, str};
use std::io::BufReader;


pub struct AtdfReader {
    pub file_path: String,
    delimiter: char,
    scale_flag: bool,
    stream: StreamT,
}

#[derive(Debug)]
pub struct AtdfRecord {
    rec_name: String,
    type_code: u64,
    data_map: HashMap<usize, String>,
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

fn get_rec_code(rec_name: &str) -> u64 {
    match rec_name {
        "FAR" => REC_FAR,
        "ATR" => REC_ATR,
        "VUR" => REC_VUR,
        "MIR" => REC_MIR,
        "MRR" => REC_MRR,
        "PCR" => REC_PCR,
        "HBR" => REC_HBR,
        "SBR" => REC_SBR,
        "PMR" => REC_PMR,
        "PGR" => REC_PGR,
        "PLR" => REC_PLR,
        "RDR" => REC_RDR,
        "SDR" => REC_SDR,
        "PSR" => REC_PSR,
        "NMR" => REC_NMR,
        "CNR" => REC_CNR,
        "SSR" => REC_SSR,
        "CDR" => REC_CDR,
        "WIR" => REC_WIR,
        "WRR" => REC_WRR,
        "WCR" => REC_WCR,
        "PIR" => REC_PIR,
        "PRR" => REC_PRR,
        "TSR" => REC_TSR,
        "PTR" => REC_PTR,
        "MPR" => REC_MPR,
        "FTR" => REC_FTR,
        "STR" => REC_STR,
        "BPS" => REC_BPS,
        "EPS" => REC_EPS,
        "GDR" => REC_GDR,
        "DTR" => REC_DTR,
        _ => REC_INVALID,
    }
}

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

            if tmp_line.len() > 0 && tmp_line.starts_with(" ") {
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
            if !eof && clean_line.len() == 0 {
                continue;
            }

            // a possible new rec found! or EOF reached
            // clone the completed_rec for processing
            let complete_rec = self.incomplete_rec.clone();
            // store clean_line to incomplete_rec
            self.incomplete_rec = String::from(clean_line);
            // if previous incomplete_rec is empty && EOF, we should stop
            if eof && complete_rec.len() == 0 {
                return None;
            } else if complete_rec.len() == 0 {
                // not eof, but not content in complete_rec
                // happens in the beginning
                continue;
            }

            // do some ATDF syntax checking here, start parsing ATDF rec
            let (rec_name, rec_data) = complete_rec.split_once(":").unwrap_or(("", &complete_rec));
            let type_code = get_rec_code(rec_name);
            if type_code == REC_INVALID {
                println!("Unrecognized record name {}, remaining data {}", rec_name, rec_data);
                return None;
            }
            let data_map: HashMap<usize, String> = rec_data.split(self.inner.delimiter).enumerate().map(|(i, s)| (i, s.to_string())).collect();
            // send...
            return Some(AtdfRecord { 
                rec_name: rec_name.to_string(), 
                type_code, 
                data_map
            });
        }
    }
}
