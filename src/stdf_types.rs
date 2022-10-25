//
// stdf_types.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Tue Oct 25 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use self::{atdf_record_field::*, stdf_record_type::*};
use crate::stdf_error::StdfError;
use chrono::NaiveDateTime;
extern crate smart_default;
use smart_default::SmartDefault;
use std::{collections::hash_map::HashMap, convert::From};

// Common Type
#[derive(Debug)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

pub(crate) enum CompressType {
    Uncompressed,
    GzipCompressed,
    BzipCompressed,
    ZipCompressed,
}

#[derive(SmartDefault, Debug)]
pub(crate) struct RecordHeader {
    pub len: u16,
    pub typ: u8,
    pub sub: u8,
    #[default(stdf_record_type::REC_INVALID)]
    pub type_code: u64,
}

// Data Types

/// Altough B1 can be treated as u8, but its representation
/// in ATDF is differ from U1, so I used a array of one u8 for B1
pub type B1 = [u8; 1];
/// Rust char is 4 bytes long, however STDF char is only 1 byte
/// we will read u8 from file stream and convert to Rust char during parse step
pub type C1 = char;
pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;
pub type U8 = u64;
pub type I1 = i8;
pub type I2 = i16;
pub type I4 = i32;
pub type R4 = f32;
pub type R8 = f64;

// Cn;	//first byte = unsigned count of bytes to follow (maximum of 255 bytes)
pub type Cn = String;

// Variable length character string, string length is stored in another field
pub type Cf = String;

// first two bytes = unsigned count of bytes to follow (maximum of 65535 bytes)
pub type Sn = String;

// Bn;	//First byte = unsigned count of bytes to follow (maximum of 255 bytes)
pub type Bn = Vec<u8>;

// Dn;	//First two bytes = unsigned count of bits to follow (maximum of 65,535 bits)
pub type Dn = Vec<u8>;

pub type KxCn = Vec<Cn>;
pub type KxSn = Vec<Sn>;
pub type KxCf = Vec<Cf>;
pub type KxU1 = Vec<U1>;
pub type KxU2 = Vec<U2>;
pub type KxU4 = Vec<U4>;
pub type KxU8 = Vec<U8>;
pub type KxR4 = Vec<R4>;
pub type KxN1 = Vec<U1>;

/// This enum is for STR that
/// introduced in STDF V4-2007.
///
/// the nested data is a vector of Uf type,
/// where f = 1, 2, 4 or 8
#[derive(SmartDefault, Debug, PartialEq, Eq)]
pub enum KxUf {
    #[default]
    F1(KxU1),
    F2(KxU2),
    F4(KxU4),
    F8(KxU8),
}

/// This enum is for storing
/// generic data V1, the data type
/// is the field name.
#[derive(Clone, Debug, PartialEq)]
pub enum V1 {
    B0,
    U1(U1),
    U2(U2),
    U4(U4),
    I1(I1),
    I2(I2),
    I4(I4),
    R4(R4),
    R8(R8),
    Cn(Cn),
    Bn(Bn),
    Dn(Dn),
    N1(U1),
    Invalid,
}

pub type Vn = Vec<V1>;

// Record Types

/// This module contains constants
/// for STDF Record type check
///
/// # Example
///
/// ```
/// use rust_stdf::{StdfRecord, stdf_record_type::*};
///
/// // use constant for record initializing
/// let mut rec = StdfRecord::new(REC_MIR);
///
/// // for type check
/// let t = REC_MIR | REC_MRR | REC_PTR;
/// let is_t = rec.is_type(t);      // true
/// ```
pub mod stdf_record_type {
    // rec type 0
    pub const REC_FAR: u64 = 1;
    pub const REC_ATR: u64 = 1 << 1;
    pub const REC_VUR: u64 = 1 << 2;
    // rec type 1
    pub const REC_MIR: u64 = 1 << 3;
    pub const REC_MRR: u64 = 1 << 4;
    pub const REC_PCR: u64 = 1 << 5;
    pub const REC_HBR: u64 = 1 << 6;
    pub const REC_SBR: u64 = 1 << 7;
    pub const REC_PMR: u64 = 1 << 8;
    pub const REC_PGR: u64 = 1 << 9;
    pub const REC_PLR: u64 = 1 << 10;
    pub const REC_RDR: u64 = 1 << 11;
    pub const REC_SDR: u64 = 1 << 12;
    pub const REC_PSR: u64 = 1 << 13;
    pub const REC_NMR: u64 = 1 << 14;
    pub const REC_CNR: u64 = 1 << 15;
    pub const REC_SSR: u64 = 1 << 16;
    pub const REC_CDR: u64 = 1 << 17;
    // rec type 2
    pub const REC_WIR: u64 = 1 << 18;
    pub const REC_WRR: u64 = 1 << 19;
    pub const REC_WCR: u64 = 1 << 20;
    // rec type 5
    pub const REC_PIR: u64 = 1 << 21;
    pub const REC_PRR: u64 = 1 << 22;
    // rec type 10
    pub const REC_TSR: u64 = 1 << 23;
    // rec type 15
    pub const REC_PTR: u64 = 1 << 24;
    pub const REC_MPR: u64 = 1 << 25;
    pub const REC_FTR: u64 = 1 << 26;
    pub const REC_STR: u64 = 1 << 27;
    // rec type 20
    pub const REC_BPS: u64 = 1 << 28;
    pub const REC_EPS: u64 = 1 << 29;
    // rec type 50
    pub const REC_GDR: u64 = 1 << 30;
    pub const REC_DTR: u64 = 1 << 31;
    // rec type 180: Reserved
    // rec type 181: Reserved
    pub const REC_RESERVE: u64 = 1 << 32;
    pub const REC_INVALID: u64 = 1 << 33;
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

/// `StdfRecord` is the data that returned from StdfReader iterator.
///
/// it contains the actually structs
/// that contain STDF data.
///
/// use `match` structure to access the nested data.
///
/// # Example
///
/// ```
/// use rust_stdf::{StdfRecord, stdf_record_type::*};
///
/// let mut rec = StdfRecord::new(REC_PTR);
/// if let StdfRecord::PTR(ref mut ptr_data) = rec {
///     ptr_data.result = 100.0;
/// }
/// println!("{:?}", rec);
/// ```
#[derive(Debug)]
pub enum StdfRecord {
    // rec type 0
    FAR(FAR),
    ATR(ATR),
    VUR(VUR),
    // rec type 1
    MIR(MIR),
    MRR(MRR),
    PCR(PCR),
    HBR(HBR),
    SBR(SBR),
    PMR(PMR),
    PGR(PGR),
    PLR(PLR),
    RDR(RDR),
    SDR(SDR),
    PSR(PSR),
    NMR(NMR),
    CNR(CNR),
    SSR(SSR),
    CDR(CDR),
    // rec type 2
    WIR(WIR),
    WRR(WRR),
    WCR(WCR),
    // rec type 5
    PIR(PIR),
    PRR(PRR),
    // rec type 10
    TSR(TSR),
    // rec type 15
    PTR(PTR),
    MPR(MPR),
    FTR(FTR),
    STR(STR),
    // rec type 20
    BPS(BPS),
    EPS(EPS),
    // rec type 50
    GDR(GDR),
    DTR(DTR),
    // rec type 180: Reserved
    // rec type 181: Reserved
    ReservedRec(ReservedRec),
    InvalidRec,
}

#[derive(Debug)]
pub struct AtdfRecord {
    rec_name: String,
    type_code: u64,
    scale_flag: bool,
    data_map: HashMap<String, String>,
}

#[derive(SmartDefault, Debug)]
pub struct FAR {
    pub cpu_type: U1, // CPU type that wrote this file
    pub stdf_ver: U1, // STDF version number
}

#[derive(SmartDefault, Debug)]
pub struct ATR {
    pub mod_tim: U4,  //Date and time of STDF file modification
    pub cmd_line: Cn, //Command line of program
}

#[derive(SmartDefault, Debug)]
pub struct VUR {
    pub upd_nam: Cn, //Update Version Name
}

#[derive(SmartDefault, Debug)]
pub struct MIR {
    pub setup_t: U4,  // Date and time of job setup
    pub start_t: U4,  // Date and time first part tested
    pub stat_num: U1, // Tester station number
    #[default = ' ']
    pub mode_cod: C1, // Test mode code (e.g. prod, dev)
    #[default = ' ']
    pub rtst_cod: C1, // Lot retest code
    #[default = ' ']
    pub prot_cod: C1, // Data protection code
    #[default = 65535]
    pub burn_tim: U2, // Burn-in time (in minutes)
    #[default = ' ']
    pub cmod_cod: C1, // Command mode code
    pub lot_id: Cn,   // Lot ID (customer specified)
    pub part_typ: Cn, // Part Type (or product ID)
    pub node_nam: Cn, // Name of node that generated data
    pub tstr_typ: Cn, // Tester type
    pub job_nam: Cn,  // Job name (test program name)
    pub job_rev: Cn,  // Job (test program) revision number
    pub sblot_id: Cn, // Sublot ID
    pub oper_nam: Cn, // Operator name or ID (at setup time)
    pub exec_typ: Cn, // Tester executive software type
    pub exec_ver: Cn, // Tester exec software version number
    pub test_cod: Cn, // Test phase or step code
    pub tst_temp: Cn, // Test temperature
    pub user_txt: Cn, // Generic user text
    pub aux_file: Cn, // Name of auxiliary data file
    pub pkg_typ: Cn,  // Package type
    pub famly_id: Cn, // Product family ID
    pub date_cod: Cn, // Date code
    pub facil_id: Cn, // Test facility ID
    pub floor_id: Cn, // Test floor ID
    pub proc_id: Cn,  // Fabrication process ID
    pub oper_frq: Cn, // Operation frequency or step
    pub spec_nam: Cn, // Test specification name
    pub spec_ver: Cn, // Test specification version number
    pub flow_id: Cn,  // Test flow ID
    pub setup_id: Cn, // Test setup ID
    pub dsgn_rev: Cn, // Device design revision
    pub eng_id: Cn,   // Engineering lot ID
    pub rom_cod: Cn,  // ROM code ID
    pub serl_num: Cn, // Tester serial number
    pub supr_nam: Cn, // Supervisor name or ID
}

#[derive(SmartDefault, Debug)]
pub struct MRR {
    pub finish_t: U4, // Date and time last part tested
    #[default = ' ']
    pub disp_cod: C1, // Lot disposition code,default: space
    pub usr_desc: Cn, // Lot description supplied by user
    pub exc_desc: Cn, // Lot description supplied by exec
}

#[derive(SmartDefault, Debug)]
pub struct PCR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub part_cnt: U4, // Number of parts tested
    #[default = 4_294_967_295]
    pub rtst_cnt: U4, // Number of parts retested
    #[default = 4_294_967_295]
    pub abrt_cnt: U4, // Number of aborts during testing
    #[default = 4_294_967_295]
    pub good_cnt: U4, // Number of good (passed) parts tested
    #[default = 4_294_967_295]
    pub func_cnt: U4, // Number of functional parts tested
}

#[derive(SmartDefault, Debug)]
pub struct HBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub hbin_num: U2, // Hardware bin number
    pub hbin_cnt: U4, // Number of parts in bin
    #[default = ' ']
    pub hbin_pf: C1, // Pass/fail indication
    pub hbin_nam: Cn, // Name of hardware bin
}

#[derive(SmartDefault, Debug)]
pub struct SBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub sbin_num: U2, // Software bin number
    pub sbin_cnt: U4, // Number of parts in bin
    #[default = ' ']
    pub sbin_pf: C1, // Pass/fail indication
    pub sbin_nam: Cn, // Name of software bin
}

#[derive(SmartDefault, Debug)]
pub struct PMR {
    pub pmr_indx: U2, // Unique index associated with pin
    #[default = 0]
    pub chan_typ: U2, // Channel type
    pub chan_nam: Cn, // Channel name
    pub phy_nam: Cn,  // Physical name of pin
    pub log_nam: Cn,  // Logical name of pin
    #[default = 1]
    pub head_num: U1, // Head number associated with channel
    #[default = 1]
    pub site_num: U1, // Site number associated with channel
}

#[derive(SmartDefault, Debug)]
pub struct PGR {
    pub grp_indx: U2,   // Unique index associated with pin group
    pub grp_nam: Cn,    // Name of pin group
    pub indx_cnt: U2,   // Count of PMR indexes
    pub pmr_indx: KxU2, // Array of indexes for pins in the group
}

#[derive(SmartDefault, Debug)]
pub struct PLR {
    pub grp_cnt: U2,    // Count (k) of pins or pin groups
    pub grp_indx: KxU2, // Array of pin or pin group indexes
    pub grp_mode: KxU2, // Operating mode of pin group
    pub grp_radx: KxU1, // Display radix of pin group
    pub pgm_char: KxCn, // Program state encoding characters
    pub rtn_char: KxCn, // Return state encoding characters
    pub pgm_chal: KxCn, // Program state encoding characters
    pub rtn_chal: KxCn, // Return state encoding characters
}

#[derive(SmartDefault, Debug)]
pub struct RDR {
    pub num_bins: U2,   // Number (k) of bins being retested
    pub rtst_bin: KxU2, // Array of retest bin numbers
}

#[derive(SmartDefault, Debug)]
pub struct SDR {
    pub head_num: U1,   // Test head number
    pub site_grp: U1,   // Site group number
    pub site_cnt: U1,   // Number (k) of test sites in site group
    pub site_num: KxU1, // Array of test site numbers
    pub hand_typ: Cn,   // Handler or prober type
    pub hand_id: Cn,    // Handler or prober ID
    pub card_typ: Cn,   // Probe card type
    pub card_id: Cn,    // Probe card ID
    pub load_typ: Cn,   // Load board type
    pub load_id: Cn,    // Load board ID
    pub dib_typ: Cn,    // DIB board type
    pub dib_id: Cn,     // DIB board ID
    pub cabl_typ: Cn,   // Interface cable type
    pub cabl_id: Cn,    // Interface cable ID
    pub cont_typ: Cn,   // Handler contactor type
    pub cont_id: Cn,    // Handler contactor ID
    pub lasr_typ: Cn,   // Laser type
    pub lasr_id: Cn,    // Laser ID
    pub extr_typ: Cn,   // Extra equipment type field
    pub extr_id: Cn,    // Extra equipment ID
}

#[derive(SmartDefault, Debug)]
pub struct PSR {
    pub cont_flg: B1,   // Continuation PSR record exist
    pub psr_indx: U2,   // PSR Record Index (used by STR records)
    pub psr_nam: Cn,    // Symbolic name of PSR record
    pub opt_flg: B1, // Contains PAT_LBL, FILE_UID, ATPG_DSC, and SRC_ID field missing flag bits and flag for start index for first cycle number.
    pub totp_cnt: U2, // Count of total pattern file information sets in the complete PSR data set
    pub locp_cnt: U2, // Count (k) of pattern file information sets in this record
    pub pat_bgn: KxU8, // Array of Cycle #’s patterns begins on
    pub pat_end: KxU8, // Array of Cycle #’s patterns stops at
    pub pat_file: KxCn, // Array of Pattern File Names
    pub pat_lbl: KxCn, // Optional pattern symbolic name
    pub file_uid: KxCn, // Optional array of file identifier code
    pub atpg_dsc: KxCn, // Optional array of ATPG information
    pub src_id: KxCn, // Optional array of PatternInSrcFileID
}

#[derive(SmartDefault, Debug)]
pub struct NMR {
    pub cont_flg: B1,   // Continuation NMR record follows if not 0
    pub totm_cnt: U2,   // Count of PMR indexes and ATPG_NAM entries
    pub locm_cnt: U2,   // Count of (k) PMR indexes and ATPG_NAM entries in this record
    pub pmr_indx: KxU2, // Array of PMR indexes
    pub atpg_nam: KxCn, // Array of ATPG signal names
}

#[derive(SmartDefault, Debug)]
pub struct CNR {
    pub chn_num: U2,  // Chain number. Referenced by the CHN_NUM array in an STR record
    pub bit_pos: U4,  // Bit position in the chain
    pub cell_nam: Sn, // Scan Cell Name
}

#[derive(SmartDefault, Debug)]
pub struct SSR {
    pub ssr_nam: Cn,    // Name of the STIL Scan pub structure for reference
    pub chn_cnt: U2,    // Count (k) of number of Chains listed in CHN_LIST
    pub chn_list: KxU2, // Array of CDR Indexes
}

#[derive(SmartDefault, Debug)]
pub struct CDR {
    pub cont_flg: B1, // Continuation CDR record follows if not 0
    pub cdr_indx: U2, // SCR Index
    pub chn_nam: Cn,  // Chain Name
    pub chn_len: U4,  // Chain Length (# of scan cells in chain)
    pub sin_pin: U2,  // PMR index of the chain's Scan In Signal
    pub sout_pin: U2, // PMR index of the chain's Scan Out Signal
    pub mstr_cnt: U1, // Count (m) of master clock pins specified for this scan chain
    pub m_clks: KxU2, // Array of PMR indexes for the master clocks assigned to this chain
    pub slav_cnt: U1, // Count (n) of slave clock pins specified for this scan chain
    pub s_clks: KxU2, // Array of PMR indexes for the slave clocks assigned to this chain
    #[default = 255]
    pub inv_val: U1, // 0: No Inversion, 1: Inversion
    pub lst_cnt: U2,  // Count (k) of scan cells listed in this record
    pub cell_lst: KxSn, // Array of Scan Cell Names
}

#[derive(SmartDefault, Debug)]
pub struct WIR {
    pub head_num: U1, // Test head number
    #[default = 255]
    pub site_grp: U1, // Site group number
    pub start_t: U4,  // Date and time first part tested
    pub wafer_id: Cn, // Wafer ID length byte = 0
}

#[derive(SmartDefault, Debug)]
pub struct WRR {
    pub head_num: U1, // Test head number
    #[default = 255]
    pub site_grp: U1, // Site group number
    pub finish_t: U4, // Date and time last part tested
    pub part_cnt: U4, // Number of parts tested
    #[default = 4_294_967_295]
    pub rtst_cnt: U4, // Number of parts retested
    #[default = 4_294_967_295]
    pub abrt_cnt: U4, // Number of aborts during testing
    #[default = 4_294_967_295]
    pub good_cnt: U4, // Number of good (passed) parts tested
    #[default = 4_294_967_295]
    pub func_cnt: U4, // Number of functional parts tested
    pub wafer_id: Cn, // Wafer ID
    pub fabwf_id: Cn, // Fab wafer ID
    pub frame_id: Cn, // Wafer frame ID
    pub mask_id: Cn,  // Wafer mask ID
    pub usr_desc: Cn, // Wafer description supplied by user
    pub exc_desc: Cn, // Wafer description supplied by exec
}

#[derive(SmartDefault, Debug)]
pub struct WCR {
    #[default = 0.0]
    pub wafr_siz: R4, // Diameter of wafer in WF_UNITS
    #[default = 0.0]
    pub die_ht: R4, // Height of die in WF_UNITS
    #[default = 0.0]
    pub die_wid: R4, // Width of die in WF_UNITS
    #[default = 0]
    pub wf_units: U1, // Units for wafer and die dimensions
    #[default = ' ']
    pub wf_flat: C1, // Orientation of wafer flat
    #[default(-32768)]
    pub center_x: I2, // X coordinate of center die on wafer
    #[default(-32768)]
    pub center_y: I2, // Y coordinate of center die on wafer
    #[default = ' ']
    pub pos_x: C1, // Positive X direction of wafer
    #[default = ' ']
    pub pos_y: C1, // Positive Y direction of wafer
}

#[derive(SmartDefault, Debug)]
pub struct PIR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
}

#[derive(SmartDefault, Debug)]
pub struct PRR {
    pub head_num: U1, //Test head number
    pub site_num: U1, //Test site number
    pub part_flg: B1, //Part information flag
    pub num_test: U2, //Number of tests executed
    pub hard_bin: U2, //Hardware bin number
    #[default = 65535]
    pub soft_bin: U2, //Software bin number
    #[default(-32768)]
    pub x_coord: I2, //(Wafer) X coordinate
    #[default(-32768)]
    pub y_coord: I2, //(Wafer) Y coordinate
    #[default = 0]
    pub test_t: U4, //Elapsed test time in milliseconds
    pub part_id: Cn,  //Part identification
    pub part_txt: Cn, //Part description text
    pub part_fix: Bn, //Part repair information
}

#[derive(SmartDefault, Debug)]
pub struct TSR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    #[default = ' ']
    pub test_typ: C1, // Test type
    pub test_num: U4, // Test number
    #[default = 4_294_967_295]
    pub exec_cnt: U4, // Number of test executions
    #[default = 4_294_967_295]
    pub fail_cnt: U4, // Number of test failures
    #[default = 4_294_967_295]
    pub alrm_cnt: U4, // Number of alarmed tests
    pub test_nam: Cn, // Test name
    pub seq_name: Cn, // Sequencer (program segment/flow) name
    pub test_lbl: Cn, // Test label or text
    pub opt_flag: B1, // Optional data flag
    pub test_tim: R4, // Average test execution time in seconds
    pub test_min: R4, // Lowest test result value
    pub test_max: R4, // Highest test result value
    pub tst_sums: R4, // Sum of test result values
    pub tst_sqrs: R4, // Sum of squares of test result values
}

#[derive(SmartDefault, Debug)]
pub struct PTR {
    pub test_num: U4, // Test number
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub test_flg: B1, // Test flags (fail, alarm, etc.)
    pub parm_flg: B1, // Parametric test flags (drift, etc.)
    pub result: R4,   // Test result
    pub test_txt: Cn, // Test description text or label
    pub alarm_id: Cn, // Name of alarm
    pub opt_flag: B1, // Optional data flag
    pub res_scal: I1, // Test results scaling exponent
    pub llm_scal: I1, // Low limit scaling exponent
    pub hlm_scal: I1, // High limit scaling exponent
    pub lo_limit: R4, // Low test limit value
    pub hi_limit: R4, // High test limit value
    pub units: Cn,    // Test units
    pub c_resfmt: Cn, // ANSI C result format string
    pub c_llmfmt: Cn, // ANSI C low limit format string
    pub c_hlmfmt: Cn, // ANSI C high limit format string
    pub lo_spec: R4,  // Low specification limit value
    pub hi_spec: R4,  // High specification limit value
}

#[derive(SmartDefault, Debug)]
pub struct MPR {
    pub test_num: U4,   // Test number
    pub head_num: U1,   // Test head number
    pub site_num: U1,   // Test site number
    pub test_flg: B1,   // Test flags (fail, alarm, etc.)
    pub parm_flg: B1,   // Parametric test flags (drift, etc.)
    pub rtn_icnt: U2,   // Count of PMR indexes
    pub rslt_cnt: U2,   // Count of returned results
    pub rtn_stat: KxN1, // Array of returned states
    pub rtn_rslt: KxR4, // Array of returned results
    pub test_txt: Cn,   // Descriptive text or label
    pub alarm_id: Cn,   // Name of alarm
    pub opt_flag: B1,   // Optional data flag
    pub res_scal: I1,   // Test result scaling exponent
    pub llm_scal: I1,   // Test low limit scaling exponent
    pub hlm_scal: I1,   // Test high limit scaling exponent
    pub lo_limit: R4,   // Test low limit value
    pub hi_limit: R4,   // Test high limit value
    pub start_in: R4,   // Starting input value (condition)
    pub incr_in: R4,    // Increment of input condition
    pub rtn_indx: KxU2, // Array of PMR indexes
    pub units: Cn,      // Units of returned results
    pub units_in: Cn,   // Input condition units
    pub c_resfmt: Cn,   // ANSI C result format string
    pub c_llmfmt: Cn,   // ANSI C low limit format string
    pub c_hlmfmt: Cn,   // ANSI C high limit format string
    pub lo_spec: R4,    // Low specification limit value
    pub hi_spec: R4,    // High specification limit value
}

#[derive(SmartDefault, Debug)]
pub struct FTR {
    pub test_num: U4,   // Test number
    pub head_num: U1,   // Test head number
    pub site_num: U1,   // Test site number
    pub test_flg: B1,   // Test flags (fail, alarm, etc.)
    pub opt_flag: B1,   // Optional data flag
    pub cycl_cnt: U4,   // Cycle count of vector
    pub rel_vadr: U4,   // Relative vector address
    pub rept_cnt: U4,   // Repeat count of vector
    pub num_fail: U4,   // Number of pins with 1 or more failures
    pub xfail_ad: I4,   // X logical device failure address
    pub yfail_ad: I4,   // Y logical device failure address
    pub vect_off: I2,   // Offset from vector of interest
    pub rtn_icnt: U2,   // Count j of return data PMR indexes
    pub pgm_icnt: U2,   // Count k of programmed state indexes
    pub rtn_indx: KxU2, // Array j of return data PMR indexes
    pub rtn_stat: KxN1, // Array j of returned states
    pub pgm_indx: KxU2, // Array k of programmed state indexes
    pub pgm_stat: KxN1, // Array k of programmed states
    pub fail_pin: Dn,   // Failing pin bitfield
    pub vect_nam: Cn,   // Vector module pattern name
    pub time_set: Cn,   // Time set name
    pub op_code: Cn,    // Vector Op Code
    pub test_txt: Cn,   // Descriptive text or label
    pub alarm_id: Cn,   // Name of alarm
    pub prog_txt: Cn,   // Additional programmed information
    pub rslt_txt: Cn,   // Additional result information
    #[default = 255]
    pub patg_num: U1, // Pattern generator number
    pub spin_map: Dn,   // Bit map of enabled comparators
}

#[derive(SmartDefault, Debug)]
pub struct STR {
    pub cont_flg: B1,   // Continuation STR follows if not 0
    pub test_num: U4,   // Test number
    pub head_num: U1,   // Test head number
    pub site_num: U1,   // Test site number
    pub psr_ref: U2,    // PSR Index (Pattern Sequence Record)
    pub test_flg: B1,   // Test flags (fail, alarm, etc.)
    pub log_typ: Cn,    // User defined description of datalog
    pub test_txt: Cn,   // Descriptive text or label
    pub alarm_id: Cn,   // Name of alarm
    pub prog_txt: Cn,   // Additional Programmed information
    pub rslt_txt: Cn,   // Additional result information
    pub z_val: U1,      // Z Handling Flag
    pub fmu_flg: B1,    // MASK_MAP & FAL_MAP field status & Pattern Changed flag
    pub mask_map: Dn,   // Bit map of Globally Masked Pins
    pub fal_map: Dn,    // Bit map of failures after buffer full
    pub cyc_cnt_t: U8,  // Total cycles executed in test
    pub totf_cnt: U4,   // Total failures (pin x cycle) detected in test execution
    pub totl_cnt: U4,   // Total fails logged across the complete STR data set
    pub cyc_base: U8,   // Cycle offset to apply for the values in the CYCL_NUM array
    pub bit_base: U4,   // Offset to apply for the values in the BIT_POS array
    pub cond_cnt: U2, // Count (g) of Test Conditions and optional data specifications in present record
    pub lim_cnt: U2,  // Count (j) of LIM Arrays in present record, 1 for global specification
    pub cyc_size: U1, // Size (f) of data (1,2,4, or 8 byes) in CYC_OFST field
    pub pmr_size: U1, // Size (f) of data (1 or 2 bytes) in PMR_INDX field
    pub chn_size: U1, // Size (f) of data (1, 2 or 4 bytes) in CHN_NUM field
    pub pat_size: U1, // Size (f) of data (1,2, or 4 bytes) in PAT_NUM field
    pub bit_size: U1, // Size (f) of data (1,2, or 4 bytes) in BIT_POS field
    pub u1_size: U1,  // Size (f) of data (1,2,4 or 8 bytes) in USR1 field
    pub u2_size: U1,  // Size (f) of data (1,2,4 or 8 bytes) in USR2 field
    pub u3_size: U1,  // Size (f) of data (1,2,4 or 8 bytes) in USR3 field
    pub utx_size: U1, // Size (f) of each string entry in USER_TXT array
    pub cap_bgn: U2,  // Offset added to BIT_POS value to indicate capture cycles
    pub lim_indx: KxU2, // Array of PMR indexes that require unique limit specifications
    pub lim_spec: KxU4, // Array of fail datalogging limits for the PMRs listed in LIM_INDX
    pub cond_lst: KxCn, // Array of test condition (Name=value) pairs
    pub cyc_cnt: U2,  // Count (k) of entries in CYC_OFST array
    pub cyc_ofst: KxUf, // Array of cycle numbers relative to CYC_BASE
    pub pmr_cnt: U2,  // Count (k) of entries in the PMR_INDX array
    pub pmr_indx: KxUf, // Array of PMR Indexes (All Formats)
    pub chn_cnt: U2,  // Count (k) of entries in the CHN_NUM array
    pub chn_num: KxUf, // Array of Chain No for FF Name Mapping
    pub exp_cnt: U2,  // Count (k) of EXP_DATA array entries
    pub exp_data: KxU1, // Array of expected vector data
    pub cap_cnt: U2,  // Count (k) of CAP_DATA array entries
    pub cap_data: KxU1, // Array of captured data
    pub new_cnt: U2,  // Count (k) of NEW_DATA array entries
    pub new_data: KxU1, // Array of new vector data
    pub pat_cnt: U2,  // Count (k) of PAT_NUM array entries
    pub pat_num: KxUf, // Array of pattern # (Ptn/Chn/Bit format)
    pub bpos_cnt: U2, // Count (k) of BIT_POS array entries
    pub bit_pos: KxUf, // Array of chain bit positions (Ptn/Chn/Bit format)
    pub usr1_cnt: U2, // Count (k) of USR1 array entries
    pub usr1: KxUf,   // Array of user defined data for each logged fail
    pub usr2_cnt: U2, // Count (k) of USR2 array entries
    pub usr2: KxUf,   // Array of user defined data for each logged fail
    pub usr3_cnt: U2, // Count (k) of USR3 array entries
    pub usr3: KxUf,   // Array of user defined data for each logged fail
    pub txt_cnt: U2,  // Count (k) of USER_TXT array entries
    pub user_txt: KxCf, // Array of user defined fixed length strings for each logged fail
}

#[derive(SmartDefault, Debug)]
pub struct BPS {
    pub seq_name: Cn, // Program section (or sequencer) name length byte = 0
}

#[derive(SmartDefault, Debug)]
pub struct EPS {}

#[derive(SmartDefault, Debug)]
pub struct GDR {
    pub fld_cnt: U2,  // Count of data fields in record
    pub gen_data: Vn, // Data type code and data for one field(Repeat GEN_DATA for each data field)
}

#[derive(SmartDefault, Debug)]
pub struct DTR {
    pub text_dat: Cn, // ASCII text string
}

#[derive(SmartDefault, Debug)]
pub struct ReservedRec {
    pub raw_data: Cn, // unparsed data
}

// implementation

impl RecordHeader {
    pub(crate) fn new() -> Self {
        RecordHeader::default()
    }

    /// Construct a STDF record header from first 4 elements of given byte array.
    ///
    /// If array size is less than 4, this function return a StdfError
    pub(crate) fn read_from_bytes(
        mut self,
        raw_data: &[u8],
        order: &ByteOrder,
    ) -> Result<Self, StdfError> {
        if raw_data.len() >= 4 {
            let len_bytes = [raw_data[0], raw_data[1]];
            self.len = match order {
                ByteOrder::LittleEndian => u16::from_le_bytes(len_bytes),
                ByteOrder::BigEndian => u16::from_be_bytes(len_bytes),
            };
            self.typ = raw_data[2];
            self.sub = raw_data[3];
            // validate header
            self.type_code = match (self.typ, self.sub) {
                // rec type 15
                (15, 10) => stdf_record_type::REC_PTR,
                (15, 15) => stdf_record_type::REC_MPR,
                (15, 20) => stdf_record_type::REC_FTR,
                (15, 30) => stdf_record_type::REC_STR,
                // rec type 5
                (5, 10) => stdf_record_type::REC_PIR,
                (5, 20) => stdf_record_type::REC_PRR,
                // rec type 2
                (2, 10) => stdf_record_type::REC_WIR,
                (2, 20) => stdf_record_type::REC_WRR,
                (2, 30) => stdf_record_type::REC_WCR,
                // rec type 50
                (50, 10) => stdf_record_type::REC_GDR,
                (50, 30) => stdf_record_type::REC_DTR,
                // rec type 0
                (0, 10) => stdf_record_type::REC_FAR,
                (0, 20) => stdf_record_type::REC_ATR,
                (0, 30) => stdf_record_type::REC_VUR,
                // rec type 1
                (1, 10) => stdf_record_type::REC_MIR,
                (1, 20) => stdf_record_type::REC_MRR,
                (1, 30) => stdf_record_type::REC_PCR,
                (1, 40) => stdf_record_type::REC_HBR,
                (1, 50) => stdf_record_type::REC_SBR,
                (1, 60) => stdf_record_type::REC_PMR,
                (1, 62) => stdf_record_type::REC_PGR,
                (1, 63) => stdf_record_type::REC_PLR,
                (1, 70) => stdf_record_type::REC_RDR,
                (1, 80) => stdf_record_type::REC_SDR,
                (1, 90) => stdf_record_type::REC_PSR,
                (1, 91) => stdf_record_type::REC_NMR,
                (1, 92) => stdf_record_type::REC_CNR,
                (1, 93) => stdf_record_type::REC_SSR,
                (1, 94) => stdf_record_type::REC_CDR,
                // rec type 10
                (10, 30) => stdf_record_type::REC_TSR,
                // rec type 20
                (20, 10) => stdf_record_type::REC_BPS,
                (20, 20) => stdf_record_type::REC_EPS,
                // rec type 180: Reserved
                // rec type 181: Reserved
                (180 | 181, _) => stdf_record_type::REC_RESERVE,
                // not matched
                (_, _) => stdf_record_type::REC_INVALID,
            };

            if self.type_code == stdf_record_type::REC_INVALID {
                Err(StdfError {
                    code: 2,
                    msg: format!("{:?}", self),
                })
            } else {
                Ok(self)
            }
        } else {
            Err(StdfError {
                code: 1,
                msg: String::from("Not enough data to construct record header"),
            })
        }
    }
}

impl FAR {
    pub fn new() -> Self {
        FAR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.cpu_type = read_uint8(raw_data, pos);
        self.stdf_ver = read_uint8(raw_data, pos);
        self
    }
}

impl ATR {
    pub fn new() -> Self {
        ATR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.mod_tim = read_u4(raw_data, pos, order);
        self.cmd_line = read_cn(raw_data, pos);
        self
    }
}

impl VUR {
    pub fn new() -> Self {
        VUR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.upd_nam = read_cn(raw_data, pos);
        self
    }
}

impl MIR {
    pub fn new() -> Self {
        MIR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.setup_t = read_u4(raw_data, pos, order);
        self.start_t = read_u4(raw_data, pos, order);
        self.stat_num = read_uint8(raw_data, pos);
        // if raw_data is completely parsed,
        // don't overwrite fields with default data
        if *pos < raw_data.len() {
            self.mode_cod = read_uint8(raw_data, pos) as char;
        }
        if *pos < raw_data.len() {
            self.rtst_cod = read_uint8(raw_data, pos) as char;
        }
        if *pos < raw_data.len() {
            self.prot_cod = read_uint8(raw_data, pos) as char;
        }
        if *pos + 2 <= raw_data.len() {
            self.burn_tim = read_u2(raw_data, pos, order);
        }
        if *pos < raw_data.len() {
            self.cmod_cod = read_uint8(raw_data, pos) as char;
        }
        self.lot_id = read_cn(raw_data, pos);
        self.part_typ = read_cn(raw_data, pos);
        self.node_nam = read_cn(raw_data, pos);
        self.tstr_typ = read_cn(raw_data, pos);
        self.job_nam = read_cn(raw_data, pos);
        self.job_rev = read_cn(raw_data, pos);
        self.sblot_id = read_cn(raw_data, pos);
        self.oper_nam = read_cn(raw_data, pos);
        self.exec_typ = read_cn(raw_data, pos);
        self.exec_ver = read_cn(raw_data, pos);
        self.test_cod = read_cn(raw_data, pos);
        self.tst_temp = read_cn(raw_data, pos);
        self.user_txt = read_cn(raw_data, pos);
        self.aux_file = read_cn(raw_data, pos);
        self.pkg_typ = read_cn(raw_data, pos);
        self.famly_id = read_cn(raw_data, pos);
        self.date_cod = read_cn(raw_data, pos);
        self.facil_id = read_cn(raw_data, pos);
        self.floor_id = read_cn(raw_data, pos);
        self.proc_id = read_cn(raw_data, pos);
        self.oper_frq = read_cn(raw_data, pos);
        self.spec_nam = read_cn(raw_data, pos);
        self.spec_ver = read_cn(raw_data, pos);
        self.flow_id = read_cn(raw_data, pos);
        self.setup_id = read_cn(raw_data, pos);
        self.dsgn_rev = read_cn(raw_data, pos);
        self.eng_id = read_cn(raw_data, pos);
        self.rom_cod = read_cn(raw_data, pos);
        self.serl_num = read_cn(raw_data, pos);
        self.supr_nam = read_cn(raw_data, pos);
        self
    }
}

impl MRR {
    pub fn new() -> Self {
        MRR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.finish_t = read_u4(raw_data, pos, order);
        if *pos < raw_data.len() {
            self.disp_cod = read_uint8(raw_data, pos) as char;
        }
        self.usr_desc = read_cn(raw_data, pos);
        self.exc_desc = read_cn(raw_data, pos);
        self
    }
}

impl PCR {
    pub fn new() -> Self {
        PCR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.part_cnt = read_u4(raw_data, pos, order);
        if *pos + 4 <= raw_data.len() {
            self.rtst_cnt = read_u4(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.abrt_cnt = read_u4(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.good_cnt = read_u4(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.func_cnt = read_u4(raw_data, pos, order);
        }
        self
    }
}

impl HBR {
    pub fn new() -> Self {
        HBR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.hbin_num = read_u2(raw_data, pos, order);
        self.hbin_cnt = read_u4(raw_data, pos, order);
        if *pos < raw_data.len() {
            self.hbin_pf = read_uint8(raw_data, pos) as char;
        }
        self.hbin_nam = read_cn(raw_data, pos);
        self
    }
}

impl SBR {
    pub fn new() -> Self {
        SBR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.sbin_num = read_u2(raw_data, pos, order);
        self.sbin_cnt = read_u4(raw_data, pos, order);
        if *pos < raw_data.len() {
            self.sbin_pf = read_uint8(raw_data, pos) as char;
        }
        self.sbin_nam = read_cn(raw_data, pos);
        self
    }
}

impl PMR {
    pub fn new() -> Self {
        PMR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.pmr_indx = read_u2(raw_data, pos, order);
        if *pos + 2 <= raw_data.len() {
            self.chan_typ = read_u2(raw_data, pos, order);
        }
        self.chan_nam = read_cn(raw_data, pos);
        self.phy_nam = read_cn(raw_data, pos);
        self.log_nam = read_cn(raw_data, pos);
        if *pos < raw_data.len() {
            self.head_num = read_uint8(raw_data, pos)
        };
        if *pos < raw_data.len() {
            self.site_num = read_uint8(raw_data, pos)
        };
        self
    }
}

impl PGR {
    pub fn new() -> Self {
        PGR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.grp_indx = read_u2(raw_data, pos, order);
        self.grp_nam = read_cn(raw_data, pos);
        self.indx_cnt = read_u2(raw_data, pos, order);
        self.pmr_indx = read_kx_u2(raw_data, pos, order, self.indx_cnt);
        self
    }
}

impl PLR {
    pub fn new() -> Self {
        PLR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.grp_cnt = read_u2(raw_data, pos, order);
        self.grp_indx = read_kx_u2(raw_data, pos, order, self.grp_cnt);
        self.grp_mode = read_kx_u2(raw_data, pos, order, self.grp_cnt);
        self.grp_radx = read_kx_u1(raw_data, pos, self.grp_cnt);
        self.pgm_char = read_kx_cn(raw_data, pos, self.grp_cnt);
        self.rtn_char = read_kx_cn(raw_data, pos, self.grp_cnt);
        self.pgm_chal = read_kx_cn(raw_data, pos, self.grp_cnt);
        self.rtn_chal = read_kx_cn(raw_data, pos, self.grp_cnt);
        self
    }
}

impl RDR {
    pub fn new() -> Self {
        RDR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.num_bins = read_u2(raw_data, pos, order);
        self.rtst_bin = read_kx_u2(raw_data, pos, order, self.num_bins);
        self
    }
}

impl SDR {
    pub fn new() -> Self {
        SDR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_grp = read_uint8(raw_data, pos);
        self.site_cnt = read_uint8(raw_data, pos);
        self.site_num = read_kx_u1(raw_data, pos, self.site_cnt as u16);
        self.hand_typ = read_cn(raw_data, pos);
        self.hand_id = read_cn(raw_data, pos);
        self.card_typ = read_cn(raw_data, pos);
        self.card_id = read_cn(raw_data, pos);
        self.load_typ = read_cn(raw_data, pos);
        self.load_id = read_cn(raw_data, pos);
        self.dib_typ = read_cn(raw_data, pos);
        self.dib_id = read_cn(raw_data, pos);
        self.cabl_typ = read_cn(raw_data, pos);
        self.cabl_id = read_cn(raw_data, pos);
        self.cont_typ = read_cn(raw_data, pos);
        self.cont_id = read_cn(raw_data, pos);
        self.lasr_typ = read_cn(raw_data, pos);
        self.lasr_id = read_cn(raw_data, pos);
        self.extr_typ = read_cn(raw_data, pos);
        self.extr_id = read_cn(raw_data, pos);
        self
    }
}

impl PSR {
    pub fn new() -> Self {
        PSR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.cont_flg = [read_uint8(raw_data, pos)];
        self.psr_indx = read_u2(raw_data, pos, order);
        self.psr_nam = read_cn(raw_data, pos);
        self.opt_flg = [read_uint8(raw_data, pos)];
        self.totp_cnt = read_u2(raw_data, pos, order);
        self.locp_cnt = read_u2(raw_data, pos, order);
        self.pat_bgn = read_kx_u8(raw_data, pos, order, self.locp_cnt);
        self.pat_end = read_kx_u8(raw_data, pos, order, self.locp_cnt);
        self.pat_file = read_kx_cn(raw_data, pos, self.locp_cnt);
        self.pat_lbl = read_kx_cn(raw_data, pos, self.locp_cnt);
        self.file_uid = read_kx_cn(raw_data, pos, self.locp_cnt);
        self.atpg_dsc = read_kx_cn(raw_data, pos, self.locp_cnt);
        self.src_id = read_kx_cn(raw_data, pos, self.locp_cnt);
        self
    }
}

impl NMR {
    pub fn new() -> Self {
        NMR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.cont_flg = [read_uint8(raw_data, pos)];
        self.totm_cnt = read_u2(raw_data, pos, order);
        self.locm_cnt = read_u2(raw_data, pos, order);
        self.pmr_indx = read_kx_u2(raw_data, pos, order, self.locm_cnt);
        self.atpg_nam = read_kx_cn(raw_data, pos, self.locm_cnt);
        self
    }
}

impl CNR {
    pub fn new() -> Self {
        CNR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.chn_num = read_u2(raw_data, pos, order);
        self.bit_pos = read_u4(raw_data, pos, order);
        self.cell_nam = read_sn(raw_data, pos, order);
        self
    }
}

impl SSR {
    pub fn new() -> Self {
        SSR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.ssr_nam = read_cn(raw_data, pos);
        self.chn_cnt = read_u2(raw_data, pos, order);
        self.chn_list = read_kx_u2(raw_data, pos, order, self.chn_cnt);
        self
    }
}

impl CDR {
    pub fn new() -> Self {
        CDR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.cont_flg = [read_uint8(raw_data, pos)];
        self.cdr_indx = read_u2(raw_data, pos, order);
        self.chn_nam = read_cn(raw_data, pos);
        self.chn_len = read_u4(raw_data, pos, order);
        self.sin_pin = read_u2(raw_data, pos, order);
        self.sout_pin = read_u2(raw_data, pos, order);
        self.mstr_cnt = read_uint8(raw_data, pos);
        self.m_clks = read_kx_u2(raw_data, pos, order, self.mstr_cnt as u16);
        self.slav_cnt = read_uint8(raw_data, pos);
        self.s_clks = read_kx_u2(raw_data, pos, order, self.slav_cnt as u16);
        if *pos < raw_data.len() {
            self.inv_val = read_uint8(raw_data, pos);
        }
        self.lst_cnt = read_u2(raw_data, pos, order);
        self.cell_lst = read_kx_sn(raw_data, pos, order, self.lst_cnt);
        self
    }
}

impl WIR {
    pub fn new() -> Self {
        WIR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        if *pos < raw_data.len() {
            self.site_grp = read_uint8(raw_data, pos);
        }
        self.start_t = read_u4(raw_data, pos, order);
        self.wafer_id = read_cn(raw_data, pos);
        self
    }
}

impl WRR {
    pub fn new() -> Self {
        WRR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        if *pos < raw_data.len() {
            self.site_grp = read_uint8(raw_data, pos);
        }
        self.finish_t = read_u4(raw_data, pos, order);
        self.part_cnt = read_u4(raw_data, pos, order);
        if *pos + 4 <= raw_data.len() {
            self.rtst_cnt = read_u4(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.abrt_cnt = read_u4(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.good_cnt = read_u4(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.func_cnt = read_u4(raw_data, pos, order);
        }
        self.wafer_id = read_cn(raw_data, pos);
        self.fabwf_id = read_cn(raw_data, pos);
        self.frame_id = read_cn(raw_data, pos);
        self.mask_id = read_cn(raw_data, pos);
        self.usr_desc = read_cn(raw_data, pos);
        self.exc_desc = read_cn(raw_data, pos);
        self
    }
}

impl WCR {
    pub fn new() -> Self {
        WCR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.wafr_siz = read_r4(raw_data, pos, order);
        self.die_ht = read_r4(raw_data, pos, order);
        self.die_wid = read_r4(raw_data, pos, order);
        self.wf_units = read_uint8(raw_data, pos);
        if *pos < raw_data.len() {
            self.wf_flat = read_uint8(raw_data, pos) as char;
        }
        if *pos + 2 <= raw_data.len() {
            self.center_x = read_i2(raw_data, pos, order);
        }
        if *pos + 2 <= raw_data.len() {
            self.center_y = read_i2(raw_data, pos, order);
        }
        if *pos < raw_data.len() {
            self.pos_x = read_uint8(raw_data, pos) as char;
        }
        if *pos < raw_data.len() {
            self.pos_y = read_uint8(raw_data, pos) as char;
        }
        self
    }
}

impl PIR {
    pub fn new() -> Self {
        PIR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self
    }
}

impl PRR {
    pub fn new() -> Self {
        PRR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.part_flg = [read_uint8(raw_data, pos)];
        self.num_test = read_u2(raw_data, pos, order);
        self.hard_bin = read_u2(raw_data, pos, order);
        if *pos + 2 <= raw_data.len() {
            self.soft_bin = read_u2(raw_data, pos, order);
        }
        if *pos + 2 <= raw_data.len() {
            self.x_coord = read_i2(raw_data, pos, order);
        }
        if *pos + 2 <= raw_data.len() {
            self.y_coord = read_i2(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.test_t = read_u4(raw_data, pos, order);
        }
        self.part_id = read_cn(raw_data, pos);
        self.part_txt = read_cn(raw_data, pos);
        self.part_fix = read_bn(raw_data, pos);
        self
    }
}

impl TSR {
    pub fn new() -> Self {
        TSR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        if *pos < raw_data.len() {
            self.test_typ = read_uint8(raw_data, pos) as char;
        }
        self.test_num = read_u4(raw_data, pos, order);
        if *pos + 4 <= raw_data.len() {
            self.exec_cnt = read_u4(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.fail_cnt = read_u4(raw_data, pos, order);
        }
        if *pos + 4 <= raw_data.len() {
            self.alrm_cnt = read_u4(raw_data, pos, order);
        }
        self.test_nam = read_cn(raw_data, pos);
        self.seq_name = read_cn(raw_data, pos);
        self.test_lbl = read_cn(raw_data, pos);
        self.opt_flag = [read_uint8(raw_data, pos)];
        self.test_tim = read_r4(raw_data, pos, order);
        self.test_min = read_r4(raw_data, pos, order);
        self.test_max = read_r4(raw_data, pos, order);
        self.tst_sums = read_r4(raw_data, pos, order);
        self.tst_sqrs = read_r4(raw_data, pos, order);
        self
    }
}

impl PTR {
    pub fn new() -> Self {
        PTR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.test_num = read_u4(raw_data, pos, order);
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.test_flg = [read_uint8(raw_data, pos)];
        self.parm_flg = [read_uint8(raw_data, pos)];
        self.result = read_r4(raw_data, pos, order);
        self.test_txt = read_cn(raw_data, pos);
        self.alarm_id = read_cn(raw_data, pos);
        self.opt_flag = [read_uint8(raw_data, pos)];
        self.res_scal = read_i1(raw_data, pos);
        self.llm_scal = read_i1(raw_data, pos);
        self.hlm_scal = read_i1(raw_data, pos);
        self.lo_limit = read_r4(raw_data, pos, order);
        self.hi_limit = read_r4(raw_data, pos, order);
        self.units = read_cn(raw_data, pos);
        self.c_resfmt = read_cn(raw_data, pos);
        self.c_llmfmt = read_cn(raw_data, pos);
        self.c_hlmfmt = read_cn(raw_data, pos);
        self.lo_spec = read_r4(raw_data, pos, order);
        self.hi_spec = read_r4(raw_data, pos, order);
        self
    }
}

impl MPR {
    pub fn new() -> Self {
        MPR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.test_num = read_u4(raw_data, pos, order);
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.test_flg = [read_uint8(raw_data, pos)];
        self.parm_flg = [read_uint8(raw_data, pos)];
        self.rtn_icnt = read_u2(raw_data, pos, order);
        self.rslt_cnt = read_u2(raw_data, pos, order);
        self.rtn_stat = read_kx_n1(raw_data, pos, self.rtn_icnt);
        self.rtn_rslt = read_kx_r4(raw_data, pos, order, self.rslt_cnt);
        self.test_txt = read_cn(raw_data, pos);
        self.alarm_id = read_cn(raw_data, pos);
        self.opt_flag = [read_uint8(raw_data, pos)];
        self.res_scal = read_i1(raw_data, pos);
        self.llm_scal = read_i1(raw_data, pos);
        self.hlm_scal = read_i1(raw_data, pos);
        self.lo_limit = read_r4(raw_data, pos, order);
        self.hi_limit = read_r4(raw_data, pos, order);
        self.start_in = read_r4(raw_data, pos, order);
        self.incr_in = read_r4(raw_data, pos, order);
        self.rtn_indx = read_kx_u2(raw_data, pos, order, self.rtn_icnt);
        self.units = read_cn(raw_data, pos);
        self.units_in = read_cn(raw_data, pos);
        self.c_resfmt = read_cn(raw_data, pos);
        self.c_llmfmt = read_cn(raw_data, pos);
        self.c_hlmfmt = read_cn(raw_data, pos);
        self.lo_spec = read_r4(raw_data, pos, order);
        self.hi_spec = read_r4(raw_data, pos, order);
        self
    }
}

impl FTR {
    pub fn new() -> Self {
        FTR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.test_num = read_u4(raw_data, pos, order);
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.test_flg = [read_uint8(raw_data, pos)];
        self.opt_flag = [read_uint8(raw_data, pos)];
        self.cycl_cnt = read_u4(raw_data, pos, order);
        self.rel_vadr = read_u4(raw_data, pos, order);
        self.rept_cnt = read_u4(raw_data, pos, order);
        self.num_fail = read_u4(raw_data, pos, order);
        self.xfail_ad = read_i4(raw_data, pos, order);
        self.yfail_ad = read_i4(raw_data, pos, order);
        self.vect_off = read_i2(raw_data, pos, order);
        self.rtn_icnt = read_u2(raw_data, pos, order);
        self.pgm_icnt = read_u2(raw_data, pos, order);
        self.rtn_indx = read_kx_u2(raw_data, pos, order, self.rtn_icnt);
        self.rtn_stat = read_kx_n1(raw_data, pos, self.rtn_icnt);
        self.pgm_indx = read_kx_u2(raw_data, pos, order, self.pgm_icnt);
        self.pgm_stat = read_kx_n1(raw_data, pos, self.pgm_icnt);
        self.fail_pin = read_dn(raw_data, pos, order);
        self.vect_nam = read_cn(raw_data, pos);
        self.time_set = read_cn(raw_data, pos);
        self.op_code = read_cn(raw_data, pos);
        self.test_txt = read_cn(raw_data, pos);
        self.alarm_id = read_cn(raw_data, pos);
        self.prog_txt = read_cn(raw_data, pos);
        self.rslt_txt = read_cn(raw_data, pos);
        if *pos < raw_data.len() {
            self.patg_num = read_uint8(raw_data, pos);
        }
        self.spin_map = read_dn(raw_data, pos, order);
        self
    }
}

impl STR {
    pub fn new() -> Self {
        STR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.cont_flg = [read_uint8(raw_data, pos)];
        self.test_num = read_u4(raw_data, pos, order);
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.psr_ref = read_u2(raw_data, pos, order);
        self.test_flg = [read_uint8(raw_data, pos)];
        self.log_typ = read_cn(raw_data, pos);
        self.test_txt = read_cn(raw_data, pos);
        self.alarm_id = read_cn(raw_data, pos);
        self.prog_txt = read_cn(raw_data, pos);
        self.rslt_txt = read_cn(raw_data, pos);
        self.z_val = read_uint8(raw_data, pos);
        self.fmu_flg = [read_uint8(raw_data, pos)];
        self.mask_map = read_dn(raw_data, pos, order);
        self.fal_map = read_dn(raw_data, pos, order);
        self.cyc_cnt_t = read_u8(raw_data, pos, order);
        self.totf_cnt = read_u4(raw_data, pos, order);
        self.totl_cnt = read_u4(raw_data, pos, order);
        self.cyc_base = read_u8(raw_data, pos, order);
        self.bit_base = read_u4(raw_data, pos, order);
        self.cond_cnt = read_u2(raw_data, pos, order);
        self.lim_cnt = read_u2(raw_data, pos, order);
        self.cyc_size = read_uint8(raw_data, pos);
        self.pmr_size = read_uint8(raw_data, pos);
        self.chn_size = read_uint8(raw_data, pos);
        self.pat_size = read_uint8(raw_data, pos);
        self.bit_size = read_uint8(raw_data, pos);
        self.u1_size = read_uint8(raw_data, pos);
        self.u2_size = read_uint8(raw_data, pos);
        self.u3_size = read_uint8(raw_data, pos);
        self.utx_size = read_uint8(raw_data, pos);
        self.cap_bgn = read_u2(raw_data, pos, order);
        // k: LIM_CNT
        self.lim_indx = read_kx_u2(raw_data, pos, order, self.lim_cnt);
        self.lim_spec = read_kx_u4(raw_data, pos, order, self.lim_cnt);
        // k: COND_CNT
        self.cond_lst = read_kx_cn(raw_data, pos, self.cond_cnt);
        self.cyc_cnt = read_u2(raw_data, pos, order);
        // k: CYC_CNT, f: CYC_SIZE
        self.cyc_ofst = read_kx_uf(raw_data, pos, order, self.cyc_cnt, self.cyc_size);
        self.pmr_cnt = read_u2(raw_data, pos, order);
        // k: PMR_CNT, f: PMR_SIZE
        self.pmr_indx = read_kx_uf(raw_data, pos, order, self.pmr_cnt, self.pmr_size);
        self.chn_cnt = read_u2(raw_data, pos, order);
        // k: CHN_CNT, f: CHN_SIZE
        self.chn_num = read_kx_uf(raw_data, pos, order, self.chn_cnt, self.chn_size);
        self.exp_cnt = read_u2(raw_data, pos, order);
        // k: EXP_CNT
        self.exp_data = read_kx_u1(raw_data, pos, self.exp_cnt);
        self.cap_cnt = read_u2(raw_data, pos, order);
        // k: CAP_CNT
        self.cap_data = read_kx_u1(raw_data, pos, self.cap_cnt);
        self.new_cnt = read_u2(raw_data, pos, order);
        // k: NEW_CNT
        self.new_data = read_kx_u1(raw_data, pos, self.new_cnt);
        self.pat_cnt = read_u2(raw_data, pos, order);
        // k: PAT_CNT, f: PAT_SIZE
        self.pat_num = read_kx_uf(raw_data, pos, order, self.pat_cnt, self.pat_size);
        self.bpos_cnt = read_u2(raw_data, pos, order);
        // k: BPOS_CNT, f: BIT_SIZE
        self.bit_pos = read_kx_uf(raw_data, pos, order, self.bpos_cnt, self.bit_size);
        self.usr1_cnt = read_u2(raw_data, pos, order);
        // k: USR1_CNT, f: U1_SIZE
        self.usr1 = read_kx_uf(raw_data, pos, order, self.usr1_cnt, self.u1_size);
        self.usr2_cnt = read_u2(raw_data, pos, order);
        // k: USR2_CNT, f: U2_SIZE
        self.usr2 = read_kx_uf(raw_data, pos, order, self.usr2_cnt, self.u2_size);
        self.usr3_cnt = read_u2(raw_data, pos, order);
        // k: USR3_CNT, f: U3_SIZE
        self.usr3 = read_kx_uf(raw_data, pos, order, self.usr3_cnt, self.u3_size);
        self.txt_cnt = read_u2(raw_data, pos, order);
        // k: TXT_CNT
        self.user_txt = read_kx_cf(raw_data, pos, self.txt_cnt, self.utx_size);
        self
    }
}

impl BPS {
    pub fn new() -> Self {
        BPS::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.seq_name = read_cn(raw_data, pos);
        self
    }
}

impl EPS {
    pub fn new() -> Self {
        EPS::default()
    }

    pub fn read_from_bytes(self, _raw_data: &[u8], _order: &ByteOrder) -> Self {
        self
    }
}

impl GDR {
    pub fn new() -> Self {
        GDR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.fld_cnt = read_u2(raw_data, pos, order);
        self.gen_data = read_vn(raw_data, pos, order, self.fld_cnt);
        self
    }
}

impl DTR {
    pub fn new() -> Self {
        DTR::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.text_dat = read_cn(raw_data, pos);
        self
    }
}

impl ReservedRec {
    pub fn new() -> Self {
        ReservedRec::default()
    }

    pub fn read_from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.raw_data = read_cn(raw_data, pos);
        self
    }
}

impl StdfRecord {
    pub fn new(rec_type: u64) -> Self {
        match rec_type {
            // rec type 15
            stdf_record_type::REC_PTR => StdfRecord::PTR(PTR::new()),
            stdf_record_type::REC_MPR => StdfRecord::MPR(MPR::new()),
            stdf_record_type::REC_FTR => StdfRecord::FTR(FTR::new()),
            stdf_record_type::REC_STR => StdfRecord::STR(STR::new()),
            // rec type 5
            stdf_record_type::REC_PIR => StdfRecord::PIR(PIR::new()),
            stdf_record_type::REC_PRR => StdfRecord::PRR(PRR::new()),
            // rec type 2
            stdf_record_type::REC_WIR => StdfRecord::WIR(WIR::new()),
            stdf_record_type::REC_WRR => StdfRecord::WRR(WRR::new()),
            stdf_record_type::REC_WCR => StdfRecord::WCR(WCR::new()),
            // rec type 50
            stdf_record_type::REC_GDR => StdfRecord::GDR(GDR::new()),
            stdf_record_type::REC_DTR => StdfRecord::DTR(DTR::new()),
            // rec type 0
            stdf_record_type::REC_FAR => StdfRecord::FAR(FAR::new()),
            stdf_record_type::REC_ATR => StdfRecord::ATR(ATR::new()),
            stdf_record_type::REC_VUR => StdfRecord::VUR(VUR::new()),
            // rec type 1
            stdf_record_type::REC_MIR => StdfRecord::MIR(MIR::new()),
            stdf_record_type::REC_MRR => StdfRecord::MRR(MRR::new()),
            stdf_record_type::REC_PCR => StdfRecord::PCR(PCR::new()),
            stdf_record_type::REC_HBR => StdfRecord::HBR(HBR::new()),
            stdf_record_type::REC_SBR => StdfRecord::SBR(SBR::new()),
            stdf_record_type::REC_PMR => StdfRecord::PMR(PMR::new()),
            stdf_record_type::REC_PGR => StdfRecord::PGR(PGR::new()),
            stdf_record_type::REC_PLR => StdfRecord::PLR(PLR::new()),
            stdf_record_type::REC_RDR => StdfRecord::RDR(RDR::new()),
            stdf_record_type::REC_SDR => StdfRecord::SDR(SDR::new()),
            stdf_record_type::REC_PSR => StdfRecord::PSR(PSR::new()),
            stdf_record_type::REC_NMR => StdfRecord::NMR(NMR::new()),
            stdf_record_type::REC_CNR => StdfRecord::CNR(CNR::new()),
            stdf_record_type::REC_SSR => StdfRecord::SSR(SSR::new()),
            stdf_record_type::REC_CDR => StdfRecord::CDR(CDR::new()),
            // rec type 10
            stdf_record_type::REC_TSR => StdfRecord::TSR(TSR::new()),
            // rec type 20
            stdf_record_type::REC_BPS => StdfRecord::BPS(BPS::new()),
            stdf_record_type::REC_EPS => StdfRecord::EPS(EPS::new()),
            // rec type 180: Reserved
            // rec type 181: Reserved
            stdf_record_type::REC_RESERVE => StdfRecord::ReservedRec(ReservedRec::new()),
            // not matched
            _ => StdfRecord::InvalidRec,
        }
    }

    pub fn get_type(&self) -> u64 {
        match &self {
            // rec type 15
            StdfRecord::PTR(_) => stdf_record_type::REC_PTR,
            StdfRecord::MPR(_) => stdf_record_type::REC_MPR,
            StdfRecord::FTR(_) => stdf_record_type::REC_FTR,
            StdfRecord::STR(_) => stdf_record_type::REC_STR,
            // rec type 5
            StdfRecord::PIR(_) => stdf_record_type::REC_PIR,
            StdfRecord::PRR(_) => stdf_record_type::REC_PRR,
            // rec type 2
            StdfRecord::WIR(_) => stdf_record_type::REC_WIR,
            StdfRecord::WRR(_) => stdf_record_type::REC_WRR,
            StdfRecord::WCR(_) => stdf_record_type::REC_WCR,
            // rec type 50
            StdfRecord::GDR(_) => stdf_record_type::REC_GDR,
            StdfRecord::DTR(_) => stdf_record_type::REC_DTR,
            // rec type 10
            StdfRecord::TSR(_) => stdf_record_type::REC_TSR,
            // rec type 1
            StdfRecord::MIR(_) => stdf_record_type::REC_MIR,
            StdfRecord::MRR(_) => stdf_record_type::REC_MRR,
            StdfRecord::PCR(_) => stdf_record_type::REC_PCR,
            StdfRecord::HBR(_) => stdf_record_type::REC_HBR,
            StdfRecord::SBR(_) => stdf_record_type::REC_SBR,
            StdfRecord::PMR(_) => stdf_record_type::REC_PMR,
            StdfRecord::PGR(_) => stdf_record_type::REC_PGR,
            StdfRecord::PLR(_) => stdf_record_type::REC_PLR,
            StdfRecord::RDR(_) => stdf_record_type::REC_RDR,
            StdfRecord::SDR(_) => stdf_record_type::REC_SDR,
            StdfRecord::PSR(_) => stdf_record_type::REC_PSR,
            StdfRecord::NMR(_) => stdf_record_type::REC_NMR,
            StdfRecord::CNR(_) => stdf_record_type::REC_CNR,
            StdfRecord::SSR(_) => stdf_record_type::REC_SSR,
            StdfRecord::CDR(_) => stdf_record_type::REC_CDR,
            // rec type 0
            StdfRecord::FAR(_) => stdf_record_type::REC_FAR,
            StdfRecord::ATR(_) => stdf_record_type::REC_ATR,
            StdfRecord::VUR(_) => stdf_record_type::REC_VUR,
            // rec type 20
            StdfRecord::BPS(_) => stdf_record_type::REC_BPS,
            StdfRecord::EPS(_) => stdf_record_type::REC_EPS,
            // rec type 180: Reserved
            // rec type 181: Reserved
            StdfRecord::ReservedRec(_) => stdf_record_type::REC_RESERVE,
            // not matched
            StdfRecord::InvalidRec => stdf_record_type::REC_INVALID,
        }
    }

    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.get_type() & rec_type) != 0
    }

    pub fn read_from_bytes(self, raw_data: &[u8], order: &ByteOrder) -> Self {
        match self {
            // rec type 15
            StdfRecord::PTR(ptr_rec) => StdfRecord::PTR(ptr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::MPR(mpr_rec) => StdfRecord::MPR(mpr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::FTR(ftr_rec) => StdfRecord::FTR(ftr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::STR(str_rec) => StdfRecord::STR(str_rec.read_from_bytes(raw_data, order)),
            // rec type 5
            StdfRecord::PIR(pir_rec) => StdfRecord::PIR(pir_rec.read_from_bytes(raw_data, order)),
            StdfRecord::PRR(prr_rec) => StdfRecord::PRR(prr_rec.read_from_bytes(raw_data, order)),
            // rec type 2
            StdfRecord::WIR(wir_rec) => StdfRecord::WIR(wir_rec.read_from_bytes(raw_data, order)),
            StdfRecord::WRR(wrr_rec) => StdfRecord::WRR(wrr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::WCR(wcr_rec) => StdfRecord::WCR(wcr_rec.read_from_bytes(raw_data, order)),
            // rec type 50
            StdfRecord::GDR(gdr_rec) => StdfRecord::GDR(gdr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::DTR(dtr_rec) => StdfRecord::DTR(dtr_rec.read_from_bytes(raw_data, order)),
            // rec type 10
            StdfRecord::TSR(tsr_rec) => StdfRecord::TSR(tsr_rec.read_from_bytes(raw_data, order)),
            // rec type 1
            StdfRecord::MIR(mir_rec) => StdfRecord::MIR(mir_rec.read_from_bytes(raw_data, order)),
            StdfRecord::MRR(mrr_rec) => StdfRecord::MRR(mrr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::PCR(pcr_rec) => StdfRecord::PCR(pcr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::HBR(hbr_rec) => StdfRecord::HBR(hbr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::SBR(sbr_rec) => StdfRecord::SBR(sbr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::PMR(pmr_rec) => StdfRecord::PMR(pmr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::PGR(pgr_rec) => StdfRecord::PGR(pgr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::PLR(plr_rec) => StdfRecord::PLR(plr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::RDR(rdr_rec) => StdfRecord::RDR(rdr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::SDR(sdr_rec) => StdfRecord::SDR(sdr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::PSR(psr_rec) => StdfRecord::PSR(psr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::NMR(nmr_rec) => StdfRecord::NMR(nmr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::CNR(cnr_rec) => StdfRecord::CNR(cnr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::SSR(ssr_rec) => StdfRecord::SSR(ssr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::CDR(cdr_rec) => StdfRecord::CDR(cdr_rec.read_from_bytes(raw_data, order)),
            // rec type 0
            StdfRecord::FAR(far_rec) => StdfRecord::FAR(far_rec.read_from_bytes(raw_data, order)),
            StdfRecord::ATR(atr_rec) => StdfRecord::ATR(atr_rec.read_from_bytes(raw_data, order)),
            StdfRecord::VUR(vur_rec) => StdfRecord::VUR(vur_rec.read_from_bytes(raw_data, order)),
            // rec type 20
            StdfRecord::BPS(bps_rec) => StdfRecord::BPS(bps_rec.read_from_bytes(raw_data, order)),
            StdfRecord::EPS(eps_rec) => StdfRecord::EPS(eps_rec.read_from_bytes(raw_data, order)),
            // rec type 180: Reserved
            // rec type 181: Reserved
            StdfRecord::ReservedRec(reserve_rec) => {
                StdfRecord::ReservedRec(reserve_rec.read_from_bytes(raw_data, order))
            }
            // not matched
            StdfRecord::InvalidRec => self,
        }
    }
}

impl From<&AtdfRecord> for StdfRecord {
    fn from(atdf_rec: &AtdfRecord) -> Self {
        let mut stdf_rec = StdfRecord::new(atdf_rec.type_code);

        stdf_rec
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

// data type functions
/// Read uint8 from byte array with offset "pos", compatible with B1, C1 and U1
#[inline(always)]
pub(crate) fn read_uint8(raw_data: &[u8], pos: &mut usize) -> u8 {
    if *pos < raw_data.len() {
        let value = (*raw_data)[*pos];
        *pos += 1;
        value
    } else {
        0
    }
}

/// Read U2 (u16) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_u2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U2 {
    let pos_after_read = *pos + 2;

    if pos_after_read <= raw_data.len() {
        let mut tmp = [0u8; 2];
        tmp.copy_from_slice(&raw_data[*pos..pos_after_read]);
        *pos = pos_after_read;
        match order {
            ByteOrder::LittleEndian => U2::from_le_bytes(tmp),
            ByteOrder::BigEndian => U2::from_be_bytes(tmp),
        }
    } else {
        0
    }
}

/// Read U4 (u32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U4 {
    let pos_after_read = *pos + 4;

    if pos_after_read <= raw_data.len() {
        let mut tmp = [0u8; 4];
        tmp.copy_from_slice(&raw_data[*pos..pos_after_read]);
        *pos = pos_after_read;
        match order {
            ByteOrder::LittleEndian => U4::from_le_bytes(tmp),
            ByteOrder::BigEndian => U4::from_be_bytes(tmp),
        }
    } else {
        0
    }
}

/// Read U8 (u64) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U8 {
    let pos_after_read = *pos + 8;

    if pos_after_read <= raw_data.len() {
        let mut tmp = [0u8; 8];
        tmp.copy_from_slice(&raw_data[*pos..pos_after_read]);
        *pos = pos_after_read;
        match order {
            ByteOrder::LittleEndian => U8::from_le_bytes(tmp),
            ByteOrder::BigEndian => U8::from_be_bytes(tmp),
        }
    } else {
        0
    }
}

/// Read I1 (i8) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_i1(raw_data: &[u8], pos: &mut usize) -> I1 {
    if *pos < raw_data.len() {
        let value = (*raw_data)[*pos] as I1;
        *pos += 1;
        value
    } else {
        0
    }
}

/// Read I2 (i16) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_i2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> I2 {
    let pos_after_read = *pos + 2;

    if pos_after_read <= raw_data.len() {
        let mut tmp = [0u8; 2];
        tmp.copy_from_slice(&raw_data[*pos..pos_after_read]);
        *pos = pos_after_read;
        match order {
            ByteOrder::LittleEndian => I2::from_le_bytes(tmp),
            ByteOrder::BigEndian => I2::from_be_bytes(tmp),
        }
    } else {
        0
    }
}

/// Read I4 (i32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_i4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> I4 {
    let pos_after_read = *pos + 4;

    if pos_after_read <= raw_data.len() {
        let mut tmp = [0u8; 4];
        tmp.copy_from_slice(&raw_data[*pos..pos_after_read]);
        *pos = pos_after_read;
        match order {
            ByteOrder::LittleEndian => I4::from_le_bytes(tmp),
            ByteOrder::BigEndian => I4::from_be_bytes(tmp),
        }
    } else {
        0
    }
}

/// Read R4 (f32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_r4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> R4 {
    let pos_after_read = *pos + 4;

    if pos_after_read <= raw_data.len() {
        let mut tmp = [0u8; 4];
        tmp.copy_from_slice(&raw_data[*pos..pos_after_read]);
        *pos = pos_after_read;
        match order {
            ByteOrder::LittleEndian => R4::from_le_bytes(tmp),
            ByteOrder::BigEndian => R4::from_be_bytes(tmp),
        }
    } else {
        0.0
    }
}

/// Read R8 (f64) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_r8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> R8 {
    let pos_after_read = *pos + 8;

    if pos_after_read <= raw_data.len() {
        let mut tmp = [0u8; 8];
        tmp.copy_from_slice(&raw_data[*pos..pos_after_read]);
        *pos = pos_after_read;
        match order {
            ByteOrder::LittleEndian => R8::from_le_bytes(tmp),
            ByteOrder::BigEndian => R8::from_be_bytes(tmp),
        }
    } else {
        0.0
    }
}

/// Read Cn (u8 + String) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_cn(raw_data: &[u8], pos: &mut usize) -> Cn {
    let count = read_uint8(raw_data, pos) as usize;
    let mut value = String::default();
    if count != 0 {
        let pos_after_read = *pos + count;
        if pos_after_read <= raw_data.len() {
            // read count
            value = bytes_to_string(&raw_data[*pos..pos_after_read]);
            *pos = pos_after_read;
        } else {
            // read all
            value = bytes_to_string(&raw_data[*pos..]);
            *pos = raw_data.len();
        }
    }
    value
}

/// Read Sn (u16 + String) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> Sn {
    let count = read_u2(raw_data, pos, order) as usize;
    let mut value = String::default();
    if count != 0 {
        let pos_after_read = *pos + count;
        if pos_after_read <= raw_data.len() {
            // read count
            value = bytes_to_string(&raw_data[*pos..pos_after_read]);
            *pos = pos_after_read;
        } else {
            // read all
            value = bytes_to_string(&raw_data[*pos..]);
            *pos = raw_data.len();
        }
    }
    value
}

/// Read Cf (String) from byte array with offset "pos", String length is provide by "f"
#[inline(always)]
pub(crate) fn read_cf(raw_data: &[u8], pos: &mut usize, f: u8) -> Cf {
    let mut value = String::default();
    if f != 0 {
        let pos_after_read = *pos + (f as usize);
        if pos_after_read <= raw_data.len() {
            // read count
            value = bytes_to_string(&raw_data[*pos..pos_after_read]);
            *pos = pos_after_read;
        } else {
            // read all
            value = bytes_to_string(&raw_data[*pos..]);
            *pos = raw_data.len();
        }
    }
    value
}

/// Read Bn (u8 + Vec<u8>) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_bn(raw_data: &[u8], pos: &mut usize) -> Bn {
    let count = read_uint8(raw_data, pos) as usize;
    if count != 0 {
        let pos_after_read = *pos + count;
        let data_slice: &[u8];
        if pos_after_read <= raw_data.len() {
            // read count
            data_slice = &raw_data[*pos..pos_after_read];
            *pos = pos_after_read;
        } else {
            // read all
            data_slice = &raw_data[*pos..];
            *pos = raw_data.len();
        }
        let mut value = vec![0u8; data_slice.len()];
        value.copy_from_slice(data_slice);
        value
    } else {
        vec![0u8; 0]
    }
}

/// Read Dn (u16 + Vec<u8>) from byte array with offset "pos", u16 is bit counts
#[inline(always)]
pub(crate) fn read_dn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> Dn {
    let bitcount = read_u2(raw_data, pos, order) as usize;
    let bytecount = bitcount / 8 + bitcount % 8;
    if bytecount != 0 {
        let pos_after_read = *pos + bytecount;
        let data_slice: &[u8];
        if pos_after_read <= raw_data.len() {
            // read count
            data_slice = &raw_data[*pos..pos_after_read];
            *pos = pos_after_read;
        } else {
            // read all
            data_slice = &raw_data[*pos..];
            *pos = raw_data.len();
        }
        let mut value = vec![0u8; data_slice.len()];
        value.copy_from_slice(data_slice);
        value
    } else {
        vec![0u8; 0]
    }
}

/// Read KxCn (Vec<Cn>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_cn(raw_data: &[u8], pos: &mut usize, k: u16) -> KxCn {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_cn(raw_data, pos));
        }
        value
    } else {
        vec!["".to_string(); 0]
    }
}

/// Read KxSn (Vec<Sn>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxSn {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_sn(raw_data, pos, order));
        }
        value
    } else {
        vec!["".to_string(); 0]
    }
}

/// Read KxCf (Vec<Cf>) from byte array with offset "pos", vector size is provide by "k", String size is "f"
#[inline(always)]
pub(crate) fn read_kx_cf(raw_data: &[u8], pos: &mut usize, k: u16, f: u8) -> KxCf {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_cf(raw_data, pos, f));
        }
        value
    } else {
        vec!["".to_string(); 0]
    }
}

/// Read KxU1 (Vec<u8>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u1(raw_data: &[u8], pos: &mut usize, k: u16) -> KxU1 {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_uint8(raw_data, pos));
        }
        value
    } else {
        vec![0u8; 0]
    }
}

/// Read KxU2 (Vec<u16>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU2 {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_u2(raw_data, pos, order));
        }
        value
    } else {
        vec![0u16; 0]
    }
}

/// Read KxU4 (Vec<u32>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU4 {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_u4(raw_data, pos, order));
        }
        value
    } else {
        vec![0u32; 0]
    }
}

/// Read KxU8 (Vec<u64>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU8 {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_u8(raw_data, pos, order));
        }
        value
    } else {
        vec![0u64; 0]
    }
}

/// Read KxUf (Vec<u8|u16|u32|u64>) from byte array with offset "pos", vector size is provide by "k", size of number is "f"
#[inline(always)]
pub(crate) fn read_kx_uf(
    raw_data: &[u8],
    pos: &mut usize,
    order: &ByteOrder,
    k: u16,
    f: u8,
) -> KxUf {
    if k != 0 {
        match f {
            1 => KxUf::F1(read_kx_u1(raw_data, pos, k)),
            2 => KxUf::F2(read_kx_u2(raw_data, pos, order, k)),
            4 => KxUf::F4(read_kx_u4(raw_data, pos, order, k)),
            8 => KxUf::F8(read_kx_u8(raw_data, pos, order, k)),
            _ => KxUf::F1(vec![0u8; 0]),
        }
    } else {
        KxUf::F1(vec![0u8; 0])
    }
}

/// Read KxR4 (Vec<f32>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_r4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxR4 {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_r4(raw_data, pos, order));
        }
        value
    } else {
        vec![0.0f32; 0]
    }
}

/// Read KxN1 (Vec<u8>) from byte array with offset "pos", vector size is provide by "k"
///
/// size of N1 = 4 bits, hence total bytes of k * N1 = k/2 + k%2
#[inline(always)]
pub(crate) fn read_kx_n1(raw_data: &[u8], pos: &mut usize, k: u16) -> KxN1 {
    if k != 0 {
        let bytecount = k / 2 + k % 2; // k = nibble counts, 1 byte = 2 nibble
        let mut value = Vec::with_capacity(k as usize);
        for i in 0..bytecount {
            let tmp = read_uint8(raw_data, pos);
            value.push(tmp & 0x0F);
            if (2 * i + 1) < k {
                value.push((tmp & 0xF0) >> 4);
            }
        }
        value
    } else {
        vec![0u8; 0]
    }
}

/// Read V1 (u8 + generic value) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_v1(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> V1 {
    let type_byte = if (*pos as usize) < raw_data.len() {
        read_uint8(raw_data, pos)
    } else {
        0xF
    };

    match type_byte {
        0 => V1::B0,
        1 => V1::U1(read_uint8(raw_data, pos)),
        2 => V1::U2(read_u2(raw_data, pos, order)),
        3 => V1::U4(read_u4(raw_data, pos, order)),
        4 => V1::I1(read_i1(raw_data, pos)),
        5 => V1::I2(read_i2(raw_data, pos, order)),
        6 => V1::I4(read_i4(raw_data, pos, order)),
        7 => V1::R4(read_r4(raw_data, pos, order)),
        8 => V1::R8(read_r8(raw_data, pos, order)),
        10 => V1::Cn(read_cn(raw_data, pos)),
        11 => V1::Bn(read_bn(raw_data, pos)),
        12 => V1::Dn(read_dn(raw_data, pos, order)),
        13 => V1::N1(read_uint8(raw_data, pos) & 0x0F),
        _ => V1::Invalid,
    }
}

/// Read Vn (Vec<V1>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_vn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> Vn {
    if k != 0 {
        let mut value = Vec::with_capacity(k as usize);
        for _ in 0..k {
            value.push(read_v1(raw_data, pos, order));
        }
        value
    } else {
        vec![V1::Invalid; 0]
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

#[inline(always)]
pub(crate) fn bytes_to_string(data: &[u8]) -> String {
    data.iter().map(|&x| x as char).collect()
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

pub(crate) fn atdf_data_from_ptr(rec: &PTR) -> Vec<String> {
    vec![]
}

pub(crate) fn atdf_data_from_mpr(rec: &MPR) -> Vec<String> {
    vec![]
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
    vec![
        ser_stdf_kx_data(&rec.grp_indx), //
                                         //TODO
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
