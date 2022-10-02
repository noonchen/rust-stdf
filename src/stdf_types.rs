//
// stdf_types.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Mon Oct 03 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//


use crate::stdf_error;

// Data Types
pub type B1 = u8;
pub type C1 = u8;
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
pub type Bn = Box<[u8]>;

// Dn;	//First two bytes = unsigned count of bits to follow (maximum of 65,535 bits)
pub type Dn = Box<[u8]>;

pub type KxCn = Vec<Cn>;
pub type KxSn = Vec<Sn>;
pub type KxCf = Vec<Cf>;
pub type KxU1 = Vec<U1>;
pub type KxU2 = Vec<U2>;
pub type KxU4 = Vec<U4>;
pub type KxU8 = Vec<U8>;
pub type KxR4 = Vec<R4>;
pub type KxN1 = Vec<U1>;

pub enum KxUf {
    F1(KxU1),
    F2(KxU2),
    F4(KxU4),
}

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
	N1(B1)
}

pub type Vn = Vec<V1>;


// Record Types
pub enum StdfRecords {
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
}


pub struct FAR {
    pub cpu_type: U1,  // CPU type that wrote this file
    pub stdf_ver: U1,  // STDF version number
}

pub struct ATR {
    pub mod_tim: U4, //Date and time of STDF file modification
    pub cmd_line: Cn, //Command line of program
}

pub struct VUR {
    pub upd_nam: Cn, //Update Version Name
}

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

pub struct MRR {
    pub finish_t: U4, // Date and time last part tested
    pub disp_cod: C1, // Lot disposition code,default: space
    pub usr_desc: Cn, // Lot description supplied by user
    pub exc_desc: Cn, // Lot description supplied by exec
}

pub struct PCR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub part_cnt: U4, // Number of parts tested
    pub rtst_cnt: U4, // Number of parts retested
    pub abrt_cnt: U4, // Number of aborts during testing
    pub good_cnt: U4, // Number of good (passed) parts tested
    pub func_cnt: U4, // Number of functional parts tested
}

pub struct HBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub hbin_num: U2, // Hardware bin number
    pub hbin_cnt: U4, // Number of parts in bin
    pub hbin_pf: C1, // Pass/fail indication
    pub hbin_nam: Cn, // Name of hardware bin
}

pub struct SBR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub sbin_num: U2, // Software bin number
    pub sbin_cnt: U4, // Number of parts in bin
    pub sbin_pf: C1, // Pass/fail indication
    pub sbin_nam: Cn, // Name of software bin
}

pub struct PMR {
    pub pmr_indx: U2, // Unique index associated with pin
    pub chan_typ: U2, // Channel type
    pub chan_nam: Cn, // Channel name
    pub phy_nam: Cn, // Physical name of pin
    pub log_nam: Cn, // Logical name of pin
    pub head_num: U1, // Head number associated with channel
    pub site_num: U1, // Site number associated with channel
}

pub struct PGR {
    pub grp_indx: U2, // Unique index associated with pin group
    pub grp_nam: Cn, // Name of pin group
    pub indx_cnt: U2, // Count of PMR indexes
    pub pmr_indx: KxU2, // Array of indexes for pins in the group
}

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

pub struct RDR {
    pub num_bins: U2, // Number (k) of bins being retested
    pub rtst_bin: KxU2, // Array of retest bin numbers
}

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

pub struct NMR {
    pub cont_flg: B1, // Continuation NMR record follows if not 0
    pub totm_cnt: U2, // Count of PMR indexes and ATPG_NAM entries
    pub locm_cnt: U2, // Count of (k) PMR indexes and ATPG_NAM entries in this record
    pub pmr_indx: KxU2, // Array of PMR indexes
    pub atpg_nam: KxCn, // Array of ATPG signal names
}

pub struct CNR {
    pub chn_num: U2, // Chain number. Referenced by the CHN_NUM array in an STR record
    pub bit_pos: U4, // Bit position in the chain
    pub cell_nam: Sn, // Scan Cell Name
}

pub struct SSR {
    pub ssr_nam: Cn, // Name of the STIL Scan pub structure for reference
    pub chn_cnt: U2, // Count (k) of number of Chains listed in CHN_LIST
    pub chn_list: KxU2, // Array of CDR Indexes
}

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

pub struct WIR {
    pub head_num: U1, // Test head number
    pub site_grp: U1, // Site group number 255
    pub start_t: U4, // Date and time first part tested
    pub wafer_id: Cn, // Wafer ID length byte = 0
}

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

pub struct PIR {
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
}

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

pub struct BPS {
    pub seq_name: Cn, // Program section (or sequencer) name length byte = 0
}

pub struct EPS {}

pub struct GDR {
    pub fld_cnt: U2, // Count of data fields in record
    pub gen_data: Vn, // Data type code and data for one field(Repeat GEN_DATA for each data field)
}

pub struct DTR {
    pub text_dat: Cn, // ASCII text string
}
