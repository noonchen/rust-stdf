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
