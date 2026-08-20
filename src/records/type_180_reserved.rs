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
    pub typ: u8,
    pub sub: u8,
    #[default(ByteOrder::LittleEndian)]
    pub byte_order: ByteOrder,
    pub raw_data: Vec<u8>, // unparsed field data
}

impl ReservedRec {
    pub fn new() -> Self {
        ReservedRec::default()
    }

    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        let mut dataclone = Vec::with_capacity(raw_data.len());
        dataclone.extend_from_slice(raw_data);
        self.raw_data = dataclone;
        self.byte_order = *order;
    }
}

/// Borrowed view of a reserved/unknown record.
///
/// This is the view-side counterpart of [`ReservedRec`]. It keeps the record
/// header fields (`typ`/`sub`) and byte order so it can be converted back to an
/// owned record without losing information needed by a future STDF writer.
#[derive(SmartDefault, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReservedRecView<'a> {
    pub typ: u8,
    pub sub: u8,
    #[default(ByteOrder::LittleEndian)]
    pub byte_order: ByteOrder,
    pub raw_data: &'a [u8],
}
