mod primitive_read;
mod primitive_validate;
mod primitive_write;
mod primitives;

pub(crate) use primitive_read::*;
pub(crate) use primitive_validate::*;
pub(crate) use primitive_write::*;
pub use primitives::*;

use crate::stdf_error::StdfError;

/// Wire-level write implementation generated for every `StdfRecordCodec` record.
///
/// The main public entry point is [`StdfWriter`](crate::stdf_file::StdfWriter).
/// The trait is intentionally `pub(crate)`; `StdfWriter::write_record` is still
/// callable externally because it is an inherent method whose bound is checked
/// for the concrete record type.
pub(crate) trait StdfRecordWrite {
    const REC_TYP: u8;
    const REC_SUB: u8;

    /// Structural validation. Always called before `payload_len`/`write_payload`.
    fn validate(&self) -> Result<(), StdfError>;

    /// Exact serialized payload size (excludes the 4-byte record header).
    fn payload_len(&self) -> usize;

    fn write_payload<W: std::io::Write>(
        &self,
        w: &mut W,
        order: &ByteOrder,
    ) -> Result<(), StdfError>;
}
