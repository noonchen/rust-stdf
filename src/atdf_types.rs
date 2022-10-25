//
// atdf_types.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 26th 2022
// -----
// Last Modified: Wed Oct 26 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use self::atdf_record_field::*;
use crate::{stdf_error::StdfError, stdf_record_type::*, *};
use chrono::NaiveDateTime;
use std::collections::hash_map::HashMap;

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

#[derive(Debug)]
pub struct AtdfRecord {
    rec_name: String,
    type_code: u64,
    scale_flag: bool, // currently not used... maybe in the future
    data_map: HashMap<String, String>,
}

impl From<&AtdfRecord> for StdfRecord {
    fn from(atdf_rec: &AtdfRecord) -> Self {
        //TODO
        if atdf_rec.scale_flag {}
        StdfRecord::new(atdf_rec.type_code)
    }
}

impl AtdfRecord {
    pub fn from_atdf_string(
        atdf_str: &str,
        delim: char,
        scale_flag: bool,
    ) -> Result<Self, StdfError> {
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
            scale_flag,
            data_map,
        })
    }

    pub fn to_atdf_string(&self) -> String {
        let field_name = get_atdf_fields(self.type_code);
        let rec_data = if self.type_code == REC_GDR {
            (0..self.data_map.len())
                .map(|num| num.to_string())
                .map(|num_str| {
                    self.data_map
                        .get(&num_str)
                        .unwrap_or(&String::from(""))
                        .clone()
                })
                .collect::<Vec<String>>()
                .join("|")
        } else {
            field_name
                .iter()
                .map(|&(nam, _b)| self.data_map.get(nam).unwrap_or(&String::from("")).clone())
                .collect::<Vec<String>>()
                .join("|")
        };
        format!("{}:{}", self.rec_name, rec_data)
    }
}

impl From<&StdfRecord> for AtdfRecord {
    /// Records introduced in V4-2007 **CANNOT**
    /// be converted to ATDF.
    ///
    /// If you have ATDF spec for V4-2007, it would
    /// be most helpful for me to dev the full feature.
    fn from(stdf_rec: &StdfRecord) -> Self {
        let type_code;
        let rec_name;
        let atdf_fields: &[(&str, bool)];
        let data_list;

        match stdf_rec {
            // rec type 15
            StdfRecord::PTR(rec) => {
                type_code = REC_PTR;
                rec_name = "PTR".to_string();
                atdf_fields = &PTR_FIELD;
                data_list = atdf_data_from_ptr(rec);
            }
            StdfRecord::MPR(rec) => {
                type_code = REC_MPR;
                rec_name = "MPR".to_string();
                atdf_fields = &MPR_FIELD;
                data_list = atdf_data_from_mpr(rec);
            }
            StdfRecord::FTR(rec) => {
                type_code = REC_FTR;
                rec_name = "FTR".to_string();
                atdf_fields = &FTR_FIELD;
                data_list = atdf_data_from_ftr(rec);
            }
            // StdfRecord::STR(rec) => {
            //     type_code = REC_STR;
            //     rec_name = "STR".to_string();
            //     atdf_fields = &STR_FIELD;
            //     data_list = atdf_data_from_str_rec(rec);
            // }
            // rec type 5
            StdfRecord::PIR(rec) => {
                type_code = REC_PIR;
                rec_name = "PIR".to_string();
                atdf_fields = &PIR_FIELD;
                data_list = atdf_data_from_pir(rec);
            }

            StdfRecord::PRR(rec) => {
                type_code = REC_PRR;
                rec_name = "PRR".to_string();
                atdf_fields = &PRR_FIELD;
                data_list = atdf_data_from_prr(rec);
            }
            // rec type 2
            StdfRecord::WIR(rec) => {
                type_code = REC_WIR;
                rec_name = "WIR".to_string();
                atdf_fields = &WIR_FIELD;
                data_list = atdf_data_from_wir(rec);
            }
            StdfRecord::WRR(rec) => {
                type_code = REC_WRR;
                rec_name = "WRR".to_string();
                atdf_fields = &WRR_FIELD;
                data_list = atdf_data_from_wrr(rec);
            }
            StdfRecord::WCR(rec) => {
                type_code = REC_WCR;
                rec_name = "WCR".to_string();
                atdf_fields = &WCR_FIELD;
                data_list = atdf_data_from_wcr(rec);
            }
            // rec type 50
            StdfRecord::GDR(rec) => {
                type_code = REC_GDR;
                rec_name = "GDR".to_string();
                atdf_fields = &GDR_FIELD;
                data_list = atdf_data_from_gdr(rec);
            }
            StdfRecord::DTR(rec) => {
                type_code = REC_DTR;
                rec_name = "DTR".to_string();
                atdf_fields = &DTR_FIELD;
                data_list = atdf_data_from_dtr(rec);
            }
            // rec type 10
            StdfRecord::TSR(rec) => {
                type_code = REC_TSR;
                rec_name = "TSR".to_string();
                atdf_fields = &TSR_FIELD;
                data_list = atdf_data_from_tsr(rec);
            }
            // rec type 1
            StdfRecord::MIR(rec) => {
                type_code = REC_MIR;
                rec_name = "MIR".to_string();
                atdf_fields = &MIR_FIELD;
                data_list = atdf_data_from_mir(rec);
            }
            StdfRecord::MRR(rec) => {
                type_code = REC_MRR;
                rec_name = "MRR".to_string();
                atdf_fields = &MRR_FIELD;
                data_list = atdf_data_from_mrr(rec);
            }
            StdfRecord::PCR(rec) => {
                type_code = REC_PCR;
                rec_name = "PCR".to_string();
                atdf_fields = &PCR_FIELD;
                data_list = atdf_data_from_pcr(rec);
            }
            StdfRecord::HBR(rec) => {
                type_code = REC_HBR;
                rec_name = "HBR".to_string();
                atdf_fields = &HBR_FIELD;
                data_list = atdf_data_from_hbr(rec);
            }
            StdfRecord::SBR(rec) => {
                type_code = REC_SBR;
                rec_name = "SBR".to_string();
                atdf_fields = &SBR_FIELD;
                data_list = atdf_data_from_sbr(rec);
            }
            StdfRecord::PMR(rec) => {
                type_code = REC_PMR;
                rec_name = "PMR".to_string();
                atdf_fields = &PMR_FIELD;
                data_list = atdf_data_from_pmr(rec);
            }
            StdfRecord::PGR(rec) => {
                type_code = REC_PGR;
                rec_name = "PGR".to_string();
                atdf_fields = &PGR_FIELD;
                data_list = atdf_data_from_pgr(rec);
            }
            StdfRecord::PLR(rec) => {
                type_code = REC_PLR;
                rec_name = "PLR".to_string();
                atdf_fields = &PLR_FIELD;
                data_list = atdf_data_from_plr(rec);
            }
            StdfRecord::RDR(rec) => {
                type_code = REC_RDR;
                rec_name = "RDR".to_string();
                atdf_fields = &RDR_FIELD;
                data_list = atdf_data_from_rdr(rec);
            }
            StdfRecord::SDR(rec) => {
                type_code = REC_SDR;
                rec_name = "SDR".to_string();
                atdf_fields = &SDR_FIELD;
                data_list = atdf_data_from_sdr(rec);
            }
            // StdfRecord::PSR(rec) => {
            //     type_code = REC_PSR;
            //     rec_name = "PSR".to_string();
            //     atdf_fields = &PSR_FIELD;
            //     data_list = atdf_data_from_psr(rec);
            // }
            // StdfRecord::NMR(rec) => {
            //     type_code = REC_NMR;
            //     rec_name = "NMR".to_string();
            //     atdf_fields = &NMR_FIELD;
            //     data_list = atdf_data_from_nmr(rec);
            // }
            // StdfRecord::CNR(rec) => {
            //     type_code = REC_CNR;
            //     rec_name = "CNR".to_string();
            //     atdf_fields = &CNR_FIELD;
            //     data_list = atdf_data_from_cnr(rec);
            // }
            // StdfRecord::SSR(rec) => {
            //     type_code = REC_SSR;
            //     rec_name = "SSR".to_string();
            //     atdf_fields = &SSR_FIELD;
            //     data_list = atdf_data_from_ssr(rec);
            // }
            // StdfRecord::CDR(rec) => {
            //     type_code = REC_CDR;
            //     rec_name = "CDR".to_string();
            //     atdf_fields = &CDR_FIELD;
            //     data_list = atdf_data_from_cdr(rec);
            // }
            // rec type 0
            StdfRecord::FAR(rec) => {
                type_code = REC_FAR;
                rec_name = "FAR".to_string();
                atdf_fields = &FAR_FIELD;
                data_list = atdf_data_from_far(rec);
            }
            StdfRecord::ATR(rec) => {
                type_code = REC_ATR;
                rec_name = "ATR".to_string();
                atdf_fields = &ATR_FIELD;
                data_list = atdf_data_from_atr(rec);
            }
            // StdfRecord::VUR(rec) => {
            //     type_code = REC_VUR;
            //     rec_name = "VUR".to_string();
            //     atdf_fields = &VUR_FIELD;
            //     data_list = atdf_data_from_vur(rec);
            // }
            // rec type 20
            StdfRecord::BPS(rec) => {
                type_code = REC_BPS;
                rec_name = "BPS".to_string();
                atdf_fields = &BPS_FIELD;
                data_list = atdf_data_from_bps(rec);
            }
            StdfRecord::EPS(rec) => {
                type_code = REC_EPS;
                rec_name = "EPS".to_string();
                atdf_fields = &EPS_FIELD;
                data_list = atdf_data_from_eps(rec);
            }
            // rec type 180: Reserved
            // rec type 181: Reserved
            // not matched
            _ => {
                type_code = REC_INVALID;
                rec_name = "INVALID".to_string();
                atdf_fields = &INVALID_FIELD;
                data_list = vec![];
            }
        };

        AtdfRecord {
            rec_name,
            type_code,
            scale_flag: false, // default Unscale
            data_map: if type_code == REC_GDR {
                create_atdf_gdr_map(data_list)
            } else {
                create_atdf_map_from_fields_and_data(atdf_fields, data_list)
            },
        }
    }
}

// ATDF help functions
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

pub(crate) fn get_atdf_fields(rec_type: u64) -> &'static [(&'static str, bool)] {
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

// STDF -> ATDF convertion help functions
// parameter test value will be scaled by default

pub(crate) fn atdf_data_from_ptr(rec: &PTR) -> Vec<String> {
    let test_bits = flag_to_array(&rec.test_flg);
    let parm_bits = flag_to_array(&rec.parm_flg);
    let mut alarm_flags = "".to_string();
    if test_bits[0] == 1 {
        alarm_flags.push('A')
    }
    if test_bits[2] == 1 {
        alarm_flags.push('U')
    }
    if test_bits[3] == 1 {
        alarm_flags.push('T')
    }
    if test_bits[4] == 1 {
        alarm_flags.push('N')
    }
    if test_bits[5] == 1 {
        alarm_flags.push('X')
    }
    if parm_bits[0] == 1 {
        alarm_flags.push('S')
    }
    if parm_bits[1] == 1 {
        alarm_flags.push('D')
    }
    if parm_bits[2] == 1 {
        alarm_flags.push('O')
    }
    if parm_bits[3] == 1 {
        alarm_flags.push('H')
    }
    if parm_bits[4] == 1 {
        alarm_flags.push('L')
    }

    vec![
        rec.test_num.to_string(), //TEST_NUM
        rec.head_num.to_string(), //HEAD_NUM
        rec.site_num.to_string(), //SITE_NUM
        rec.result.to_string(),   //RESULT
        //Pass/Fail, TEST_FLG bits 6 & 7, PARM_FLG bit 5
        if parm_bits[5] == 1 {
            "A".to_string()
        } else if test_bits[6] | test_bits[7] == 0 {
            "P".to_string()
        } else if test_bits[6] == 1 {
            "".to_string()
        } else {
            "F".to_string()
        },
        alarm_flags,          //AlarmFlags
        rec.test_txt.clone(), //TEST_TXT
        rec.alarm_id.clone(), //ALARM_ID
        //LimitCompare
        if parm_bits[6] | parm_bits[7] == 0 {
            "".to_string()
        } else if parm_bits[6] == 1 {
            ">=".to_string()
        } else {
            "<=".to_string()
        },
        rec.units.clone(),        //UNITS
        rec.lo_limit.to_string(), //LO_LIMIT
        rec.hi_limit.to_string(), //HI_LIMIT
        rec.c_resfmt.clone(),     //C_RESFMT
        rec.c_llmfmt.clone(),     //C_LLMFMT
        rec.c_hlmfmt.clone(),     //C_HLMFMT
        rec.lo_spec.to_string(),  //LO_SPEC
        rec.hi_spec.to_string(),  //HI_SPEC
        rec.res_scal.to_string(), //RES_SCAL
        rec.llm_scal.to_string(), //LLM_SCAL
        rec.hlm_scal.to_string(), //HLM_SCAL
    ]
}

pub(crate) fn atdf_data_from_mpr(rec: &MPR) -> Vec<String> {
    let test_bits = flag_to_array(&rec.test_flg);
    let parm_bits = flag_to_array(&rec.parm_flg);
    let mut alarm_flags = "".to_string();
    if test_bits[0] == 1 {
        alarm_flags.push('A')
    }
    if test_bits[2] == 1 {
        alarm_flags.push('U')
    }
    if test_bits[3] == 1 {
        alarm_flags.push('T')
    }
    if test_bits[4] == 1 {
        alarm_flags.push('N')
    }
    if test_bits[5] == 1 {
        alarm_flags.push('X')
    }
    if parm_bits[0] == 1 {
        alarm_flags.push('S')
    }
    if parm_bits[1] == 1 {
        alarm_flags.push('D')
    }
    if parm_bits[2] == 1 {
        alarm_flags.push('O')
    }
    if parm_bits[3] == 1 {
        alarm_flags.push('H')
    }
    if parm_bits[4] == 1 {
        alarm_flags.push('L')
    }

    vec![
        rec.test_num.to_string(),        //TEST_NUM
        rec.head_num.to_string(),        //HEAD_NUM
        rec.site_num.to_string(),        //SITE_NUM
        ser_kx_digit_hex(&rec.rtn_stat), //RTN_STAT
        ser_stdf_kx_data(&rec.rtn_rslt), //RTN_RSLT
        //Pass/Fail, TEST_FLG bits 6 & 7, PARM_FLG bit 5
        if parm_bits[5] == 1 {
            "A".to_string()
        } else if test_bits[6] | test_bits[7] == 0 {
            "P".to_string()
        } else if test_bits[6] == 1 {
            "".to_string()
        } else {
            "F".to_string()
        },
        alarm_flags,          //AlarmFlags
        rec.test_txt.clone(), //TEST_TXT
        rec.alarm_id.clone(), //ALARM_ID
        //LimitCompare
        if parm_bits[6] | parm_bits[7] == 0 {
            "".to_string()
        } else if parm_bits[6] == 1 {
            ">=".to_string()
        } else {
            "<=".to_string()
        },
        rec.units.clone(),               //UNITS
        rec.lo_limit.to_string(),        //LO_LIMIT
        rec.hi_limit.to_string(),        //HI_LIMIT
        rec.start_in.to_string(),        //START_IN
        rec.incr_in.to_string(),         //INCR_IN
        rec.units_in.clone(),            //UNITS_IN
        ser_stdf_kx_data(&rec.rtn_indx), //RTN_INDX
        rec.c_resfmt.clone(),            //C_RESFMT
        rec.c_llmfmt.clone(),            //C_LLMFMT
        rec.c_hlmfmt.clone(),            //C_HLMFMT
        rec.lo_spec.to_string(),         //LO_SPEC
        rec.hi_spec.to_string(),         //HI_SPEC
        rec.res_scal.to_string(),        //RES_SCAL
        rec.llm_scal.to_string(),        //LLM_SCAL
        rec.hlm_scal.to_string(),        //HLM_SCAL
    ]
}

pub(crate) fn atdf_data_from_ftr(rec: &FTR) -> Vec<String> {
    let test_bits = flag_to_array(&rec.test_flg);
    let mut alarm_flags = "".to_string();
    if test_bits[0] == 1 {
        alarm_flags.push('A')
    }
    if test_bits[2] == 1 {
        alarm_flags.push('N')
    }
    if test_bits[3] == 1 {
        alarm_flags.push('T')
    }
    if test_bits[4] == 1 {
        alarm_flags.push('U')
    }
    if test_bits[5] == 1 {
        alarm_flags.push('X')
    }

    vec![
        rec.test_num.to_string(), //TEST_NUM
        rec.head_num.to_string(), //HEAD_NUM
        rec.site_num.to_string(), //SITE_NUM
        //Pass/Fail
        if test_bits[6] | test_bits[7] == 0 {
            "P".to_string()
        } else if test_bits[6] == 1 {
            "".to_string()
        } else {
            "F".to_string()
        },
        alarm_flags,                     //AlarmFlags
        rec.vect_nam.clone(),            //VECT_NAM
        rec.time_set.clone(),            //TIME_SET
        rec.cycl_cnt.to_string(),        //CYCL_CNT
        rec.rel_vadr.to_string(),        //REL_VADR
        rec.rept_cnt.to_string(),        //REPT_CNT
        rec.num_fail.to_string(),        //NUM_FAIL
        rec.xfail_ad.to_string(),        //XFAIL_AD
        rec.yfail_ad.to_string(),        //YFAIL_AD
        rec.vect_off.to_string(),        //VECT_OFF
        ser_stdf_kx_data(&rec.rtn_indx), //RTN_INDX
        ser_kx_digit_hex(&rec.rtn_stat), //RTN_STAT
        ser_stdf_kx_data(&rec.pgm_indx), //PGM_INDX
        ser_kx_digit_hex(&rec.pgm_stat), //PGM_STAT
        ser_stdf_kx_data(&rec.fail_pin), //FAIL_PIN
        rec.op_code.clone(),             //OP_CODE
        rec.test_txt.clone(),            //TEST_TXT
        rec.alarm_id.clone(),            //ALARM_ID
        rec.prog_txt.clone(),            //PROG_TXT
        rec.rslt_txt.clone(),            //RSLT_TXT
        rec.patg_num.to_string(),        //PATG_NUM
        ser_stdf_kx_data(&rec.spin_map), //SPIN_MAP
    ]
}

/// ignored because I do not know ATDF structure in V4-2007
// pub(crate) fn atdf_data_from_str_rec(rec: &STR) -> Vec<String>  {
//     vec![]}

pub(crate) fn atdf_data_from_pir(rec: &PIR) -> Vec<String> {
    vec![
        rec.head_num.to_string(), //HEAD_NUM
        rec.site_num.to_string(), //SITE_NUM
    ]
}

pub(crate) fn atdf_data_from_prr(rec: &PRR) -> Vec<String> {
    let flg_bits = flag_to_array(&rec.part_flg);
    vec![
        rec.head_num.to_string(), //HEAD_NUM
        rec.site_num.to_string(), //SITE_NUM
        rec.part_id.clone(),      //PART_ID
        rec.num_test.to_string(), //NUM_TEST
        //Pass/Fail, bits 3 & 4
        if flg_bits[3] | flg_bits[4] == 0 {
            "P".to_string()
        } else {
            "F".to_string()
        },
        rec.hard_bin.to_string(), //HARD_BIN
        rec.soft_bin.to_string(), //SOFT_BIN
        rec.x_coord.to_string(),  //X_COORD
        rec.y_coord.to_string(),  //Y_COORD
        //RetestCode, bit 0 or 1
        if flg_bits[0] | flg_bits[1] != 0 {
            if flg_bits[0] != 0 {
                "I".to_string()
            } else {
                "C".to_string()
            }
        } else {
            "".to_string()
        },
        //AbortCode
        if flg_bits[2] == 0 {
            "".to_string()
        } else {
            "Y".to_string()
        },
        rec.test_t.to_string(),   //TEST_T
        rec.part_txt.clone(),     //PART_TXT
        ser_bn_dn(&rec.part_fix), //PART_FIX
    ]
}

pub(crate) fn atdf_data_from_wir(rec: &WIR) -> Vec<String> {
    vec![
        rec.head_num.to_string(), //HEAD_NUM
        rec.start_t.to_string(),  //START_T
        rec.site_grp.to_string(), //SITE_GRP
        rec.wafer_id.clone(),     //WAFER_ID
    ]
}

pub(crate) fn atdf_data_from_wrr(rec: &WRR) -> Vec<String> {
    vec![
        rec.head_num.to_string(), //HEAD_NUM
        rec.finish_t.to_string(), //FINISH_T
        rec.part_cnt.to_string(), //PART_CNT
        rec.wafer_id.clone(),     //WAFER_ID
        rec.site_grp.to_string(), //SITE_GRP
        rec.rtst_cnt.to_string(), //RTST_CNT
        rec.abrt_cnt.to_string(), //ABRT_CNT
        rec.good_cnt.to_string(), //GOOD_CNT
        rec.func_cnt.to_string(), //FUNC_CNT
        rec.fabwf_id.clone(),     //FABWF_ID
        rec.frame_id.clone(),     //FRAME_ID
        rec.mask_id.clone(),      //MASK_ID
        rec.usr_desc.clone(),     //USR_DESC
        rec.exc_desc.clone(),     //EXC_DESC
    ]
}

pub(crate) fn atdf_data_from_wcr(rec: &WCR) -> Vec<String> {
    vec![
        rec.wf_flat.to_string(),  //WF_FLAT
        rec.pos_x.to_string(),    //POS_X
        rec.pos_y.to_string(),    //POS_Y
        rec.wafr_siz.to_string(), //WAFR_SIZ
        rec.die_ht.to_string(),   //DIE_HT
        rec.die_wid.to_string(),  //DIE_WID
        rec.wf_units.to_string(), //WF_UNITS
        rec.center_x.to_string(), //CENTER_X
        rec.center_y.to_string(), //CENTER_Y
    ]
}

pub(crate) fn atdf_data_from_gdr(rec: &GDR) -> Vec<String> {
    let mut gen_data_list = vec![];
    for v1_data in &rec.gen_data {
        match v1_data {
            V1::U1(u1) => gen_data_list.push(format!("U{}", u1)),
            V1::U2(u2) => gen_data_list.push(format!("M{}", u2)),
            V1::U4(u4) => gen_data_list.push(format!("B{}", u4)),
            V1::I1(i1) => gen_data_list.push(format!("I{}", i1)),
            V1::I2(i2) => gen_data_list.push(format!("S{}", i2)),
            V1::I4(i4) => gen_data_list.push(format!("L{}", i4)),
            V1::R4(r4) => gen_data_list.push(format!("F{}", r4)),
            V1::R8(r8) => gen_data_list.push(format!("D{}", r8)),
            V1::Cn(cn) => gen_data_list.push(format!("T{}", cn)),
            V1::Bn(bn) => gen_data_list.push(format!("X{}", ser_bn_dn(bn))),
            V1::Dn(dn) => gen_data_list.push(format!("Y{}", ser_bn_dn(dn))),
            V1::N1(n1) => gen_data_list.push(format!("N{}", n1)),
            // No pad bytes in ATDF
            _ => {
                continue;
            }
        }
    }
    gen_data_list
}

pub(crate) fn atdf_data_from_dtr(rec: &DTR) -> Vec<String> {
    vec![
        rec.text_dat.clone(), // TEST_DAT
    ]
}

pub(crate) fn atdf_data_from_tsr(rec: &TSR) -> Vec<String> {
    vec![
        //HEAD_NUM
        if rec.head_num == 255 {
            "".to_string()
        } else {
            rec.head_num.to_string()
        },
        //SITE_NUM
        if rec.site_num == 255 {
            "".to_string()
        } else {
            rec.site_num.to_string()
        },
        rec.test_num.to_string(), //TEST_NUM
        rec.test_nam.clone(),     //TEST_NAM
        rec.test_typ.to_string(), //TEST_TYP
        rec.exec_cnt.to_string(), //EXEC_CNT
        rec.fail_cnt.to_string(), //FAIL_CNT
        rec.alrm_cnt.to_string(), //ALRM_CNT
        rec.seq_name.clone(),     //SEQ_NAME
        rec.test_lbl.clone(),     //TEST_LBL
        rec.test_tim.to_string(), //TEST_TIM
        rec.test_min.to_string(), //TEST_MIN
        rec.test_max.to_string(), //TEST_MAX
        rec.tst_sums.to_string(), //TST_SUMS
        rec.tst_sqrs.to_string(), //TST_SQRS
    ]
}

pub(crate) fn atdf_data_from_mir(rec: &MIR) -> Vec<String> {
    vec![
        rec.lot_id.clone(),   //LOT_ID
        rec.part_typ.clone(), //PART_TYP
        rec.job_nam.clone(),  //JOB_NAM
        rec.node_nam.clone(), //NODE_NAM
        rec.tstr_typ.clone(), //TSTR_TYP
        NaiveDateTime::from_timestamp(rec.setup_t as i64, 0)
            .format("%H:%M:%S %d-%b-%Y")
            .to_string(), //SETUP_T
        NaiveDateTime::from_timestamp(rec.start_t as i64, 0)
            .format("%H:%M:%S %d-%b-%Y")
            .to_string(), //START_T
        rec.oper_nam.clone(), //OPER_NAM
        rec.mode_cod.to_string(), //MODE_COD
        rec.stat_num.to_string(), //STAT_NUM
        rec.sblot_id.clone(), //SBLOT_ID
        rec.test_cod.clone(), //TEST_COD
        rec.rtst_cod.to_string(), //RTST_COD
        rec.job_rev.clone(),  //JOB_REV
        rec.exec_typ.clone(), //EXEC_TYP
        rec.exec_ver.clone(), //EXEC_VER
        rec.prot_cod.to_string(), //PROT_COD
        rec.cmod_cod.to_string(), //CMOD_COD
        rec.burn_tim.to_string(), //BURN_TIM
        rec.tst_temp.clone(), //TST_TEMP
        rec.user_txt.clone(), //USER_TXT
        rec.aux_file.clone(), //AUX_FILE
        rec.pkg_typ.clone(),  //PKG_TYP
        rec.famly_id.clone(), //FAMLY_ID
        rec.date_cod.clone(), //DATE_COD
        rec.facil_id.clone(), //FACIL_ID
        rec.floor_id.clone(), //FLOOR_ID
        rec.proc_id.clone(),  //PROC_ID
        rec.oper_frq.clone(), //OPER_FRQ
        rec.spec_nam.clone(), //SPEC_NAM
        rec.spec_ver.clone(), //SPEC_VER
        rec.flow_id.clone(),  //FLOW_ID
        rec.setup_id.clone(), //SETUP_ID
        rec.dsgn_rev.clone(), //DSGN_REV
        rec.eng_id.clone(),   //ENG_ID
        rec.rom_cod.clone(),  //ROM_COD
        rec.serl_num.clone(), //SERL_NUM
        rec.supr_nam.clone(), //SUPR_NAM
    ]
}

pub(crate) fn atdf_data_from_mrr(rec: &MRR) -> Vec<String> {
    vec![
        NaiveDateTime::from_timestamp(rec.finish_t as i64, 0)
            .format("%H:%M:%S %d-%b-%Y")
            .to_string(), //FINISH_T
        rec.disp_cod.to_string(), //DISP_COD
        rec.usr_desc.clone(),     //USR_DESC
        rec.exc_desc.clone(),     //EXC_DESC
    ]
}

pub(crate) fn atdf_data_from_pcr(rec: &PCR) -> Vec<String> {
    vec![
        if rec.head_num == 255 {
            "".to_string()
        } else {
            rec.head_num.to_string()
        }, //HEAD_NUM
        if rec.site_num == 255 {
            "".to_string()
        } else {
            rec.site_num.to_string()
        }, //SITE_NUM
        rec.part_cnt.to_string(), //PART_CNT
        rec.rtst_cnt.to_string(), //RTST_CNT
        rec.abrt_cnt.to_string(), //ABRT_CNT
        rec.good_cnt.to_string(), //GOOD_CNT
        rec.func_cnt.to_string(), //FUNC_CNT
    ]
}

pub(crate) fn atdf_data_from_hbr(rec: &HBR) -> Vec<String> {
    vec![
        if rec.head_num == 255 {
            "".to_string()
        } else {
            rec.head_num.to_string()
        }, //HEAD_NUM
        if rec.site_num == 255 {
            "".to_string()
        } else {
            rec.site_num.to_string()
        }, //SITE_NUM
        rec.hbin_num.to_string(), //HBIN_NUM
        rec.hbin_cnt.to_string(), //HBIN_CNT
        rec.hbin_pf.to_string(),  //HBIN_PF
        rec.hbin_nam.clone(),     //HBIN_NAM
    ]
}

pub(crate) fn atdf_data_from_sbr(rec: &SBR) -> Vec<String> {
    vec![
        if rec.head_num == 255 {
            "".to_string()
        } else {
            rec.head_num.to_string()
        }, //HEAD_NUM
        if rec.site_num == 255 {
            "".to_string()
        } else {
            rec.site_num.to_string()
        }, //SITE_NUM
        rec.sbin_num.to_string(), //SBIN_NUM
        rec.sbin_cnt.to_string(), //SBIN_CNT
        rec.sbin_pf.to_string(),  //SBIN_PF
        rec.sbin_nam.clone(),     //SBIN_NAM
    ]
}

pub(crate) fn atdf_data_from_pmr(rec: &PMR) -> Vec<String> {
    vec![
        rec.pmr_indx.to_string(), //PMR_INDX
        rec.chan_typ.to_string(), //CHAN_TYP
        rec.chan_nam.clone(),     //CHAN_NAM
        rec.phy_nam.clone(),      //PHY_NAM
        rec.log_nam.clone(),      //LOG_NAM
        rec.head_num.to_string(), //HEAD_NUM
        rec.site_num.to_string(), //SITE_NUM
    ]
}

pub(crate) fn atdf_data_from_pgr(rec: &PGR) -> Vec<String> {
    vec![
        rec.grp_indx.to_string(),        //GRP_INDX
        rec.grp_nam.clone(),             //GRP_NAM
        ser_stdf_kx_data(&rec.pmr_indx), //PMR_INDX
    ]
}

pub(crate) fn atdf_data_from_plr(rec: &PLR) -> Vec<String> {
    // convert radx to ASCII symbol
    let radx_func = |x: u8| match x {
        2 => "B",
        8 => "O",
        10 => "D",
        16 => "H",
        20 => "S",
        _ => "",
    };
    let grp_mode = rec
        .grp_mode
        .iter()
        .map(|&x| format!("{:X}", x))
        .collect::<Vec<String>>()
        .join(",");
    let grp_radx = rec
        .grp_radx
        .iter()
        .map(|&x| radx_func(x).to_string())
        .collect::<Vec<String>>()
        .join(",");

    // check if string length is > 0
    fn not_empty(v: &[String]) -> bool {
        v.iter().map(|s| s.len()).fold(0, std::cmp::max) > 0
    }

    fn combine_l_r(cha_l: &[String], cha_r: &[String]) -> String {
        let chal_not_empty = not_empty(cha_l);
        let char_not_empty = not_empty(cha_r);

        if chal_not_empty | char_not_empty {
            let mut state_list = vec![];
            // if cha_l is not empty, two characters will be in `state`
            if chal_not_empty {
                // e.g. l = ["1111", "2222"], r = ["3333", "4444"]
                for (string_l, string_r) in cha_l.iter().zip(cha_r.iter()) {
                    // e.g. "1111", "2222"
                    let mut tmp_pin_state = "".to_string();
                    for (ind, (c_l, c_r)) in string_l.chars().zip(string_r.chars()).enumerate() {
                        // e.g. '1', '2'
                        if ind != 0 {
                            tmp_pin_state.push(',');
                        }
                        tmp_pin_state.push(c_l);
                        tmp_pin_state.push(c_r);
                    }
                    // e.g. "12,12,12,12"
                    state_list.push(tmp_pin_state);
                }
            } else {
                // only cha_r
                cha_r
                    .iter()
                    .map(|s| {
                        s.chars()
                            .map(|c| c.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    })
                    .map(|pin_state| state_list.push(pin_state))
                    .count();
            }
            state_list.join("/")
        } else {
            "".to_string()
        }
    }

    vec![
        ser_stdf_kx_data(&rec.grp_indx),           // ("GRP_INDX", true),
        grp_mode,                                  // ("GRP_MODE", false),
        grp_radx,                                  // ("GRP_RADX", false),
        combine_l_r(&rec.pgm_chal, &rec.pgm_char), // ("PGM_CHAL,PGM_CHAR", false),
        combine_l_r(&rec.rtn_chal, &rec.rtn_char), // ("RTN_CHAL,RTN_CHAR", false),
    ]
}

pub(crate) fn atdf_data_from_rdr(rec: &RDR) -> Vec<String> {
    vec![
        ser_stdf_kx_data(&rec.rtst_bin), //RTST_BIN
    ]
}

pub(crate) fn atdf_data_from_sdr(rec: &SDR) -> Vec<String> {
    vec![
        rec.head_num.to_string(),        //HEAD_NUM
        rec.site_grp.to_string(),        //SITE_GRP
        ser_stdf_kx_data(&rec.site_num), //SITE_NUM
        rec.hand_typ.clone(),            //HAND_TYP
        rec.hand_id.clone(),             //HAND_ID
        rec.card_typ.clone(),            //CARD_TYP
        rec.card_id.clone(),             //CARD_ID
        rec.load_typ.clone(),            //LOAD_TYP
        rec.load_id.clone(),             //LOAD_ID
        rec.dib_typ.clone(),             //DIB_TYP
        rec.dib_id.clone(),              //DIB_ID
        rec.cabl_typ.clone(),            //CABL_TYP
        rec.cabl_id.clone(),             //CABL_ID
        rec.cont_typ.clone(),            //CONT_TYP
        rec.cont_id.clone(),             //CONT_ID
        rec.lasr_typ.clone(),            //LASR_TYP
        rec.lasr_id.clone(),             //LASR_ID
        rec.extr_typ.clone(),            //EXTR_TYP
        rec.extr_id.clone(),             //EXTR_ID
    ]
}

// pub(crate) fn atdf_data_from_psr(_rec: &PSR) -> Vec<String>  {vec![]}
// pub(crate) fn atdf_data_from_nmr(_rec: &NMR) -> Vec<String>  {vec![]}
// pub(crate) fn atdf_data_from_cnr(_rec: &CNR) -> Vec<String>  {vec![]}
// pub(crate) fn atdf_data_from_ssr(_rec: &SSR) -> Vec<String>  {vec![]}
// pub(crate) fn atdf_data_from_cdr(_rec: &CDR) -> Vec<String>  {vec![]}

pub(crate) fn atdf_data_from_far(rec: &FAR) -> Vec<String> {
    vec![
        "A".to_string(),          // File type, ATDF
        rec.stdf_ver.to_string(), // STDF Version
        "2".to_string(),          // ATDF Version
        "U".to_string(),          // Unscale
    ]
}

pub(crate) fn atdf_data_from_atr(rec: &ATR) -> Vec<String> {
    vec![
        NaiveDateTime::from_timestamp(rec.mod_tim as i64, 0)
            .format("%H:%M:%S %d-%b-%Y")
            .to_string(), // MOD_TIM
        rec.cmd_line.clone(), // CMD_LINE
    ]
}

// pub(crate) fn atdf_data_from_vur(_rec: &VUR) -> Vec<String>  {vec![]}

pub(crate) fn atdf_data_from_bps(rec: &BPS) -> Vec<String> {
    vec![
        rec.seq_name.clone(), // SEQ_NAME
    ]
}

pub(crate) fn atdf_data_from_eps(_rec: &EPS) -> Vec<String> {
    vec![]
}

/// generate ATDF hashmap for records ***other than GDR***
fn create_atdf_map_from_fields_and_data(
    fields: &[(&str, bool)],
    data_list: Vec<String>,
) -> HashMap<String, String> {
    fields
        .iter()
        .zip(data_list)
        .map(|(&(fname, _), d)| (fname.to_string(), d))
        .collect::<HashMap<String, String>>()
}

/// generate ATDF hashmap for GDR record
fn create_atdf_gdr_map(data_list: Vec<String>) -> HashMap<String, String> {
    (0..data_list.len())
        .zip(data_list)
        .map(|(num, d)| (num.to_string(), d))
        .collect::<HashMap<String, String>>()
}

/// serialize STDF kx type data to String
fn ser_stdf_kx_data<T: ToString>(kx: &[T]) -> String {
    kx.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/// serialize vector of u8 to hex digit String
fn ser_kx_digit_hex(kx: &[u8]) -> String {
    kx.iter()
        .map(|&x| format!("{:X}", x))
        .collect::<Vec<String>>()
        .join(",")
}

/// convert a 1 byte STDF flag to vector of u8
fn flag_to_array(flag: &[u8; 1]) -> Vec<u8> {
    let mut flag = flag[0];
    let mut bits = Vec::with_capacity(8);
    for _ in 0..8 {
        bits.push(flag & 1u8);
        flag >>= 1;
    }
    bits
}

/// serialize bit data
fn ser_bn_dn(d: &[u8]) -> String {
    hex::encode_upper(d)
}
