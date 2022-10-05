//
// stdf_types.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Wed Oct 05 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//


use crate::stdf_error::StdfError;
extern crate smart_default;
use smart_default::SmartDefault;


// Common Type
#[derive(Debug)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug)]
pub enum CompressType {
    Uncompressed,
    GzipCompressed,
    BzipCompressed,
    ZipCompressed,
}

#[derive(Debug)]
pub struct RecordHeader {
    pub len: u16,
    pub typ: u8,
    pub sub: u8,
    pub code: StdfRecordType,
}


// Data Types
pub type B1 = [u8;1];
// Rust char is 4 bytes long, however STDF char is only 1 byte
// we will read u8 from file stream and convert to Rust char during parse step
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

#[derive(SmartDefault, Debug)]
pub enum KxUf {
    #[default]
    F1(KxU1),
    F2(KxU2),
    F4(KxU4),
    F8(KxU8),
}

#[derive(Clone, Debug)]
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
#[derive(Debug, PartialEq)]
pub enum StdfRecordType {
    // rec type 0
    RecFAR,
    RecATR,
    RecVUR,
    // rec type 1
    RecMIR,
    RecMRR,
    RecPCR,
    RecHBR,
    RecSBR,
    RecPMR,
    RecPGR,
    RecPLR,
    RecRDR,
    RecSDR,
    RecPSR,
    RecNMR,
    RecCNR,
    RecSSR,
    RecCDR,
    // rec type 2
    RecWIR,
    RecWRR,
    RecWCR,
    // rec type 5
    RecPIR,
    RecPRR,
    // rec type 10
    RecTSR,
    // rec type 15
    RecPTR,
    RecMPR,
    RecFTR,
    RecSTR,
    // rec type 20
    RecBPS,
    RecEPS,
    // rec type 50
    RecGDR,
    RecDTR,
    // rec type 180: Reserved
    // rec type 181: Reserved
    RecReserved,
    RecInvalid,    // for debug
}

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

#[derive(SmartDefault, Debug)]
pub struct FAR {
    pub cpu_type: U1,  // CPU type that wrote this file
    pub stdf_ver: U1,  // STDF version number
}

#[derive(SmartDefault, Debug)]
pub struct ATR {
    pub mod_tim: U4, //Date and time of STDF file modification
    pub cmd_line: Cn, //Command line of program
}

#[derive(SmartDefault, Debug)]
pub struct VUR {
    pub upd_nam: Cn, //Update Version Name
}

#[derive(SmartDefault, Debug)]
pub struct MIR {
    pub setup_t: U4, // Date and time of job setup
    pub start_t: U4, // Date and time first part tested
    pub stat_num: U1, // Tester station number
    pub mode_cod: C1, // Test mode code (e.g. prod, dev)
    pub rtst_cod: C1, // Lot retest code
    pub prot_cod: C1, // Data protection code
    pub burn_tim: U2, // Burn-in time (in minutes)
    pub cmod_cod: C1, // Command mode code
    pub lot_id: Cn, // Lot ID (customer specified)
    pub part_typ: Cn, // Part Type (or product ID)
    pub node_nam: Cn, // Name of node that generated data
    pub tstr_typ: Cn, // Tester type
    pub job_nam: Cn, // Job name (test program name)
    pub job_rev: Cn, // Job (test program) revision number
    pub sblot_id: Cn, // Sublot ID
    pub oper_nam: Cn, // Operator name or ID (at setup time)
    pub exec_typ: Cn, // Tester executive software type
    pub exec_ver: Cn, // Tester exec software version number
    pub test_cod: Cn, // Test phase or step code
    pub tst_temp: Cn, // Test temperature
    pub user_txt: Cn, // Generic user text
    pub aux_file: Cn, // Name of auxiliary data file
    pub pkg_typ: Cn, // Package type
    pub famly_id: Cn, // Product family ID
    pub date_cod: Cn, // Date code
    pub facil_id: Cn, // Test facility ID
    pub floor_id: Cn, // Test floor ID
    pub proc_id: Cn, // Fabrication process ID
    pub oper_frq: Cn, // Operation frequency or step
    pub spec_nam: Cn, // Test specification name
    pub spec_ver: Cn, // Test specification version number
    pub flow_id: Cn, // Test flow ID
    pub setup_id: Cn, // Test setup ID
    pub dsgn_rev: Cn, // Device design revision
    pub eng_id: Cn, // Engineering lot ID
    pub rom_cod: Cn, // ROM code ID
    pub serl_num: Cn, // Tester serial number
    pub supr_nam: Cn, // Supervisor name or ID
}

#[derive(SmartDefault, Debug)]
pub struct MRR {
    pub finish_t: U4, // Date and time last part tested
    pub disp_cod: C1, // Lot disposition code,default: space
    pub usr_desc: Cn, // Lot description supplied by user
    pub exc_desc: Cn, // Lot description supplied by exec
}

#[derive(SmartDefault, Debug)]
pub struct PCR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub part_cnt: U4, // Number of parts tested
    pub rtst_cnt: U4, // Number of parts retested
    pub abrt_cnt: U4, // Number of aborts during testing
    pub good_cnt: U4, // Number of good (passed) parts tested
    pub func_cnt: U4, // Number of functional parts tested
}

#[derive(SmartDefault, Debug)]
pub struct HBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub hbin_num: U2, // Hardware bin number
    pub hbin_cnt: U4, // Number of parts in bin
    pub hbin_pf: C1, // Pass/fail indication
    pub hbin_nam: Cn, // Name of hardware bin
}

#[derive(SmartDefault, Debug)]
pub struct SBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub sbin_num: U2, // Software bin number
    pub sbin_cnt: U4, // Number of parts in bin
    pub sbin_pf: C1, // Pass/fail indication
    pub sbin_nam: Cn, // Name of software bin
}

#[derive(SmartDefault, Debug)]
pub struct PMR {
    pub pmr_indx: U2, // Unique index associated with pin
    pub chan_typ: U2, // Channel type
    pub chan_nam: Cn, // Channel name
    pub phy_nam: Cn, // Physical name of pin
    pub log_nam: Cn, // Logical name of pin
    pub head_num: U1, // Head number associated with channel
    pub site_num: U1, // Site number associated with channel
}

#[derive(SmartDefault, Debug)]
pub struct PGR {
    pub grp_indx: U2, // Unique index associated with pin group
    pub grp_nam: Cn, // Name of pin group
    pub indx_cnt: U2, // Count of PMR indexes
    pub pmr_indx: KxU2, // Array of indexes for pins in the group
}

#[derive(SmartDefault, Debug)]
pub struct PLR {
    pub grp_cnt: U2, // Count (k) of pins or pin groups
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
    pub num_bins: U2, // Number (k) of bins being retested
    pub rtst_bin: KxU2, // Array of retest bin numbers
}

#[derive(SmartDefault, Debug)]
pub struct SDR {
    pub head_num: U1, // Test head number
    pub site_grp: U1, // Site group number
    pub site_cnt: U1, // Number (k) of test sites in site group
    pub site_num: KxU1, // Array of test site numbers
    pub hand_typ: Cn, // Handler or prober type
    pub hand_id: Cn, // Handler or prober ID
    pub card_typ: Cn, // Probe card type
    pub card_id: Cn, // Probe card ID
    pub load_typ: Cn, // Load board type
    pub load_id: Cn, // Load board ID
    pub dib_typ: Cn, // DIB board type
    pub dib_id: Cn, // DIB board ID
    pub cabl_typ: Cn, // Interface cable type
    pub cabl_id: Cn, // Interface cable ID
    pub cont_typ: Cn, // Handler contactor type
    pub cont_id: Cn, // Handler contactor ID
    pub lasr_typ: Cn, // Laser type
    pub lasr_id: Cn, // Laser ID
    pub extr_typ: Cn, // Extra equipment type field
    pub extr_id: Cn, // Extra equipment ID
}

#[derive(SmartDefault, Debug)]
pub struct PSR {
    pub cont_flg: B1, // Continuation PSR record exist
    pub psr_indx: U2, // PSR Record Index (used by STR records)
    pub psr_nam: Cn, // Symbolic name of PSR record
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
    pub cont_flg: B1, // Continuation NMR record follows if not 0
    pub totm_cnt: U2, // Count of PMR indexes and ATPG_NAM entries
    pub locm_cnt: U2, // Count of (k) PMR indexes and ATPG_NAM entries in this record
    pub pmr_indx: KxU2, // Array of PMR indexes
    pub atpg_nam: KxCn, // Array of ATPG signal names
}

#[derive(SmartDefault, Debug)]
pub struct CNR {
    pub chn_num: U2, // Chain number. Referenced by the CHN_NUM array in an STR record
    pub bit_pos: U4, // Bit position in the chain
    pub cell_nam: Sn, // Scan Cell Name
}

#[derive(SmartDefault, Debug)]
pub struct SSR {
    pub ssr_nam: Cn, // Name of the STIL Scan pub structure for reference
    pub chn_cnt: U2, // Count (k) of number of Chains listed in CHN_LIST
    pub chn_list: KxU2, // Array of CDR Indexes
}

#[derive(SmartDefault, Debug)]
pub struct CDR {
    pub cont_flg: B1, // Continuation CDR record follows if not 0
    pub cdr_indx: U2, // SCR Index
    pub chn_nam: Cn, // Chain Name
    pub chn_len: U4, // Chain Length (# of scan cells in chain)
    pub sin_pin: U2, // PMR index of the chain's Scan In Signal
    pub sout_pin: U2, // PMR index of the chain's Scan Out Signal
    pub mstr_cnt: U1, // Count (m) of master clock pins specified for this scan chain
    pub m_clks: KxU2, // Array of PMR indexes for the master clocks assigned to this chain
    pub slav_cnt: U1, // Count (n) of slave clock pins specified for this scan chain
    pub s_clks: KxU2, // Array of PMR indexes for the slave clocks assigned to this chain
    pub inv_val: U1, // 0: No Inversion, 1: Inversion
    pub lst_cnt: U2, // Count (k) of scan cells listed in this record
    pub cell_lst: KxSn, // Array of Scan Cell Names
}

#[derive(SmartDefault, Debug)]
pub struct WIR {
    pub head_num: U1, // Test head number
    pub site_grp: U1, // Site group number 255
    pub start_t: U4, // Date and time first part tested
    pub wafer_id: Cn, // Wafer ID length byte = 0
}

#[derive(SmartDefault, Debug)]
pub struct WRR {
    pub head_num: U1, // Test head number
    pub site_grp: U1, // Site group number
    pub finish_t: U4, // Date and time last part tested
    pub part_cnt: U4, // Number of parts tested
    pub rtst_cnt: U4, // Number of parts retested
    pub abrt_cnt: U4, // Number of aborts during testing
    pub good_cnt: U4, // Number of good (passed) parts tested
    pub func_cnt: U4, // Number of functional parts tested
    pub wafer_id: Cn, // Wafer ID
    pub fabwf_id: Cn, // Fab wafer ID
    pub frame_id: Cn, // Wafer frame ID
    pub mask_id: Cn, // Wafer mask ID
    pub usr_desc: Cn, // Wafer description supplied by user
    pub exc_desc: Cn, // Wafer description supplied by exec
}

#[derive(SmartDefault, Debug)]
pub struct WCR {
    pub wafr_siz: R4, // Diameter of wafer in WF_UNITS
    pub die_ht: R4, // Height of die in WF_UNITS
    pub die_wid: R4, // Width of die in WF_UNITS
    pub wf_units: U1, // Units for wafer and die dimensions
    pub wf_flat: C1, // Orientation of wafer flat
    pub center_x: I2, // X coordinate of center die on wafer
    pub center_y: I2, // Y coordinate of center die on wafer
    pub pos_x: C1, // Positive X direction of wafer
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
    pub soft_bin: U2, //Software bin number
    pub x_coord: I2, //(Wafer) X coordinate
    pub y_coord: I2, //(Wafer) Y coordinate
    pub test_t: U4, //Elapsed test time in milliseconds
    pub part_id: Cn, //Part identification
    pub part_txt: Cn, //Part description text
    pub part_fix: Bn, //Part repair information
}

#[derive(SmartDefault, Debug)]
pub struct TSR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub test_typ: C1, // Test type
    pub test_num: U4, // Test number
    pub exec_cnt: U4, // Number of test executions
    pub fail_cnt: U4, // Number of test failures
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
    pub result: R4, // Test result
    pub test_txt: Cn, // Test description text or label
    pub alarm_id: Cn, // Name of alarm
    pub opt_flag: B1, // Optional data flag
    pub res_scal: I1, // Test results scaling exponent
    pub llm_scal: I1, // Low limit scaling exponent
    pub hlm_scal: I1, // High limit scaling exponent
    pub lo_limit: R4, // Low test limit value
    pub hi_limit: R4, // High test limit value
    pub units: Cn, // Test units
    pub c_resfmt: Cn, // ANSI C result format string
    pub c_llmfmt: Cn, // ANSI C low limit format string
    pub c_hlmfmt: Cn, // ANSI C high limit format string
    pub lo_spec: R4, // Low specification limit value
    pub hi_spec: R4, // High specification limit value
}

#[derive(SmartDefault, Debug)]
pub struct MPR {
    pub test_num: U4, // Test number
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub test_flg: B1, // Test flags (fail, alarm, etc.)
    pub parm_flg: B1, // Parametric test flags (drift, etc.)
    pub rtn_icnt: U2, // Count of PMR indexes
    pub rslt_cnt: U2, // Count of returned results
    pub rtn_stat: KxN1, // Array of returned states
    pub rtn_rslt: KxR4, // Array of returned results
    pub test_txt: Cn, // Descriptive text or label
    pub alarm_id: Cn, // Name of alarm
    pub opt_flag: B1, // Optional data flag
    pub res_scal: I1, // Test result scaling exponent
    pub llm_scal: I1, // Test low limit scaling exponent
    pub hlm_scal: I1, // Test high limit scaling exponent
    pub lo_limit: R4, // Test low limit value
    pub hi_limit: R4, // Test high limit value
    pub start_in: R4, // Starting input value (condition)
    pub incr_in: R4, // Increment of input condition
    pub rtn_indx: KxU2, // Array of PMR indexes
    pub units: Cn, // Units of returned results
    pub units_in: Cn, // Input condition units
    pub c_resfmt: Cn, // ANSI C result format string
    pub c_llmfmt: Cn, // ANSI C low limit format string
    pub c_hlmfmt: Cn, // ANSI C high limit format string
    pub lo_spec: R4, // Low specification limit value
    pub hi_spec: R4, // High specification limit value
}

#[derive(SmartDefault, Debug)]
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
    pub rtn_indx: KxU2, // Array j of return data PMR indexes
    pub rtn_stat: KxN1, // Array j of returned states
    pub pgm_indx: KxU2, // Array k of programmed state indexes
    pub pgm_stat: KxN1, // Array k of programmed states
    pub fail_pin: Dn, // Failing pin bitfield
    pub vect_nam: Cn, // Vector module pattern name
    pub time_set: Cn, // Time set name
    pub op_code: Cn, // Vector Op Code
    pub test_txt: Cn, // Descriptive text or label
    pub alarm_id: Cn, // Name of alarm
    pub prog_txt: Cn, // Additional programmed information
    pub rslt_txt: Cn, // Additional result information
    pub patg_num: U1, // Pattern generator number
    pub spin_map: Dn, // Bit map of enabled comparators
}

#[derive(SmartDefault, Debug)]
pub struct STR {
    pub cont_flg: B1, // Continuation STR follows if not 0
    pub test_num: U4, // Test number
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub psr_ref: U2, // PSR Index (Pattern Sequence Record)
    pub test_flg: B1, // Test flags (fail, alarm, etc.)
    pub log_typ: Cn, // User defined description of datalog
    pub test_txt: Cn, // Descriptive text or label
    pub alarm_id: Cn, // Name of alarm
    pub prog_txt: Cn, // Additional Programmed information
    pub rslt_txt: Cn, // Additional result information
    pub z_val: U1, // Z Handling Flag
    pub fmu_flg: B1, // MASK_MAP & FAL_MAP field status & Pattern Changed flag
    pub mask_map: Dn, // Bit map of Globally Masked Pins
    pub fal_map: Dn, // Bit map of failures after buffer full
    pub cyc_cnt_t: U8, // Total cycles executed in test
    pub totf_cnt: U4, // Total failures (pin x cycle) detected in test execution
    pub totl_cnt: U4, // Total fails logged across the complete STR data set
    pub cyc_base: U8, // Cycle offset to apply for the values in the CYCL_NUM array
    pub bit_base: U4, // Offset to apply for the values in the BIT_POS array
    pub cond_cnt: U2, // Count (g) of Test Conditions and optional data specifications in present record
    pub lim_cnt: U2, // Count (j) of LIM Arrays in present record, 1 for global specification
    pub cyc_size: U1, // Size (f) of data (1,2,4, or 8 byes) in CYC_OFST field
    pub pmr_size: U1, // Size (f) of data (1 or 2 bytes) in PMR_INDX field
    pub chn_size: U1, // Size (f) of data (1, 2 or 4 bytes) in CHN_NUM field
    pub pat_size: U1, // Size (f) of data (1,2, or 4 bytes) in PAT_NUM field
    pub bit_size: U1, // Size (f) of data (1,2, or 4 bytes) in BIT_POS field
    pub u1_size: U1, // Size (f) of data (1,2,4 or 8 bytes) in USR1 field
    pub u2_size: U1, // Size (f) of data (1,2,4 or 8 bytes) in USR2 field
    pub u3_size: U1, // Size (f) of data (1,2,4 or 8 bytes) in USR3 field
    pub utx_size: U1, // Size (f) of each string entry in USER_TXT array
    pub cap_bgn: U2, // Offset added to BIT_POS value to indicate capture cycles
    pub lim_indx: KxU2, // Array of PMR indexes that require unique limit specifications
    pub lim_spec: KxU4, // Array of fail datalogging limits for the PMRs listed in LIM_INDX
    pub cond_lst: KxCn, // Array of test condition (Name=value) pairs
    pub cyc_cnt: U2, // Count (k) of entries in CYC_OFST array
    pub cyc_ofst: KxUf, // Array of cycle numbers relative to CYC_BASE
    pub pmr_cnt: U2, // Count (k) of entries in the PMR_INDX array
    pub pmr_indx: KxUf, // Array of PMR Indexes (All Formats)
    pub chn_cnt: U2, // Count (k) of entries in the CHN_NUM array
    pub chn_num: KxUf, // Array of Chain No for FF Name Mapping
    pub exp_cnt: U2, // Count (k) of EXP_DATA array entries
    pub exp_data: KxU1, // Array of expected vector data
    pub cap_cnt: U2, // Count (k) of CAP_DATA array entries
    pub cap_data: KxU1, // Array of captured data
    pub new_cnt: U2, // Count (k) of NEW_DATA array entries
    pub new_data: KxU1, // Array of new vector data
    pub pat_cnt: U2, // Count (k) of PAT_NUM array entries
    pub pat_num: KxUf, // Array of pattern # (Ptn/Chn/Bit format)
    pub bpos_cnt: U2, // Count (k) of BIT_POS array entries
    pub bit_pos: KxUf, // Array of chain bit positions (Ptn/Chn/Bit format)
    pub usr1_cnt: U2, // Count (k) of USR1 array entries
    pub usr1: KxUf, // Array of user defined data for each logged fail
    pub usr2_cnt: U2, // Count (k) of USR2 array entries
    pub usr2: KxUf, // Array of user defined data for each logged fail
    pub usr3_cnt: U2, // Count (k) of USR3 array entries
    pub usr3: KxUf, // Array of user defined data for each logged fail
    pub txt_cnt: U2, // Count (k) of USER_TXT array entries
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
    pub fld_cnt: U2, // Count of data fields in record
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
    pub fn new() -> Self {
        RecordHeader { 
            len: 0, 
            typ: 0, 
            sub: 0, 
            code: StdfRecordType::RecInvalid
        }
    }

    /// Construct a STDF record header from first 4 elements of given byte array.
    /// 
    /// If array size is less than 4, this function return a StdfError
    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Result<Self, StdfError> {
        if raw_data.len() >= 4 {
            let len_bytes = [raw_data[0], raw_data[1]];
            self.len = match order {
                ByteOrder::LittleEndian => u16::from_le_bytes(len_bytes),
                ByteOrder::BigEndian => u16::from_be_bytes(len_bytes)
            };
            self.typ = raw_data[2];
            self.sub = raw_data[3];
            // validate header
            self.code = match (self.typ, self.sub) {
                // rec type 15
                (15, 10) => StdfRecordType::RecPTR,
                (15, 15) => StdfRecordType::RecMPR,
                (15, 20) => StdfRecordType::RecFTR,
                (15, 30) => StdfRecordType::RecSTR,
                // rec type 5
                (5, 10) => StdfRecordType::RecPIR,
                (5, 20) => StdfRecordType::RecPRR,
                // rec type 2
                (2, 10) => StdfRecordType::RecWIR,
                (2, 20) => StdfRecordType::RecWRR,
                (2, 30) => StdfRecordType::RecWCR,
                // rec type 50
                (50, 10) => StdfRecordType::RecGDR,
                (50, 30) => StdfRecordType::RecDTR,
                // rec type 0
                (0, 10) => StdfRecordType::RecFAR,
                (0, 20) => StdfRecordType::RecATR,
                (0, 30) => StdfRecordType::RecVUR,
                // rec type 1
                (1, 10) => StdfRecordType::RecMIR,
                (1, 20) => StdfRecordType::RecMRR,
                (1, 30) => StdfRecordType::RecPCR,
                (1, 40) => StdfRecordType::RecHBR,
                (1, 50) => StdfRecordType::RecSBR,
                (1, 60) => StdfRecordType::RecPMR,
                (1, 62) => StdfRecordType::RecPGR,
                (1, 63) => StdfRecordType::RecPLR,
                (1, 70) => StdfRecordType::RecRDR,
                (1, 80) => StdfRecordType::RecSDR,
                (1, 90) => StdfRecordType::RecPSR,
                (1, 91) => StdfRecordType::RecNMR,
                (1, 92) => StdfRecordType::RecCNR,
                (1, 93) => StdfRecordType::RecSSR,
                (1, 94) => StdfRecordType::RecCDR,
                // rec type 10
                (10, 30) => StdfRecordType::RecTSR,
                // rec type 20
                (20, 10) => StdfRecordType::RecBPS,
                (20, 20) => StdfRecordType::RecEPS,
                // rec type 180: Reserved
                // rec type 181: Reserved
                (180 | 181, _) => StdfRecordType::RecReserved,
                // not matched
                (_, _) => StdfRecordType::RecInvalid,
            };
            
            if self.code == StdfRecordType::RecInvalid {
                Err(StdfError {code: 2, msg: format!("{:?}", self)})
            } else {
                Ok(self)
            }
        } else {
            Err(StdfError {code: 1, msg: String::from("Not enough data to construct record header")})
        }
    }
}

impl FAR {
    pub fn new() -> Self {
        FAR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.upd_nam = read_cn(raw_data, pos);
        self
    }
}

impl MIR {
    pub fn new() -> Self {
        MIR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.setup_t = read_u4(raw_data, pos, order);
        self.start_t = read_u4(raw_data, pos, order);
        self.stat_num = read_uint8(raw_data, pos);
        self.mode_cod = read_uint8(raw_data, pos) as char;
        self.rtst_cod = read_uint8(raw_data, pos) as char;
        self.prot_cod = read_uint8(raw_data, pos) as char;
        self.burn_tim = read_u2(raw_data, pos, order);
        self.cmod_cod = read_uint8(raw_data, pos) as char;
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.finish_t = read_u4(raw_data, pos, order);
        self.disp_cod = read_uint8(raw_data, pos) as char;
        self.usr_desc = read_cn(raw_data, pos);
        self.exc_desc = read_cn(raw_data, pos);
        self
    }
}

impl PCR {
    pub fn new() -> Self {
        PCR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.part_cnt = read_u4(raw_data, pos, order);
        self.rtst_cnt = read_u4(raw_data, pos, order);
        self.abrt_cnt = read_u4(raw_data, pos, order);
        self.good_cnt = read_u4(raw_data, pos, order);
        self.func_cnt = read_u4(raw_data, pos, order);
        self
    }
}

impl HBR {
    pub fn new() -> Self {
        HBR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.hbin_num = read_u2(raw_data, pos, order);
        self.hbin_cnt = read_u4(raw_data, pos, order);
        self.hbin_pf = read_uint8(raw_data, pos) as char;
        self.hbin_nam = read_cn(raw_data, pos);
        self

    }
}

impl SBR {
    pub fn new() -> Self {
        SBR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.sbin_num = read_u2(raw_data, pos, order);
        self.sbin_cnt = read_u4(raw_data, pos, order);
        self.sbin_pf = read_uint8(raw_data, pos) as char;
        self.sbin_nam = read_cn(raw_data, pos);
        self
    }
}

impl PMR {
    pub fn new() -> Self {
        PMR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.pmr_indx = read_u2(raw_data, pos, order);
        self.chan_typ = read_u2(raw_data, pos, order);
        self.chan_nam = read_cn(raw_data, pos);
        self.phy_nam = read_cn(raw_data, pos);
        self.log_nam = read_cn(raw_data, pos);
        // default value for head & site is 1
        self.head_num = if *pos < raw_data.len() 
            { read_uint8(raw_data, pos) } else { 1 };
        self.site_num = if *pos < raw_data.len() 
            { read_uint8(raw_data, pos) } else { 1 };
        self
    }
}

impl PGR {
    pub fn new() -> Self {
        PGR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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
        self.inv_val = read_uint8(raw_data, pos);
        self.lst_cnt = read_u2(raw_data, pos, order);
        self.cell_lst = read_kx_sn(raw_data, pos, order, self.lst_cnt);
        self
    }
}

impl WIR {
    pub fn new() -> Self {
        WIR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_grp = read_uint8(raw_data, pos);
        self.start_t = read_u4(raw_data, pos, order);
        self.wafer_id = read_cn(raw_data, pos);
        self
    }
}

impl WRR {
    pub fn new() -> Self {
        WRR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_grp = read_uint8(raw_data, pos);
        self.finish_t = read_u4(raw_data, pos, order);
        self.part_cnt = read_u4(raw_data, pos, order);
        self.rtst_cnt = read_u4(raw_data, pos, order);
        self.abrt_cnt = read_u4(raw_data, pos, order);
        self.good_cnt = read_u4(raw_data, pos, order);
        self.func_cnt = read_u4(raw_data, pos, order);
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.wafr_siz = read_r4(raw_data, pos, order);
        self.die_ht = read_r4(raw_data, pos, order);
        self.die_wid = read_r4(raw_data, pos, order);
        self.wf_units = read_uint8(raw_data, pos);
        self.wf_flat = read_uint8(raw_data, pos) as char;
        self.center_x = read_i2(raw_data, pos, order);
        self.center_y = read_i2(raw_data, pos, order);
        self.pos_x = read_uint8(raw_data, pos) as char;
        self.pos_y = read_uint8(raw_data, pos) as char;
        self
    }
}

impl PIR {
    pub fn new() -> Self {
        PIR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.part_flg = [read_uint8(raw_data, pos)];
        self.num_test = read_u2(raw_data, pos, order);
        self.hard_bin = read_u2(raw_data, pos, order);
        self.soft_bin = read_u2(raw_data, pos, order);
        self.x_coord = read_i2(raw_data, pos, order);
        self.y_coord = read_i2(raw_data, pos, order);
        self.test_t = read_u4(raw_data, pos, order);
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.head_num = read_uint8(raw_data, pos);
        self.site_num = read_uint8(raw_data, pos);
        self.test_typ = read_uint8(raw_data, pos) as char;
        self.test_num = read_u4(raw_data, pos, order);
        self.exec_cnt = read_u4(raw_data, pos, order);
        self.fail_cnt = read_u4(raw_data, pos, order);
        self.alrm_cnt = read_u4(raw_data, pos, order);
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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
        self.patg_num = read_uint8(raw_data, pos);  // default 255
        self.spin_map = read_dn(raw_data, pos, order);
        self
    }
}

impl STR {
    pub fn new() -> Self {
        STR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.seq_name = read_cn(raw_data, pos);
        self
    }
}

impl EPS {
    pub fn new() -> Self {
        EPS::default()
    }

    pub fn from_bytes(self, _raw_data: &[u8], _order: &ByteOrder) -> Self {
        self
    }
}

impl GDR {
    pub fn new() -> Self {
        GDR::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], order: &ByteOrder) -> Self {
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

    pub fn from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.text_dat = read_cn(raw_data, pos);
        self
    }
}

impl ReservedRec {
    pub fn new() -> Self {
        ReservedRec::default()
    }

    pub fn from_bytes(mut self, raw_data: &[u8], _order: &ByteOrder) -> Self {
        let pos = &mut 0;
        self.raw_data = read_cn(raw_data, pos);
        self
    }
}


impl StdfRecord {
    pub fn new(rec_type: &StdfRecordType) -> Self {
        match rec_type {
            // rec type 15
            StdfRecordType::RecPTR => StdfRecord::PTR(PTR::new()),
            StdfRecordType::RecMPR => StdfRecord::MPR(MPR::new()),
            StdfRecordType::RecFTR => StdfRecord::FTR(FTR::new()),
            StdfRecordType::RecSTR => StdfRecord::STR(STR::new()),
            // rec type 5
            StdfRecordType::RecPIR => StdfRecord::PIR(PIR::new()),
            StdfRecordType::RecPRR => StdfRecord::PRR(PRR::new()),
            // rec type 2
            StdfRecordType::RecWIR => StdfRecord::WIR(WIR::new()),
            StdfRecordType::RecWRR => StdfRecord::WRR(WRR::new()),
            StdfRecordType::RecWCR => StdfRecord::WCR(WCR::new()),
            // rec type 50
            StdfRecordType::RecGDR => StdfRecord::GDR(GDR::new()),
            StdfRecordType::RecDTR => StdfRecord::DTR(DTR::new()),
            // rec type 0
            StdfRecordType::RecFAR => StdfRecord::FAR(FAR::new()),
            StdfRecordType::RecATR => StdfRecord::ATR(ATR::new()),
            StdfRecordType::RecVUR => StdfRecord::VUR(VUR::new()),
            // rec type 1
            StdfRecordType::RecMIR => StdfRecord::MIR(MIR::new()),
            StdfRecordType::RecMRR => StdfRecord::MRR(MRR::new()),
            StdfRecordType::RecPCR => StdfRecord::PCR(PCR::new()),
            StdfRecordType::RecHBR => StdfRecord::HBR(HBR::new()),
            StdfRecordType::RecSBR => StdfRecord::SBR(SBR::new()),
            StdfRecordType::RecPMR => StdfRecord::PMR(PMR::new()),
            StdfRecordType::RecPGR => StdfRecord::PGR(PGR::new()),
            StdfRecordType::RecPLR => StdfRecord::PLR(PLR::new()),
            StdfRecordType::RecRDR => StdfRecord::RDR(RDR::new()),
            StdfRecordType::RecSDR => StdfRecord::SDR(SDR::new()),
            StdfRecordType::RecPSR => StdfRecord::PSR(PSR::new()),
            StdfRecordType::RecNMR => StdfRecord::NMR(NMR::new()),
            StdfRecordType::RecCNR => StdfRecord::CNR(CNR::new()),
            StdfRecordType::RecSSR => StdfRecord::SSR(SSR::new()),
            StdfRecordType::RecCDR => StdfRecord::CDR(CDR::new()),
            // rec type 10
            StdfRecordType::RecTSR => StdfRecord::TSR(TSR::new()),
            // rec type 20
            StdfRecordType::RecBPS => StdfRecord::BPS(BPS::new()),
            StdfRecordType::RecEPS => StdfRecord::EPS(EPS::new()),
            // rec type 180: Reserved
            // rec type 181: Reserved
            StdfRecordType::RecReserved => StdfRecord::ReservedRec(ReservedRec::new()),
            // not matched
            StdfRecordType::RecInvalid => StdfRecord::InvalidRec,
        }
    }

    pub fn get_type(&self) -> StdfRecordType {
        match &self {
            // rec type 15
            StdfRecord::PTR(_) => StdfRecordType::RecPTR,
            StdfRecord::MPR(_) => StdfRecordType::RecMPR,
            StdfRecord::FTR(_) => StdfRecordType::RecFTR,
            StdfRecord::STR(_) => StdfRecordType::RecSTR,
            // rec type 5
            StdfRecord::PIR(_) => StdfRecordType::RecPIR,
            StdfRecord::PRR(_) => StdfRecordType::RecPRR,
            // rec type 2
            StdfRecord::WIR(_) => StdfRecordType::RecWIR,
            StdfRecord::WRR(_) => StdfRecordType::RecWRR,
            StdfRecord::WCR(_) => StdfRecordType::RecWCR,
            // rec type 50
            StdfRecord::GDR(_) => StdfRecordType::RecGDR,
            StdfRecord::DTR(_) => StdfRecordType::RecDTR,
            // rec type 10
            StdfRecord::TSR(_) => StdfRecordType::RecTSR,            
            // rec type 1
            StdfRecord::MIR(_) => StdfRecordType::RecMIR,
            StdfRecord::MRR(_) => StdfRecordType::RecMRR,
            StdfRecord::PCR(_) => StdfRecordType::RecPCR,
            StdfRecord::HBR(_) => StdfRecordType::RecHBR,
            StdfRecord::SBR(_) => StdfRecordType::RecSBR,
            StdfRecord::PMR(_) => StdfRecordType::RecPMR,
            StdfRecord::PGR(_) => StdfRecordType::RecPGR,
            StdfRecord::PLR(_) => StdfRecordType::RecPLR,
            StdfRecord::RDR(_) => StdfRecordType::RecRDR,
            StdfRecord::SDR(_) => StdfRecordType::RecSDR,
            StdfRecord::PSR(_) => StdfRecordType::RecPSR,
            StdfRecord::NMR(_) => StdfRecordType::RecNMR,
            StdfRecord::CNR(_) => StdfRecordType::RecCNR,
            StdfRecord::SSR(_) => StdfRecordType::RecSSR,
            StdfRecord::CDR(_) => StdfRecordType::RecCDR,
            // rec type 0
            StdfRecord::FAR(_) => StdfRecordType::RecFAR,
            StdfRecord::ATR(_) => StdfRecordType::RecATR,
            StdfRecord::VUR(_) => StdfRecordType::RecVUR,            
            // rec type 20
            StdfRecord::BPS(_) => StdfRecordType::RecBPS,
            StdfRecord::EPS(_) => StdfRecordType::RecEPS,
            // rec type 180: Reserved
            // rec type 181: Reserved
            StdfRecord::ReservedRec(_) => StdfRecordType::RecReserved,
            // not matched
            StdfRecord::InvalidRec => StdfRecordType::RecInvalid,
        }
    }
    
    pub fn is_type(&self, rec_type: StdfRecordType) -> bool {
        (self.get_type()) == rec_type
    }

    pub fn from_bytes(self, raw_data: &[u8], order: &ByteOrder) -> Self {
        match self {
            // rec type 15
            StdfRecord::PTR(ptr_rec) => StdfRecord::PTR(ptr_rec.from_bytes(raw_data, order)),
            StdfRecord::MPR(mpr_rec) => StdfRecord::MPR(mpr_rec.from_bytes(raw_data, order)),
            StdfRecord::FTR(ftr_rec) => StdfRecord::FTR(ftr_rec.from_bytes(raw_data, order)),
            StdfRecord::STR(str_rec) => StdfRecord::STR(str_rec.from_bytes(raw_data, order)),
            // rec type 5
            StdfRecord::PIR(pir_rec) => StdfRecord::PIR(pir_rec.from_bytes(raw_data, order)),
            StdfRecord::PRR(prr_rec) => StdfRecord::PRR(prr_rec.from_bytes(raw_data, order)),
            // rec type 2
            StdfRecord::WIR(wir_rec) => StdfRecord::WIR(wir_rec.from_bytes(raw_data, order)),
            StdfRecord::WRR(wrr_rec) => StdfRecord::WRR(wrr_rec.from_bytes(raw_data, order)),
            StdfRecord::WCR(wcr_rec) => StdfRecord::WCR(wcr_rec.from_bytes(raw_data, order)),
            // rec type 50
            StdfRecord::GDR(gdr_rec) => StdfRecord::GDR(gdr_rec.from_bytes(raw_data, order)),
            StdfRecord::DTR(dtr_rec) => StdfRecord::DTR(dtr_rec.from_bytes(raw_data, order)),
            // rec type 10
            StdfRecord::TSR(tsr_rec) => StdfRecord::TSR(tsr_rec.from_bytes(raw_data, order)),            
            // rec type 1
            StdfRecord::MIR(mir_rec) => StdfRecord::MIR(mir_rec.from_bytes(raw_data, order)),
            StdfRecord::MRR(mrr_rec) => StdfRecord::MRR(mrr_rec.from_bytes(raw_data, order)),
            StdfRecord::PCR(pcr_rec) => StdfRecord::PCR(pcr_rec.from_bytes(raw_data, order)),
            StdfRecord::HBR(hbr_rec) => StdfRecord::HBR(hbr_rec.from_bytes(raw_data, order)),
            StdfRecord::SBR(sbr_rec) => StdfRecord::SBR(sbr_rec.from_bytes(raw_data, order)),
            StdfRecord::PMR(pmr_rec) => StdfRecord::PMR(pmr_rec.from_bytes(raw_data, order)),
            StdfRecord::PGR(pgr_rec) => StdfRecord::PGR(pgr_rec.from_bytes(raw_data, order)),
            StdfRecord::PLR(plr_rec) => StdfRecord::PLR(plr_rec.from_bytes(raw_data, order)),
            StdfRecord::RDR(rdr_rec) => StdfRecord::RDR(rdr_rec.from_bytes(raw_data, order)),
            StdfRecord::SDR(sdr_rec) => StdfRecord::SDR(sdr_rec.from_bytes(raw_data, order)),
            StdfRecord::PSR(psr_rec) => StdfRecord::PSR(psr_rec.from_bytes(raw_data, order)),
            StdfRecord::NMR(nmr_rec) => StdfRecord::NMR(nmr_rec.from_bytes(raw_data, order)),
            StdfRecord::CNR(cnr_rec) => StdfRecord::CNR(cnr_rec.from_bytes(raw_data, order)),
            StdfRecord::SSR(ssr_rec) => StdfRecord::SSR(ssr_rec.from_bytes(raw_data, order)),
            StdfRecord::CDR(cdr_rec) => StdfRecord::CDR(cdr_rec.from_bytes(raw_data, order)),
            // rec type 0
            StdfRecord::FAR(far_rec) => StdfRecord::FAR(far_rec.from_bytes(raw_data, order)),
            StdfRecord::ATR(atr_rec) => StdfRecord::ATR(atr_rec.from_bytes(raw_data, order)),
            StdfRecord::VUR(vur_rec) => StdfRecord::VUR(vur_rec.from_bytes(raw_data, order)),            
            // rec type 20
            StdfRecord::BPS(bps_rec) => StdfRecord::BPS(bps_rec.from_bytes(raw_data, order)),
            StdfRecord::EPS(eps_rec) => StdfRecord::EPS(eps_rec.from_bytes(raw_data, order)),
            // rec type 180: Reserved
            // rec type 181: Reserved
            StdfRecord::ReservedRec(reserve_rec) => StdfRecord::ReservedRec(reserve_rec.from_bytes(raw_data, order)),
            // not matched
            StdfRecord::InvalidRec => self,
        }
    }
}


// data type functions
/// Read uint8 from byte array with offset "pos", compatible with B1, C1 and U1
fn read_uint8(raw_data: &[u8], pos: &mut usize) -> u8 {
    if *pos < raw_data.len() {
        let value = (*raw_data)[*pos];
        *pos += 1;
        value
    } else {
        0
    }
}

/// Read U2 (u16) from byte array with offset "pos"
fn read_u2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U2 {
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
fn read_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U4 {
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
fn read_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U8 {
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
fn read_i1(raw_data: &[u8], pos: &mut usize) -> I1 {
    if *pos < raw_data.len() {
        let value = (*raw_data)[*pos] as I1;
        *pos += 1;
        value
    } else {
        0
    }
}

/// Read I2 (i16) from byte array with offset "pos"
fn read_i2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> I2 {
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
fn read_i4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> I4 {
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
fn read_r4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> R4 {
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
fn read_r8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> R8 {
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
fn read_cn(raw_data: &[u8], pos: &mut usize) -> Cn {
    let count = read_uint8(raw_data, pos) as usize;
    let mut value = String::from("");
    if count != 0 {
        let pos_after_read = *pos + count;
        if pos_after_read <= raw_data.len() {
            // read count
            value.push_str(std::str::from_utf8(&raw_data[*pos..pos_after_read]).unwrap());
            *pos = pos_after_read;
        } else {
            // read all
            value.push_str(std::str::from_utf8(&raw_data[*pos..]).unwrap());
            *pos = raw_data.len();
        }
    }
    value
}

/// Read Sn (u16 + String) from byte array with offset "pos"
fn read_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> Sn {
    let count = read_u2(raw_data, pos, order) as usize;
    let mut value = String::from("");
    if count != 0 {
        let pos_after_read = *pos + count;
        if pos_after_read <= raw_data.len() {
            // read count
            value.push_str(std::str::from_utf8(&raw_data[*pos..pos_after_read]).unwrap());
            *pos = pos_after_read;
        } else {
            // read all
            value.push_str(std::str::from_utf8(&raw_data[*pos..]).unwrap());
            *pos = raw_data.len();
        }
    }
    value
}

/// Read Cf (String) from byte array with offset "pos", String length is provide by "f"
fn read_cf(raw_data: &[u8], pos: &mut usize, f: u8) -> Cf {
    let mut value = String::from("");
    if f != 0 {
        let pos_after_read = *pos + (f as usize);
        if pos_after_read <= raw_data.len() {
            // read count
            value.push_str(std::str::from_utf8(&raw_data[*pos..pos_after_read]).unwrap());
            *pos = pos_after_read;
        } else {
            // read all
            value.push_str(std::str::from_utf8(&raw_data[*pos..]).unwrap());
            *pos = raw_data.len();
        }
    }
    value
}

/// Read Bn (u8 + Vec<u8>) from byte array with offset "pos"
fn read_bn(raw_data: &[u8], pos: &mut usize) -> Bn {
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
fn read_dn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> Dn {
    let bitcount = read_u2(raw_data, pos, order) as usize;
    let bytecount = bitcount/8 + bitcount%8;
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
fn read_kx_cn(raw_data: &[u8], pos: &mut usize, k: u16) -> KxCn {
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
fn read_kx_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxSn {
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
fn read_kx_cf(raw_data: &[u8], pos: &mut usize, k: u16, f: u8) -> KxCf {
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
fn read_kx_u1(raw_data: &[u8], pos: &mut usize, k: u16) -> KxU1 {
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
fn read_kx_u2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU2 {
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
fn read_kx_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU4 {
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
fn read_kx_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU8 {
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
fn read_kx_uf(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16, f: u8) -> KxUf {
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
fn read_kx_r4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxR4 {
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
fn read_kx_n1(raw_data: &[u8], pos: &mut usize, k: u16) -> KxN1 {
    if k != 0 {
        let bytecount = k/2 + k%2;   // k = nibble counts, 1 byte = 2 nibble
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
fn read_v1(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> V1 {
    let type_byte = if (*pos as usize) < raw_data.len() 
    { read_uint8(raw_data, pos) } else { 0xF };

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
        13 => V1::N1(read_uint8(raw_data, pos)),
        _ => V1::Invalid,
    }
}

/// Read Vn (Vec<V1>) from byte array with offset "pos", vector size is provide by "k"
fn read_vn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> Vn {
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