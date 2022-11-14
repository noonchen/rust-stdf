//! `rust-stdf` is a library for parsing
//! Standard Test Data Format (STDF) files
//! of version V4 and V4-2007.
//!
//! Current capability:
//!  - Reading & parsing STDF files.
//!  - Reading & parsing ATDF files. (feature: `atdf`)
//!  - Support several compressed formats.
//!
//! Available features:
//!  - `gzip`: gzip compression (.gz) support powered by `flate2`
//!  - `bzip`: bzip compression (.bz2) support powered by `bzip2`
//!  - `zipfile`: zip compression (.zip) support powered by `zip`
//!  - `atdf`: ATDF reader + STDF -> ATDF convertor (in dev)
//!  - `serialize`: serialize STDF records by `serde`
//!
//! In development:
//!  - (dev) Dump `StdfRecord` to a new stdf file.
//!  - (dev) Functions for ATDF <-> STDF format.

// lib.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 3rd 2022
// -----
// Last Modified: Thu Oct 06 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

extern crate smart_default;

#[cfg(feature = "atdf")]
mod atdf_types;
mod stdf_error;
mod stdf_types;
pub use stdf_types::*;

/// This module contains STDF Reader
/// and record iterator
///
/// For more detailed example, see [`StdfReader`].
pub mod stdf_file;

/// This module contains ATDF Reader
/// and record iterator
///
/// For more detailed example, click `AtdfReader`
#[cfg(feature = "atdf")]
pub mod atdf_file;

#[cfg(test)]
mod tests {
    use crate::*;
    use stdf_types::ByteOrder;

    // unsigned data type
    #[test]
    fn test_read_uint8() {
        let raw_data = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(raw_data[pos], stdf_types::read_uint8(&raw_data, &mut pos));
            assert_eq!(pos, i + 1);
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_uint8(&raw_data, &mut pos));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_u2_le() {
        let byte_len = 2;
        let raw_data = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
        let expect = [0x0201, 0x0302, 0x0403, 0x0504, 0x0605, 0x0706, 0x0807, 0u16];
        let order = ByteOrder::LittleEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expect[pos],
                stdf_types::read_u2(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_u2(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_u2_be() {
        let byte_len = 2;
        let raw_data = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
        let expect = [0x0102, 0x0203, 0x0304, 0x0405, 0x0506, 0x0607, 0x0708, 0u16];
        let order = ByteOrder::BigEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expect[pos],
                stdf_types::read_u2(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_u2(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_u4_le() {
        let byte_len = 4;
        let raw_data = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
        let expect = [
            0x4030201, 0x5040302, 0x6050403, 0x7060504, 0x8070605, 0, 0, 0u32,
        ];
        let order = ByteOrder::LittleEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expect[pos],
                stdf_types::read_u4(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_u4(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_u4_be() {
        let byte_len = 4;
        let raw_data = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
        let expect = [
            0x1020304, 0x2030405, 0x3040506, 0x4050607, 0x5060708, 0, 0, 0u32,
        ];
        let order = ByteOrder::BigEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expect[pos],
                stdf_types::read_u4(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_u4(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_u8_le() {
        let byte_len = 8;
        let raw_data = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];
        let expect = [0x807060504030201, 0x908070605040302, 0, 0, 0, 0, 0, 0, 0u64];
        let order = ByteOrder::LittleEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expect[pos],
                stdf_types::read_u8(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_u8(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_u8_be() {
        let byte_len = 8;
        let raw_data = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];
        let expect = [0x102030405060708, 0x203040506070809, 0, 0, 0, 0, 0, 0, 0u64];
        let order = ByteOrder::BigEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expect[pos],
                stdf_types::read_u8(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_u8(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    // signed data type
    #[test]
    fn test_read_i1() {
        let raw_data: [u8; 8] = [0x00, 0x01, 0x7F, 0xFE, 0x80, 0x81, 0x8F, 0xFF];
        let expected: [i8; 8] = [0, 1, 127, -2, -128, -127, -113, -1];
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(expected[pos], stdf_types::read_i1(&raw_data, &mut pos));
            assert_eq!(pos, i + 1);
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_uint8(&raw_data, &mut pos));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_i2_le() {
        let byte_len = 2;
        let raw_data: [u8; 8] = [0x00, 0x01, 0x7F, 0xFE, 0x80, 0x81, 0x8F, 0xFF];
        let expected: [i16; 8] = [0x100, 0x7F01, -385, -32514, -32384, -28799, -113, 0];
        let order = ByteOrder::LittleEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expected[pos],
                stdf_types::read_i2(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_i2(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_i2_be() {
        let byte_len = 2;
        let raw_data: [u8; 8] = [0x00, 0x01, 0x7F, 0xFE, 0x80, 0x81, 0x8F, 0xFF];
        let expected: [i16; 8] = [0x1, 0x17F, 0x7FFE, -384, -32639, -32369, -28673, 0];
        let order = ByteOrder::BigEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expected[pos],
                stdf_types::read_i2(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_i2(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_i4_le() {
        let byte_len = 4;
        let raw_data: [u8; 8] = [0x00, 0x01, 0x7F, 0xFE, 0x80, 0x81, 0x8F, 0xFF];
        let expected: [i32; 8] = [
            -25231104,
            -2130804991,
            -2122252673,
            -1887338242,
            -7372416,
            0,
            0,
            0,
        ];
        let order = ByteOrder::LittleEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expected[pos],
                stdf_types::read_i4(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_i4(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_i4_be() {
        let byte_len = 4;
        let raw_data: [u8; 8] = [0x00, 0x01, 0x7F, 0xFE, 0x80, 0x81, 0x8F, 0xFF];
        let expected: [i32; 8] = [
            0x17FFE,
            0x17FFE80,
            0x7FFE8081,
            -25132657,
            -2138992641,
            0,
            0,
            0,
        ];
        let order = ByteOrder::BigEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expected[pos],
                stdf_types::read_i4(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0, stdf_types::read_i4(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    // float
    #[test]
    fn test_read_r4_le() {
        let byte_len = 4;
        let raw_data: [u8; 8] = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
        let expected: [f32; 8] = [
            1.5399896e-36,
            6.2071626e-36,
            2.5017467e-35,
            1.0082514e-34,
            4.063216e-34,
            0.0,
            0.0,
            0.0,
        ];
        let order = ByteOrder::LittleEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expected[pos],
                stdf_types::read_r4(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0.0, stdf_types::read_r4(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_r4_be() {
        let byte_len = 4;
        let raw_data: [u8; 8] = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8];
        let expected: [f32; 8] = [
            2.3879393e-38,
            9.625514e-38,
            3.879708e-37,
            1.5636842e-36,
            6.301941e-36,
            0.0,
            0.0,
            0.0,
        ];
        let order = ByteOrder::BigEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expected[pos],
                stdf_types::read_r4(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0.0, stdf_types::read_r4(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_r8_le() {
        let byte_len = 8;
        let raw_data: [u8; 9] = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];
        let expected: [f64; 9] = [
            5.447603722011605e-270,
            3.7258146895053074e-265,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ];
        let order = ByteOrder::LittleEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expected[pos],
                stdf_types::read_r8(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0.0, stdf_types::read_r8(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    #[test]
    fn test_read_r8_be() {
        let byte_len = 8;
        let raw_data: [u8; 9] = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8];
        let expected: [f64; 9] = [
            8.20788039913184e-304,
            5.678932010640861e-299,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ];
        let order = ByteOrder::BigEndian;
        for i in 0..raw_data.len() {
            let mut pos = i;
            assert_eq!(
                expected[pos],
                stdf_types::read_r8(&raw_data, &mut pos, &order)
            );

            if i <= raw_data.len() - byte_len {
                assert_eq!(pos, i + byte_len);
            } else {
                assert_eq!(pos, i);
            }
        }
        let mut pos = raw_data.len();
        assert_eq!(0.0, stdf_types::read_r8(&raw_data, &mut pos, &order));
        assert_eq!(pos, raw_data.len());
    }

    // string & array
    #[test]
    fn test_read_cn() {
        let raw_data: [u8; 9] = [7, 84, 101, 115, 116, 32, 79, 75, 0];
        let mut pos = 0;
        let expect_pos = |p: usize| std::cmp::min(1 + p + raw_data[p] as usize, raw_data.len());
        assert_eq!(
            "Test OK".to_string(),
            stdf_types::read_cn(&raw_data, &mut pos)
        );
        assert_eq!(pos, expect_pos(0));
        let mut pos = 4;
        assert_eq!(
            " OK\0".to_string(),
            stdf_types::read_cn(&raw_data, &mut pos)
        );
        assert_eq!(pos, expect_pos(4));
        let mut pos = 8;
        assert_eq!("".to_string(), stdf_types::read_cn(&raw_data, &mut pos));
        assert_eq!(pos, expect_pos(8));
        // latin1 check
        let raw_data_latin: [u8; 7] = [6, 52, 50, 176, 67, 191, 255];
        assert_eq!(
            "42°C¿ÿ".to_string(),
            stdf_types::read_cn(&raw_data_latin, &mut 0)
        );
    }

    #[test]
    fn test_read_sn_le() {
        let raw_data: [u8; 10] = [7, 0, 84, 101, 115, 116, 32, 79, 75, 0];
        let mut pos = 0;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            "Test OK".to_string(),
            stdf_types::read_sn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 9);
        let mut pos = 4;
        assert_eq!(
            " OK\0".to_string(),
            stdf_types::read_sn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 10);
        let mut pos = 9;
        assert_eq!(
            "".to_string(),
            stdf_types::read_sn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 9);
    }

    #[test]
    fn test_read_sn_be() {
        let raw_data: [u8; 10] = [0, 7, 84, 101, 115, 116, 32, 79, 75, 0];
        let mut pos = 0;
        let order = ByteOrder::BigEndian;
        assert_eq!(
            "Test OK".to_string(),
            stdf_types::read_sn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 9);
        let mut pos = 4;
        assert_eq!(
            " OK\0".to_string(),
            stdf_types::read_sn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 10);
        let mut pos = 9;
        assert_eq!(
            "".to_string(),
            stdf_types::read_sn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 9);
    }

    #[test]
    fn test_read_cf() {
        let raw_data: [u8; 9] = [7, 84, 101, 115, 116, 32, 79, 75, 0];
        let mut pos = 1;
        assert_eq!(
            "Test OK".to_string(),
            stdf_types::read_cf(&raw_data, &mut pos, 7)
        );
        assert_eq!(pos, 8);
        let mut pos = 5;
        assert_eq!(
            " OK\0".to_string(),
            stdf_types::read_cf(&raw_data, &mut pos, 100)
        );
        assert_eq!(pos, 9);
        let mut pos = 8;
        assert_eq!("".to_string(), stdf_types::read_cf(&raw_data, &mut pos, 0));
        assert_eq!(pos, 8);
    }

    #[test]
    fn test_read_bn() {
        let raw_data: [u8; 9] = [7, 84, 101, 115, 116, 32, 79, 75, 0];
        let mut pos = 0;
        assert_eq!(
            vec![84, 101, 115, 116, 32, 79, 75],
            stdf_types::read_bn(&raw_data, &mut pos)
        );
        assert_eq!(pos, 8);
        let mut pos = 4;
        assert_eq!(
            vec![32, 79, 75, 0],
            stdf_types::read_bn(&raw_data, &mut pos)
        );
        assert_eq!(pos, 9);
        let mut pos = 100;
        assert_eq!(vec![0u8; 0], stdf_types::read_bn(&raw_data, &mut pos));
        assert_eq!(pos, 100);
    }

    #[test]
    fn test_read_dn() {
        let raw_data: [u8; 10] = [56, 0, 84, 101, 115, 116, 32, 79, 75, 0];
        let mut pos = 0;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            vec![84, 101, 115, 116, 32, 79, 75],
            stdf_types::read_dn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 9);
        let mut pos = 4;
        assert_eq!(
            vec![32, 79, 75, 0],
            stdf_types::read_dn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 10);
        let mut pos = 100;
        assert_eq!(
            vec![0u8; 0],
            stdf_types::read_dn(&raw_data, &mut pos, &order)
        );
        assert_eq!(pos, 100);
    }

    // Vec
    #[test]
    fn test_read_kx_cn() {
        let raw_data: [u8; 12] = [2, 84, 101, 2, 115, 116, 1, 32, 2, 79, 75, 0];
        let mut pos = 0;
        assert_eq!(
            vec![
                "Te".to_string(),
                "st".to_string(),
                " ".to_string(),
                "OK".to_string()
            ],
            stdf_types::read_kx_cn(&raw_data, &mut pos, 4)
        );
        assert_eq!(pos, 11);
        let mut pos = 3;
        assert_eq!(
            vec![
                "st".to_string(),
                " ".to_string(),
                "OK".to_string(),
                "".to_string()
            ],
            stdf_types::read_kx_cn(&raw_data, &mut pos, 4)
        );
        assert_eq!(pos, 12);
        assert_eq!(
            vec!["".to_string(); 0],
            stdf_types::read_kx_cn(&raw_data, &mut pos, 0)
        );
    }

    #[test]
    fn test_read_kx_sn() {
        let raw_data: [u8; 16] = [2, 0, 84, 101, 2, 0, 115, 116, 1, 0, 32, 2, 0, 79, 75, 0];
        let mut pos = 0;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            vec![
                "Te".to_string(),
                "st".to_string(),
                " ".to_string(),
                "OK".to_string()
            ],
            stdf_types::read_kx_sn(&raw_data, &mut pos, &order, 4)
        );
        assert_eq!(pos, 15);
        let mut pos = 4;
        assert_eq!(
            vec![
                "st".to_string(),
                " ".to_string(),
                "OK".to_string(),
                "".to_string()
            ],
            stdf_types::read_kx_sn(&raw_data, &mut pos, &order, 4)
        );
        assert_eq!(pos, 15);
        assert_eq!(
            vec!["".to_string(); 0],
            stdf_types::read_kx_sn(&raw_data, &mut pos, &order, 0)
        );
    }

    #[test]
    fn test_read_kx_cf() {
        let raw_data: [u8; 9] = [84, 101, 115, 116, 32, 32, 79, 75, 0];
        let mut pos = 0;
        assert_eq!(
            vec![
                "Te".to_string(),
                "st".to_string(),
                "  ".to_string(),
                "OK".to_string()
            ],
            stdf_types::read_kx_cf(&raw_data, &mut pos, 4, 2)
        );
        assert_eq!(pos, 8);
        let mut pos = 3;
        assert_eq!(
            vec![
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string()
            ],
            stdf_types::read_kx_cf(&raw_data, &mut pos, 4, 0)
        );
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_read_kx_u1() {
        let raw_data: [u8; 9] = [84, 101, 115, 116, 32, 32, 79, 75, 0];
        let mut pos = 0;
        assert_eq!(
            vec![84, 101, 115, 116, 32, 32, 79, 75, 0],
            stdf_types::read_kx_u1(&raw_data, &mut pos, 9)
        );
        assert_eq!(pos, 9);
        let mut pos = 3;
        assert_eq!(vec![0u8; 0], stdf_types::read_kx_u1(&raw_data, &mut pos, 0));
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_read_kx_u2() {
        let raw_data: [u8; 9] = [0x12, 0x23, 0x45, 0x78, 0x9A, 0xBC, 0xDE, 0xFF, 0];
        let mut pos = 0;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            vec![0x2312, 0x7845, 0xBC9A, 0xFFDE, 0],
            stdf_types::read_kx_u2(&raw_data, &mut pos, &order, 5)
        );
        assert_eq!(pos, 8);
        let mut pos = 3;
        assert_eq!(
            vec![0u16; 0],
            stdf_types::read_kx_u2(&raw_data, &mut pos, &order, 0)
        );
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_read_kx_u4() {
        let raw_data: [u8; 9] = [0x12, 0x23, 0x45, 0x78, 0x9A, 0xBC, 0xDE, 0xFF, 0];
        let mut pos = 0;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            vec![0x78452312, 0xFFDEBC9A, 0, 0, 0],
            stdf_types::read_kx_u4(&raw_data, &mut pos, &order, 5)
        );
        assert_eq!(pos, 8);
        let mut pos = 3;
        assert_eq!(
            vec![0u32; 0],
            stdf_types::read_kx_u4(&raw_data, &mut pos, &order, 0)
        );
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_read_kx_u8() {
        let raw_data: [u8; 9] = [0x12, 0x23, 0x45, 0x78, 0x9A, 0xBC, 0xDE, 0xFF, 0];
        let mut pos = 0;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            vec![0xFFDEBC9A78452312, 0, 0, 0, 0],
            stdf_types::read_kx_u8(&raw_data, &mut pos, &order, 5)
        );
        assert_eq!(pos, 8);
        let mut pos = 3;
        assert_eq!(
            vec![0u64; 0],
            stdf_types::read_kx_u8(&raw_data, &mut pos, &order, 0)
        );
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_read_kx_uf() {
        let raw_data: [u8; 9] = [0x12, 0x23, 0x45, 0x78, 0x9A, 0xBC, 0xDE, 0xFF, 0];
        let mut pos = 0;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            stdf_types::KxUf::F1(vec![0x12, 0x23, 0x45, 0x78, 0x9A]),
            stdf_types::read_kx_uf(&raw_data, &mut pos, &order, 5, 1)
        );
        assert_eq!(pos, 5);
        let mut pos = 0;
        assert_eq!(
            stdf_types::KxUf::F2(vec![0x2312, 0x7845, 0xBC9A, 0xFFDE, 0]),
            stdf_types::read_kx_uf(&raw_data, &mut pos, &order, 5, 2)
        );
        assert_eq!(pos, 8);
        let mut pos = 0;
        assert_eq!(
            stdf_types::KxUf::F4(vec![0x78452312, 0xFFDEBC9A, 0, 0, 0]),
            stdf_types::read_kx_uf(&raw_data, &mut pos, &order, 5, 4)
        );
        assert_eq!(pos, 8);
        let mut pos = 0;
        assert_eq!(
            stdf_types::KxUf::F8(vec![0xFFDEBC9A78452312, 0, 0, 0, 0]),
            stdf_types::read_kx_uf(&raw_data, &mut pos, &order, 5, 8)
        );
        assert_eq!(pos, 8);
        let mut pos = 3;
        assert_eq!(
            stdf_types::KxUf::F1(vec![0u8; 0]),
            stdf_types::read_kx_uf(&raw_data, &mut pos, &order, 100, 0)
        );
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_read_kx_r4() {
        let raw_data: [u8; 9] = [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 0];
        let mut pos = 0;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            vec![1.5399896e-36, 4.063216e-34, 0.0, 0.0, 0.0],
            stdf_types::read_kx_r4(&raw_data, &mut pos, &order, 5)
        );
        assert_eq!(pos, 8);
        let mut pos = 3;
        assert_eq!(
            vec![0.0; 0],
            stdf_types::read_kx_r4(&raw_data, &mut pos, &order, 0)
        );
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_read_kx_n1() {
        let raw_data: [u8; 9] = [0x12, 0x23, 0x45, 0x78, 0x9A, 0xBC, 0xDE, 0xFF, 0];
        let mut pos = 0;
        assert_eq!(
            vec![0x2, 0x1, 0x3, 0x2, 0x5],
            stdf_types::read_kx_n1(&raw_data, &mut pos, 5)
        );
        assert_eq!(pos, 3);
        let mut pos = 3;
        assert_eq!(vec![0u8; 0], stdf_types::read_kx_n1(&raw_data, &mut pos, 0));
        assert_eq!(pos, 3);
    }

    // generic data
    #[test]
    fn test_read_vn() {
        let raw_data: [u8; 14] = [
            0x4, 0x0, 0xA, 0x2, 0x41, 0x42, 0x1, 0xFF, 0x0, 0x5, 0xFE, 0x1, 0xD, 0x45,
        ];
        let mut pos = 2;
        let order = ByteOrder::LittleEndian;
        assert_eq!(
            vec![
                stdf_types::V1::Cn("AB".to_string()),
                stdf_types::V1::U1(0xFF),
                stdf_types::V1::B0,
                stdf_types::V1::I2(510),
                stdf_types::V1::N1(0x5),
            ],
            stdf_types::read_vn(&raw_data, &mut pos, &order, 5)
        );
        assert_eq!(pos, 14);
        let mut pos = 3;
        assert_eq!(
            vec![stdf_types::V1::Invalid; 0],
            stdf_types::read_vn(&raw_data, &mut pos, &order, 0)
        );
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_record_type() {
        for rec_type in (0..=33).map(|x| 1 << x) {
            assert!(
                StdfRecord::new(rec_type).is_type(rec_type),
                "match type incorrect"
            );
        }
    }

    #[cfg(feature = "atdf")]
    use atdf_types::atdf_record_field::*;

    #[cfg(feature = "atdf")]
    #[test]
    fn test_atdf_fields_duplicate() {
        use std::collections::HashMap;
        let find_dup = |arr: &[(&str, bool)], name: &str| {
            arr.iter()
                // .copied()
                .fold(HashMap::new(), |mut map, val| {
                    map.entry(val.0).and_modify(|frq| *frq += 1).or_insert(1);
                    map
                })
                .iter()
                .map(|(s, &n)| assert_eq!(n, 1, "dup field {} in {}", s, name))
                .count()
        };

        find_dup(&FAR_FIELD, "FAR");
        find_dup(&ATR_FIELD, "ATR");
        find_dup(&MIR_FIELD, "MIR");
        find_dup(&MRR_FIELD, "MRR");
        find_dup(&PCR_FIELD, "PCR");
        find_dup(&HBR_FIELD, "HBR");
        find_dup(&SBR_FIELD, "SBR");
        find_dup(&PMR_FIELD, "PMR");
        find_dup(&PGR_FIELD, "PGR");
        find_dup(&PLR_FIELD, "PLR");
        find_dup(&RDR_FIELD, "RDR");
        find_dup(&SDR_FIELD, "SDR");
        find_dup(&WIR_FIELD, "WIR");
        find_dup(&WRR_FIELD, "WRR");
        find_dup(&WCR_FIELD, "WCR");
        find_dup(&PIR_FIELD, "PIR");
        find_dup(&PRR_FIELD, "PRR");
        find_dup(&TSR_FIELD, "TSR");
        find_dup(&PTR_FIELD, "PTR");
        find_dup(&MPR_FIELD, "MPR");
        find_dup(&FTR_FIELD, "FTR");
        find_dup(&BPS_FIELD, "BPS");
        find_dup(&EPS_FIELD, "EPS");
        find_dup(&GDR_FIELD, "GDR");
        find_dup(&DTR_FIELD, "DTR");
    }

    #[cfg(feature = "atdf")]
    #[test]
    fn test_atdf_field_req_count() {
        assert_eq!(3, atdf_types::count_reqired(&PTR_FIELD));
        assert_eq!(0, atdf_types::count_reqired(&GDR_FIELD));
    }
}
