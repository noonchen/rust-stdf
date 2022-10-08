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

use self::atdf_record_field::*;
use crate::stdf_error::StdfError;
use crate::stdf_file::{rewind_stream_position, StdfStream, StreamT};
use crate::stdf_record_type::*;
use crate::stdf_types::{CompressType, StdfRecord};
use bzip2::bufread::BzDecoder;
use flate2::bufread::GzDecoder;
use std::collections::hash_map::HashMap;
use std::io::BufReader;
use std::{fs, str};

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
    data_map: HashMap<String, String>,
}

pub struct AtdfRecordIter<'a> {
    inner: &'a mut AtdfReader,
    // ATDF record might be divided
    // into multiple lines.
    incomplete_rec: String,
}

pub(crate) mod atdf_record_field {
    // ATDF fields may not map to STDF fields
    // (ATDF field name, is required? or must presented)
    pub(crate) const FAR_FIELD: [(&str, bool); 4] = [
        ("FileType", true),
        ("STDF_VER", true),
        ("ATDFVer", true),
        ("ScaleFlag", false),
    ];

    pub(crate) const ATR_FIELD: [(&str, bool); 2] = [("MOD_TIM", false), ("CMD_LINE", false)];

    pub(crate) const MIR_FIELD: [(&str, bool); 38] = [
        ("LOT_ID", true),
        ("PART_TYP", true),
        ("JOB_NAM", true),
        ("NODE_NAM", true),
        ("TSTR_TYP", true),
        ("SETUP_T", true),
        ("START_T", true),
        ("OPER_NAM", true),
        ("MODE_COD", true),
        ("STAT_NUM", true),
        ("SBLOT_ID", false),
        ("TEST_COD", false),
        ("RTST_COD", false),
        ("JOB_REV", false),
        ("EXEC_TYP", false),
        ("EXEC_VER", false),
        ("PROT_COD", false),
        ("CMOD_COD", false),
        ("BURN_TIM", false),
        ("TST_TEMP", false),
        ("USER_TXT", false),
        ("AUX_FILE", false),
        ("PKG_TYP", false),
        ("FAMLY_ID", false),
        ("DATE_COD", false),
        ("FACIL_ID", false),
        ("FLOOR_ID", false),
        ("PROC_ID", false),
        ("OPER_FRQ", false),
        ("SPEC_NAM", false),
        ("SPEC_VER", false),
        ("FLOW_ID", false),
        ("SETUP_ID", false),
        ("DSGN_REV", false),
        ("ENG_ID", false),
        ("ROM_COD", false),
        ("SERL_NUM", false),
        ("SUPR_NAM", false),
    ];

    pub(crate) const MRR_FIELD: [(&str, bool); 4] = [
        ("FINISH_T", true),
        ("DISP_COD", false),
        ("USR_DESC", false),
        ("EXC_DESC", false),
    ];

    pub(crate) const PCR_FIELD: [(&str, bool); 7] = [
        ("HEAD_NUM", true),
        ("SITE_NUM", true),
        ("PART_CNT", false),
        ("RTST_CNT", false),
        ("ABRT_CNT", false),
        ("GOOD_CNT", false),
        ("FUNC_CNT", false),
    ];

    pub(crate) const HBR_FIELD: [(&str, bool); 6] = [
        ("HEAD_NUM", true),
        ("SITE_NUM", true),
        ("HBIN_NUM", true),
        ("HBIN_CNT", true),
        ("HBIN_PF", false),
        ("HBIN_NAM", false),
    ];

    pub(crate) const SBR_FIELD: [(&str, bool); 6] = [
        ("HEAD_NUM", true),
        ("SITE_NUM", true),
        ("SBIN_NUM", true),
        ("SBIN_CNT", true),
        ("SBIN_PF", false),
        ("SBIN_NAM", false),
    ];

    pub(crate) const PMR_FIELD: [(&str, bool); 7] = [
        ("PMR_INDX", true),
        ("CHAN_TYP", false),
        ("CHAN_NAM", false),
        ("PHY_NAM", false),
        ("LOG_NAM", false),
        ("HEAD_NUM", false),
        ("SITE_NUM", false),
    ];

    pub(crate) const PGR_FIELD: [(&str, bool); 3] =
        [("GRP_INDX", true), ("GRP_NAM", false), ("PMR_INDX", false)];

    pub(crate) const PLR_FIELD: [(&str, bool); 5] = [
        ("GRP_INDX", true),
        ("GRP_MODE", false),
        ("GRP_RADX", false),
        ("PGM_CHAL,PGM_CHAR", false),
        ("RTN_CHAL,RTN_CHAR", false),
    ];

    //required by spec, but it could be missing thou
    pub(crate) const RDR_FIELD: [(&str, bool); 1] = [("RTST_BIN", false)];

    pub(crate) const SDR_FIELD: [(&str, bool); 19] = [
        ("HEAD_NUM", true),
        ("SITE_GRP", true),
        ("SITE_NUM", true),
        ("HAND_TYP", false),
        ("HAND_ID", false),
        ("CARD_TYP", false),
        ("CARD_ID", false),
        ("LOAD_TYP", false),
        ("LOAD_ID", false),
        ("DIB_TYP", false),
        ("DIB_ID", false),
        ("CABL_TYP", false),
        ("CABL_ID", false),
        ("CONT_TYP", false),
        ("CONT_ID", false),
        ("LASR_TYP", false),
        ("LASR_ID", false),
        ("EXTR_TYP", false),
        ("EXTR_ID", false),
    ];

    pub(crate) const WIR_FIELD: [(&str, bool); 4] = [
        ("HEAD_NUM", true),
        ("START_T", true),
        ("SITE_GRP", false),
        ("WAFER_ID", false),
    ];

    pub(crate) const WRR_FIELD: [(&str, bool); 14] = [
        ("HEAD_NUM", true),
        ("FINISH_T", true),
        ("PART_CNT", true),
        ("WAFER_ID", false),
        ("SITE_GRP", false),
        ("RTST_CNT", false),
        ("ABRT_CNT", false),
        ("GOOD_CNT", false),
        ("FUNC_CNT", false),
        ("FABWF_ID", false),
        ("FRAME_ID", false),
        ("MASK_ID", false),
        ("USR_DESC", false),
        ("EXC_DESC", false),
    ];

    pub(crate) const WCR_FIELD: [(&str, bool); 9] = [
        ("WF_FLAT", false),
        ("POS_X", false),
        ("POS_Y", false),
        ("WAFR_SIZ", false),
        ("DIE_HT", false),
        ("DIE_WID", false),
        ("WF_UNITS", false),
        ("CENTER_X", false),
        ("CENTER_Y", false),
    ];

    pub(crate) const PIR_FIELD: [(&str, bool); 2] = [("HEAD_NUM", true), ("SITE_NUM", true)];

    pub(crate) const PRR_FIELD: [(&str, bool); 14] = [
        ("HEAD_NUM", true),
        ("SITE_NUM", true),
        ("PART_ID", true),
        ("NUM_TEST", true),
        ("Pass/Fail", true),
        ("HARD_BIN", true),
        ("SOFT_BIN", false),
        ("X_COORD", false),
        ("Y_COORD", false),
        ("RetestCode", false),
        ("AbortCode", false),
        ("TEST_T", false),
        ("PART_TXT", false),
        ("PART_FIX", false),
    ];

    pub(crate) const TSR_FIELD: [(&str, bool); 15] = [
        ("HEAD_NUM", true),
        ("SITE_NUM", true),
        ("TEST_NUM", true),
        ("TEST_NAM", false),
        ("TEST_TYP", false),
        ("EXEC_CNT", false),
        ("FAIL_CNT", false),
        ("ALRM_CNT", false),
        ("SEQ_NAME", false),
        ("TEST_LBL", false),
        ("TEST_TIM", false),
        ("TEST_MIN", false),
        ("TEST_MAX", false),
        ("TST_SUMS", false),
        ("TST_SQRS", false),
    ];

    pub(crate) const PTR_FIELD: [(&str, bool); 20] = [
        ("TEST_NUM", true),
        ("HEAD_NUM", true),
        ("SITE_NUM", true),
        ("RESULT", false),
        ("Pass/Fail", false),
        ("AlarmFlags", false),
        ("TEST_TXT", false),
        ("ALARM_ID", false),
        ("LimitCompare", false),
        ("UNITS", false),
        ("LO_LIMIT", false),
        ("HI_LIMIT", false),
        ("C_RESFMT", false),
        ("C_LLMFMT", false),
        ("C_HLMFMT", false),
        ("LO_SPEC", false),
        ("HI_SPEC", false),
        ("RES_SCAL", false),
        ("LLM_SCAL", false),
        ("HLM_SCAL", false),
    ];

    pub(crate) const MPR_FIELD: [(&str, bool); 25] = [
        ("TEST_NUM", true),
        ("HEAD_NUM", true),
        ("SITE_NUM", true),
        ("RTN_STAT", false),
        ("RTN_RSLT", false),
        ("Pass/Fail", false),
        ("AlarmFlags", false),
        ("TEST_TXT", false),
        ("ALARM_ID", false),
        ("LimitCompare", false),
        ("UNITS", false),
        ("LO_LIMIT", false),
        ("HI_LIMIT", false),
        ("START_IN", false),
        ("INCR_IN", false),
        ("UNITS_IN", false),
        ("RTN_INDX", false),
        ("C_RESFMT", false),
        ("C_LLMFMT", false),
        ("C_HLMFMT", false),
        ("LO_SPEC", false),
        ("HI_SPEC", false),
        ("RES_SCAL", false),
        ("LLM_SCAL", false),
        ("HLM_SCAL", false),
    ];

    pub(crate) const FTR_FIELD: [(&str, bool); 26] = [
        ("TEST_NUM", true),
        ("HEAD_NUM", true),
        ("SITE_NUM", true),
        ("Pass/Fail", true),
        ("AlarmFlags", false),
        ("VECT_NAM", false),
        ("TIME_SET", false),
        ("CYCL_CNT", false),
        ("REL_VADR", false),
        ("REPT_CNT", false),
        ("NUM_FAIL", false),
        ("XFAIL_AD", false),
        ("YFAIL_AD", false),
        ("VECT_OFF", false),
        ("RTN_INDX", false),
        ("RTN_STAT", false),
        ("PGM_INDX", false),
        ("PGM_STAT", false),
        ("FAIL_PIN", false),
        ("OP_CODE", false),
        ("TEST_TXT", false),
        ("ALARM_ID", false),
        ("PROG_TXT", false),
        ("RSLT_TXT", false),
        ("PATG_NUM", false),
        ("SPIN_MAP", false),
    ];

    pub(crate) const BPS_FIELD: [(&str, bool); 1] = [("SEQ_NAME", false)];
    pub(crate) const EPS_FIELD: [(&str, bool); 0] = [];
    // GDR is a special case, there is only GEN_DATA, however it's data is delimited by | symbol
    pub(crate) const GDR_FIELD: [(&str, bool); 0] = [];

    pub(crate) const DTR_FIELD: [(&str, bool); 1] = [("TEST_DAT", false)];

    pub(crate) const INVALID_FIELD: [(&str, bool); 0] = [];
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

impl AtdfRecord {
    pub fn from_atdf_string(atdf_str: &str, delim: char) -> Result<Self, StdfError> {
        // do some ATDF syntax checking here, start parsing ATDF rec
        let (rec_name, rec_data) = atdf_str.split_once(':').unwrap_or(("", atdf_str));
        let type_code = get_code_from_rec_name(rec_name);
        if type_code == REC_INVALID {
            return Err(StdfError {
                code: 2,
                msg: format!(
                    "Unrecognized record name {}, remaining data {}",
                    rec_name, rec_data
                ),
            });
        }
        // map data to each atdf fields, use empty string as default field data
        let field_data: Vec<&str> = rec_data.split(delim).collect();
        let field_name = get_atdf_fields(type_code);
        // check required fields exist
        if field_data.len() < count_reqired(field_name) {
            return Err(StdfError {
                code: 2,
                msg: format!(
                    "{} record has {} required fields, only {} found in {:?}",
                    rec_name,
                    count_reqired(field_name),
                    field_data.len(),
                    field_data
                ),
            });
        }
        let data_map = if type_code == REC_GDR {
            // GDR is a special case, data is split with delimiter
            (0..field_data.len())
                .zip(field_data)
                .map(|(i, d)| (i.to_string(), d.to_string()))
                .collect()
        } else {
            field_name
                .iter()
                .enumerate()
                .map(|(i, &(fname, _))| {
                    (
                        fname.to_string(),
                        field_data.get(i).unwrap_or(&"").to_string(),
                    )
                })
                .collect()
        };
        Ok(AtdfRecord {
            rec_name: rec_name.to_string(),
            type_code,
            data_map,
        })
    }

    pub fn to_atdf_string(&self) -> String {
        let field_name = get_atdf_fields(self.type_code);
        let rec_data = field_name
            .iter()
            .map(|&(nam, _b)| self.data_map.get(nam).unwrap_or(&String::from("")).clone())
            .collect::<Vec<String>>()
            .join("|");
        format!("{}:{}", self.rec_name, rec_data)
    }
}

// help functions
pub(crate) fn str_trim(input: &str) -> &str {
    let no_pre_space = input.strip_prefix(' ').unwrap_or(input);
    no_pre_space
        .strip_suffix("\r\n")
        .or_else(|| input.strip_suffix('\n'))
        .unwrap_or(no_pre_space)
}

/// This function convert record name string to
/// record type constant during ATDF parsing
///
/// *currently not support V4-2007*
pub(crate) fn get_code_from_rec_name(rec_name: &str) -> u64 {
    match rec_name {
        "FAR" => REC_FAR,
        "ATR" => REC_ATR,
        // "VUR" => REC_VUR,
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
        // "PSR" => REC_PSR,
        // "NMR" => REC_NMR,
        // "CNR" => REC_CNR,
        // "SSR" => REC_SSR,
        // "CDR" => REC_CDR,
        "WIR" => REC_WIR,
        "WRR" => REC_WRR,
        "WCR" => REC_WCR,
        "PIR" => REC_PIR,
        "PRR" => REC_PRR,
        "TSR" => REC_TSR,
        "PTR" => REC_PTR,
        "MPR" => REC_MPR,
        "FTR" => REC_FTR,
        // "STR" => REC_STR,
        "BPS" => REC_BPS,
        "EPS" => REC_EPS,
        "GDR" => REC_GDR,
        "DTR" => REC_DTR,
        _ => REC_INVALID,
    }
}

/// This function convert record type constant to
/// record name string during ATDF convertion
///
/// *currently not support V4-2007*
// pub(crate) fn get_rec_name_from_code(rec_type: u64) -> &'static str {
//     match rec_type {
//         // rec type 15
//         REC_PTR => "PTR",
//         REC_MPR => "MPR",
//         REC_FTR => "FTR",
//         // REC_STR => "STR",
//         // rec type 5
//         REC_PIR => "PIR",
//         REC_PRR => "PRR",
//         // rec type 2
//         REC_WIR => "WIR",
//         REC_WRR => "WRR",
//         REC_WCR => "WCR",
//         // rec type 50
//         REC_GDR => "GDR",
//         REC_DTR => "DTR",
//         // rec type 0
//         REC_FAR => "FAR",
//         REC_ATR => "ATR",
//         // REC_VUR => "VUR",
//         // rec type 1
//         REC_MIR => "MIR",
//         REC_MRR => "MRR",
//         REC_PCR => "PCR",
//         REC_HBR => "HBR",
//         REC_SBR => "SBR",
//         REC_PMR => "PMR",
//         REC_PGR => "PGR",
//         REC_PLR => "PLR",
//         REC_RDR => "RDR",
//         REC_SDR => "SDR",
//         // REC_PSR => "PSR",
//         // REC_NMR => "NMR",
//         // REC_CNR => "CNR",
//         // REC_SSR => "SSR",
//         // REC_CDR => "CDR",
//         // rec type 10
//         REC_TSR => "TSR",
//         // rec type 20
//         REC_BPS => "BPS",
//         REC_EPS => "EPS",
//         // rec type 180: Reserved
//         // rec type 181: Reserved
//         // REC_RESERVE => "ReservedRec",
//         // not matched
//         _ => "InvalidRec",
//     }
// }

fn get_atdf_fields(rec_type: u64) -> &'static [(&'static str, bool)] {
    match rec_type {
        REC_FAR => &FAR_FIELD,
        REC_ATR => &ATR_FIELD,
        // REC_VUR => &VUR_FIELD,
        REC_MIR => &MIR_FIELD,
        REC_MRR => &MRR_FIELD,
        REC_PCR => &PCR_FIELD,
        REC_HBR => &HBR_FIELD,
        REC_SBR => &SBR_FIELD,
        REC_PMR => &PMR_FIELD,
        REC_PGR => &PGR_FIELD,
        REC_PLR => &PLR_FIELD,
        REC_RDR => &RDR_FIELD,
        REC_SDR => &SDR_FIELD,
        // REC_PSR => &PSR,
        // REC_NMR => &NMR,
        // REC_CNR => &CNR,
        // REC_SSR => &SSR,
        // REC_CDR => &CDR,
        REC_WIR => &WIR_FIELD,
        REC_WRR => &WRR_FIELD,
        REC_WCR => &WCR_FIELD,
        REC_PIR => &PIR_FIELD,
        REC_PRR => &PRR_FIELD,
        REC_TSR => &TSR_FIELD,
        REC_PTR => &PTR_FIELD,
        REC_MPR => &MPR_FIELD,
        REC_FTR => &FTR_FIELD,
        // REC_STR => &STR_FIELD,
        REC_BPS => &BPS_FIELD,
        REC_EPS => &EPS_FIELD,
        REC_GDR => &GDR_FIELD,
        REC_DTR => &DTR_FIELD,
        _ => &INVALID_FIELD,
    }
}

pub(crate) fn count_reqired(p_arr: &[(&str, bool)]) -> usize {
    p_arr
        .iter()
        .fold(0, |cnt: usize, (_, b)| cnt + (*b as usize))
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
            return match AtdfRecord::from_atdf_string(&complete_rec, self.inner.delimiter) {
                Ok(atdf_rec) => Some(atdf_rec),
                Err(e) => {
                    println!("{}", e);
                    None
                }
            };
        }
    }
}
