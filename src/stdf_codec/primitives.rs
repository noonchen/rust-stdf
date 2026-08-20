use super::primitive_read::*;
use smart_default::SmartDefault;
use std::borrow::Cow;

#[cfg(feature = "serialize")]
use serde::Serialize;

// Common Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressType {
    Uncompressed,
    #[cfg(feature = "gzip")]
    GzipCompressed,
    #[cfg(feature = "bzip")]
    BzipCompressed,
    #[cfg(feature = "zipfile")]
    ZipCompressed,
}

// Data Types

/// Altough B1 can be treated as u8, but its representation
/// in ATDF is differ from U1, so I used a array of one u8 for B1
pub type B1 = [u8; 1];
/// Rust char is 4 bytes long, however STDF char is only 1 byte
/// we will read u8 from file stream and convert to Rust char during parse step
pub type C1 = char;
pub type U1 = u8;
pub type U2 = u16;
pub type U4 = u32;
pub type U8 = u64;
pub type I1 = i8;
pub type I2 = i16;
pub type I4 = i32;
pub type R4 = f32;
pub type R8 = f64;

// Variable length character string, first byte = unsigned count of bytes to follow (maximum of 255 bytes)
pub type Cn = String;

// Variable length character string, string length is stored in another field
pub type Cf = String;

// Variable length character string, first two bytes = unsigned count of bytes to follow (maximum of 65535 bytes)
pub type Sn = String;

// Variable length byte array, first byte = unsigned count of bytes to follow (maximum of 255 bytes)
pub type Bn = Vec<u8>;

/// Variable length bit array, first two bytes = unsigned count of bits to follow (maximum of 65,535 bits)
///
/// `bit_count` is the declared number of bits, and `bit_data` stores the
/// packed bytes (length should be `ceil(bit_count / 8)`).
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub struct Dn {
    pub bit_count: u16,
    pub bit_data: Vec<u8>,
}

pub type KxCn = Vec<Cn>;
pub type KxSn = Vec<Sn>;
pub type KxCf = Vec<Cf>;
pub type KxU1 = Vec<U1>;
pub type KxU2 = Vec<U2>;
pub type KxU4 = Vec<U4>;
pub type KxU8 = Vec<U8>;
pub type KxR4 = Vec<R4>;
pub type KxN1 = Vec<U1>;

/// This enum is for STR that
/// introduced in STDF V4-2007.
///
/// the nested data is a vector of Uf type,
/// where f = 1, 2, 4 or 8
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(SmartDefault, Debug, Clone, PartialEq, Eq)]
pub enum KxUf {
    #[default]
    F1(KxU1),
    F2(KxU2),
    F4(KxU4),
    F8(KxU8),
}

/// This enum is for storing
/// generic data V1, the data type
/// is the field name.
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum V1 {
    B0,
    U1(U1),
    U2(U2),
    U4(U4),
    I1(I1),
    I2(I2),
    I4(I4),
    R4(R4),
    R8(R8),
    Cn(Cn),
    Bn(Bn),
    Dn(Dn),
    N1(U1),
    Invalid,
}

pub type Vn = Vec<V1>;

// ----------------------------------------------------------------------------
// Zero-copy borrows of the variable-length payload types, used by the generated
// `*View` getters. Each borrows into the raw record buffer; the `to_owned`
// method reproduces the owned value with the same semantics as the eager
// `read_*` helpers, so view and eager results match.
// ----------------------------------------------------------------------------

/// Sentinel stored in a view when a (trailing/optional) field is absent.
pub(crate) const VIEW_ABSENT_OFT: u16 = u16::MAX;

/// Validate a stored offset, turning it into `Some(pos)`, or `None` when the field is absent.
#[inline]
pub(crate) fn validate_offset(off: u16) -> Option<usize> {
    if off == VIEW_ABSENT_OFT {
        None
    } else {
        Some(off as usize)
    }
}

/// Zero-copy borrow of a `Cn` payload (the bytes *after* the 1-byte length prefix).
#[derive(SmartDefault, Clone, Copy, Debug)]
pub struct CnRef<'a>(&'a [u8]);

/// Zero-copy borrow of an `Sn` payload (the bytes *after* the 2-byte,
/// byte-order-dependent length prefix).
#[derive(SmartDefault, Clone, Copy, Debug)]
pub struct SnRef<'a>(&'a [u8]);

/// Zero-copy borrow of a `Bn` payload (the bytes *after* the 1-byte length prefix).
#[derive(SmartDefault, Clone, Copy, Debug)]
pub struct BnRef<'a>(&'a [u8]);

/// Zero-copy borrow of a `Dn` payload (the bytes *after* the 2-byte,
/// byte-order-dependent bit-count prefix).
#[derive(SmartDefault, Clone, Copy, Debug)]
pub struct DnRef<'a> {
    bit_count: u16,
    bit_data: &'a [u8],
}

/// Zero-copy view over the `k` elements of a `KxCn` array (each element is a
/// 1-byte length prefix followed by the payload).
///
/// Elements are decoded with the same rule as [`CnRef::as_str`] and can be
/// iterated directly:
///
/// ```
/// use rust_stdf::KxCnRef;
///
/// let raw = [2, b'A', b'B', 2, b'C', b'D']; // two elements: "AB", "CD"
/// let kx = KxCnRef::new(&raw, 0, 2);
/// for s in &kx {
///     println!("{s}");
/// }
/// ```
#[derive(Clone, Copy, Default)]
pub struct KxCnRef<'a> {
    raw: &'a [u8],
    start: usize,
    k: usize,
}

/// Iterator over the elements of a [`KxCnRef`], one pass.
pub struct KxCnRefIter<'a> {
    raw: &'a [u8],
    pos: usize,
    remaining: usize,
}

/// Zero-copy view over the `k` elements of a `KxSn` array (each element is a
/// 2-byte, byte-order-dependent length prefix followed by the payload).
///
/// Elements are decoded with the same rule as [`SnRef::as_str`] and can be
/// iterated directly:
///
/// ```
/// use rust_stdf::{ByteOrder, KxSnRef};
///
/// let raw = [2, 0, b'A', b'B', 2, 0, b'C', b'D']; // two elements: "AB", "CD"
/// let kx = KxSnRef::new(&raw, 0, 2, ByteOrder::LittleEndian);
/// for s in &kx {
///     println!("{s}");
/// }
/// ```
#[derive(SmartDefault, Clone, Copy)]
pub struct KxSnRef<'a> {
    raw: &'a [u8],
    start: usize,
    k: usize,
    #[default(ByteOrder::LittleEndian)]
    order: ByteOrder,
}

/// Iterator over the elements of a [`KxSnRef`], one pass.
pub struct KxSnRefIter<'a> {
    raw: &'a [u8],
    pos: usize,
    remaining: usize,
    order: ByteOrder,
}

/// Zero-copy view over the `k` elements of a `KxCf` array (each element is a
/// fixed-width `f`-byte string, no length prefix).
///
/// Elements are decoded with the same rule as [`CnRef::as_str`] and can be
/// iterated directly:
///
/// ```
/// use rust_stdf::KxCfRef;
///
/// let raw = [b'A', b'B', b'C', b'D']; // two fixed 2-byte elements: "AB", "CD"
/// let kx = KxCfRef::new(&raw, 0, 2, 2);
/// for s in &kx {
///     println!("{s}");
/// }
/// ```
#[derive(Clone, Copy, Default)]
pub struct KxCfRef<'a> {
    raw: &'a [u8],
    start: usize,
    k: usize,
    f: usize,
}

/// Iterator over the elements of a [`KxCfRef`], one pass.
pub struct KxCfRefIter<'a> {
    raw: &'a [u8],
    pos: usize,
    remaining: usize,
    f: usize,
}

impl<'a> CnRef<'a> {
    /// Read a `Cn` starting at the stored offset `off` (which points at the
    /// length byte). Returns `None` if the field is absent.
    #[inline]
    pub(crate) fn read_at(raw: &'a [u8], off: u16) -> Option<Self> {
        let p = validate_offset(off)?;
        let cnt = *raw.get(p)? as usize;
        let start = p + 1;
        let end = std::cmp::min(start + cnt, raw.len()); // clamp truncated payloads
        Some(CnRef(&raw[start..end]))
    }

    /// Raw bytes of the string, always zero-copy.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// Zero-copy borrow when the payload is valid UTF-8; otherwise an owned
    /// `String` decoded byte → char (Latin-1). Same decoding as
    /// [`to_owned`](Self::to_owned).
    #[inline]
    pub fn as_str(&self) -> Cow<'a, str> {
        bytes_to_cow_str(self.0)
    }

    /// Allocating; reproduces the owned `Cn` string (valid UTF-8 decoded as
    /// UTF-8, otherwise byte → char Latin-1), matching the eager parser.
    pub fn to_owned(&self) -> Cn {
        bytes_to_string(self.0)
    }
}

impl<'a> SnRef<'a> {
    /// Read an `Sn` starting at the stored offset `off` (which points at the
    /// 2-byte length prefix). Returns `None` if the field is absent.
    #[inline]
    pub(crate) fn read_at(raw: &'a [u8], off: u16, order: &ByteOrder) -> Option<Self> {
        let p = validate_offset(off)?;
        let mut cp = p;
        let cnt = read_u2(raw, &mut cp, order) as usize;
        let start = cp;
        let end = std::cmp::min(start + cnt, raw.len());
        Some(SnRef(raw.get(start..end).unwrap_or(&[])))
    }

    /// Raw bytes of the string, always zero-copy.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// Zero-copy borrow when the payload is valid UTF-8; otherwise an owned
    /// `String` decoded byte → char (Latin-1). Same decoding as
    /// [`to_owned`](Self::to_owned).
    #[inline]
    pub fn as_str(&self) -> Cow<'a, str> {
        bytes_to_cow_str(self.0)
    }

    /// Allocating; reproduces the owned `Sn` string (valid UTF-8 decoded as
    /// UTF-8, otherwise byte → char Latin-1), matching the eager parser.
    pub fn to_owned(&self) -> Sn {
        bytes_to_string(self.0)
    }
}

impl<'a> BnRef<'a> {
    /// Read a `Bn` starting at the stored offset `off` (which points at the
    /// length byte). Returns `None` if the field is absent.
    #[inline]
    pub(crate) fn read_at(raw: &'a [u8], off: u16) -> Option<Self> {
        let p = validate_offset(off)?;
        let cnt = *raw.get(p)? as usize;
        let start = p + 1;
        let end = std::cmp::min(start + cnt, raw.len());
        Some(BnRef(&raw[start..end]))
    }

    /// Raw bytes, always zero-copy.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Allocating; reproduces the owned `Bn` (byte copy) semantics used by
    /// `StdfRecord`, so results match the eager parser.
    pub fn to_owned(&self) -> Bn {
        self.0.to_vec()
    }
}

impl<'a> DnRef<'a> {
    /// Read a `Dn` starting at the stored offset `off` (which points at the
    /// 2-byte bit-count prefix). Returns `None` if the field is absent.
    #[inline]
    pub(crate) fn read_at(raw: &'a [u8], off: u16, order: &ByteOrder) -> Option<Self> {
        let p = validate_offset(off)?;
        let mut cp = p;
        let bitcount = read_u2(raw, &mut cp, order);
        let bytecount = (bitcount as usize).div_ceil(8);
        let start = cp;
        let end = std::cmp::min(start + bytecount, raw.len());
        Some(DnRef {
            bit_count: bitcount,
            bit_data: raw.get(start..end).unwrap_or(&[]),
        })
    }

    /// Declared number of bits in this `Dn` field.
    #[inline]
    pub fn bit_count(&self) -> u16 {
        self.bit_count
    }

    /// Raw packed bytes, always zero-copy.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bit_data
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.bit_data.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.bit_data.len()
    }

    /// Allocating; reproduces the owned `Dn` (byte copy) semantics used by
    /// `StdfRecord`, so results match the eager parser.
    pub fn to_owned(&self) -> Dn {
        Dn {
            bit_count: self.bit_count,
            bit_data: self.bit_data.to_vec(),
        }
    }
}

impl<'a> KxCnRef<'a> {
    /// Construct a view over `k` elements starting at byte offset `start`.
    ///
    /// Normally obtained from a generated `*View` getter; only needed when
    /// parsing a known raw layout directly.
    #[inline]
    pub fn new(raw: &'a [u8], start: usize, k: usize) -> Self {
        KxCnRef { raw, start, k }
    }

    /// Number of string elements in the array.
    #[inline]
    pub fn len(&self) -> usize {
        self.k
    }

    /// `true` when the array has no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.k == 0
    }

    /// Decoded `i`-th element; `None` when `i` is out of range.
    ///
    /// Valid UTF-8 payloads are borrowed zero-copy; other payloads are decoded
    /// byte → char (Latin-1) into an owned `String`.
    #[inline]
    pub fn get_str(&self, i: usize) -> Option<Cow<'a, str>> {
        self.get_bytes(i).map(bytes_to_cow_str)
    }

    /// Raw payload bytes of the `i`-th element (without the length prefix);
    /// `None` when `i` is out of range.
    #[inline]
    pub fn get_bytes(&self, i: usize) -> Option<&'a [u8]> {
        if i >= self.k {
            return None;
        }
        // Walk to the start of element `i`: each previous element advances by
        // 1 (length byte) + clamped payload.
        let mut pos = self.start;
        for _ in 0..i {
            let cnt = *self.raw.get(pos)? as usize;
            pos = std::cmp::min(pos + 1 + cnt, self.raw.len());
        }
        let cnt = *self.raw.get(pos)? as usize;
        let start = pos + 1;
        let end = std::cmp::min(start + cnt, self.raw.len());
        Some(&self.raw[start..end])
    }

    /// Iterator over the decoded elements, one pass.
    #[inline]
    pub fn iter(&self) -> KxCnRefIter<'a> {
        KxCnRefIter {
            raw: self.raw,
            pos: self.start,
            remaining: self.k,
        }
    }

    /// Allocating; collects all elements into `Vec<Cow<'a, str>>`.
    #[inline]
    pub fn as_vec(&self) -> Vec<Cow<'a, str>> {
        self.iter().collect()
    }

    /// Allocating; reproduces the owned `KxCn` (`Vec<String>`), matching the
    /// eager parser.
    #[inline]
    pub fn to_owned(&self) -> KxCn {
        self.iter().map(Cow::into_owned).collect()
    }
}

impl<'a> Iterator for KxCnRefIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let cnt = *self.raw.get(self.pos)? as usize;
        self.pos += 1;
        let end = std::cmp::min(self.pos + cnt, self.raw.len());
        let slice = &self.raw[self.pos..end];
        self.pos = end;
        Some(bytes_to_cow_str(slice))
    }
}

impl<'a> IntoIterator for &'a KxCnRef<'a> {
    type Item = Cow<'a, str>;
    type IntoIter = KxCnRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> KxSnRef<'a> {
    /// Construct a view over `k` elements starting at byte offset `start`,
    /// using `order` for the 2-byte length prefixes.
    ///
    /// Normally obtained from a generated `*View` getter; only needed when
    /// parsing a known raw layout directly.
    #[inline]
    pub fn new(raw: &'a [u8], start: usize, k: usize, order: ByteOrder) -> Self {
        KxSnRef {
            raw,
            start,
            k,
            order,
        }
    }

    /// Number of string elements in the array.
    #[inline]
    pub fn len(&self) -> usize {
        self.k
    }

    /// `true` when the array has no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.k == 0
    }

    /// Decoded `i`-th element; `None` when `i` is out of range.
    ///
    /// Valid UTF-8 payloads are borrowed zero-copy; other payloads are decoded
    /// byte → char (Latin-1) into an owned `String`.
    #[inline]
    pub fn get_str(&self, i: usize) -> Option<Cow<'a, str>> {
        self.get_bytes(i).map(bytes_to_cow_str)
    }

    /// Raw payload bytes of the `i`-th element (without the length prefix);
    /// `None` when `i` is out of range.
    #[inline]
    pub fn get_bytes(&self, i: usize) -> Option<&'a [u8]> {
        if i >= self.k {
            return None;
        }
        // Walk to the start of element `i`: each previous element advances by
        // 2 (length prefix) + clamped payload.
        let mut pos = self.start;
        for _ in 0..i {
            let mut cp = pos;
            let cnt = read_u2(self.raw, &mut cp, &self.order) as usize;
            pos = std::cmp::min(cp + cnt, self.raw.len());
        }
        let mut cp = pos;
        let cnt = read_u2(self.raw, &mut cp, &self.order) as usize;
        let start = cp;
        let end = std::cmp::min(start + cnt, self.raw.len());
        Some(self.raw.get(start..end).unwrap_or(&[]))
    }

    /// Iterator over the decoded elements, one pass.
    #[inline]
    pub fn iter(&self) -> KxSnRefIter<'a> {
        KxSnRefIter {
            raw: self.raw,
            pos: self.start,
            remaining: self.k,
            order: self.order,
        }
    }

    /// Allocating; collects all elements into `Vec<Cow<'a, str>>`.
    #[inline]
    pub fn as_vec(&self) -> Vec<Cow<'a, str>> {
        self.iter().collect()
    }

    /// Allocating; reproduces the owned `KxSn` (`Vec<String>`), matching the
    /// eager parser.
    #[inline]
    pub fn to_owned(&self) -> KxSn {
        self.iter().map(Cow::into_owned).collect()
    }
}

impl<'a> Iterator for KxSnRefIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let mut cp = self.pos;
        let cnt = read_u2(self.raw, &mut cp, &self.order) as usize;
        self.pos = cp;
        let end = std::cmp::min(self.pos + cnt, self.raw.len());
        let slice = self.raw.get(self.pos..end).unwrap_or(&[]);
        self.pos = end;
        Some(bytes_to_cow_str(slice))
    }
}

impl<'a> IntoIterator for &'a KxSnRef<'a> {
    type Item = Cow<'a, str>;
    type IntoIter = KxSnRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> KxCfRef<'a> {
    /// Construct a view over `k` fixed `f`-byte elements starting at byte
    /// offset `start`.
    ///
    /// Normally obtained from a generated `*View` getter; only needed when
    /// parsing a known raw layout directly.
    #[inline]
    pub fn new(raw: &'a [u8], start: usize, k: usize, f: usize) -> Self {
        KxCfRef { raw, start, k, f }
    }

    /// Number of string elements in the array.
    #[inline]
    pub fn len(&self) -> usize {
        self.k
    }

    /// `true` when the array has no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.k == 0
    }

    /// Decoded `i`-th element; `None` when `i` is out of range.
    ///
    /// Valid UTF-8 payloads are borrowed zero-copy; other payloads are decoded
    /// byte → char (Latin-1) into an owned `String`.
    #[inline]
    pub fn get_str(&self, i: usize) -> Option<Cow<'a, str>> {
        self.get_bytes(i).map(bytes_to_cow_str)
    }

    /// Raw payload bytes of the `i`-th element; `None` when `i` is out of
    /// range or lies past the end of the buffer.
    #[inline]
    pub fn get_bytes(&self, i: usize) -> Option<&'a [u8]> {
        if i >= self.k {
            return None;
        }
        let start = self.start + i * self.f;
        let end = std::cmp::min(start + self.f, self.raw.len());
        self.raw.get(start..end)
    }

    /// Iterator over the decoded elements, one pass.
    #[inline]
    pub fn iter(&self) -> KxCfRefIter<'a> {
        KxCfRefIter {
            raw: self.raw,
            pos: self.start,
            remaining: self.k,
            f: self.f,
        }
    }

    /// Allocating; collects all elements into `Vec<Cow<'a, str>>`.
    #[inline]
    pub fn as_vec(&self) -> Vec<Cow<'a, str>> {
        self.iter().collect()
    }

    /// Allocating; reproduces the owned `KxCf` (`Vec<String>`), matching the
    /// eager parser.
    #[inline]
    pub fn to_owned(&self) -> KxCf {
        self.iter().map(Cow::into_owned).collect()
    }
}

impl<'a> Iterator for KxCfRefIter<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let end = std::cmp::min(self.pos + self.f, self.raw.len());
        let slice = self.raw.get(self.pos..end).unwrap_or(&[]);
        self.pos = end;
        Some(bytes_to_cow_str(slice))
    }
}

impl<'a> IntoIterator for &'a KxCfRef<'a> {
    type Item = Cow<'a, str>;
    type IntoIter = KxCfRefIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Decode STDF string bytes to `Cow<'a, str>`: valid UTF-8 is borrowed as-is,
/// otherwise each byte is widened to a `char` (Latin-1) into an owned `String`.
/// Shared by the eager `read_*` helpers and the `*Ref`/`Kx*Ref` views.
#[inline(always)]
pub(crate) fn bytes_to_cow_str(data: &[u8]) -> Cow<'_, str> {
    match std::str::from_utf8(data) {
        Ok(s) => Cow::Borrowed(s),
        Err(_) => Cow::Owned(data.iter().map(|&x| x as char).collect()),
    }
}

/// Decode STDF string bytes to `String`; same rule as [`bytes_to_cow_str`].
#[inline(always)]
pub(crate) fn bytes_to_string(data: &[u8]) -> String {
    bytes_to_cow_str(data).into_owned()
}
