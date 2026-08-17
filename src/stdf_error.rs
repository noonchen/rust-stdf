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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum StdfErrorKind {
    InvalidStdfFile = 1,
    InvalidRecordType = 2,
    Io = 3,
    Eof = 4,
    UnexpectedEof = 5,
    NonAscii = 6,
    InvalidAtdfFile = 7,
    #[cfg(feature = "zipfile")]
    Zip = 8,
}

impl StdfErrorKind {
    pub(crate) fn from_u8(code: u8) -> Option<Self> {
        Some(match code {
            1 => Self::InvalidStdfFile,
            2 => Self::InvalidRecordType,
            3 => Self::Io,
            4 => Self::Eof,
            5 => Self::UnexpectedEof,
            6 => Self::NonAscii,
            7 => Self::InvalidAtdfFile,
            #[cfg(feature = "zipfile")]
            8 => Self::Zip,
            _ => return None,
        })
    }
}

impl StdfError {
    pub(crate) fn new(kind: StdfErrorKind, msg: impl Into<String>) -> Self {
        Self {
            code: kind as u8,
            msg: msg.into(),
        }
    }

    pub(crate) fn kind(&self) -> Option<StdfErrorKind> {
        StdfErrorKind::from_u8(self.code)
    }
}

impl fmt::Display for StdfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_msg = match StdfErrorKind::from_u8(self.code) {
            Some(StdfErrorKind::InvalidStdfFile) => "Invalid STDF File",
            Some(StdfErrorKind::InvalidRecordType) => "Invalid Record Type",
            Some(StdfErrorKind::Io) => "IO Error",
            Some(StdfErrorKind::Eof) => "EOF",
            Some(StdfErrorKind::UnexpectedEof) => "Unexpected EOF",
            Some(StdfErrorKind::NonAscii) => "Non-ASCII Found",
            Some(StdfErrorKind::InvalidAtdfFile) => "Invalid ATDF File",
            #[cfg(feature = "zipfile")]
            Some(StdfErrorKind::Zip) => "Zip related",
            _ => "Other error",
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
