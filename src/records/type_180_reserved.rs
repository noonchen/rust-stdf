use crate::stdf_codec::*;
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
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct ReservedRec {
    pub raw_data: Vec<u8>, // unparsed data
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
