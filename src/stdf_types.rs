//
// stdf_types.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Mon Aug 17 2026
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use crate::stdf_error::StdfError;
extern crate smart_default;
use rust_stdf_derive::{stdf_match_expr, stdf_records, StdfRecordCodec};
use smart_default::SmartDefault;
use std::borrow::Cow;
use std::convert::From;

#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use struct_field_names_as_array::FieldNamesAsArray;

// Common Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressType {
    Uncompressed,
    #[cfg(feature = "gzip")]
    GzipCompressed,
    #[cfg(feature = "bzip")]
    BzipCompressed,
    #[cfg(feature = "zipfile")]
    ZipCompressed,
}

#[derive(SmartDefault, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordHeader {
    pub len: u16,
    pub typ: u8,
    pub sub: u8,
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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
#[cfg_attr(feature = "serialize", derive(Serialize))]
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

// ----------------------------------------------------------------------------
// Zero-copy borrows of the variable-length payload types, used by the generated
// `*View` getters. Each borrows into the raw record buffer; the `to_owned`
// method reproduces the owned value with the same semantics as the eager
// `read_*` helpers, so view and eager results match.
// ----------------------------------------------------------------------------

/// Sentinel stored in a view when a (trailing/optional) field is absent.
const VIEW_ABSENT_OFT: u16 = u16::MAX;

/// Validate a stored offset, turning it into `Some(pos)`, or `None` when the field is absent.
#[inline]
fn validate_offset(off: u16) -> Option<usize> {
    if off == VIEW_ABSENT_OFT {
        None
    } else {
        Some(off as usize)
    }
}

/// Zero-copy borrow of a `Cn` payload (the bytes *after* the 1-byte length prefix).
#[derive(SmartDefault, Clone, Copy, Debug)]
pub struct CnRef<'a>(&'a [u8]);

/// Zero-copy borrow of an `Sn` payload (the bytes *after* the 2-byte,
/// byte-order-dependent length prefix).
#[derive(SmartDefault, Clone, Copy, Debug)]
pub struct SnRef<'a>(&'a [u8]);

/// Zero-copy borrow of a `Bn` payload (the bytes *after* the 1-byte length prefix).
#[derive(SmartDefault, Clone, Copy, Debug)]
pub struct BnRef<'a>(&'a [u8]);

/// Zero-copy borrow of a `Dn` payload (the bytes *after* the 2-byte,
/// byte-order-dependent bit-count prefix).
#[derive(SmartDefault, Clone, Copy, Debug)]
pub struct DnRef<'a>(&'a [u8]);

// Record Types

/// This module contains constants
/// for STDF Record type check and
/// some help functions
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
    use rust_stdf_derive::{stdf_match_expr, stdf_records};

    // Generates the `REC_*` record-type code constants.
    stdf_records!(rec_codes);

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
        stdf_match_expr!(typ_sub_from_code)
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
        stdf_match_expr!(code_from_typ_sub)
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
        stdf_match_expr!(name_from_code)
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
        stdf_match_expr!(code_from_name)
    }
}

// Generates the `StdfRecord` and `StdfRecordView` enums.
stdf_records!(rec_enums);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Element yielded by [`RawDataIter`](crate::stdf_file::RawDataIter). It owns unprocessed STDF record data, 
/// and can be converted to [`StdfRecord`], or borrowed as [`StdfRecordView`].
/// 
/// ```
/// use rust_stdf::{RawDataElement, ByteOrder, StdfRecord, RecordHeader, stdf_record_type::REC_FAR};
///
/// let rde = RawDataElement {
///     offset: 0,
///     header: RecordHeader {typ: 0, sub: 10, len: 2},
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

    /// used for identifying StdfRecord types
    pub header: RecordHeader,

    /// field data of current STDF Record
    pub raw_data: Vec<u8>,
    pub byte_order: ByteOrder,
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct FAR {
    pub cpu_type: U1, // CPU type that wrote this file
    pub stdf_ver: U1, // STDF version number
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct ATR {
    pub mod_tim: U4,  //Date and time of STDF file modification
    pub cmd_line: Cn, //Command line of program
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct VUR {
    pub upd_nam: Cn, //Update Version Name
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct MRR {
    pub finish_t: U4, // Date and time last part tested
    #[default = ' ']
    pub disp_cod: C1, // Lot disposition code,default: space
    pub usr_desc: Cn, // Lot description supplied by user
    pub exc_desc: Cn, // Lot description supplied by exec
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct HBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub hbin_num: U2, // Hardware bin number
    pub hbin_cnt: U4, // Number of parts in bin
    #[default = ' ']
    pub hbin_pf: C1, // Pass/fail indication
    pub hbin_nam: Cn, // Name of hardware bin
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct SBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub sbin_num: U2, // Software bin number
    pub sbin_cnt: U4, // Number of parts in bin
    #[default = ' ']
    pub sbin_pf: C1, // Pass/fail indication
    pub sbin_nam: Cn, // Name of software bin
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct PGR {
    pub grp_indx: U2,   // Unique index associated with pin group
    pub grp_nam: Cn,    // Name of pin group
    pub indx_cnt: U2,   // Count of PMR indexes
    #[stdf(count = indx_cnt)]
    pub pmr_indx: KxU2, // Array of indexes for pins in the group
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct PLR {
    pub grp_cnt: U2,    // Count (k) of pins or pin groups
    #[stdf(count = grp_cnt)]
    pub grp_indx: KxU2, // Array of pin or pin group indexes
    #[stdf(count = grp_cnt)]
    pub grp_mode: KxU2, // Operating mode of pin group
    #[stdf(count = grp_cnt)]
    pub grp_radx: KxU1, // Display radix of pin group
    #[stdf(count = grp_cnt)]
    pub pgm_char: KxCn, // Program state encoding characters
    #[stdf(count = grp_cnt)]
    pub rtn_char: KxCn, // Return state encoding characters
    #[stdf(count = grp_cnt)]
    pub pgm_chal: KxCn, // Program state encoding characters
    #[stdf(count = grp_cnt)]
    pub rtn_chal: KxCn, // Return state encoding characters
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct RDR {
    pub num_bins: U2,   // Number (k) of bins being retested
    #[stdf(count = num_bins)]
    pub rtst_bin: KxU2, // Array of retest bin numbers
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct SDR {
    pub head_num: U1,   // Test head number
    pub site_grp: U1,   // Site group number
    pub site_cnt: U1,   // Number (k) of test sites in site group
    #[stdf(count = site_cnt)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct PSR {
    pub cont_flg: B1,   // Continuation PSR record exist
    pub psr_indx: U2,   // PSR Record Index (used by STR records)
    pub psr_nam: Cn,    // Symbolic name of PSR record
    pub opt_flg: B1, // Contains PAT_LBL, FILE_UID, ATPG_DSC, and SRC_ID field missing flag bits and flag for start index for first cycle number.
    pub totp_cnt: U2, // Count of total pattern file information sets in the complete PSR data set
    pub locp_cnt: U2, // Count (k) of pattern file information sets in this record
    #[stdf(count = locp_cnt)]
    pub pat_bgn: KxU8, // Array of Cycle #’s patterns begins on
    #[stdf(count = locp_cnt)]
    pub pat_end: KxU8, // Array of Cycle #’s patterns stops at
    #[stdf(count = locp_cnt)]
    pub pat_file: KxCn, // Array of Pattern File Names
    #[stdf(count = locp_cnt)]
    pub pat_lbl: KxCn, // Optional pattern symbolic name
    #[stdf(count = locp_cnt)]
    pub file_uid: KxCn, // Optional array of file identifier code
    #[stdf(count = locp_cnt)]
    pub atpg_dsc: KxCn, // Optional array of ATPG information
    #[stdf(count = locp_cnt)]
    pub src_id: KxCn, // Optional array of PatternInSrcFileID
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct NMR {
    pub cont_flg: B1,   // Continuation NMR record follows if not 0
    pub totm_cnt: U2,   // Count of PMR indexes and ATPG_NAM entries
    pub locm_cnt: U2,   // Count of (k) PMR indexes and ATPG_NAM entries in this record
    #[stdf(count = locm_cnt)]
    pub pmr_indx: KxU2, // Array of PMR indexes
    #[stdf(count = locm_cnt)]
    pub atpg_nam: KxCn, // Array of ATPG signal names
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct CNR {
    pub chn_num: U2,  // Chain number. Referenced by the CHN_NUM array in an STR record
    pub bit_pos: U4,  // Bit position in the chain
    pub cell_nam: Sn, // Scan Cell Name
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct SSR {
    pub ssr_nam: Cn,    // Name of the STIL Scan pub structure for reference
    pub chn_cnt: U2,    // Count (k) of number of Chains listed in CHN_LIST
    #[stdf(count = chn_cnt)]
    pub chn_list: KxU2, // Array of CDR Indexes
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct CDR {
    pub cont_flg: B1, // Continuation CDR record follows if not 0
    pub cdr_indx: U2, // SCR Index
    pub chn_nam: Cn,  // Chain Name
    pub chn_len: U4,  // Chain Length (# of scan cells in chain)
    pub sin_pin: U2,  // PMR index of the chain's Scan In Signal
    pub sout_pin: U2, // PMR index of the chain's Scan Out Signal
    pub mstr_cnt: U1, // Count (m) of master clock pins specified for this scan chain
    #[stdf(count = mstr_cnt)]
    pub m_clks: KxU2, // Array of PMR indexes for the master clocks assigned to this chain
    pub slav_cnt: U1, // Count (n) of slave clock pins specified for this scan chain
    #[stdf(count = slav_cnt)]
    pub s_clks: KxU2, // Array of PMR indexes for the slave clocks assigned to this chain
    #[default = 255]
    pub inv_val: U1, // 0: No Inversion, 1: Inversion
    pub lst_cnt: U2,  // Count (k) of scan cells listed in this record
    #[stdf(count = lst_cnt)]
    pub cell_lst: KxSn, // Array of Scan Cell Names
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct WIR {
    pub head_num: U1, // Test head number
    #[default = 255]
    pub site_grp: U1, // Site group number
    pub start_t: U4,  // Date and time first part tested
    pub wafer_id: Cn, // Wafer ID length byte = 0
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, StdfRecordCodec)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct PIR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, StdfRecordCodec)]
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

// PTR (15, 10): the struct, its eager `read_from_bytes`, and `PTRView`
// are all generated from this single field list by `#[derive(StdfRecordCodec)]`.
#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, StdfRecordCodec)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, StdfRecordCodec)]
pub struct MPR {
    pub test_num: U4,           // Test number
    pub head_num: U1,           // Test head number
    pub site_num: U1,           // Test site number
    pub test_flg: B1,           // Test flags (fail, alarm, etc.)
    pub parm_flg: B1,           // Parametric test flags (drift, etc.)
    pub rtn_icnt: U2,           // Count of PMR indexes
    pub rslt_cnt: U2,           // Count of returned results
    #[stdf(count = rtn_icnt)]
    pub rtn_stat: KxN1, // Array of returned states
    #[stdf(count = rslt_cnt)]
    pub rtn_rslt: KxR4, // Array of returned results
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
    #[stdf(count = rtn_icnt)]
    pub rtn_indx: Option<KxU2>, // Array of PMR indexes
    pub units: Option<Cn>,      // Units of returned results
    pub units_in: Option<Cn>,   // Input condition units
    pub c_resfmt: Option<Cn>,   // ANSI C result format string
    pub c_llmfmt: Option<Cn>,   // ANSI C low limit format string
    pub c_hlmfmt: Option<Cn>,   // ANSI C high limit format string
    pub lo_spec: Option<R4>,    // Low specification limit value
    pub hi_spec: Option<R4>,    // High specification limit value
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
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
    #[stdf(count = rtn_icnt)]
    pub rtn_indx: KxU2, // Array j of return data PMR indexes
    #[stdf(count = rtn_icnt)]
    pub rtn_stat: KxN1, // Array j of returned states
    #[stdf(count = pgm_icnt)]
    pub pgm_indx: KxU2, // Array k of programmed state indexes
    #[stdf(count = pgm_icnt)]
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

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
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
    #[stdf(count = lim_cnt)]
    pub lim_indx: KxU2, // Array of PMR indexes that require unique limit specifications
    #[stdf(count = lim_cnt)]
    pub lim_spec: KxU4, // Array of fail datalogging limits for the PMRs listed in LIM_INDX
    #[stdf(count = cond_cnt)]
    pub cond_lst: KxCn, // Array of test condition (Name=value) pairs
    pub cyc_cnt: U2,  // Count (k) of entries in CYC_OFST array
    #[stdf(count = cyc_cnt, width = cyc_size)]
    pub cyc_ofst: KxUf, // Array of cycle numbers relative to CYC_BASE
    pub pmr_cnt: U2,  // Count (k) of entries in the PMR_INDX array
    #[stdf(count = pmr_cnt, width = pmr_size)]
    pub pmr_indx: KxUf, // Array of PMR Indexes (All Formats)
    pub chn_cnt: U2,  // Count (k) of entries in the CHN_NUM array
    #[stdf(count = chn_cnt, width = chn_size)]
    pub chn_num: KxUf, // Array of Chain No for FF Name Mapping
    pub exp_cnt: U2,  // Count (k) of EXP_DATA array entries
    #[stdf(count = exp_cnt)]
    pub exp_data: KxU1, // Array of expected vector data
    pub cap_cnt: U2,  // Count (k) of CAP_DATA array entries
    #[stdf(count = cap_cnt)]
    pub cap_data: KxU1, // Array of captured data
    pub new_cnt: U2,  // Count (k) of NEW_DATA array entries
    #[stdf(count = new_cnt)]
    pub new_data: KxU1, // Array of new vector data
    pub pat_cnt: U2,  // Count (k) of PAT_NUM array entries
    #[stdf(count = pat_cnt, width = pat_size)]
    pub pat_num: KxUf, // Array of pattern # (Ptn/Chn/Bit format)
    pub bpos_cnt: U2, // Count (k) of BIT_POS array entries
    #[stdf(count = bpos_cnt, width = bit_size)]
    pub bit_pos: KxUf, // Array of chain bit positions (Ptn/Chn/Bit format)
    pub usr1_cnt: U2, // Count (k) of USR1 array entries
    #[stdf(count = usr1_cnt, width = u1_size)]
    pub usr1: KxUf,   // Array of user defined data for each logged fail
    pub usr2_cnt: U2, // Count (k) of USR2 array entries
    #[stdf(count = usr2_cnt, width = u2_size)]
    pub usr2: KxUf,   // Array of user defined data for each logged fail
    pub usr3_cnt: U2, // Count (k) of USR3 array entries
    #[stdf(count = usr3_cnt, width = u3_size)]
    pub usr3: KxUf,   // Array of user defined data for each logged fail
    pub txt_cnt: U2,  // Count (k) of USER_TXT array entries
    #[stdf(count = txt_cnt, width = utx_size)]
    pub user_txt: KxCf, // Array of user defined fixed length strings for each logged fail
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct BPS {
    pub seq_name: Cn, // Program section (or sequencer) name length byte = 0
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct EPS {}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, StdfRecordCodec)]
pub struct GDR {
    pub fld_cnt: U2,  // Count of data fields in record
    #[stdf(count = fld_cnt)]
    pub gen_data: Vn, // Data type code and data for one field(Repeat GEN_DATA for each data field)
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct DTR {
    pub text_dat: Cn, // ASCII text string
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct ReservedRec {
    pub raw_data: Vec<u8>, // unparsed data
}

// implementation

impl RecordHeader {
    #[inline(always)]
    pub fn new() -> Self {
        RecordHeader::default()
    }

    /// Construct a STDF record header from first 4 elements of given byte array,
    /// no error would occur even if the header is invalid.
    ///
    /// Unless the array size is less than 4, StdfError of
    /// `EOF` or `Unexpected EOF` will be returned
    #[inline(always)]
    pub fn read_from_bytes(
        mut self,
        raw_data: &[u8],
        order: &ByteOrder,
    ) -> Result<Self, StdfError> {
        match raw_data.len() {
            0 => Err(StdfError {
                code: 4,
                msg: String::from("No bytes to read"),
            }),
            1..=3 => Err(StdfError {
                code: 5,
                msg: String::from("Not enough data to construct record header"),
            }),
            _ => {
                let len_bytes = [raw_data[0], raw_data[1]];
                self.len = match order {
                    ByteOrder::LittleEndian => u16::from_le_bytes(len_bytes),
                    ByteOrder::BigEndian => u16::from_be_bytes(len_bytes),
                };
                self.typ = raw_data[2];
                self.sub = raw_data[3];
                // return even if we have a invalid record type, let other code to handle it
                Ok(self)
            }
        }
    }

    /// return the type_code of current header
    pub fn get_type(&self) -> u64 {
        stdf_record_type::get_code_from_typ_sub(self.typ, self.sub)
    }
}

impl<'a> CnRef<'a> {
    /// Read a `Cn` starting at the stored offset `off` (which points at the
    /// length byte). Returns `None` if the field is absent.
    #[inline]
    fn read_at(raw: &'a [u8], off: u16) -> Option<Self> {
        let p = validate_offset(off)?;
        let cnt = *raw.get(p)? as usize;
        let start = p + 1;
        let end = std::cmp::min(start + cnt, raw.len()); // clamp truncated payloads
        Some(CnRef(&raw[start..end]))
    }

    /// Raw bytes of the string, always zero-copy.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// Zero-copy borrow when the payload is valid UTF-8; otherwise an owned
    /// `String` decoded byte → char (Latin-1). Same decoding as
    /// [`to_owned`](Self::to_owned).
    #[inline]
    pub fn as_str(&self) -> Cow<'a, str> {
        match std::str::from_utf8(self.0) {
            Ok(s) => Cow::Borrowed(s),
            Err(_) => Cow::Owned(bytes_to_string(self.0)),
        }
    }

    /// Allocating; reproduces the owned `Cn` string (valid UTF-8 decoded as
    /// UTF-8, otherwise byte → char Latin-1), matching the eager parser.
    pub fn to_owned(&self) -> Cn {
        bytes_to_string(self.0)
    }
}

impl<'a> SnRef<'a> {
    /// Read an `Sn` starting at the stored offset `off` (which points at the
    /// 2-byte length prefix). Returns `None` if the field is absent.
    #[inline]
    fn read_at(raw: &'a [u8], off: u16, order: &ByteOrder) -> Option<Self> {
        let p = validate_offset(off)?;
        let mut cp = p;
        let cnt = read_u2(raw, &mut cp, order) as usize;
        let start = cp;
        let end = std::cmp::min(start + cnt, raw.len());
        Some(SnRef(raw.get(start..end).unwrap_or(&[])))
    }

    /// Raw bytes of the string, always zero-copy.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// Zero-copy borrow when the payload is valid UTF-8; otherwise an owned
    /// `String` decoded byte → char (Latin-1). Same decoding as
    /// [`to_owned`](Self::to_owned).
    #[inline]
    pub fn as_str(&self) -> Cow<'a, str> {
        match std::str::from_utf8(self.0) {
            Ok(s) => Cow::Borrowed(s),
            Err(_) => Cow::Owned(bytes_to_string(self.0)),
        }
    }

    /// Allocating; reproduces the owned `Sn` string (valid UTF-8 decoded as
    /// UTF-8, otherwise byte → char Latin-1), matching the eager parser.
    pub fn to_owned(&self) -> Sn {
        bytes_to_string(self.0)
    }
}

impl<'a> BnRef<'a> {
    /// Read a `Bn` starting at the stored offset `off` (which points at the
    /// length byte). Returns `None` if the field is absent.
    #[inline]
    fn read_at(raw: &'a [u8], off: u16) -> Option<Self> {
        let p = validate_offset(off)?;
        let cnt = *raw.get(p)? as usize;
        let start = p + 1;
        let end = std::cmp::min(start + cnt, raw.len());
        Some(BnRef(&raw[start..end]))
    }

    /// Raw bytes, always zero-copy.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Allocating; reproduces the owned `Bn` (byte copy) semantics used by
    /// `StdfRecord`, so results match the eager parser.
    pub fn to_owned(&self) -> Bn {
        self.0.to_vec()
    }
}

impl<'a> DnRef<'a> {
    /// Read a `Dn` starting at the stored offset `off` (which points at the
    /// 2-byte bit-count prefix). Returns `None` if the field is absent.
    #[inline]
    fn read_at(raw: &'a [u8], off: u16, order: &ByteOrder) -> Option<Self> {
        let p = validate_offset(off)?;
        let mut cp = p;
        let bitcount = read_u2(raw, &mut cp, order) as usize;
        let bytecount = bitcount.div_ceil(8);
        let start = cp;
        let end = std::cmp::min(start + bytecount, raw.len());
        Some(DnRef(raw.get(start..end).unwrap_or(&[])))
    }

    /// Raw bytes, always zero-copy.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Allocating; reproduces the owned `Dn` (byte copy) semantics used by
    /// `StdfRecord`, so results match the eager parser.
    pub fn to_owned(&self) -> Dn {
        self.0.to_vec()
    }
}

impl EPS {
    pub fn new() -> Self {
        EPS::default()
    }
}

impl ReservedRec {
    pub fn new() -> Self {
        ReservedRec::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], _order: &ByteOrder) {
        let mut dataclone = Vec::with_capacity(raw_data.len());
        dataclone.extend_from_slice(raw_data);
        self.raw_data = dataclone;
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
    #[inline(always)]
    pub fn new(rec_type: u64) -> Self {
        stdf_match_expr!(record_new)
    }

    /// Create a `StdfRecord` from a `RecordHeader` with default data
    ///
    /// The difference between `new()` is that this method can save
    /// the info of an invalid record header, help the caller to
    /// debug
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, RecordHeader, stdf_record_type::REC_PMR};
    ///
    /// // create a PMR StdfRecord from header
    /// // (1, 60)
    /// let pmr_header = RecordHeader {typ: 1, sub: 60, len: 0 };
    /// let new_rec = StdfRecord::new_from_header(pmr_header);
    ///
    /// if let StdfRecord::PMR(pmr_rec) = new_rec {
    ///     assert_eq!(pmr_rec.head_num, 1);
    ///     assert_eq!(pmr_rec.site_num, 1);
    /// } else {
    ///     // this case will not be hit
    /// }
    /// ```
    #[inline(always)]
    pub fn new_from_header(header: RecordHeader) -> Self {
        let code = stdf_record_type::get_code_from_typ_sub(header.typ, header.sub);
        if code == stdf_record_type::REC_INVALID {
            // Preserve the invalid header so callers can inspect it for debugging.
            StdfRecord::InvalidRec(header)
        } else {
            StdfRecord::new(code)
        }
    }

    /// Returns the record type cdoe of the given StdfRecord,
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
    #[inline(always)]
    pub fn get_type(&self) -> u64 {
        stdf_match_expr!(record_type)
    }

    /// Check the StdfRecord belongs the given type code(s),
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
    #[inline(always)]
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.get_type() & rec_type) != 0
    }

    /// Parse StdfRecord from byte data which **DOES NOT**
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
    #[inline(always)]
    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        stdf_match_expr!(record_read)
    }

    /// Parse StdfRecord from byte data which
    /// **contains** the record header (len, typ, sub).
    ///
    /// ## Error
    /// if the input data is not a valid (wrong typ, sub),
    /// incomplete data or incorrect byte order, [`StdfError`] will be
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
    #[inline(always)]
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
        let mut rec = StdfRecord::new(header.get_type());
        rec.read_from_bytes(data_slice, order);
        Ok(rec)
    }
}

impl<'a> StdfRecordView<'a> {
    /// Build a view from `RecordHeader`, raw field data and byte order.
    ///
    /// ```
    /// use rust_stdf::{StdfRecordView, ByteOrder, RecordHeader, stdf_record_type::REC_PTR};
    ///
    /// let header = RecordHeader { len: 0, typ: 15, sub: 10 }; // PTR
    /// let raw: [u8; 0] = []; // empty field data; getters fall back to defaults
    /// let view = StdfRecordView::read_from_bytes(header, &raw, &ByteOrder::LittleEndian);
    ///
    /// if let StdfRecordView::PTR(ptr_view) = view {
    ///     assert_eq!(0, ptr_view.test_num());
    /// }
    /// assert!(view.is_type(REC_PTR));
    /// ```
    #[inline]
    pub fn read_from_bytes(
        header: RecordHeader,
        raw_data: &'a [u8],
        byte_order: &ByteOrder,
    ) -> Self {
        stdf_match_expr!(view_read)
    }

    /// Parse a `StdfRecordView` from byte data that **contains** the record
    /// header (len, typ, sub).
    ///
    /// ## Error
    /// Returns [`StdfError`] when the input is not a valid header or is
    /// incomplete (the buffer is shorter than the header declares).
    ///
    /// ```
    /// use rust_stdf::{StdfRecordView, ByteOrder};
    ///
    /// let raw_with_header: [u8; 6] = [0, 2, 0, 10, 1, 4];
    /// let view = StdfRecordView::read_from_bytes_with_header(
    ///     &raw_with_header,
    ///     &ByteOrder::BigEndian,
    /// ).unwrap();
    ///
    /// if let StdfRecordView::FAR(far_view) = view {
    ///     assert_eq!(4, far_view.stdf_ver());
    /// }
    /// ```
    #[inline(always)]
    pub fn read_from_bytes_with_header(
        raw_data: &'a [u8],
        order: &ByteOrder,
    ) -> Result<Self, StdfError> {
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
        Ok(Self::read_from_bytes(header, data_slice, order))
    }

    /// Return the record type code (see `stdf_record_type::*`).
    #[inline(always)]
    pub fn get_type(&self) -> u64 {
        stdf_match_expr!(view_type)
    }

    /// Check whether this view belongs to the given record type(s).
    #[inline(always)]
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.get_type() & rec_type) != 0
    }

    /// Parse this borrowed view into an owned [`StdfRecord`], escaping the
    /// borrow of the underlying buffer.
    #[inline]
    pub fn to_owned(&self) -> StdfRecord {
        stdf_match_expr!(view_to_owned)
    }
}

impl RawDataElement {
    #[inline(always)]
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.header.get_type() & rec_type) != 0
    }
}

impl From<&RawDataElement> for StdfRecord {
    /// it will NOT consume the input RawDataElement
    #[inline(always)]
    fn from(raw_element: &RawDataElement) -> Self {
        let mut rec = StdfRecord::new_from_header(raw_element.header);
        rec.read_from_bytes(&raw_element.raw_data, &raw_element.byte_order);
        rec
    }
}

impl<'a> From<&'a RawDataElement> for StdfRecordView<'a> {
    /// Build a zero-copy view; does not consume the input.
    #[inline]
    fn from(raw_element: &'a RawDataElement) -> Self {
        Self::read_from_bytes(
            raw_element.header,
            &raw_element.raw_data,
            &raw_element.byte_order,
        )
    }
}

impl<'a> From<&StdfRecordView<'a>> for StdfRecord {
    /// Parse the borrowed view into an owned record; does not consume the view.
    #[inline(always)]
    fn from(view: &StdfRecordView<'a>) -> Self {
        view.to_owned()
    }
}

impl From<RawDataElement> for StdfRecord {
    /// it will consume the input RawDataElement
    #[inline(always)]
    fn from(raw_element: RawDataElement) -> Self {
        let mut rec = StdfRecord::new_from_header(raw_element.header);
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
    ($count:expr, $func:ident($($arg:tt)+)) => {
        {
            let mut value = Vec::with_capacity($count as usize);
            for _ in 0..$count {
                value.push( $func($($arg)+) );
            }
            value
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
    let bytecount = bitcount.div_ceil(8);
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
    read_multi_element!(k, read_cn(raw_data, pos))
}

/// Read KxSn (Vec<Sn>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxSn {
    read_multi_element!(k, read_sn(raw_data, pos, order))
}

/// Read KxCf (Vec<Cf>) from byte array with offset "pos", vector size is provide by "k", String size is "f"
#[inline(always)]
pub(crate) fn read_kx_cf(raw_data: &[u8], pos: &mut usize, k: u16, f: u8) -> KxCf {
    let mut value = Vec::with_capacity(k as usize);
    for _ in 0..k {
        value.push(read_cf(raw_data, pos, f));
    }
    value
}

/// Read KxU1 (Vec<u8>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u1(raw_data: &[u8], pos: &mut usize, k: u16) -> KxU1 {
    read_multi_element!(k, read_uint8(raw_data, pos))
}

/// Read KxU2 (Vec<u16>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU2 {
    read_multi_element!(k, read_u2(raw_data, pos, order))
}

/// Read KxU4 (Vec<u32>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU4 {
    read_multi_element!(k, read_u4(raw_data, pos, order))
}

/// Read KxU8 (Vec<u64>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU8 {
    read_multi_element!(k, read_u8(raw_data, pos, order))
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
    read_multi_element!(k, read_r4(raw_data, pos, order))
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
    let type_byte = if *pos < raw_data.len() {
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
    read_multi_element!(k, read_v1(raw_data, pos, order))
}

/// Decode STDF string bytes to `String`: valid UTF-8 is decoded as UTF-8,
/// otherwise each byte is widened to a `char` (Latin-1). Shared by the eager
/// `read_cn`/`read_sn`/`read_cf` helpers and the `CnRef`/`SnRef` views.
#[inline(always)]
pub(crate) fn bytes_to_string(data: &[u8]) -> String {
    match std::str::from_utf8(data) {
        Ok(s) => s.to_owned(),
        Err(_) => data.iter().map(|&x| x as char).collect(),
    }
}
