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
