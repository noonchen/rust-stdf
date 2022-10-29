//
// stdf_record_tests.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 29th 2022
// -----
// Last Modified: Sat Oct 29 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use rust_stdf::{stdf_record_type::*, *};

#[test]
fn record_default_value_test() {
    let empty_raw_data = [0u8; 0];
    // mir
    let mut mir_rec = StdfRecord::new(REC_MIR);
    if let StdfRecord::MIR(ref inner) = mir_rec {
        assert_eq!(inner.mode_cod, ' ', "Testing default of new rec");
        assert_eq!(inner.rtst_cod, ' ', "Testing default of new rec");
        assert_eq!(inner.prot_cod, ' ', "Testing default of new rec");
        assert_eq!(inner.burn_tim, 65535, "Testing default of new rec");
        assert_eq!(inner.cmod_cod, ' ', "Testing default of new rec");
    }
    mir_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::MIR(ref inner) = mir_rec {
        assert_eq!(inner.mode_cod, ' ', "Testing default value after reading");
        assert_eq!(inner.rtst_cod, ' ', "Testing default value after reading");
        assert_eq!(inner.prot_cod, ' ', "Testing default value after reading");
        assert_eq!(inner.burn_tim, 65535, "Testing default value after reading");
        assert_eq!(inner.cmod_cod, ' ', "Testing default value after reading");
    }

    // mrr
    let mut mrr_rec = StdfRecord::new(REC_MRR);
    if let StdfRecord::MRR(ref inner) = mrr_rec {
        assert_eq!(inner.disp_cod, ' ', "Testing default of new rec");
    }
    mrr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::MRR(ref inner) = mrr_rec {
        assert_eq!(inner.disp_cod, ' ', "Testing default value after reading");
    }

    // pcr
    let mut pcr_rec = StdfRecord::new(REC_PCR);
    if let StdfRecord::PCR(ref inner) = pcr_rec {
        assert_eq!(inner.rtst_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.abrt_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.good_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.func_cnt, 4_294_967_295, "Testing default of new rec");
    }
    pcr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
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
    let mut hbr_rec = StdfRecord::new(REC_HBR);
    if let StdfRecord::HBR(ref inner) = hbr_rec {
        assert_eq!(inner.hbin_pf, ' ', "Testing default of new rec");
    }
    hbr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::HBR(ref inner) = hbr_rec {
        assert_eq!(inner.hbin_pf, ' ', "Testing default value after reading");
    }

    // sbr
    let mut sbr_rec = StdfRecord::new(REC_SBR);
    if let StdfRecord::SBR(ref inner) = sbr_rec {
        assert_eq!(inner.sbin_pf, ' ', "Testing default of new rec");
    }
    sbr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::SBR(ref inner) = sbr_rec {
        assert_eq!(inner.sbin_pf, ' ', "Testing default value after reading");
    }

    // pmr
    let mut pmr_rec = StdfRecord::new(REC_PMR);
    if let StdfRecord::PMR(ref inner) = pmr_rec {
        assert_eq!(inner.chan_typ, 0, "Testing default of new rec");
        assert_eq!(inner.head_num, 1, "Testing default of new rec");
        assert_eq!(inner.site_num, 1, "Testing default of new rec");
    }
    pmr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::PMR(ref inner) = pmr_rec {
        assert_eq!(inner.chan_typ, 0, "Testing default value after reading");
        assert_eq!(inner.head_num, 1, "Testing default value after reading");
        assert_eq!(inner.site_num, 1, "Testing default value after reading");
    }

    // cdr
    let mut cdr_rec = StdfRecord::new(REC_CDR);
    if let StdfRecord::CDR(ref inner) = cdr_rec {
        assert_eq!(inner.inv_val, 255, "Testing default of new rec");
    }
    cdr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::CDR(ref inner) = cdr_rec {
        assert_eq!(inner.inv_val, 255, "Testing default value after reading");
    }

    // wir
    let mut wir_rec = StdfRecord::new(REC_WIR);
    if let StdfRecord::WIR(ref inner) = wir_rec {
        assert_eq!(inner.site_grp, 255, "Testing default of new rec");
    }
    wir_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::WIR(ref inner) = wir_rec {
        assert_eq!(inner.site_grp, 255, "Testing default value after reading");
    }

    // wrr
    let mut wrr_rec = StdfRecord::new(REC_WRR);
    if let StdfRecord::WRR(ref inner) = wrr_rec {
        assert_eq!(inner.site_grp, 255, "Testing default of new rec");
        assert_eq!(inner.rtst_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.abrt_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.good_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.func_cnt, 4_294_967_295, "Testing default of new rec");
    }
    wrr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
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
    let mut wcr_rec = StdfRecord::new(REC_WCR);
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
    wcr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
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
    let mut prr_rec = StdfRecord::new(REC_PRR);
    if let StdfRecord::PRR(ref inner) = prr_rec {
        assert_eq!(inner.soft_bin, 65535, "Testing default of new rec");
        assert_eq!(inner.x_coord, -32768, "Testing default of new rec");
        assert_eq!(inner.y_coord, -32768, "Testing default of new rec");
        assert_eq!(inner.test_t, 0, "Testing default of new rec");
    }
    prr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::PRR(ref inner) = prr_rec {
        assert_eq!(inner.soft_bin, 65535, "Testing default value after reading");
        assert_eq!(inner.x_coord, -32768, "Testing default value after reading");
        assert_eq!(inner.y_coord, -32768, "Testing default value after reading");
        assert_eq!(inner.test_t, 0, "Testing default value after reading");
    }

    // tsr
    let mut tsr_rec = StdfRecord::new(REC_TSR);
    if let StdfRecord::TSR(ref inner) = tsr_rec {
        assert_eq!(inner.test_typ, ' ', "Testing default of new rec");
        assert_eq!(inner.exec_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.fail_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.alrm_cnt, 4_294_967_295, "Testing default of new rec");
    }
    tsr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
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
    let mut ftr_rec = StdfRecord::new(REC_FTR);
    if let StdfRecord::FTR(ref inner) = ftr_rec {
        assert_eq!(inner.patg_num, 255, "Testing default of new rec");
    }
    ftr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::FTR(ref inner) = ftr_rec {
        assert_eq!(inner.patg_num, 255, "Testing default value after reading");
    }
}
