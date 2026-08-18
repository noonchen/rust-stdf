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
pub struct GDR {
    pub fld_cnt: U2, // Count of data fields in record
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
