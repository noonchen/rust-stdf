//
// stdf_types.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Mon Oct 31 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use crate::stdf_error::StdfError;
extern crate smart_default;
use smart_default::SmartDefault;
use std::convert::From;

macro_rules! read_optional {
    ($var:expr, [$func:ident($raw:expr, $pos:expr)], $min_bytes:expr) => {{
        if *$pos + $min_bytes > $raw.len() {
            $var = None;
            return;
        } else {
            $var = Some([$func($raw, $pos)]);
        }
    }};
    ($var:expr, $func:ident($raw:expr, $pos:expr), $min_bytes:expr) => {{
        if *$pos + $min_bytes > $raw.len() {
            $var = None;
            return;
        } else {
            $var = Some($func($raw, $pos));
        }
    }};
    ($var:expr, $func:ident($raw:expr, $pos:expr, $order:expr), $min_bytes:expr) => {{
        if *$pos + $min_bytes > $raw.len() {
            $var = None;
        } else {
            $var = Some($func($raw, $pos, $order));
        }
    }};
    ($var:expr, $func:ident($raw:expr, $pos:expr, $order:expr, $cnt:expr), $element_bytes:expr) => {{
        if *$pos + $element_bytes * $cnt as usize > $raw.len() {
            $var = None;
        } else {
            $var = Some($func($raw, $pos, $order, $cnt));
        }
    }};
}

// Common Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressType {
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
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq)]
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
/// for STDF Record type check and
/// some help functions
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
    use crate::stdf_error::StdfError;

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

    /// This function convert record type constant to
    /// STDF record (typ, sub)
    ///
    /// ```
    /// use rust_stdf::stdf_record_type::*;
    ///
    /// let ptr_typ_sub = get_typ_sub_from_code(REC_PTR).unwrap();
    /// assert_eq!((15, 10), ptr_typ_sub);
    /// ```
    #[inline(always)]
    pub fn get_typ_sub_from_code(code: u64) -> Result<(u8, u8), StdfError> {
        match code {
            // rec type 15
            REC_PTR => Ok((15, 10)),
            REC_MPR => Ok((15, 15)),
            REC_FTR => Ok((15, 20)),
            REC_STR => Ok((15, 30)),
            // rec type 5
            REC_PIR => Ok((5, 10)),
            REC_PRR => Ok((5, 20)),
            // rec type 2
            REC_WIR => Ok((2, 10)),
            REC_WRR => Ok((2, 20)),
            REC_WCR => Ok((2, 30)),
            // rec type 50
            REC_GDR => Ok((50, 10)),
            REC_DTR => Ok((50, 30)),
            // rec type 0
            REC_FAR => Ok((0, 10)),
            REC_ATR => Ok((0, 20)),
            REC_VUR => Ok((0, 30)),
            // rec type 1
            REC_MIR => Ok((1, 10)),
            REC_MRR => Ok((1, 20)),
            REC_PCR => Ok((1, 30)),
            REC_HBR => Ok((1, 40)),
            REC_SBR => Ok((1, 50)),
            REC_PMR => Ok((1, 60)),
            REC_PGR => Ok((1, 62)),
            REC_PLR => Ok((1, 63)),
            REC_RDR => Ok((1, 70)),
            REC_SDR => Ok((1, 80)),
            REC_PSR => Ok((1, 90)),
            REC_NMR => Ok((1, 91)),
            REC_CNR => Ok((1, 92)),
            REC_SSR => Ok((1, 93)),
            REC_CDR => Ok((1, 94)),
            // rec type 10
            REC_TSR => Ok((10, 30)),
            // rec type 20
            REC_BPS => Ok((20, 10)),
            REC_EPS => Ok((20, 20)),
            // rec type 180: Reserved
            // rec type 181: Reserved
            // REC_RESERVE,(180 | 181, _)
            // not matched
            // REC_INVALID,(_, _)
            _ => Err(StdfError {
                code: 2,
                msg: "unknown type constant".to_string(),
            }),
        }
    }

    /// This function convert (typ, sub) to
    /// STDF record type constant
    ///
    /// ```
    /// use rust_stdf::stdf_record_type::*;
    ///
    /// let type_code = get_code_from_typ_sub(15, 10);
    /// assert_eq!(REC_PTR, type_code);
    /// ```
    #[inline(always)]
    pub fn get_code_from_typ_sub(typ: u8, sub: u8) -> u64 {
        match (typ, sub) {
            // rec type 15
            (15, 10) => REC_PTR,
            (15, 15) => REC_MPR,
            (15, 20) => REC_FTR,
            (15, 30) => REC_STR,
            // rec type 5
            (5, 10) => REC_PIR,
            (5, 20) => REC_PRR,
            // rec type 2
            (2, 10) => REC_WIR,
            (2, 20) => REC_WRR,
            (2, 30) => REC_WCR,
            // rec type 50
            (50, 10) => REC_GDR,
            (50, 30) => REC_DTR,
            // rec type 0
            (0, 10) => REC_FAR,
            (0, 20) => REC_ATR,
            (0, 30) => REC_VUR,
            // rec type 1
            (1, 10) => REC_MIR,
            (1, 20) => REC_MRR,
            (1, 30) => REC_PCR,
            (1, 40) => REC_HBR,
            (1, 50) => REC_SBR,
            (1, 60) => REC_PMR,
            (1, 62) => REC_PGR,
            (1, 63) => REC_PLR,
            (1, 70) => REC_RDR,
            (1, 80) => REC_SDR,
            (1, 90) => REC_PSR,
            (1, 91) => REC_NMR,
            (1, 92) => REC_CNR,
            (1, 93) => REC_SSR,
            (1, 94) => REC_CDR,
            // rec type 10
            (10, 30) => REC_TSR,
            // rec type 20
            (20, 10) => REC_BPS,
            (20, 20) => REC_EPS,
            // rec type 180: Reserved
            // rec type 181: Reserved
            (180 | 181, _) => REC_RESERVE,
            // not matched
            (_, _) => REC_INVALID,
        }
    }

    /// This function convert record type constant to
    /// STDF record name string
    ///
    /// ```
    /// use rust_stdf::stdf_record_type::*;
    ///
    /// let rec_name = get_rec_name_from_code(REC_PTR);
    /// assert_eq!("PTR", rec_name);
    /// ```
    #[inline(always)]
    pub fn get_rec_name_from_code(rec_type: u64) -> &'static str {
        match rec_type {
            // rec type 15
            REC_PTR => "PTR",
            REC_MPR => "MPR",
            REC_FTR => "FTR",
            REC_STR => "STR",
            // rec type 5
            REC_PIR => "PIR",
            REC_PRR => "PRR",
            // rec type 2
            REC_WIR => "WIR",
            REC_WRR => "WRR",
            REC_WCR => "WCR",
            // rec type 50
            REC_GDR => "GDR",
            REC_DTR => "DTR",
            // rec type 0
            REC_FAR => "FAR",
            REC_ATR => "ATR",
            REC_VUR => "VUR",
            // rec type 1
            REC_MIR => "MIR",
            REC_MRR => "MRR",
            REC_PCR => "PCR",
            REC_HBR => "HBR",
            REC_SBR => "SBR",
            REC_PMR => "PMR",
            REC_PGR => "PGR",
            REC_PLR => "PLR",
            REC_RDR => "RDR",
            REC_SDR => "SDR",
            REC_PSR => "PSR",
            REC_NMR => "NMR",
            REC_CNR => "CNR",
            REC_SSR => "SSR",
            REC_CDR => "CDR",
            // rec type 10
            REC_TSR => "TSR",
            // rec type 20
            REC_BPS => "BPS",
            REC_EPS => "EPS",
            // rec type 180: Reserved
            // rec type 181: Reserved
            REC_RESERVE => "ReservedRec",
            // not matched
            _ => "InvalidRec",
        }
    }

    /// This function convert record name string to
    /// STDF record type constant
    ///
    /// ```
    /// use rust_stdf::stdf_record_type::*;
    ///
    /// let type_code = get_code_from_rec_name("PTR");
    /// assert_eq!(REC_PTR, type_code);
    /// ```
    ///
    #[inline(always)]
    pub fn get_code_from_rec_name(rec_name: &str) -> u64 {
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
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
/// unprocessed STDF record data, contains:
///  - offset
///  - type_code
///  - raw_data
///  - byte_order
///
/// it can be converted back to `StdfRecord`
/// ```
/// use rust_stdf::{RawDataElement, ByteOrder, StdfRecord, stdf_record_type::REC_FAR};
///
/// let rde = RawDataElement {
///     offset: 0,
///     type_code: 1,
///     raw_data: vec![0u8; 0],
///     byte_order: ByteOrder::LittleEndian
/// };
/// let rec: StdfRecord = (&rde).into();    // not consume
/// let rec: StdfRecord = rde.into();       // consume
/// println!("{:?}", rec);
/// assert!(rec.is_type(REC_FAR));
/// ```
pub struct RawDataElement {
    /// file offset of `raw_data` in file,
    /// after header.len and before raw_data
    ///
    /// |-typ-|-sub-|--len--⬇️--raw..data--|
    ///
    /// note that the offset is relative to the
    /// file position that runs `get_rawdata_iter`,
    ///
    /// it can be treated as file position **only if**
    /// the iteration starts from beginning of the file.
    pub offset: u64,

    /// used for filtering and creating StdfRecord
    pub type_code: u64,

    /// field data of current STDF Record
    pub raw_data: Vec<u8>,
    pub byte_order: ByteOrder,
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct FAR {
    pub cpu_type: U1, // CPU type that wrote this file
    pub stdf_ver: U1, // STDF version number
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct ATR {
    pub mod_tim: U4,  //Date and time of STDF file modification
    pub cmd_line: Cn, //Command line of program
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct VUR {
    pub upd_nam: Cn, //Update Version Name
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct MRR {
    pub finish_t: U4, // Date and time last part tested
    #[default = ' ']
    pub disp_cod: C1, // Lot disposition code,default: space
    pub usr_desc: Cn, // Lot description supplied by user
    pub exc_desc: Cn, // Lot description supplied by exec
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct HBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub hbin_num: U2, // Hardware bin number
    pub hbin_cnt: U4, // Number of parts in bin
    #[default = ' ']
    pub hbin_pf: C1, // Pass/fail indication
    pub hbin_nam: Cn, // Name of hardware bin
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct SBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub sbin_num: U2, // Software bin number
    pub sbin_cnt: U4, // Number of parts in bin
    #[default = ' ']
    pub sbin_pf: C1, // Pass/fail indication
    pub sbin_nam: Cn, // Name of software bin
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct PGR {
    pub grp_indx: U2,   // Unique index associated with pin group
    pub grp_nam: Cn,    // Name of pin group
    pub indx_cnt: U2,   // Count of PMR indexes
    pub pmr_indx: KxU2, // Array of indexes for pins in the group
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct RDR {
    pub num_bins: U2,   // Number (k) of bins being retested
    pub rtst_bin: KxU2, // Array of retest bin numbers
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct NMR {
    pub cont_flg: B1,   // Continuation NMR record follows if not 0
    pub totm_cnt: U2,   // Count of PMR indexes and ATPG_NAM entries
    pub locm_cnt: U2,   // Count of (k) PMR indexes and ATPG_NAM entries in this record
    pub pmr_indx: KxU2, // Array of PMR indexes
    pub atpg_nam: KxCn, // Array of ATPG signal names
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct CNR {
    pub chn_num: U2,  // Chain number. Referenced by the CHN_NUM array in an STR record
    pub bit_pos: U4,  // Bit position in the chain
    pub cell_nam: Sn, // Scan Cell Name
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct SSR {
    pub ssr_nam: Cn,    // Name of the STIL Scan pub structure for reference
    pub chn_cnt: U2,    // Count (k) of number of Chains listed in CHN_LIST
    pub chn_list: KxU2, // Array of CDR Indexes
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct WIR {
    pub head_num: U1, // Test head number
    #[default = 255]
    pub site_grp: U1, // Site group number
    pub start_t: U4,  // Date and time first part tested
    pub wafer_id: Cn, // Wafer ID length byte = 0
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct PIR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct PTR {
    pub test_num: U4,         // Test number
    pub head_num: U1,         // Test head number
    pub site_num: U1,         // Test site number
    pub test_flg: B1,         // Test flags (fail, alarm, etc.)
    pub parm_flg: B1,         // Parametric test flags (drift, etc.)
    pub result: R4,           // Test result
    pub test_txt: Cn,         // Test description text or label
    pub alarm_id: Cn,         // Name of alarm
    pub opt_flag: Option<B1>, // Optional data flag
    pub res_scal: Option<I1>, // Test results scaling exponent
    pub llm_scal: Option<I1>, // Low limit scaling exponent
    pub hlm_scal: Option<I1>, // High limit scaling exponent
    pub lo_limit: Option<R4>, // Low test limit value
    pub hi_limit: Option<R4>, // High test limit value
    pub units: Option<Cn>,    // Test units
    pub c_resfmt: Option<Cn>, // ANSI C result format string
    pub c_llmfmt: Option<Cn>, // ANSI C low limit format string
    pub c_hlmfmt: Option<Cn>, // ANSI C high limit format string
    pub lo_spec: Option<R4>,  // Low specification limit value
    pub hi_spec: Option<R4>,  // High specification limit value
}

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct MPR {
    pub test_num: U4,           // Test number
    pub head_num: U1,           // Test head number
    pub site_num: U1,           // Test site number
    pub test_flg: B1,           // Test flags (fail, alarm, etc.)
    pub parm_flg: B1,           // Parametric test flags (drift, etc.)
    pub rtn_icnt: U2,           // Count of PMR indexes
    pub rslt_cnt: U2,           // Count of returned results
    pub rtn_stat: KxN1,         // Array of returned states
    pub rtn_rslt: KxR4,         // Array of returned results
    pub test_txt: Cn,           // Descriptive text or label
    pub alarm_id: Cn,           // Name of alarm
    pub opt_flag: Option<B1>,   // Optional data flag
    pub res_scal: Option<I1>,   // Test result scaling exponent
    pub llm_scal: Option<I1>,   // Test low limit scaling exponent
    pub hlm_scal: Option<I1>,   // Test high limit scaling exponent
    pub lo_limit: Option<R4>,   // Test low limit value
    pub hi_limit: Option<R4>,   // Test high limit value
    pub start_in: Option<R4>,   // Starting input value (condition)
    pub incr_in: Option<R4>,    // Increment of input condition
    pub rtn_indx: Option<KxU2>, // Array of PMR indexes
    pub units: Option<Cn>,      // Units of returned results
    pub units_in: Option<Cn>,   // Input condition units
    pub c_resfmt: Option<Cn>,   // ANSI C result format string
    pub c_llmfmt: Option<Cn>,   // ANSI C low limit format string
    pub c_hlmfmt: Option<Cn>,   // ANSI C high limit format string
    pub lo_spec: Option<R4>,    // Low specification limit value
    pub hi_spec: Option<R4>,    // High specification limit value
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct BPS {
    pub seq_name: Cn, // Program section (or sequencer) name length byte = 0
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct EPS {}

#[derive(SmartDefault, Debug, Clone, PartialEq)]
pub struct GDR {
    pub fld_cnt: U2,  // Count of data fields in record
    pub gen_data: Vn, // Data type code and data for one field(Repeat GEN_DATA for each data field)
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct DTR {
    pub text_dat: Cn, // ASCII text string
}

#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
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
            self.type_code = stdf_record_type::get_code_from_typ_sub(self.typ, self.sub);
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

    pub fn read_from_bytes(&mut self, raw_data: &[u8], _order: &ByteOrder) {
        let pos = &mut 0;
        self.cpu_type = read_uint8(raw_data, pos);
        self.stdf_ver = read_uint8(raw_data, pos);
    }
}

impl ATR {
    pub fn new() -> Self {
        ATR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.mod_tim = read_u4(raw_data, pos, order);
        self.cmd_line = read_cn(raw_data, pos);
    }
}

impl VUR {
    pub fn new() -> Self {
        VUR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], _order: &ByteOrder) {
        let pos = &mut 0;
        self.upd_nam = read_cn(raw_data, pos);
    }
}

impl MIR {
    pub fn new() -> Self {
        MIR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl MRR {
    pub fn new() -> Self {
        MRR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.finish_t = read_u4(raw_data, pos, order);
        if *pos < raw_data.len() {
            self.disp_cod = read_uint8(raw_data, pos) as char;
        }
        self.usr_desc = read_cn(raw_data, pos);
        self.exc_desc = read_cn(raw_data, pos);
    }
}

impl PCR {
    pub fn new() -> Self {
        PCR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl HBR {
    pub fn new() -> Self {
        HBR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.hbin_num = read_u2(raw_data, pos, order);
        self.hbin_cnt = read_u4(raw_data, pos, order);
        if *pos < raw_data.len() {
            self.hbin_pf = read_uint8(raw_data, pos) as char;
        }
        self.hbin_nam = read_cn(raw_data, pos);
    }
}

impl SBR {
    pub fn new() -> Self {
        SBR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.sbin_num = read_u2(raw_data, pos, order);
        self.sbin_cnt = read_u4(raw_data, pos, order);
        if *pos < raw_data.len() {
            self.sbin_pf = read_uint8(raw_data, pos) as char;
        }
        self.sbin_nam = read_cn(raw_data, pos);
    }
}

impl PMR {
    pub fn new() -> Self {
        PMR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl PGR {
    pub fn new() -> Self {
        PGR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.grp_indx = read_u2(raw_data, pos, order);
        self.grp_nam = read_cn(raw_data, pos);
        self.indx_cnt = read_u2(raw_data, pos, order);
        self.pmr_indx = read_kx_u2(raw_data, pos, order, self.indx_cnt);
    }
}

impl PLR {
    pub fn new() -> Self {
        PLR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.grp_cnt = read_u2(raw_data, pos, order);
        self.grp_indx = read_kx_u2(raw_data, pos, order, self.grp_cnt);
        self.grp_mode = read_kx_u2(raw_data, pos, order, self.grp_cnt);
        self.grp_radx = read_kx_u1(raw_data, pos, self.grp_cnt);
        self.pgm_char = read_kx_cn(raw_data, pos, self.grp_cnt);
        self.rtn_char = read_kx_cn(raw_data, pos, self.grp_cnt);
        self.pgm_chal = read_kx_cn(raw_data, pos, self.grp_cnt);
        self.rtn_chal = read_kx_cn(raw_data, pos, self.grp_cnt);
    }
}

impl RDR {
    pub fn new() -> Self {
        RDR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.num_bins = read_u2(raw_data, pos, order);
        self.rtst_bin = read_kx_u2(raw_data, pos, order, self.num_bins);
    }
}

impl SDR {
    pub fn new() -> Self {
        SDR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], _order: &ByteOrder) {
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
    }
}

impl PSR {
    pub fn new() -> Self {
        PSR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl NMR {
    pub fn new() -> Self {
        NMR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.cont_flg = [read_uint8(raw_data, pos)];
        self.totm_cnt = read_u2(raw_data, pos, order);
        self.locm_cnt = read_u2(raw_data, pos, order);
        self.pmr_indx = read_kx_u2(raw_data, pos, order, self.locm_cnt);
        self.atpg_nam = read_kx_cn(raw_data, pos, self.locm_cnt);
    }
}

impl CNR {
    pub fn new() -> Self {
        CNR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.chn_num = read_u2(raw_data, pos, order);
        self.bit_pos = read_u4(raw_data, pos, order);
        self.cell_nam = read_sn(raw_data, pos, order);
    }
}

impl SSR {
    pub fn new() -> Self {
        SSR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.ssr_nam = read_cn(raw_data, pos);
        self.chn_cnt = read_u2(raw_data, pos, order);
        self.chn_list = read_kx_u2(raw_data, pos, order, self.chn_cnt);
    }
}

impl CDR {
    pub fn new() -> Self {
        CDR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl WIR {
    pub fn new() -> Self {
        WIR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        if *pos < raw_data.len() {
            self.site_grp = read_uint8(raw_data, pos);
        }
        self.start_t = read_u4(raw_data, pos, order);
        self.wafer_id = read_cn(raw_data, pos);
    }
}

impl WRR {
    pub fn new() -> Self {
        WRR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl WCR {
    pub fn new() -> Self {
        WCR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl PIR {
    pub fn new() -> Self {
        PIR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], _order: &ByteOrder) {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
    }
}

impl PRR {
    pub fn new() -> Self {
        PRR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl TSR {
    pub fn new() -> Self {
        TSR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl PTR {
    pub fn new() -> Self {
        PTR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.test_num = read_u4(raw_data, pos, order);
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.test_flg = [read_uint8(raw_data, pos)];
        self.parm_flg = [read_uint8(raw_data, pos)];
        self.result = read_r4(raw_data, pos, order);
        self.test_txt = read_cn(raw_data, pos);
        self.alarm_id = read_cn(raw_data, pos);
        read_optional!(self.opt_flag, [read_uint8(raw_data, pos)], 1);
        read_optional!(self.res_scal, read_i1(raw_data, pos), 1);
        read_optional!(self.llm_scal, read_i1(raw_data, pos), 1);
        read_optional!(self.hlm_scal, read_i1(raw_data, pos), 1);
        read_optional!(self.lo_limit, read_r4(raw_data, pos, order), 4);
        read_optional!(self.hi_limit, read_r4(raw_data, pos, order), 4);
        read_optional!(self.units, read_cn(raw_data, pos), 1);
        read_optional!(self.c_resfmt, read_cn(raw_data, pos), 1);
        read_optional!(self.c_llmfmt, read_cn(raw_data, pos), 1);
        read_optional!(self.c_hlmfmt, read_cn(raw_data, pos), 1);
        read_optional!(self.lo_spec, read_r4(raw_data, pos, order), 4);
        read_optional!(self.hi_spec, read_r4(raw_data, pos, order), 4);
    }
}

impl MPR {
    pub fn new() -> Self {
        MPR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
        read_optional!(self.opt_flag, [read_uint8(raw_data, pos)], 1);
        read_optional!(self.res_scal, read_i1(raw_data, pos), 1);
        read_optional!(self.llm_scal, read_i1(raw_data, pos), 1);
        read_optional!(self.hlm_scal, read_i1(raw_data, pos), 1);
        read_optional!(self.lo_limit, read_r4(raw_data, pos, order), 4);
        read_optional!(self.hi_limit, read_r4(raw_data, pos, order), 4);
        read_optional!(self.start_in, read_r4(raw_data, pos, order), 4);
        read_optional!(self.incr_in, read_r4(raw_data, pos, order), 4);
        read_optional!(self.rtn_indx, read_kx_u2(raw_data, pos, order, self.rtn_icnt), 2);
        read_optional!(self.units, read_cn(raw_data, pos), 1);
        read_optional!(self.units_in, read_cn(raw_data, pos), 1);
        read_optional!(self.c_resfmt, read_cn(raw_data, pos), 1);
        read_optional!(self.c_llmfmt, read_cn(raw_data, pos), 1);
        read_optional!(self.c_hlmfmt, read_cn(raw_data, pos), 1);
        read_optional!(self.lo_spec, read_r4(raw_data, pos, order), 4);
        read_optional!(self.hi_spec, read_r4(raw_data, pos, order), 4);
    }
}

impl FTR {
    pub fn new() -> Self {
        FTR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl STR {
    pub fn new() -> Self {
        STR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
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
    }
}

impl BPS {
    pub fn new() -> Self {
        BPS::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], _order: &ByteOrder) {
        let pos = &mut 0;
        self.seq_name = read_cn(raw_data, pos);
    }
}

impl EPS {
    pub fn new() -> Self {
        EPS::default()
    }

    pub fn read_from_bytes(&mut self, _raw_data: &[u8], _order: &ByteOrder) {}
}

impl GDR {
    pub fn new() -> Self {
        GDR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let pos = &mut 0;
        self.fld_cnt = read_u2(raw_data, pos, order);
        self.gen_data = read_vn(raw_data, pos, order, self.fld_cnt);
    }
}

impl DTR {
    pub fn new() -> Self {
        DTR::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], _order: &ByteOrder) {
        let pos = &mut 0;
        self.text_dat = read_cn(raw_data, pos);
    }
}

impl ReservedRec {
    pub fn new() -> Self {
        ReservedRec::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], _order: &ByteOrder) {
        let pos = &mut 0;
        self.raw_data = read_cn(raw_data, pos);
    }
}

impl StdfRecord {
    /// Create a StdfRecord of a given type with default data
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, stdf_record_type::REC_PMR};
    ///
    /// // create StdfRecord with a nested PMR
    /// let new_rec = StdfRecord::new(REC_PMR);
    ///
    /// if let StdfRecord::PMR(pmr_rec) = new_rec {
    ///     assert_eq!(pmr_rec.head_num, 1);
    ///     assert_eq!(pmr_rec.site_num, 1);
    /// } else {
    ///     // this case will not be hit
    /// }
    /// ```
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

    /// returns the record type cdoe of the given StdfRecord,
    /// which is defined in `rust_stdf::stdf_record_type::*` module.
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, stdf_record_type::*};
    ///
    /// // `REC_PTR` type code can be used for creating a new StdfRecord
    /// let new_rec = StdfRecord::new(REC_PTR);
    /// let returned_code = new_rec.get_type();
    ///
    /// assert_eq!(REC_PTR, returned_code);
    ///
    /// // type code can be used in variety of functions
    /// // get record (typ, sub)
    /// assert_eq!((15, 10), get_typ_sub_from_code(returned_code).unwrap());
    /// // get record name
    /// assert_eq!("PTR", get_rec_name_from_code(returned_code));
    /// ```
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

    /// check the StdfRecord belongs the given type code(s),
    /// it is useful for filtering the records during the parsing iteration.
    /// ```
    /// use rust_stdf::{StdfRecord, stdf_record_type::*};
    ///
    /// let new_rec = StdfRecord::new(REC_PTR);
    ///
    /// assert!(new_rec.is_type(REC_PTR));
    /// assert!(new_rec.is_type(REC_PTR | REC_FTR | REC_MPR));
    /// assert!(!new_rec.is_type(REC_FTR | REC_MPR));
    /// ```
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.get_type() & rec_type) != 0
    }

    /// parse StdfRecord from byte data which **DOES NOT**
    /// contain the record header (len, typ, sub),
    ///
    /// requires a mutable StdfRecord to store the parsed data
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, ByteOrder, stdf_record_type::*};
    ///
    /// let raw_with_no_header: [u8; 2] = [1, 4];
    /// let mut new_rec = StdfRecord::new(REC_FAR);
    /// new_rec.read_from_bytes(&raw_with_no_header, &ByteOrder::LittleEndian);
    ///
    /// if let StdfRecord::FAR(ref far_rec) = new_rec {
    ///     assert_eq!(4, far_rec.stdf_ver);
    /// }
    /// ```
    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        match self {
            // rec type 15
            StdfRecord::PTR(ptr_rec) => ptr_rec.read_from_bytes(raw_data, order),
            StdfRecord::MPR(mpr_rec) => mpr_rec.read_from_bytes(raw_data, order),
            StdfRecord::FTR(ftr_rec) => ftr_rec.read_from_bytes(raw_data, order),
            StdfRecord::STR(str_rec) => str_rec.read_from_bytes(raw_data, order),
            // rec type 5
            StdfRecord::PIR(pir_rec) => pir_rec.read_from_bytes(raw_data, order),
            StdfRecord::PRR(prr_rec) => prr_rec.read_from_bytes(raw_data, order),
            // rec type 2
            StdfRecord::WIR(wir_rec) => wir_rec.read_from_bytes(raw_data, order),
            StdfRecord::WRR(wrr_rec) => wrr_rec.read_from_bytes(raw_data, order),
            StdfRecord::WCR(wcr_rec) => wcr_rec.read_from_bytes(raw_data, order),
            // rec type 50
            StdfRecord::GDR(gdr_rec) => gdr_rec.read_from_bytes(raw_data, order),
            StdfRecord::DTR(dtr_rec) => dtr_rec.read_from_bytes(raw_data, order),
            // rec type 10
            StdfRecord::TSR(tsr_rec) => tsr_rec.read_from_bytes(raw_data, order),
            // rec type 1
            StdfRecord::MIR(mir_rec) => mir_rec.read_from_bytes(raw_data, order),
            StdfRecord::MRR(mrr_rec) => mrr_rec.read_from_bytes(raw_data, order),
            StdfRecord::PCR(pcr_rec) => pcr_rec.read_from_bytes(raw_data, order),
            StdfRecord::HBR(hbr_rec) => hbr_rec.read_from_bytes(raw_data, order),
            StdfRecord::SBR(sbr_rec) => sbr_rec.read_from_bytes(raw_data, order),
            StdfRecord::PMR(pmr_rec) => pmr_rec.read_from_bytes(raw_data, order),
            StdfRecord::PGR(pgr_rec) => pgr_rec.read_from_bytes(raw_data, order),
            StdfRecord::PLR(plr_rec) => plr_rec.read_from_bytes(raw_data, order),
            StdfRecord::RDR(rdr_rec) => rdr_rec.read_from_bytes(raw_data, order),
            StdfRecord::SDR(sdr_rec) => sdr_rec.read_from_bytes(raw_data, order),
            StdfRecord::PSR(psr_rec) => psr_rec.read_from_bytes(raw_data, order),
            StdfRecord::NMR(nmr_rec) => nmr_rec.read_from_bytes(raw_data, order),
            StdfRecord::CNR(cnr_rec) => cnr_rec.read_from_bytes(raw_data, order),
            StdfRecord::SSR(ssr_rec) => ssr_rec.read_from_bytes(raw_data, order),
            StdfRecord::CDR(cdr_rec) => cdr_rec.read_from_bytes(raw_data, order),
            // rec type 0
            StdfRecord::FAR(far_rec) => far_rec.read_from_bytes(raw_data, order),
            StdfRecord::ATR(atr_rec) => atr_rec.read_from_bytes(raw_data, order),
            StdfRecord::VUR(vur_rec) => vur_rec.read_from_bytes(raw_data, order),
            // rec type 20
            StdfRecord::BPS(bps_rec) => bps_rec.read_from_bytes(raw_data, order),
            StdfRecord::EPS(eps_rec) => eps_rec.read_from_bytes(raw_data, order),
            // rec type 180: Reserved
            // rec type 181: Reserved
            StdfRecord::ReservedRec(reserve_rec) => reserve_rec.read_from_bytes(raw_data, order),
            // not matched
            StdfRecord::InvalidRec => (),
        };
    }

    /// parse StdfRecord from byte data which
    /// **contains** the record header (len, typ, sub).
    ///
    /// ## Error
    /// if the input data is not a valid (wrong typ, sub),
    /// incomplete data or incorrect byte order, `StdfError` will be
    /// returned instead.
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, ByteOrder, stdf_record_type::*};
    ///
    /// let raw_with_header: [u8; 6] = [0, 2, 0, 10, 1, 4];
    /// let new_rec = StdfRecord::read_from_bytes_with_header(&raw_with_header, &ByteOrder::BigEndian).unwrap();
    ///
    /// if let StdfRecord::FAR(far_rec) = new_rec {
    ///     assert_eq!(4, far_rec.stdf_ver);
    /// }
    /// ```
    pub fn read_from_bytes_with_header(
        raw_data: &[u8],
        order: &ByteOrder,
    ) -> Result<StdfRecord, StdfError> {
        let header = RecordHeader::new().read_from_bytes(raw_data, order)?;

        let expected_end_pos = 4 + header.len as usize;
        if raw_data.len() < expected_end_pos {
            return Err(StdfError {
                code: 5,
                msg: format!(
                    "Length of stdf field data ({} - 4 = {}) is less than what header specified ({})",
                    raw_data.len(),
                    raw_data.len() - 4,
                    header.len
                ),
            });
        }

        let data_slice = &raw_data[4..expected_end_pos];
        let mut rec = StdfRecord::new(header.type_code);
        rec.read_from_bytes(data_slice, order);
        Ok(rec)
    }
}

impl RawDataElement {
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.type_code & rec_type) != 0
    }
}

impl From<&RawDataElement> for StdfRecord {
    /// it will NOT consume the input RawDataElement
    fn from(raw_element: &RawDataElement) -> Self {
        let mut rec = StdfRecord::new(raw_element.type_code);
        rec.read_from_bytes(&raw_element.raw_data, &raw_element.byte_order);
        rec
    }
}

impl From<RawDataElement> for StdfRecord {
    /// it will consume the input RawDataElement
    fn from(raw_element: RawDataElement) -> Self {
        let mut rec = StdfRecord::new(raw_element.type_code);
        rec.read_from_bytes(&raw_element.raw_data, &raw_element.byte_order);
        rec
    }
}

// data type functions
macro_rules! read_multi_byte_num {
    ($num_type:ty, $length:expr, $raw:ident, $pos:expr, $order:expr, $default:expr) => {{
        let pos_after_read = *$pos + $length;
        if pos_after_read <= $raw.len() {
            let mut tmp = [0u8; $length];
            tmp.copy_from_slice(&$raw[*$pos..pos_after_read]);
            *$pos = pos_after_read;
            match $order {
                ByteOrder::LittleEndian => <$num_type>::from_le_bytes(tmp),
                ByteOrder::BigEndian => <$num_type>::from_be_bytes(tmp),
            }
        } else {
            $default
        }
    }};
}

macro_rules! read_multi_element {
    ($count:expr, $default:expr, $func:ident($($arg:tt)+)) => {
        {
            if $count != 0 {
                let mut value = Vec::with_capacity($count as usize);
                for _ in 0..$count {
                    value.push( $func($($arg)+) );
                }
                value
            } else {
                vec![$default; 0]
            }
        }
    }
}

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
    read_multi_byte_num!(U2, 2, raw_data, pos, order, 0)
}

/// Read U4 (u32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U4 {
    read_multi_byte_num!(U4, 4, raw_data, pos, order, 0)
}

/// Read U8 (u64) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U8 {
    read_multi_byte_num!(U8, 8, raw_data, pos, order, 0)
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
    read_multi_byte_num!(I2, 2, raw_data, pos, order, 0)
}

/// Read I4 (i32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_i4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> I4 {
    read_multi_byte_num!(I4, 4, raw_data, pos, order, 0)
}

/// Read R4 (f32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_r4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> R4 {
    read_multi_byte_num!(R4, 4, raw_data, pos, order, 0.0)
}

/// Read R8 (f64) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_r8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> R8 {
    read_multi_byte_num!(R8, 8, raw_data, pos, order, 0.0)
}

/// Read Cn (u8 + String) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_cn(raw_data: &[u8], pos: &mut usize) -> Cn {
    let count = read_uint8(raw_data, pos) as usize;
    let mut value = String::default();
    if count != 0 {
        let min_pos = std::cmp::min(*pos + count, raw_data.len());
        value = bytes_to_string(&raw_data[*pos..min_pos]);
        *pos = min_pos;
    }
    value
}

/// Read Sn (u16 + String) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> Sn {
    let count = read_u2(raw_data, pos, order) as usize;
    let mut value = String::default();
    if count != 0 {
        let min_pos = std::cmp::min(*pos + count, raw_data.len());
        value = bytes_to_string(&raw_data[*pos..min_pos]);
        *pos = min_pos;
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
        let min_pos = std::cmp::min(*pos + count, raw_data.len());
        let data_slice = &raw_data[*pos..min_pos];
        *pos = min_pos;
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
        let min_pos = std::cmp::min(*pos + bytecount, raw_data.len());
        let data_slice = &raw_data[*pos..min_pos];
        *pos = min_pos;
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
    read_multi_element!(k, String::new(), read_cn(raw_data, pos))
}

/// Read KxSn (Vec<Sn>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxSn {
    read_multi_element!(k, String::new(), read_sn(raw_data, pos, order))
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
    read_multi_element!(k, 0, read_uint8(raw_data, pos))
}

/// Read KxU2 (Vec<u16>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU2 {
    read_multi_element!(k, 0, read_u2(raw_data, pos, order))
}

/// Read KxU4 (Vec<u32>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU4 {
    read_multi_element!(k, 0, read_u4(raw_data, pos, order))
}

/// Read KxU8 (Vec<u64>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU8 {
    read_multi_element!(k, 0, read_u8(raw_data, pos, order))
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
    read_multi_element!(k, 0.0, read_r4(raw_data, pos, order))
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
    read_multi_element!(k, V1::Invalid, read_v1(raw_data, pos, order))
}

#[inline(always)]
pub(crate) fn bytes_to_string(data: &[u8]) -> String {
    data.iter().map(|&x| x as char).collect()
}
