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

impl EPS {
    pub fn new() -> Self {
        EPS::default()
    }
}
