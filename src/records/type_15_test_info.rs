use crate::stdf_codec::*;
use rust_stdf_derive::StdfRecordCodec;
use smart_default::SmartDefault;

#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use struct_field_names_as_array::FieldNamesAsArray;

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
    pub test_num: U4, // Test number
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub test_flg: B1, // Test flags (fail, alarm, etc.)
    pub parm_flg: B1, // Parametric test flags (drift, etc.)
    pub rtn_icnt: U2, // Count of PMR indexes
    pub rslt_cnt: U2, // Count of returned results
    #[stdf(count = rtn_icnt)]
    pub rtn_stat: KxN1, // Array of returned states
    #[stdf(count = rslt_cnt)]
    pub rtn_rslt: KxR4, // Array of returned results
    pub test_txt: Cn, // Descriptive text or label
    pub alarm_id: Cn, // Name of alarm
    pub opt_flag: Option<B1>, // Optional data flag
    pub res_scal: Option<I1>, // Test result scaling exponent
    pub llm_scal: Option<I1>, // Test low limit scaling exponent
    pub hlm_scal: Option<I1>, // Test high limit scaling exponent
    pub lo_limit: Option<R4>, // Test low limit value
    pub hi_limit: Option<R4>, // Test high limit value
    pub start_in: Option<R4>, // Starting input value (condition)
    pub incr_in: Option<R4>, // Increment of input condition
    #[stdf(count = rtn_icnt)]
    pub rtn_indx: Option<KxU2>, // Array of PMR indexes
    pub units: Option<Cn>, // Units of returned results
    pub units_in: Option<Cn>, // Input condition units
    pub c_resfmt: Option<Cn>, // ANSI C result format string
    pub c_llmfmt: Option<Cn>, // ANSI C low limit format string
    pub c_hlmfmt: Option<Cn>, // ANSI C high limit format string
    pub lo_spec: Option<R4>, // Low specification limit value
    pub hi_spec: Option<R4>, // High specification limit value
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct FTR {
    pub test_num: U4, // Test number
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub test_flg: B1, // Test flags (fail, alarm, etc.)
    pub opt_flag: B1, // Optional data flag
    pub cycl_cnt: U4, // Cycle count of vector
    pub rel_vadr: U4, // Relative vector address
    pub rept_cnt: U4, // Repeat count of vector
    pub num_fail: U4, // Number of pins with 1 or more failures
    pub xfail_ad: I4, // X logical device failure address
    pub yfail_ad: I4, // Y logical device failure address
    pub vect_off: I2, // Offset from vector of interest
    pub rtn_icnt: U2, // Count j of return data PMR indexes
    pub pgm_icnt: U2, // Count k of programmed state indexes
    #[stdf(count = rtn_icnt)]
    pub rtn_indx: KxU2, // Array j of return data PMR indexes
    #[stdf(count = rtn_icnt)]
    pub rtn_stat: KxN1, // Array j of returned states
    #[stdf(count = pgm_icnt)]
    pub pgm_indx: KxU2, // Array k of programmed state indexes
    #[stdf(count = pgm_icnt)]
    pub pgm_stat: KxN1, // Array k of programmed states
    pub fail_pin: Dn, // Failing pin bitfield
    pub vect_nam: Cn, // Vector module pattern name
    pub time_set: Cn, // Time set name
    pub op_code: Cn,  // Vector Op Code
    pub test_txt: Cn, // Descriptive text or label
    pub alarm_id: Cn, // Name of alarm
    pub prog_txt: Cn, // Additional programmed information
    pub rslt_txt: Cn, // Additional result information
    #[default = 255]
    pub patg_num: U1, // Pattern generator number
    pub spin_map: Dn, // Bit map of enabled comparators
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct STR {
    pub cont_flg: B1,  // Continuation STR follows if not 0
    pub test_num: U4,  // Test number
    pub head_num: U1,  // Test head number
    pub site_num: U1,  // Test site number
    pub psr_ref: U2,   // PSR Index (Pattern Sequence Record)
    pub test_flg: B1,  // Test flags (fail, alarm, etc.)
    pub log_typ: Cn,   // User defined description of datalog
    pub test_txt: Cn,  // Descriptive text or label
    pub alarm_id: Cn,  // Name of alarm
    pub prog_txt: Cn,  // Additional Programmed information
    pub rslt_txt: Cn,  // Additional result information
    pub z_val: U1,     // Z Handling Flag
    pub fmu_flg: B1,   // MASK_MAP & FAL_MAP field status & Pattern Changed flag
    pub mask_map: Dn,  // Bit map of Globally Masked Pins
    pub fal_map: Dn,   // Bit map of failures after buffer full
    pub cyc_cnt_t: U8, // Total cycles executed in test
    pub totf_cnt: U4,  // Total failures (pin x cycle) detected in test execution
    pub totl_cnt: U4,  // Total fails logged across the complete STR data set
    pub cyc_base: U8,  // Cycle offset to apply for the values in the CYCL_NUM array
    pub bit_base: U4,  // Offset to apply for the values in the BIT_POS array
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
    pub usr1: KxUf, // Array of user defined data for each logged fail
    pub usr2_cnt: U2, // Count (k) of USR2 array entries
    #[stdf(count = usr2_cnt, width = u2_size)]
    pub usr2: KxUf, // Array of user defined data for each logged fail
    pub usr3_cnt: U2, // Count (k) of USR3 array entries
    #[stdf(count = usr3_cnt, width = u3_size)]
    pub usr3: KxUf, // Array of user defined data for each logged fail
    pub txt_cnt: U2,  // Count (k) of USER_TXT array entries
    #[stdf(count = txt_cnt, width = utx_size)]
    pub user_txt: KxCf, // Array of user defined fixed length strings for each logged fail
}
