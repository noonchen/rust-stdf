//
// stdf_error.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Mon Oct 03 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//



use std::fmt;
use std::io;
    
#[derive(Debug)]
pub struct StdfError {
    pub code: u8,
    pub msg: String,
}

impl fmt::Display for StdfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let short_msg = match self.code {
            1 => "Invalid STDF File",
            2 => "Invalid Record Type",
            3 => "IO Error",
            4 => "Insufficient Data",
            _ => "Other error",
        };
        write!(f, "{}, {}", short_msg, self.msg)
    }
}

impl From<io::Error> for StdfError {
    fn from(error: io::Error) -> Self {
        StdfError { 
            code: 3, 
            msg: format!("{}, {}", error.kind(), error)
        }
    }
}
