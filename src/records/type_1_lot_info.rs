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
    pub grp_indx: U2, // Unique index associated with pin group
    pub grp_nam: Cn,  // Name of pin group
    pub indx_cnt: U2, // Count of PMR indexes
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
    pub grp_cnt: U2, // Count (k) of pins or pin groups
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
    pub num_bins: U2, // Number (k) of bins being retested
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
    pub head_num: U1, // Test head number
    pub site_grp: U1, // Site group number
    pub site_cnt: U1, // Number (k) of test sites in site group
    #[stdf(count = site_cnt)]
    pub site_num: KxU1, // Array of test site numbers
    pub hand_typ: Cn, // Handler or prober type
    pub hand_id: Cn,  // Handler or prober ID
    pub card_typ: Cn, // Probe card type
    pub card_id: Cn,  // Probe card ID
    pub load_typ: Cn, // Load board type
    pub load_id: Cn,  // Load board ID
    pub dib_typ: Cn,  // DIB board type
    pub dib_id: Cn,   // DIB board ID
    pub cabl_typ: Cn, // Interface cable type
    pub cabl_id: Cn,  // Interface cable ID
    pub cont_typ: Cn, // Handler contactor type
    pub cont_id: Cn,  // Handler contactor ID
    pub lasr_typ: Cn, // Laser type
    pub lasr_id: Cn,  // Laser ID
    pub extr_typ: Cn, // Extra equipment type field
    pub extr_id: Cn,  // Extra equipment ID
}

#[cfg_attr(
    feature = "serialize",
    derive(Serialize, FieldNamesAsArray),
    serde(rename_all = "UPPERCASE"),
    field_names_as_array(rename_all = "UPPERCASE")
)]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq, StdfRecordCodec)]
pub struct PSR {
    pub cont_flg: B1, // Continuation PSR record exist
    pub psr_indx: U2, // PSR Record Index (used by STR records)
    pub psr_nam: Cn,  // Symbolic name of PSR record
    pub opt_flg: B1,  // Optional data flag
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
    pub cont_flg: B1, // Continuation NMR record follows if not 0
    pub totm_cnt: U2, // Count of PMR indexes and ATPG_NAM entries
    pub locm_cnt: U2, // Count of (k) PMR indexes and ATPG_NAM entries in this record
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
    pub ssr_nam: Cn, // Name of the STIL Scan pub structure for reference
    pub chn_cnt: U2, // Count (k) of number of Chains listed in CHN_LIST
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
