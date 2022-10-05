//
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

pub mod stdf_error;
pub mod stdf_file;
pub mod stdf_types;
pub use stdf_file::StdfReader;
pub use stdf_types::{stdf_record_type, ByteOrder, StdfRecord};

#[cfg(test)]
mod tests {
    // use crate::StdfRecord;
    use crate::*;
    use stdf_record_type::*;

    #[test]
    fn record_default_value_test() {
        let empty_raw_data = [0u8; 0];
        // mir
        let mir_rec = StdfRecord::new(REC_MIR);
        if let StdfRecord::MIR(ref inner) = mir_rec {
            assert_eq!(inner.mode_cod, ' ', "Testing default of new rec");
            assert_eq!(inner.rtst_cod, ' ', "Testing default of new rec");
            assert_eq!(inner.prot_cod, ' ', "Testing default of new rec");
            assert_eq!(inner.burn_tim, 65535, "Testing default of new rec");
            assert_eq!(inner.cmod_cod, ' ', "Testing default of new rec");
        }
        let mir_rec = mir_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::MIR(ref inner) = mir_rec {
            assert_eq!(inner.mode_cod, ' ', "Testing default value after reading");
            assert_eq!(inner.rtst_cod, ' ', "Testing default value after reading");
            assert_eq!(inner.prot_cod, ' ', "Testing default value after reading");
            assert_eq!(inner.burn_tim, 65535, "Testing default value after reading");
            assert_eq!(inner.cmod_cod, ' ', "Testing default value after reading");
        }

        // mrr
        let mrr_rec = StdfRecord::new(REC_MRR);
        if let StdfRecord::MRR(ref inner) = mrr_rec {
            assert_eq!(inner.disp_cod, ' ', "Testing default of new rec");
        }
        let mrr_rec = mrr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::MRR(ref inner) = mrr_rec {
            assert_eq!(inner.disp_cod, ' ', "Testing default value after reading");
        }

        // pcr
        let pcr_rec = StdfRecord::new(REC_PCR);
        if let StdfRecord::PCR(ref inner) = pcr_rec {
            assert_eq!(inner.rtst_cnt, 4_294_967_295, "Testing default of new rec");
            assert_eq!(inner.abrt_cnt, 4_294_967_295, "Testing default of new rec");
            assert_eq!(inner.good_cnt, 4_294_967_295, "Testing default of new rec");
            assert_eq!(inner.func_cnt, 4_294_967_295, "Testing default of new rec");
        }
        let pcr_rec = pcr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::PCR(ref inner) = pcr_rec {
            assert_eq!(
                inner.rtst_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.abrt_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.good_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.func_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
        }

        // hbr
        let hbr_rec = StdfRecord::new(REC_HBR);
        if let StdfRecord::HBR(ref inner) = hbr_rec {
            assert_eq!(inner.hbin_pf, ' ', "Testing default of new rec");
        }
        let hbr_rec = hbr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::HBR(ref inner) = hbr_rec {
            assert_eq!(inner.hbin_pf, ' ', "Testing default value after reading");
        }

        // sbr
        let sbr_rec = StdfRecord::new(REC_SBR);
        if let StdfRecord::SBR(ref inner) = sbr_rec {
            assert_eq!(inner.sbin_pf, ' ', "Testing default of new rec");
        }
        let sbr_rec = sbr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::SBR(ref inner) = sbr_rec {
            assert_eq!(inner.sbin_pf, ' ', "Testing default value after reading");
        }

        // pmr
        let pmr_rec = StdfRecord::new(REC_PMR);
        if let StdfRecord::PMR(ref inner) = pmr_rec {
            assert_eq!(inner.chan_typ, 0, "Testing default of new rec");
            assert_eq!(inner.head_num, 1, "Testing default of new rec");
            assert_eq!(inner.site_num, 1, "Testing default of new rec");
        }
        let pmr_rec = pmr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::PMR(ref inner) = pmr_rec {
            assert_eq!(inner.chan_typ, 0, "Testing default value after reading");
            assert_eq!(inner.head_num, 1, "Testing default value after reading");
            assert_eq!(inner.site_num, 1, "Testing default value after reading");
        }

        // cdr
        let cdr_rec = StdfRecord::new(REC_CDR);
        if let StdfRecord::CDR(ref inner) = cdr_rec {
            assert_eq!(inner.inv_val, 255, "Testing default of new rec");
        }
        let cdr_rec = cdr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::CDR(ref inner) = cdr_rec {
            assert_eq!(inner.inv_val, 255, "Testing default value after reading");
        }

        // wir
        let wir_rec = StdfRecord::new(REC_WIR);
        if let StdfRecord::WIR(ref inner) = wir_rec {
            assert_eq!(inner.site_grp, 255, "Testing default of new rec");
        }
        let wir_rec = wir_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::WIR(ref inner) = wir_rec {
            assert_eq!(inner.site_grp, 255, "Testing default value after reading");
        }

        // wrr
        let wrr_rec = StdfRecord::new(REC_WRR);
        if let StdfRecord::WRR(ref inner) = wrr_rec {
            assert_eq!(inner.site_grp, 255, "Testing default of new rec");
            assert_eq!(inner.rtst_cnt, 4_294_967_295, "Testing default of new rec");
            assert_eq!(inner.abrt_cnt, 4_294_967_295, "Testing default of new rec");
            assert_eq!(inner.good_cnt, 4_294_967_295, "Testing default of new rec");
            assert_eq!(inner.func_cnt, 4_294_967_295, "Testing default of new rec");
        }
        let wrr_rec = wrr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::WRR(ref inner) = wrr_rec {
            assert_eq!(inner.site_grp, 255, "Testing default value after reading");
            assert_eq!(
                inner.rtst_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.abrt_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.good_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.func_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
        }

        // wcr
        let wcr_rec = StdfRecord::new(REC_WCR);
        if let StdfRecord::WCR(ref inner) = wcr_rec {
            assert_eq!(inner.wafr_siz, 0.0, "Testing default of new rec");
            assert_eq!(inner.die_ht, 0.0, "Testing default of new rec");
            assert_eq!(inner.die_wid, 0.0, "Testing default of new rec");
            assert_eq!(inner.wf_units, 0, "Testing default of new rec");
            assert_eq!(inner.wf_flat, ' ', "Testing default of new rec");
            assert_eq!(inner.center_x, -32768, "Testing default of new rec");
            assert_eq!(inner.center_y, -32768, "Testing default of new rec");
            assert_eq!(inner.pos_x, ' ', "Testing default of new rec");
            assert_eq!(inner.pos_y, ' ', "Testing default of new rec");
        }
        let wcr_rec = wcr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::WCR(ref inner) = wcr_rec {
            assert_eq!(inner.wafr_siz, 0.0, "Testing default value after reading");
            assert_eq!(inner.die_ht, 0.0, "Testing default value after reading");
            assert_eq!(inner.die_wid, 0.0, "Testing default value after reading");
            assert_eq!(inner.wf_units, 0, "Testing default value after reading");
            assert_eq!(inner.wf_flat, ' ', "Testing default value after reading");
            assert_eq!(
                inner.center_x, -32768,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.center_y, -32768,
                "Testing default value after reading"
            );
            assert_eq!(inner.pos_x, ' ', "Testing default value after reading");
            assert_eq!(inner.pos_y, ' ', "Testing default value after reading");
        }

        // prr
        let prr_rec = StdfRecord::new(REC_PRR);
        if let StdfRecord::PRR(ref inner) = prr_rec {
            assert_eq!(inner.soft_bin, 65535, "Testing default of new rec");
            assert_eq!(inner.x_coord, -32768, "Testing default of new rec");
            assert_eq!(inner.y_coord, -32768, "Testing default of new rec");
            assert_eq!(inner.test_t, 0, "Testing default of new rec");
        }
        let prr_rec = prr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::PRR(ref inner) = prr_rec {
            assert_eq!(inner.soft_bin, 65535, "Testing default value after reading");
            assert_eq!(inner.x_coord, -32768, "Testing default value after reading");
            assert_eq!(inner.y_coord, -32768, "Testing default value after reading");
            assert_eq!(inner.test_t, 0, "Testing default value after reading");
        }

        // tsr
        let tsr_rec = StdfRecord::new(REC_TSR);
        if let StdfRecord::TSR(ref inner) = tsr_rec {
            assert_eq!(inner.test_typ, ' ', "Testing default of new rec");
            assert_eq!(inner.exec_cnt, 4_294_967_295, "Testing default of new rec");
            assert_eq!(inner.fail_cnt, 4_294_967_295, "Testing default of new rec");
            assert_eq!(inner.alrm_cnt, 4_294_967_295, "Testing default of new rec");
        }
        let tsr_rec = tsr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::TSR(ref inner) = tsr_rec {
            assert_eq!(inner.test_typ, ' ', "Testing default value after reading");
            assert_eq!(
                inner.exec_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.fail_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
            assert_eq!(
                inner.alrm_cnt, 4_294_967_295,
                "Testing default value after reading"
            );
        }

        // ftr
        let ftr_rec = StdfRecord::new(REC_FTR);
        if let StdfRecord::FTR(ref inner) = ftr_rec {
            assert_eq!(inner.patg_num, 255, "Testing default of new rec");
        }
        let ftr_rec = ftr_rec.from_bytes(&empty_raw_data, &stdf_types::ByteOrder::LittleEndian);
        if let StdfRecord::FTR(ref inner) = ftr_rec {
            assert_eq!(inner.patg_num, 255, "Testing default value after reading");
        }
    }

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
}
