//
// stdf_error.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Mon Nov 14 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use std::fmt;
use std::io::{self, ErrorKind};
#[cfg(feature = "zipfile")]
use zip::result::ZipError;

#[derive(Debug)]
pub struct StdfError {
    pub code: u8,
    pub msg: String,
}

/// Category of an [`StdfError`], returned by [`StdfError::kind`].
///
/// `#[non_exhaustive]`: `match` arms must include a `_` fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum StdfErrorKind {
    InvalidStdfFile = 1,
    InvalidRecordType = 2,
    Io = 3,
    Eof = 4,
    UnexpectedEof = 5,
    NonAscii = 6,
    InvalidAtdfFile = 7,
    #[cfg(feature = "zipfile")]
    Zip = 8,
    InvalidLength = 9,
    CountMismatch = 10,
    WidthMismatch = 11,
    InvalidOptionalOrder = 12,
    InvalidValue = 13,
    RecordTooLarge = 14,
    ByteOrderMismatch = 15,
    Unknown = 255,
}

impl StdfErrorKind {
    pub(crate) fn from_u8(code: u8) -> Self {
        match code {
            1 => Self::InvalidStdfFile,
            2 => Self::InvalidRecordType,
            3 => Self::Io,
            4 => Self::Eof,
            5 => Self::UnexpectedEof,
            6 => Self::NonAscii,
            7 => Self::InvalidAtdfFile,
            #[cfg(feature = "zipfile")]
            8 => Self::Zip,
            9 => Self::InvalidLength,
            10 => Self::CountMismatch,
            11 => Self::WidthMismatch,
            12 => Self::InvalidOptionalOrder,
            13 => Self::InvalidValue,
            14 => Self::RecordTooLarge,
            15 => Self::ByteOrderMismatch,
            _ => Self::Unknown,
        }
    }
}

impl StdfError {
    pub(crate) fn new(kind: StdfErrorKind, msg: impl Into<String>) -> Self {
        Self {
            code: kind as u8,
            msg: msg.into(),
        }
    }

    /// Returns the structured [`StdfErrorKind`] category for this error,
    /// falling back to [`StdfErrorKind::Unknown`] for an unrecognized code.
    pub fn kind(&self) -> StdfErrorKind {
        StdfErrorKind::from_u8(self.code)
    }
}

impl fmt::Display for StdfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_msg = match StdfErrorKind::from_u8(self.code) {
            StdfErrorKind::InvalidStdfFile => "Invalid STDF File",
            StdfErrorKind::InvalidRecordType => "Invalid Record Type",
            StdfErrorKind::Io => "IO Error",
            StdfErrorKind::Eof => "EOF",
            StdfErrorKind::UnexpectedEof => "Unexpected EOF",
            StdfErrorKind::NonAscii => "Non-ASCII Found",
            StdfErrorKind::InvalidAtdfFile => "Invalid ATDF File",
            #[cfg(feature = "zipfile")]
            StdfErrorKind::Zip => "Zip related",
            StdfErrorKind::InvalidLength => "Invalid Length",
            StdfErrorKind::CountMismatch => "Field Count Mismatch",
            StdfErrorKind::WidthMismatch => "Field Width Mismatch",
            StdfErrorKind::InvalidOptionalOrder => "Invalid Optional Field Order",
            StdfErrorKind::InvalidValue => "Invalid Value",
            StdfErrorKind::RecordTooLarge => "Record Too Large",
            StdfErrorKind::ByteOrderMismatch => "Byte Order Mismatch",
            StdfErrorKind::Unknown => "Unknown error",
        };
        write!(f, "{}, {}", short_msg, self.msg)
    }
}

impl From<io::Error> for StdfError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            ErrorKind::UnexpectedEof => {
                StdfError::new(StdfErrorKind::Eof, String::from("End of file detected"))
            }
            _ => StdfError::new(StdfErrorKind::Io, format!("{}, {}", error.kind(), error)),
        }
    }
}

#[cfg(feature = "zipfile")]
impl From<ZipError> for StdfError {
    fn from(error: ZipError) -> Self {
        match error {
            ZipError::Io(err) => StdfError::new(StdfErrorKind::Io, err.to_string()),
            _ => StdfError::new(StdfErrorKind::Zip, error.to_string()),
        }
    }
}
