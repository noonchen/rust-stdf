use crate::stdf_codec::*;
use crate::stdf_error::{StdfError, StdfErrorKind};
use rust_stdf_derive::{stdf_match_expr, stdf_records};
use smart_default::SmartDefault;

mod type_0_file_info;
mod type_10_test_synopsis;
mod type_15_test_info;
mod type_180_reserved;
mod type_1_lot_info;
mod type_20_generic_data;
mod type_2_wafer_info;
mod type_50_program_segment;
mod type_5_part_info;

pub use type_0_file_info::*;
pub use type_10_test_synopsis::*;
pub use type_15_test_info::*;
pub use type_180_reserved::*;
pub use type_1_lot_info::*;
pub use type_20_generic_data::*;
pub use type_2_wafer_info::*;
pub use type_50_program_segment::*;
pub use type_5_part_info::*;

#[derive(SmartDefault, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordHeader {
    pub len: u16,
    pub typ: u8,
    pub sub: u8,
}

// Record Types

/// This module contains constants
/// for STDF Record type check and
/// some help functions
///
/// ```
/// use rust_stdf::{StdfRecord, stdf_record_type::*};
///
/// // use constant for record initializing
/// let mut rec = StdfRecord::new(REC_MIR);
///
/// // for type check
/// let t = REC_MIR | REC_MRR | REC_PTR;
/// let is_t = rec.is_type(t);      // true
/// ```
pub mod stdf_record_type {
    use crate::stdf_error::{StdfError, StdfErrorKind};
    use rust_stdf_derive::{stdf_match_expr, stdf_records};

    // Generates the `REC_*` record-type code constants.
    stdf_records!(rec_codes);

    /// This function convert record type constant to
    /// STDF record (typ, sub)
    ///
    /// ```
    /// use rust_stdf::stdf_record_type::*;
    ///
    /// let ptr_typ_sub = get_typ_sub_from_code(REC_PTR).unwrap();
    /// assert_eq!((15, 10), ptr_typ_sub);
    /// ```
    #[inline(always)]
    pub fn get_typ_sub_from_code(code: u64) -> Result<(u8, u8), StdfError> {
        stdf_match_expr!(typ_sub_from_code)
    }

    /// This function convert (typ, sub) to
    /// STDF record type constant
    ///
    /// ```
    /// use rust_stdf::stdf_record_type::*;
    ///
    /// let type_code = get_code_from_typ_sub(15, 10);
    /// assert_eq!(REC_PTR, type_code);
    /// ```
    #[inline(always)]
    pub fn get_code_from_typ_sub(typ: u8, sub: u8) -> u64 {
        stdf_match_expr!(code_from_typ_sub)
    }

    /// This function convert record type constant to
    /// STDF record name string
    ///
    /// ```
    /// use rust_stdf::stdf_record_type::*;
    ///
    /// let rec_name = get_rec_name_from_code(REC_PTR);
    /// assert_eq!("PTR", rec_name);
    /// ```
    #[inline(always)]
    pub fn get_rec_name_from_code(rec_type: u64) -> &'static str {
        stdf_match_expr!(name_from_code)
    }

    /// This function convert record name string to
    /// STDF record type constant
    ///
    /// ```
    /// use rust_stdf::stdf_record_type::*;
    ///
    /// let type_code = get_code_from_rec_name("PTR");
    /// assert_eq!(REC_PTR, type_code);
    /// ```
    ///
    #[inline(always)]
    pub fn get_code_from_rec_name(rec_name: &str) -> u64 {
        stdf_match_expr!(code_from_name)
    }
}

// Generates the `StdfRecord` and `StdfRecordView` enums.
stdf_records!(rec_enums);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Element yielded by [`RawDataIter`](crate::stdf_file::RawDataIter). It owns unprocessed STDF record data,
/// and can be converted to [`StdfRecord`], or borrowed as [`StdfRecordView`].
///
/// ```
/// use rust_stdf::{RawDataElement, ByteOrder, StdfRecord, RecordHeader, stdf_record_type::REC_FAR};
///
/// let rde = RawDataElement {
///     offset: 0,
///     header: RecordHeader {typ: 0, sub: 10, len: 2},
///     raw_data: vec![0u8; 0],
///     byte_order: ByteOrder::LittleEndian
/// };
/// let rec: StdfRecord = (&rde).into();    // not consume
/// let rec: StdfRecord = rde.into();       // consume
/// println!("{:?}", rec);
/// assert!(rec.is_type(REC_FAR));
/// ```
pub struct RawDataElement {
    /// file offset of `raw_data` in file,
    /// after header.len and before raw_data
    ///
    /// |-typ-|-sub-|--len--⬇️--raw..data--|
    ///
    /// note that the offset is relative to the
    /// file position that runs `get_rawdata_iter`,
    ///
    /// it can be treated as file position **only if**
    /// the iteration starts from beginning of the file.
    pub offset: u64,

    /// used for identifying StdfRecord types
    pub header: RecordHeader,

    /// field data of current STDF Record
    pub raw_data: Vec<u8>,
    pub byte_order: ByteOrder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Element yielded by [`RawDataViewIter`](crate::stdf_file::RawDataViewIter). Unlike [`RawDataElement`],
/// it borrows the unprocessed record bytes from the iterator's reused buffer, so it's only valid
/// until the next call to [`RawDataViewIter::next`](crate::stdf_file::RawDataViewIter::next).
///
/// In return, it offers better performance and a lower heap footprint.
///
/// It can be converted to [`StdfRecord`], borrowed as [`StdfRecordView`], or copied into an owned
/// [`RawDataElement`] when it needs to outlive the current iteration scope.
///
/// ```
/// use rust_stdf::{RawDataElementView, ByteOrder, StdfRecord, RecordHeader, stdf_record_type::REC_FAR};
///
/// let raw = [0u8; 0];
/// let rdv = RawDataElementView {
///     offset: 0,
///     header: RecordHeader {typ: 0, sub: 10, len: 2},
///     raw_data: &raw,
///     byte_order: ByteOrder::LittleEndian
/// };
/// let rec: StdfRecord = (&rdv).into();
/// println!("{:?}", rec);
/// assert!(rec.is_type(REC_FAR));
/// ```
pub struct RawDataElementView<'a> {
    /// file offset of `raw_data` in file,
    /// after header.len and before raw_data
    ///
    /// |-typ-|-sub-|--len--⬇️--raw..data--|
    ///
    /// note that the offset is relative to the
    /// file position that runs `get_rawdata_view_iter`,
    ///
    /// it can be treated as file position **only if**
    /// the iteration starts from beginning of the file.
    pub offset: u64,

    /// used for identifying StdfRecord types
    pub header: RecordHeader,

    /// field data of current STDF Record, borrowed from the iterator's reused buffer
    pub raw_data: &'a [u8],
    pub byte_order: ByteOrder,
}

// implementation

impl RecordHeader {
    #[inline(always)]
    pub fn new() -> Self {
        RecordHeader::default()
    }

    /// Construct a STDF record header from first 4 elements of given byte array,
    /// no error would occur even if the header is invalid.
    ///
    /// Unless the array size is less than 4, StdfError of
    /// `EOF` or `Unexpected EOF` will be returned
    #[inline(always)]
    pub fn read_from_bytes(
        mut self,
        raw_data: &[u8],
        order: &ByteOrder,
    ) -> Result<Self, StdfError> {
        match raw_data.len() {
            0 => Err(StdfError::new(
                StdfErrorKind::Eof,
                String::from("No bytes to read"),
            )),
            1..=3 => Err(StdfError::new(
                StdfErrorKind::UnexpectedEof,
                String::from("Not enough data to construct record header"),
            )),
            _ => {
                let len_bytes = [raw_data[0], raw_data[1]];
                self.len = match order {
                    ByteOrder::LittleEndian => u16::from_le_bytes(len_bytes),
                    ByteOrder::BigEndian => u16::from_be_bytes(len_bytes),
                };
                self.typ = raw_data[2];
                self.sub = raw_data[3];
                // return even if we have a invalid record type, let other code to handle it
                Ok(self)
            }
        }
    }

    /// return the type_code of current header
    pub fn get_type(&self) -> u64 {
        stdf_record_type::get_code_from_typ_sub(self.typ, self.sub)
    }
}

impl StdfRecord {
    /// Create a StdfRecord of a given type with default data
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, stdf_record_type::REC_PMR};
    ///
    /// // create StdfRecord with a nested PMR
    /// let new_rec = StdfRecord::new(REC_PMR);
    ///
    /// if let StdfRecord::PMR(pmr_rec) = new_rec {
    ///     assert_eq!(pmr_rec.head_num, 1);
    ///     assert_eq!(pmr_rec.site_num, 1);
    /// } else {
    ///     // this case will not be hit
    /// }
    /// ```
    #[inline(always)]
    pub fn new(rec_type: u64) -> Self {
        stdf_match_expr!(record_new)
    }

    /// Create a `StdfRecord` from a `RecordHeader` with default data
    ///
    /// The difference between `new()` is that this method can save
    /// the info of an unknown/reserved record header, help the caller to
    /// debug
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, RecordHeader, stdf_record_type::REC_PMR};
    ///
    /// // create a PMR StdfRecord from header
    /// // (1, 60)
    /// let pmr_header = RecordHeader {typ: 1, sub: 60, len: 0 };
    /// let new_rec = StdfRecord::new_from_header(pmr_header);
    ///
    /// if let StdfRecord::PMR(pmr_rec) = new_rec {
    ///     assert_eq!(pmr_rec.head_num, 1);
    ///     assert_eq!(pmr_rec.site_num, 1);
    /// } else {
    ///     // this case will not be hit
    /// }
    /// ```
    #[inline(always)]
    pub fn new_from_header(header: RecordHeader) -> Self {
        let code = stdf_record_type::get_code_from_typ_sub(header.typ, header.sub);
        match code {
            stdf_record_type::REC_RESERVE => {
                let mut rec = ReservedRec::new();
                rec.typ = header.typ;
                rec.sub = header.sub;
                StdfRecord::ReservedRec(rec)
            }
            stdf_record_type::REC_UNKNOWN => {
                let mut rec = ReservedRec::new();
                rec.typ = header.typ;
                rec.sub = header.sub;
                StdfRecord::UnknownRec(rec)
            }
            _ => StdfRecord::new(code),
        }
    }

    /// Returns the record type cdoe of the given StdfRecord,
    /// which is defined in `rust_stdf::stdf_record_type::*` module.
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, stdf_record_type::*};
    ///
    /// // `REC_PTR` type code can be used for creating a new StdfRecord
    /// let new_rec = StdfRecord::new(REC_PTR);
    /// let returned_code = new_rec.get_type();
    ///
    /// assert_eq!(REC_PTR, returned_code);
    ///
    /// // type code can be used in variety of functions
    /// // get record (typ, sub)
    /// assert_eq!((15, 10), get_typ_sub_from_code(returned_code).unwrap());
    /// // get record name
    /// assert_eq!("PTR", get_rec_name_from_code(returned_code));
    /// ```
    #[inline(always)]
    pub fn get_type(&self) -> u64 {
        stdf_match_expr!(record_type)
    }

    /// Check the StdfRecord belongs the given type code(s),
    /// it is useful for filtering the records during the parsing iteration.
    /// ```
    /// use rust_stdf::{StdfRecord, stdf_record_type::*};
    ///
    /// let new_rec = StdfRecord::new(REC_PTR);
    ///
    /// assert!(new_rec.is_type(REC_PTR));
    /// assert!(new_rec.is_type(REC_PTR | REC_FTR | REC_MPR));
    /// assert!(!new_rec.is_type(REC_FTR | REC_MPR));
    /// ```
    #[inline(always)]
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.get_type() & rec_type) != 0
    }

    /// Parse StdfRecord from byte data which **DOES NOT**
    /// contain the record header (len, typ, sub),
    ///
    /// requires a mutable StdfRecord to store the parsed data
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, ByteOrder, stdf_record_type::*};
    ///
    /// let raw_with_no_header: [u8; 2] = [1, 4];
    /// let mut new_rec = StdfRecord::new(REC_FAR);
    /// new_rec.read_from_bytes(&raw_with_no_header, &ByteOrder::LittleEndian);
    ///
    /// if let StdfRecord::FAR(ref far_rec) = new_rec {
    ///     assert_eq!(4, far_rec.stdf_ver);
    /// }
    /// ```
    #[inline(always)]
    pub fn read_from_bytes(&mut self, raw_data: &[u8], order: &ByteOrder) {
        stdf_match_expr!(record_read)
    }

    /// Parse StdfRecord from byte data which
    /// **contains** the record header (len, typ, sub).
    ///
    /// ## Error
    /// if the input data is not a valid (wrong typ, sub),
    /// incomplete data or incorrect byte order, [`StdfError`] will be
    /// returned instead.
    ///
    /// ```
    /// use rust_stdf::{StdfRecord, ByteOrder, stdf_record_type::*};
    ///
    /// let raw_with_header: [u8; 6] = [0, 2, 0, 10, 1, 4];
    /// let new_rec = StdfRecord::read_from_bytes_with_header(&raw_with_header, &ByteOrder::BigEndian).unwrap();
    ///
    /// if let StdfRecord::FAR(far_rec) = new_rec {
    ///     assert_eq!(4, far_rec.stdf_ver);
    /// }
    /// ```
    #[inline(always)]
    pub fn read_from_bytes_with_header(
        raw_data: &[u8],
        order: &ByteOrder,
    ) -> Result<StdfRecord, StdfError> {
        let header = RecordHeader::new().read_from_bytes(raw_data, order)?;

        let expected_end_pos = 4 + header.len as usize;
        if raw_data.len() < expected_end_pos {
            return Err(StdfError::new(
                StdfErrorKind::UnexpectedEof,
                format!(
                    "Length of stdf field data ({} - 4 = {}) is less than what header specified ({})",
                    raw_data.len(),
                    raw_data.len() - 4,
                    header.len
                ),
            ));
        }

        let data_slice = &raw_data[4..expected_end_pos];
        let mut rec = StdfRecord::new_from_header(&header);
        rec.read_from_bytes(data_slice, order);
        Ok(rec)
    }
}

impl<'a> StdfRecordView<'a> {
    /// Build a view from `RecordHeader`, raw field data and byte order.
    ///
    /// ```
    /// use rust_stdf::{StdfRecordView, ByteOrder, RecordHeader, stdf_record_type::REC_PTR};
    ///
    /// let header = RecordHeader { len: 0, typ: 15, sub: 10 }; // PTR
    /// let raw: [u8; 0] = []; // empty field data; getters fall back to defaults
    /// let view = StdfRecordView::read_from_bytes(header, &raw, &ByteOrder::LittleEndian);
    ///
    /// if let StdfRecordView::PTR(ptr_view) = view {
    ///     assert_eq!(0, ptr_view.test_num());
    /// }
    /// assert!(view.is_type(REC_PTR));
    /// ```
    #[inline]
    pub fn read_from_bytes(
        header: RecordHeader,
        raw_data: &'a [u8],
        byte_order: &ByteOrder,
    ) -> Self {
        stdf_match_expr!(view_read)
    }

    /// Parse a `StdfRecordView` from byte data that **contains** the record
    /// header (len, typ, sub).
    ///
    /// ## Error
    /// Returns [`StdfError`] when the input is not a valid header or is
    /// incomplete (the buffer is shorter than the header declares).
    ///
    /// ```
    /// use rust_stdf::{StdfRecordView, ByteOrder};
    ///
    /// let raw_with_header: [u8; 6] = [0, 2, 0, 10, 1, 4];
    /// let view = StdfRecordView::read_from_bytes_with_header(
    ///     &raw_with_header,
    ///     &ByteOrder::BigEndian,
    /// ).unwrap();
    ///
    /// if let StdfRecordView::FAR(far_view) = view {
    ///     assert_eq!(4, far_view.stdf_ver());
    /// }
    /// ```
    #[inline(always)]
    pub fn read_from_bytes_with_header(
        raw_data: &'a [u8],
        order: &ByteOrder,
    ) -> Result<Self, StdfError> {
        let header = RecordHeader::new().read_from_bytes(raw_data, order)?;

        let expected_end_pos = 4 + header.len as usize;
        if raw_data.len() < expected_end_pos {
            return Err(StdfError::new(
                StdfErrorKind::UnexpectedEof,
                format!(
                    "Length of stdf field data ({} - 4 = {}) is less than what header specified ({})",
                    raw_data.len(),
                    raw_data.len() - 4,
                    header.len
                ),
            ));
        }

        let data_slice = &raw_data[4..expected_end_pos];
        Ok(Self::read_from_bytes(header, data_slice, order))
    }

    /// Return the record type code (see `stdf_record_type::*`).
    #[inline(always)]
    pub fn get_type(&self) -> u64 {
        stdf_match_expr!(view_type)
    }

    /// Check whether this view belongs to the given record type(s).
    #[inline(always)]
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.get_type() & rec_type) != 0
    }

    /// Parse this borrowed view into an owned [`StdfRecord`], escaping the
    /// borrow of the underlying buffer.
    #[inline]
    pub fn to_owned(&self) -> StdfRecord {
        stdf_match_expr!(view_to_owned)
    }
}

impl RawDataElement {
    #[inline(always)]
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.header.get_type() & rec_type) != 0
    }
}

impl From<&RawDataElement> for StdfRecord {
    /// it will NOT consume the input RawDataElement
    #[inline(always)]
    fn from(raw_element: &RawDataElement) -> Self {
        let mut rec = StdfRecord::new_from_header(raw_element.header);
        rec.read_from_bytes(&raw_element.raw_data, &raw_element.byte_order);
        rec
    }
}

impl<'a> From<&'a RawDataElement> for StdfRecordView<'a> {
    /// Build a zero-copy view; does not consume the input.
    #[inline]
    fn from(raw_element: &'a RawDataElement) -> Self {
        Self::read_from_bytes(
            raw_element.header,
            &raw_element.raw_data,
            &raw_element.byte_order,
        )
    }
}

impl From<RawDataElement> for StdfRecord {
    /// it will consume the input RawDataElement
    #[inline(always)]
    fn from(raw_element: RawDataElement) -> Self {
        let mut rec = StdfRecord::new_from_header(raw_element.header);
        rec.read_from_bytes(&raw_element.raw_data, &raw_element.byte_order);
        rec
    }
}

impl RawDataElementView<'_> {
    #[inline(always)]
    pub fn is_type(&self, rec_type: u64) -> bool {
        (self.header.get_type() & rec_type) != 0
    }
}

impl From<&RawDataElementView<'_>> for StdfRecord {
    /// Parse the borrowed bytes into an owned record; does not consume the input.
    #[inline(always)]
    fn from(raw_view: &RawDataElementView<'_>) -> Self {
        let mut rec = StdfRecord::new_from_header(raw_view.header);
        rec.read_from_bytes(raw_view.raw_data, &raw_view.byte_order);
        rec
    }
}

impl<'a> From<&RawDataElementView<'a>> for StdfRecordView<'a> {
    /// Build a zero-copy view; does not consume the input.
    #[inline]
    fn from(raw_view: &RawDataElementView<'a>) -> Self {
        Self::read_from_bytes(raw_view.header, raw_view.raw_data, &raw_view.byte_order)
    }
}

impl From<&RawDataElementView<'_>> for RawDataElement {
    /// Copy the borrowed bytes into an owned [`RawDataElement`]; does not consume the input.
    #[inline(always)]
    fn from(raw_view: &RawDataElementView<'_>) -> Self {
        RawDataElement {
            offset: raw_view.offset,
            header: raw_view.header,
            raw_data: raw_view.raw_data.to_vec(),
            byte_order: raw_view.byte_order,
        }
    }
}

impl<'a> From<&StdfRecordView<'a>> for StdfRecord {
    /// Parse the borrowed view into an owned record; does not consume the view.
    #[inline(always)]
    fn from(view: &StdfRecordView<'a>) -> Self {
        view.to_owned()
    }
}
