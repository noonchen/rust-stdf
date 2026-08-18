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
    pub head_num: U1, // Test head number
    pub site_num: U1, // Test site number
    pub part_flg: B1, // Part information flag
    pub num_test: U2, // Number of tests executed
    pub hard_bin: U2, // Hardware bin number
    #[default = 65535]
    pub soft_bin: U2, // Software bin number
    #[default(-32768)]
    pub x_coord: I2, // (Wafer) X coordinate
    #[default(-32768)]
    pub y_coord: I2, // (Wafer) Y coordinate
    #[default = 0]
    pub test_t: U4, // Elapsed test time in milliseconds
    pub part_id: Cn,  // Part identification
    pub part_txt: Cn, // Part description text
    pub part_fix: Bn, // Part repair information
}
