//
// stdf_record_tests.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 29th 2022
// -----
// Last Modified: Mon Aug 17 2026
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use rust_stdf::{stdf_record_type::*, *};

#[test]
fn record_default_value_test() {
    let empty_raw_data = [0u8; 0];
    // mir
    let (typ, sub) = get_typ_sub_from_code(REC_MIR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut mir_rec = StdfRecord::new(REC_MIR);
    if let StdfRecord::MIR(ref inner) = mir_rec {
        assert_eq!(inner.mode_cod, ' ', "Testing default of new rec");
        assert_eq!(inner.rtst_cod, ' ', "Testing default of new rec");
        assert_eq!(inner.prot_cod, ' ', "Testing default of new rec");
        assert_eq!(inner.burn_tim, 65535, "Testing default of new rec");
        assert_eq!(inner.cmod_cod, ' ', "Testing default of new rec");
    }
    mir_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let mir_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::MIR(ref inner) = mir_rec {
        assert_eq!(inner.mode_cod, ' ', "Testing default value after reading");
        assert_eq!(inner.rtst_cod, ' ', "Testing default value after reading");
        assert_eq!(inner.prot_cod, ' ', "Testing default value after reading");
        assert_eq!(inner.burn_tim, 65535, "Testing default value after reading");
        assert_eq!(inner.cmod_cod, ' ', "Testing default value after reading");

        if let StdfRecordView::MIR(v) = mir_view {
            assert_eq!(v.mode_cod(), inner.mode_cod, "Testing default value of MIRView and MIR");
            assert_eq!(v.rtst_cod(), inner.rtst_cod, "Testing default value of MIRView and MIR");
            assert_eq!(v.prot_cod(), inner.prot_cod, "Testing default value of MIRView and MIR");
            assert_eq!(v.burn_tim(), inner.burn_tim, "Testing default value of MIRView and MIR");
            assert_eq!(v.cmod_cod(), inner.cmod_cod, "Testing default value of MIRView and MIR");
        }
    }

    // mrr
    let (typ, sub) = get_typ_sub_from_code(REC_MRR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut mrr_rec = StdfRecord::new(REC_MRR);
    if let StdfRecord::MRR(ref inner) = mrr_rec {
        assert_eq!(inner.disp_cod, ' ', "Testing default of new rec");
    }
    mrr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let mrr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::MRR(ref inner) = mrr_rec {
        assert_eq!(inner.disp_cod, ' ', "Testing default value after reading");

        if let StdfRecordView::MRR(v) = mrr_view {
            assert_eq!(v.disp_cod(), inner.disp_cod, "Testing default value of MRRView and MRR");
        }
    }

    // pcr
    let (typ, sub) = get_typ_sub_from_code(REC_PCR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut pcr_rec = StdfRecord::new(REC_PCR);
    if let StdfRecord::PCR(ref inner) = pcr_rec {
        assert_eq!(inner.rtst_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.abrt_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.good_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.func_cnt, 4_294_967_295, "Testing default of new rec");
    }
    pcr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let pcr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
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

        if let StdfRecordView::PCR(v) = pcr_view {
            assert_eq!(v.rtst_cnt(), inner.rtst_cnt, "Testing default value of PCRView and PCR");
            assert_eq!(v.abrt_cnt(), inner.abrt_cnt, "Testing default value of PCRView and PCR");
            assert_eq!(v.good_cnt(), inner.good_cnt, "Testing default value of PCRView and PCR");
            assert_eq!(v.func_cnt(), inner.func_cnt, "Testing default value of PCRView and PCR");
        }
    }

    // hbr
    let (typ, sub) = get_typ_sub_from_code(REC_HBR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut hbr_rec = StdfRecord::new(REC_HBR);
    if let StdfRecord::HBR(ref inner) = hbr_rec {
        assert_eq!(inner.hbin_pf, ' ', "Testing default of new rec");
    }
    hbr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let hbr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::HBR(ref inner) = hbr_rec {
        assert_eq!(inner.hbin_pf, ' ', "Testing default value after reading");

        if let StdfRecordView::HBR(v) = hbr_view {
            assert_eq!(v.hbin_pf(), inner.hbin_pf, "Testing default value of HBRView and HBR");
        }
    }

    // sbr
    let (typ, sub) = get_typ_sub_from_code(REC_SBR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut sbr_rec = StdfRecord::new(REC_SBR);
    if let StdfRecord::SBR(ref inner) = sbr_rec {
        assert_eq!(inner.sbin_pf, ' ', "Testing default of new rec");
    }
    sbr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let sbr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::SBR(ref inner) = sbr_rec {
        assert_eq!(inner.sbin_pf, ' ', "Testing default value after reading");

        if let StdfRecordView::SBR(v) = sbr_view {
            assert_eq!(v.sbin_pf(), inner.sbin_pf, "Testing default value of SBRView and SBR");
        }
    }

    // pmr
    let (typ, sub) = get_typ_sub_from_code(REC_PMR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut pmr_rec = StdfRecord::new(REC_PMR);
    if let StdfRecord::PMR(ref inner) = pmr_rec {
        assert_eq!(inner.chan_typ, 0, "Testing default of new rec");
        assert_eq!(inner.head_num, 1, "Testing default of new rec");
        assert_eq!(inner.site_num, 1, "Testing default of new rec");
    }
    pmr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let pmr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::PMR(ref inner) = pmr_rec {
        assert_eq!(inner.chan_typ, 0, "Testing default value after reading");
        assert_eq!(inner.head_num, 1, "Testing default value after reading");
        assert_eq!(inner.site_num, 1, "Testing default value after reading");

        if let StdfRecordView::PMR(v) = pmr_view {
            assert_eq!(v.chan_typ(), inner.chan_typ, "Testing default value of PMRView and PMR");
            assert_eq!(v.head_num(), inner.head_num, "Testing default value of PMRView and PMR");
            assert_eq!(v.site_num(), inner.site_num, "Testing default value of PMRView and PMR");
        }
    }

    // cdr
    let (typ, sub) = get_typ_sub_from_code(REC_CDR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut cdr_rec = StdfRecord::new(REC_CDR);
    if let StdfRecord::CDR(ref inner) = cdr_rec {
        assert_eq!(inner.inv_val, 255, "Testing default of new rec");
    }
    cdr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let cdr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::CDR(ref inner) = cdr_rec {
        assert_eq!(inner.inv_val, 255, "Testing default value after reading");

        if let StdfRecordView::CDR(v) = cdr_view {
            assert_eq!(v.inv_val(), inner.inv_val, "Testing default value of CDRView and CDR");
        }
    }

    // wir
    let (typ, sub) = get_typ_sub_from_code(REC_WIR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut wir_rec = StdfRecord::new(REC_WIR);
    if let StdfRecord::WIR(ref inner) = wir_rec {
        assert_eq!(inner.site_grp, 255, "Testing default of new rec");
    }
    wir_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let wir_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::WIR(ref inner) = wir_rec {
        assert_eq!(inner.site_grp, 255, "Testing default value after reading");

        if let StdfRecordView::WIR(v) = wir_view {
            assert_eq!(v.site_grp(), inner.site_grp, "Testing default value of WIRView and WIR");
        }
    }

    // wrr
    let (typ, sub) = get_typ_sub_from_code(REC_WRR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut wrr_rec = StdfRecord::new(REC_WRR);
    if let StdfRecord::WRR(ref inner) = wrr_rec {
        assert_eq!(inner.site_grp, 255, "Testing default of new rec");
        assert_eq!(inner.rtst_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.abrt_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.good_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.func_cnt, 4_294_967_295, "Testing default of new rec");
    }
    wrr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let wrr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
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

        if let StdfRecordView::WRR(v) = wrr_view {
            assert_eq!(v.site_grp(), inner.site_grp, "Testing default value of WRRView and WRR");
            assert_eq!(v.rtst_cnt(), inner.rtst_cnt, "Testing default value of WRRView and WRR");
            assert_eq!(v.abrt_cnt(), inner.abrt_cnt, "Testing default value of WRRView and WRR");
            assert_eq!(v.good_cnt(), inner.good_cnt, "Testing default value of WRRView and WRR");
            assert_eq!(v.func_cnt(), inner.func_cnt, "Testing default value of WRRView and WRR");
        }
    }

    // wcr
    let (typ, sub) = get_typ_sub_from_code(REC_WCR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
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
    let wcr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
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

        if let StdfRecordView::WCR(v) = wcr_view {
            assert_eq!(v.wafr_siz(), inner.wafr_siz, "Testing default value of WCRView and WCR");
            assert_eq!(v.die_ht(), inner.die_ht, "Testing default value of WCRView and WCR");
            assert_eq!(v.die_wid(), inner.die_wid, "Testing default value of WCRView and WCR");
            assert_eq!(v.wf_units(), inner.wf_units, "Testing default value of WCRView and WCR");
            assert_eq!(v.wf_flat(), inner.wf_flat, "Testing default value of WCRView and WCR");
            assert_eq!(v.center_x(), inner.center_x, "Testing default value of WCRView and WCR");
            assert_eq!(v.center_y(), inner.center_y, "Testing default value of WCRView and WCR");
            assert_eq!(v.pos_x(), inner.pos_x, "Testing default value of WCRView and WCR");
            assert_eq!(v.pos_y(), inner.pos_y, "Testing default value of WCRView and WCR");
        }
    }

    // prr
    let (typ, sub) = get_typ_sub_from_code(REC_PRR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut prr_rec = StdfRecord::new(REC_PRR);
    if let StdfRecord::PRR(ref inner) = prr_rec {
        assert_eq!(inner.soft_bin, 65535, "Testing default of new rec");
        assert_eq!(inner.x_coord, -32768, "Testing default of new rec");
        assert_eq!(inner.y_coord, -32768, "Testing default of new rec");
        assert_eq!(inner.test_t, 0, "Testing default of new rec");
    }
    prr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);

    let prr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::PRR(ref inner) = prr_rec {
        assert_eq!(inner.soft_bin, 65535, "Testing default value after reading");
        assert_eq!(inner.x_coord, -32768, "Testing default value after reading");
        assert_eq!(inner.y_coord, -32768, "Testing default value after reading");
        assert_eq!(inner.test_t, 0, "Testing default value after reading");
        if let StdfRecordView::PRR(v) = prr_view {
            assert_eq!(v.soft_bin(), inner.soft_bin, "Testing default value of PRRView and PRR");
            assert_eq!(v.x_coord(), inner.x_coord, "Testing default value of PRRView and PRR");
            assert_eq!(v.y_coord(), inner.y_coord, "Testing default value of PRRView and PRR");
            assert_eq!(v.test_t(), inner.test_t, "Testing default value of PRRView and PRR");
        }
    }

    // tsr
    let (typ, sub) = get_typ_sub_from_code(REC_TSR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut tsr_rec = StdfRecord::new(REC_TSR);
    if let StdfRecord::TSR(ref inner) = tsr_rec {
        assert_eq!(inner.test_typ, ' ', "Testing default of new rec");
        assert_eq!(inner.exec_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.fail_cnt, 4_294_967_295, "Testing default of new rec");
        assert_eq!(inner.alrm_cnt, 4_294_967_295, "Testing default of new rec");
    }
    tsr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let tsr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
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

        if let StdfRecordView::TSR(v) = tsr_view {
            assert_eq!(v.test_typ(), inner.test_typ, "Testing default value of TSRView and TSR");
            assert_eq!(v.exec_cnt(), inner.exec_cnt, "Testing default value of TSRView and TSR");
            assert_eq!(v.fail_cnt(), inner.fail_cnt, "Testing default value of TSRView and TSR");
            assert_eq!(v.alrm_cnt(), inner.alrm_cnt, "Testing default value of TSRView and TSR");
        }
    }

    // ftr
    let (typ, sub) = get_typ_sub_from_code(REC_FTR).unwrap();
    let rec_header = RecordHeader { typ: typ, sub: sub, len: empty_raw_data.len() as u16 };
    let mut ftr_rec = StdfRecord::new(REC_FTR);
    if let StdfRecord::FTR(ref inner) = ftr_rec {
        assert_eq!(inner.patg_num, 255, "Testing default of new rec");
    }
    ftr_rec.read_from_bytes(&empty_raw_data, &ByteOrder::LittleEndian);
    let ftr_view = StdfRecordView::read_from_bytes(rec_header, &empty_raw_data, &ByteOrder::LittleEndian);
    if let StdfRecord::FTR(ref inner) = ftr_rec {
        assert_eq!(inner.patg_num, 255, "Testing default value after reading");
        
        if let StdfRecordView::FTR(v) = ftr_view {
            assert_eq!(v.patg_num(), inner.patg_num, "Testing default value of FTRView and FTR");
        }
    }
}
