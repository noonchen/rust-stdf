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

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

/// Little-endian buffer builders for the variable-length field kinds.
fn cn(buf: &mut Vec<u8>, s: &str) {
    buf.push(s.len() as u8);
    buf.extend_from_slice(s.as_bytes());
}
fn sn(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(&(s.len() as u16).to_le_bytes());
    buf.extend_from_slice(s.as_bytes());
}
fn bn(buf: &mut Vec<u8>, b: &[u8]) {
    buf.push(b.len() as u8);
    buf.extend_from_slice(b);
}
fn dn(buf: &mut Vec<u8>, bits: u16, b: &[u8]) {
    buf.extend_from_slice(&bits.to_le_bytes());
    buf.extend_from_slice(b);
}

/// Assert eager scalar/vec/enum field == view getter.
macro_rules! eq {
    ($eager:ident, $view:ident; $($f:ident),+ $(,)?) => {
        $( assert_eq!($eager.$f, $view.$f()); )+
    };
}

/// Assert eager string field == view getter (allocating `to_owned`).
macro_rules! eq_str {
    ($eager:ident, $view:ident; $($f:ident),+ $(,)?) => {
        $( assert_eq!($eager.$f, $view.$f().to_owned()); )+
    };
}

fn f32_opt_eq(a: Option<f32>, b: Option<f32>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => x.to_bits() == y.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

fn f32_vec_eq(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

// ---------------------------------------------------------------------------
// default values
// ---------------------------------------------------------------------------

#[test]
fn record_default_value_test() {
    let empty_raw_data = [0u8; 0];
    // mir
    let (typ, sub) = get_typ_sub_from_code(REC_MIR).unwrap();
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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
    let rec_header = RecordHeader { typ, sub, len: empty_raw_data.len() as u16 };
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

// ---------------------------------------------------------------------------
// record <-> record view equality
// ---------------------------------------------------------------------------

// For every record type, build a synthetic full buffer, parse it eagerly, and
// verify every `*View` getter returns exactly the eager field. Distinct values
// are used per field so an offset mistake in the view's scan is caught loudly.
#[test]
fn record_view_eq_record_test() {
    let order = ByteOrder::LittleEndian;

    // --- FAR ---
    {
        let raw = vec![
            2, // cpu_type
            4, // stdf_ver
        ];
        let mut r = FAR::new();
        r.read_from_bytes(&raw, &order);
        let v = FARView::new(&raw, &order);
        eq!(r, v; cpu_type, stdf_ver);
    }

    // --- ATR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&0x12345678u32.to_le_bytes()); // mod_tim
        cn(&mut raw, "CMD"); // cmd_line
        let mut r = ATR::new();
        r.read_from_bytes(&raw, &order);
        let v = ATRView::new(&raw, &order);
        eq!(r, v; mod_tim);
        eq_str!(r, v; cmd_line);
    }

    // --- VUR ---
    {
        let mut raw = Vec::new();
        cn(&mut raw, "VUR1"); // upd_nam
        let mut r = VUR::new();
        r.read_from_bytes(&raw, &order);
        let v = VURView::new(&raw, &order);
        eq_str!(r, v; upd_nam);
    }

    // --- MIR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&100u32.to_le_bytes()); // setup_t
        raw.extend_from_slice(&200u32.to_le_bytes()); // start_t
        raw.push(3); // stat_num
        raw.push(b'M'); // mode_cod
        raw.push(b'R'); // rtst_cod
        raw.push(b'P'); // prot_cod
        raw.extend_from_slice(&60u16.to_le_bytes()); // burn_tim
        raw.push(b'C'); // cmod_cod
        cn(&mut raw, "LOT1");  // lot_id
        cn(&mut raw, "PART1"); // part_typ
        cn(&mut raw, "NODE");  // node_nam
        cn(&mut raw, "TSTR");  // tstr_typ
        cn(&mut raw, "JOB");   // job_nam
        cn(&mut raw, "REV");   // job_rev
        cn(&mut raw, "SBLOT"); // sblot_id
        cn(&mut raw, "OPER");  // oper_nam
        cn(&mut raw, "EXEC");  // exec_typ
        cn(&mut raw, "VER");   // exec_ver
        cn(&mut raw, "TEST");  // test_cod
        cn(&mut raw, "TEMP");  // tst_temp
        cn(&mut raw, "USER");  // user_txt
        cn(&mut raw, "AUX");   // aux_file
        cn(&mut raw, "PKG");   // pkg_typ
        cn(&mut raw, "FAM");   // famly_id
        cn(&mut raw, "DATE");  // date_cod
        cn(&mut raw, "FAC");   // facil_id
        cn(&mut raw, "FLOOR"); // floor_id
        cn(&mut raw, "PROC");  // proc_id
        cn(&mut raw, "FREQ");  // oper_frq
        cn(&mut raw, "SPEC");  // spec_nam
        cn(&mut raw, "SVER");  // spec_ver
        cn(&mut raw, "FLOW");  // flow_id
        cn(&mut raw, "SETUP"); // setup_id
        cn(&mut raw, "DSGN");  // dsgn_rev
        cn(&mut raw, "ENG");   // eng_id
        cn(&mut raw, "ROM");   // rom_cod
        cn(&mut raw, "SERL");  // serl_num
        cn(&mut raw, "SUPR");  // supr_nam
        let mut r = MIR::new();
        r.read_from_bytes(&raw, &order);
        let v = MIRView::new(&raw, &order);
        eq!(r, v; setup_t, start_t, stat_num, mode_cod, rtst_cod, prot_cod, burn_tim, cmod_cod);
        eq_str!(r, v;
            lot_id, part_typ, node_nam, tstr_typ, job_nam, job_rev, sblot_id, oper_nam,
            exec_typ, exec_ver, test_cod, tst_temp, user_txt, aux_file, pkg_typ, famly_id,
            date_cod, facil_id, floor_id, proc_id, oper_frq, spec_nam, spec_ver, flow_id,
            setup_id, dsgn_rev, eng_id, rom_cod, serl_num, supr_nam
        );
    }

    // --- MRR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&999u32.to_le_bytes()); // finish_t
        raw.push(b'D'); // disp_cod
        cn(&mut raw, "USD"); // usr_desc
        cn(&mut raw, "EXD"); // exc_desc
        let mut r = MRR::new();
        r.read_from_bytes(&raw, &order);
        let v = MRRView::new(&raw, &order);
        eq!(r, v; finish_t, disp_cod);
        eq_str!(r, v; usr_desc, exc_desc);
    }

    // --- PCR ---
    {
        let mut raw = Vec::new();
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.extend_from_slice(&5u32.to_le_bytes()); // part_cnt
        raw.extend_from_slice(&1u32.to_le_bytes()); // rtst_cnt
        raw.extend_from_slice(&2u32.to_le_bytes()); // abrt_cnt
        raw.extend_from_slice(&3u32.to_le_bytes()); // good_cnt
        raw.extend_from_slice(&4u32.to_le_bytes()); // func_cnt
        let mut r = PCR::new();
        r.read_from_bytes(&raw, &order);
        let v = PCRView::new(&raw, &order);
        eq!(r, v; head_num, site_num, part_cnt, rtst_cnt, abrt_cnt, good_cnt, func_cnt);
    }

    // --- HBR ---
    {
        let mut raw = Vec::new();
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.extend_from_slice(&3u16.to_le_bytes()); // hbin_num
        raw.extend_from_slice(&4u32.to_le_bytes()); // hbin_cnt
        raw.push(b'P'); // hbin_pf
        cn(&mut raw, "HBIN"); // hbin_nam
        let mut r = HBR::new();
        r.read_from_bytes(&raw, &order);
        let v = HBRView::new(&raw, &order);
        eq!(r, v; head_num, site_num, hbin_num, hbin_cnt, hbin_pf);
        eq_str!(r, v; hbin_nam);
    }

    // --- SBR ---
    {
        let mut raw = Vec::new();
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.extend_from_slice(&3u16.to_le_bytes()); // sbin_num
        raw.extend_from_slice(&4u32.to_le_bytes()); // sbin_cnt
        raw.push(b'F'); // sbin_pf
        cn(&mut raw, "SBIN"); // sbin_nam
        let mut r = SBR::new();
        r.read_from_bytes(&raw, &order);
        let v = SBRView::new(&raw, &order);
        eq!(r, v; head_num, site_num, sbin_num, sbin_cnt, sbin_pf);
        eq_str!(r, v; sbin_nam);
    }

    // --- PMR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&1u16.to_le_bytes()); // pmr_indx
        raw.extend_from_slice(&2u16.to_le_bytes()); // chan_typ
        cn(&mut raw, "CHN"); // chan_nam
        cn(&mut raw, "PHY"); // phy_nam
        cn(&mut raw, "LOG"); // log_nam
        raw.push(3); // head_num
        raw.push(4); // site_num
        let mut r = PMR::new();
        r.read_from_bytes(&raw, &order);
        let v = PMRView::new(&raw, &order);
        eq!(r, v; pmr_indx, chan_typ, head_num, site_num);
        eq_str!(r, v; chan_nam, phy_nam, log_nam);
    }

    // --- PGR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&7u16.to_le_bytes()); // grp_indx
        cn(&mut raw, "GRP"); // grp_nam
        raw.extend_from_slice(&2u16.to_le_bytes()); // indx_cnt = 2
        raw.extend_from_slice(&10u16.to_le_bytes()); // pmr_indx[0]
        raw.extend_from_slice(&20u16.to_le_bytes()); // pmr_indx[1]
        let mut r = PGR::new();
        r.read_from_bytes(&raw, &order);
        let v = PGRView::new(&raw, &order);
        eq!(r, v; grp_indx, indx_cnt, pmr_indx);
        eq_str!(r, v; grp_nam);
    }

    // --- PLR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&2u16.to_le_bytes()); // grp_cnt = 2
        raw.extend_from_slice(&1u16.to_le_bytes()); // grp_indx[0]
        raw.extend_from_slice(&2u16.to_le_bytes()); // grp_indx[1]
        raw.extend_from_slice(&3u16.to_le_bytes()); // grp_mode[0]
        raw.extend_from_slice(&4u16.to_le_bytes()); // grp_mode[1]
        raw.push(5); // grp_radx[0]
        raw.push(6); // grp_radx[1]
        cn(&mut raw, "P1"); // pgm_char[0]
        cn(&mut raw, "P2"); // pgm_char[1]
        cn(&mut raw, "R1"); // rtn_char[0]
        cn(&mut raw, "R2"); // rtn_char[1]
        cn(&mut raw, "C1"); // pgm_chal[0]
        cn(&mut raw, "C2"); // pgm_chal[1]
        cn(&mut raw, "D1"); // rtn_chal[0]
        cn(&mut raw, "D2"); // rtn_chal[1]
        let mut r = PLR::new();
        r.read_from_bytes(&raw, &order);
        let v = PLRView::new(&raw, &order);
        eq!(r, v; grp_cnt, grp_indx, grp_mode, grp_radx, pgm_char, rtn_char, pgm_chal, rtn_chal);
    }

    // --- RDR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&2u16.to_le_bytes()); // num_bins = 2
        raw.extend_from_slice(&5u16.to_le_bytes()); // rtst_bin[0]
        raw.extend_from_slice(&6u16.to_le_bytes()); // rtst_bin[1]
        let mut r = RDR::new();
        r.read_from_bytes(&raw, &order);
        let v = RDRView::new(&raw, &order);
        eq!(r, v; num_bins, rtst_bin);
    }

    // --- SDR ---
    {
        let mut raw = vec![
            1, // head_num
            2, // site_grp
            2, // site_cnt = 2
            3, // site_num[0]
            4, // site_num[1]
        ];
        cn(&mut raw, "HANDT");  // hand_typ
        cn(&mut raw, "HANDID"); // hand_id
        cn(&mut raw, "CARDT");  // card_typ
        cn(&mut raw, "CARDID"); // card_id
        cn(&mut raw, "LOADT");  // load_typ
        cn(&mut raw, "LOADID"); // load_id
        cn(&mut raw, "DIBT");   // dib_typ
        cn(&mut raw, "DIBID");  // dib_id
        cn(&mut raw, "CABLT");  // cabl_typ
        cn(&mut raw, "CABLID"); // cabl_id
        cn(&mut raw, "CONTT");  // cont_typ
        cn(&mut raw, "CONTID"); // cont_id
        cn(&mut raw, "LASRT");  // lasr_typ
        cn(&mut raw, "LASRID"); // lasr_id
        cn(&mut raw, "EXTRT");  // extr_typ
        cn(&mut raw, "EXTRID"); // extr_id
        let mut r = SDR::new();
        r.read_from_bytes(&raw, &order);
        let v = SDRView::new(&raw, &order);
        eq!(r, v; head_num, site_grp, site_cnt, site_num);
        eq_str!(r, v;
            hand_typ, hand_id, card_typ, card_id, load_typ, load_id, dib_typ, dib_id,
            cabl_typ, cabl_id, cont_typ, cont_id, lasr_typ, lasr_id, extr_typ, extr_id
        );
    }

    // --- PSR ---
    {
        let mut raw = Vec::new();
        raw.push(0); // cont_flg
        raw.extend_from_slice(&1u16.to_le_bytes()); // psr_indx
        cn(&mut raw, "PSR"); // psr_nam
        raw.push(0); // opt_flg
        raw.extend_from_slice(&10u16.to_le_bytes()); // totp_cnt
        raw.extend_from_slice(&2u16.to_le_bytes()); // locp_cnt = 2
        raw.extend_from_slice(&100u64.to_le_bytes()); // pat_bgn[0]
        raw.extend_from_slice(&200u64.to_le_bytes()); // pat_bgn[1]
        raw.extend_from_slice(&300u64.to_le_bytes()); // pat_end[0]
        raw.extend_from_slice(&400u64.to_le_bytes()); // pat_end[1]
        cn(&mut raw, "PF1"); // pat_file[0]
        cn(&mut raw, "PF2"); // pat_file[1]
        cn(&mut raw, "PL1"); // pat_lbl[0]
        cn(&mut raw, "PL2"); // pat_lbl[1]
        cn(&mut raw, "FU1"); // file_uid[0]
        cn(&mut raw, "FU2"); // file_uid[1]
        cn(&mut raw, "AD1"); // atpg_dsc[0]
        cn(&mut raw, "AD2"); // atpg_dsc[1]
        cn(&mut raw, "SR1"); // src_id[0]
        cn(&mut raw, "SR2"); // src_id[1]
        let mut r = PSR::new();
        r.read_from_bytes(&raw, &order);
        let v = PSRView::new(&raw, &order);
        eq!(r, v; cont_flg, psr_indx, opt_flg, totp_cnt, locp_cnt, pat_bgn, pat_end, pat_file, pat_lbl, file_uid, atpg_dsc, src_id);
        eq_str!(r, v; psr_nam);
    }

    // --- NMR ---
    {
        let mut raw = Vec::new();
        raw.push(0); // cont_flg
        raw.extend_from_slice(&8u16.to_le_bytes()); // totm_cnt
        raw.extend_from_slice(&2u16.to_le_bytes()); // locm_cnt = 2
        raw.extend_from_slice(&1u16.to_le_bytes()); // pmr_indx[0]
        raw.extend_from_slice(&2u16.to_le_bytes()); // pmr_indx[1]
        cn(&mut raw, "AN1"); // atpg_nam[0]
        cn(&mut raw, "AN2"); // atpg_nam[1]
        let mut r = NMR::new();
        r.read_from_bytes(&raw, &order);
        let v = NMRView::new(&raw, &order);
        eq!(r, v; cont_flg, totm_cnt, locm_cnt, pmr_indx, atpg_nam);
    }

    // --- CNR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&3u16.to_le_bytes()); // chn_num
        raw.extend_from_slice(&0xFFFFFFFFu32.to_le_bytes()); // bit_pos
        sn(&mut raw, "CELL1"); // cell_nam
        let mut r = CNR::new();
        r.read_from_bytes(&raw, &order);
        let v = CNRView::new(&raw, &order);
        eq!(r, v; chn_num, bit_pos);
        eq_str!(r, v; cell_nam);
    }

    // --- SSR ---
    {
        let mut raw = Vec::new();
        cn(&mut raw, "SSR"); // ssr_nam
        raw.extend_from_slice(&2u16.to_le_bytes()); // chn_cnt = 2
        raw.extend_from_slice(&1u16.to_le_bytes()); // chn_list[0]
        raw.extend_from_slice(&2u16.to_le_bytes()); // chn_list[1]
        let mut r = SSR::new();
        r.read_from_bytes(&raw, &order);
        let v = SSRView::new(&raw, &order);
        eq!(r, v; chn_cnt, chn_list);
        eq_str!(r, v; ssr_nam);
    }

    // --- CDR ---
    {
        let mut raw = Vec::new();
        raw.push(0); // cont_flg
        raw.extend_from_slice(&1u16.to_le_bytes()); // cdr_indx
        cn(&mut raw, "CHN"); // chn_nam
        raw.extend_from_slice(&100u32.to_le_bytes()); // chn_len
        raw.extend_from_slice(&2u16.to_le_bytes()); // sin_pin
        raw.extend_from_slice(&3u16.to_le_bytes()); // sout_pin
        raw.push(1); // mstr_cnt = 1
        raw.extend_from_slice(&4u16.to_le_bytes()); // m_clks[0]
        raw.push(1); // slav_cnt = 1
        raw.extend_from_slice(&5u16.to_le_bytes()); // s_clks[0]
        raw.push(0); // inv_val
        raw.extend_from_slice(&2u16.to_le_bytes()); // lst_cnt = 2
        sn(&mut raw, "CELL_A"); // cell_lst[0]
        sn(&mut raw, "CELL_B"); // cell_lst[1]
        let mut r = CDR::new();
        r.read_from_bytes(&raw, &order);
        let v = CDRView::new(&raw, &order);
        eq!(r, v; cont_flg, cdr_indx, chn_len, sin_pin, sout_pin, mstr_cnt, m_clks, slav_cnt, s_clks, inv_val, lst_cnt, cell_lst);
        eq_str!(r, v; chn_nam);
    }

    // --- WIR ---
    {
        let mut raw = Vec::new();
        raw.push(1); // head_num
        raw.push(2); // site_grp
        raw.extend_from_slice(&1000u32.to_le_bytes()); // start_t
        cn(&mut raw, "WAFER"); // wafer_id
        let mut r = WIR::new();
        r.read_from_bytes(&raw, &order);
        let v = WIRView::new(&raw, &order);
        eq!(r, v; head_num, site_grp, start_t);
        eq_str!(r, v; wafer_id);
    }

    // --- WRR ---
    {
        let mut raw = Vec::new();
        raw.push(1); // head_num
        raw.push(2); // site_grp
        raw.extend_from_slice(&2000u32.to_le_bytes()); // finish_t
        raw.extend_from_slice(&5u32.to_le_bytes()); // part_cnt
        raw.extend_from_slice(&1u32.to_le_bytes()); // rtst_cnt
        raw.extend_from_slice(&2u32.to_le_bytes()); // abrt_cnt
        raw.extend_from_slice(&3u32.to_le_bytes()); // good_cnt
        raw.extend_from_slice(&4u32.to_le_bytes()); // func_cnt
        cn(&mut raw, "W1"); // wafer_id
        cn(&mut raw, "F1"); // fabwf_id
        cn(&mut raw, "FR1"); // frame_id
        cn(&mut raw, "M1"); // mask_id
        cn(&mut raw, "UD"); // usr_desc
        cn(&mut raw, "ED"); // exc_desc
        let mut r = WRR::new();
        r.read_from_bytes(&raw, &order);
        let v = WRRView::new(&raw, &order);
        eq!(r, v; head_num, site_grp, finish_t, part_cnt, rtst_cnt, abrt_cnt, good_cnt, func_cnt);
        eq_str!(r, v; wafer_id, fabwf_id, frame_id, mask_id, usr_desc, exc_desc);
    }

    // --- WCR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&300.0f32.to_le_bytes()); // wafr_siz
        raw.extend_from_slice(&10.0f32.to_le_bytes()); // die_ht
        raw.extend_from_slice(&20.0f32.to_le_bytes()); // die_wid
        raw.push(1); // wf_units
        raw.push(b'F'); // wf_flat
        raw.extend_from_slice(&(-5i16).to_le_bytes()); // center_x
        raw.extend_from_slice(&(7i16).to_le_bytes()); // center_y
        raw.push(b'X'); // pos_x
        raw.push(b'Y'); // pos_y
        let mut r = WCR::new();
        r.read_from_bytes(&raw, &order);
        let v = WCRView::new(&raw, &order);
        eq!(r, v; wafr_siz, die_ht, die_wid, wf_units, wf_flat, center_x, center_y, pos_x, pos_y);
    }

    // --- PIR ---
    {
        let raw = vec![
            1, // head_num
            2, // site_num
        ];
        let mut r = PIR::new();
        r.read_from_bytes(&raw, &order);
        let v = PIRView::new(&raw, &order);
        eq!(r, v; head_num, site_num);
    }

    // --- PRR ---
    {
        let mut raw = Vec::new();
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(0); // part_flg
        raw.extend_from_slice(&5u16.to_le_bytes()); // num_test
        raw.extend_from_slice(&6u16.to_le_bytes()); // hard_bin
        raw.extend_from_slice(&7u16.to_le_bytes()); // soft_bin
        raw.extend_from_slice(&(-3i16).to_le_bytes()); // x_coord
        raw.extend_from_slice(&(4i16).to_le_bytes()); // y_coord
        raw.extend_from_slice(&1000u32.to_le_bytes()); // test_t
        cn(&mut raw, "PID"); // part_id
        cn(&mut raw, "PTXT"); // part_txt
        bn(&mut raw, &[1, 2, 3]); // part_fix
        let mut r = PRR::new();
        r.read_from_bytes(&raw, &order);
        let v = PRRView::new(&raw, &order);
        eq!(r, v; head_num, site_num, part_flg, num_test, hard_bin, soft_bin, x_coord, y_coord, test_t);
        eq_str!(r, v; part_id, part_txt);
        assert_eq!(r.part_fix, v.part_fix().to_owned());
    }

    // --- TSR ---
    {
        let mut raw = Vec::new();
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(b'P'); // test_typ
        raw.extend_from_slice(&100u32.to_le_bytes()); // test_num
        raw.extend_from_slice(&10u32.to_le_bytes()); // exec_cnt
        raw.extend_from_slice(&3u32.to_le_bytes()); // fail_cnt
        raw.extend_from_slice(&0u32.to_le_bytes()); // alrm_cnt
        cn(&mut raw, "VDD"); // test_nam
        cn(&mut raw, "SEQ"); // seq_name
        cn(&mut raw, "L1"); // test_lbl
        raw.push(0); // opt_flag
        raw.extend_from_slice(&1.5f32.to_le_bytes()); // test_tim
        raw.extend_from_slice(&(-2.0f32).to_le_bytes()); // test_min
        raw.extend_from_slice(&5.0f32.to_le_bytes()); // test_max
        raw.extend_from_slice(&12.0f32.to_le_bytes()); // tst_sums
        raw.extend_from_slice(&30.0f32.to_le_bytes()); // tst_sqrs
        let mut r = TSR::new();
        r.read_from_bytes(&raw, &order);
        let v = TSRView::new(&raw, &order);
        eq!(r, v; head_num, site_num, test_typ, test_num, exec_cnt, fail_cnt, alrm_cnt, opt_flag, test_tim, test_min, test_max, tst_sums, tst_sqrs);
        eq_str!(r, v; test_nam, seq_name, test_lbl);
    }

    // --- PTR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&9u32.to_le_bytes()); // test_num
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(0); // test_flg
        raw.push(0); // parm_flg
        raw.extend_from_slice(&1.5f32.to_le_bytes()); // result
        cn(&mut raw, "TEST"); // test_txt
        cn(&mut raw, "ALM"); // alarm_id
        raw.push(1); // opt_flag
        raw.push((-2i8) as u8); // res_scal
        raw.push((-3i8) as u8); // llm_scal
        raw.push((-4i8) as u8); // hlm_scal
        raw.extend_from_slice(&(-1.0f32).to_le_bytes()); // lo_limit
        raw.extend_from_slice(&10.0f32.to_le_bytes()); // hi_limit
        cn(&mut raw, "V"); // units
        cn(&mut raw, "%f"); // c_resfmt
        cn(&mut raw, "%g"); // c_llmfmt
        cn(&mut raw, "%h"); // c_hlmfmt
        raw.extend_from_slice(&(-5.0f32).to_le_bytes()); // lo_spec
        raw.extend_from_slice(&50.0f32.to_le_bytes()); // hi_spec
        let mut r = PTR::new();
        r.read_from_bytes(&raw, &order);
        let v = PTRView::new(&raw, &order);
        eq!(r, v; test_num, head_num, site_num, test_flg, parm_flg, result);
        eq_str!(r, v; test_txt, alarm_id);
        assert_eq!(r.opt_flag, v.opt_flag());
        assert_eq!(r.res_scal, v.res_scal());
        assert_eq!(r.llm_scal, v.llm_scal());
        assert_eq!(r.hlm_scal, v.hlm_scal());
        assert!(f32_opt_eq(r.lo_limit, v.lo_limit()));
        assert!(f32_opt_eq(r.hi_limit, v.hi_limit()));
        assert_eq!(r.units, v.units().map(|c| c.to_owned()));
        assert_eq!(r.c_resfmt, v.c_resfmt().map(|c| c.to_owned()));
        assert_eq!(r.c_llmfmt, v.c_llmfmt().map(|c| c.to_owned()));
        assert_eq!(r.c_hlmfmt, v.c_hlmfmt().map(|c| c.to_owned()));
        assert!(f32_opt_eq(r.lo_spec, v.lo_spec()));
        assert!(f32_opt_eq(r.hi_spec, v.hi_spec()));
    }

    // --- MPR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&42u32.to_le_bytes()); // test_num
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(0); // test_flg
        raw.push(0); // parm_flg
        raw.extend_from_slice(&2u16.to_le_bytes()); // rtn_icnt = 2
        raw.extend_from_slice(&2u16.to_le_bytes()); // rslt_cnt = 2
        raw.push(0x21); // rtn_stat = [1, 2]
        raw.extend_from_slice(&1.0f32.to_le_bytes()); // rtn_rslt[0]
        raw.extend_from_slice(&2.0f32.to_le_bytes()); // rtn_rslt[1]
        cn(&mut raw, "T1"); // test_txt
        cn(&mut raw, "A1"); // alarm_id
        raw.push(1); // opt_flag
        raw.push((-1i8) as u8); // res_scal
        raw.push((-2i8) as u8); // llm_scal
        raw.push((-3i8) as u8); // hlm_scal
        raw.extend_from_slice(&0.0f32.to_le_bytes()); // lo_limit
        raw.extend_from_slice(&10.0f32.to_le_bytes()); // hi_limit
        raw.extend_from_slice(&1.0f32.to_le_bytes()); // start_in
        raw.extend_from_slice(&2.0f32.to_le_bytes()); // incr_in
        raw.extend_from_slice(&1u16.to_le_bytes()); // rtn_indx[0]
        raw.extend_from_slice(&2u16.to_le_bytes()); // rtn_indx[1]
        cn(&mut raw, "V"); // units
        cn(&mut raw, "U"); // units_in
        cn(&mut raw, "%f"); // c_resfmt
        cn(&mut raw, "%g"); // c_llmfmt
        cn(&mut raw, "%h"); // c_hlmfmt
        raw.extend_from_slice(&0.0f32.to_le_bytes()); // lo_spec
        raw.extend_from_slice(&100.0f32.to_le_bytes()); // hi_spec
        let mut r = MPR::new();
        r.read_from_bytes(&raw, &order);
        let v = MPRView::new(&raw, &order);
        eq!(r, v; test_num, head_num, site_num, test_flg, parm_flg, rtn_icnt, rslt_cnt, rtn_stat);
        assert!(f32_vec_eq(&r.rtn_rslt, &v.rtn_rslt()));
        eq_str!(r, v; test_txt, alarm_id);
        assert_eq!(r.opt_flag, v.opt_flag());
        assert_eq!(r.res_scal, v.res_scal());
        assert_eq!(r.llm_scal, v.llm_scal());
        assert_eq!(r.hlm_scal, v.hlm_scal());
        assert!(f32_opt_eq(r.lo_limit, v.lo_limit()));
        assert!(f32_opt_eq(r.hi_limit, v.hi_limit()));
        assert!(f32_opt_eq(r.start_in, v.start_in()));
        assert!(f32_opt_eq(r.incr_in, v.incr_in()));
        assert_eq!(r.rtn_indx, v.rtn_indx());
        assert_eq!(r.units, v.units().map(|c| c.to_owned()));
        assert_eq!(r.units_in, v.units_in().map(|c| c.to_owned()));
        assert_eq!(r.c_resfmt, v.c_resfmt().map(|c| c.to_owned()));
        assert_eq!(r.c_llmfmt, v.c_llmfmt().map(|c| c.to_owned()));
        assert_eq!(r.c_hlmfmt, v.c_hlmfmt().map(|c| c.to_owned()));
        assert!(f32_opt_eq(r.lo_spec, v.lo_spec()));
        assert!(f32_opt_eq(r.hi_spec, v.hi_spec()));
    }

    // --- FTR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&1u32.to_le_bytes()); // test_num
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(0); // test_flg
        raw.push(0); // opt_flag
        raw.extend_from_slice(&10u32.to_le_bytes()); // cycl_cnt
        raw.extend_from_slice(&20u32.to_le_bytes()); // rel_vadr
        raw.extend_from_slice(&30u32.to_le_bytes()); // rept_cnt
        raw.extend_from_slice(&40u32.to_le_bytes()); // num_fail
        raw.extend_from_slice(&(-50i32).to_le_bytes()); // xfail_ad
        raw.extend_from_slice(&(60i32).to_le_bytes()); // yfail_ad
        raw.extend_from_slice(&(-70i16).to_le_bytes()); // vect_off
        raw.extend_from_slice(&2u16.to_le_bytes()); // rtn_icnt = 2
        raw.extend_from_slice(&2u16.to_le_bytes()); // pgm_icnt = 2
        raw.extend_from_slice(&1u16.to_le_bytes()); // rtn_indx[0]
        raw.extend_from_slice(&2u16.to_le_bytes()); // rtn_indx[1]
        raw.push(0x21); // rtn_stat = [1, 2]
        raw.extend_from_slice(&3u16.to_le_bytes()); // pgm_indx[0]
        raw.extend_from_slice(&4u16.to_le_bytes()); // pgm_indx[1]
        raw.push(0x43); // pgm_stat = [3, 4]
        dn(&mut raw, 8, &[0b10101010]); // fail_pin
        cn(&mut raw, "VN"); // vect_nam
        cn(&mut raw, "TS"); // time_set
        cn(&mut raw, "OP"); // op_code
        cn(&mut raw, "TT"); // test_txt
        cn(&mut raw, "AL"); // alarm_id
        cn(&mut raw, "PT"); // prog_txt
        cn(&mut raw, "RT"); // rslt_txt
        raw.push(255); // patg_num
        dn(&mut raw, 8, &[0b01010101]); // spin_map
        let mut r = FTR::new();
        r.read_from_bytes(&raw, &order);
        let v = FTRView::new(&raw, &order);
        eq!(r, v; test_num, head_num, site_num, test_flg, opt_flag, cycl_cnt, rel_vadr, rept_cnt, num_fail, xfail_ad, yfail_ad, vect_off, rtn_icnt, pgm_icnt, rtn_indx, rtn_stat, pgm_indx, pgm_stat, patg_num);
        assert_eq!(r.fail_pin, v.fail_pin().to_owned());
        eq_str!(r, v; vect_nam, time_set, op_code, test_txt, alarm_id, prog_txt, rslt_txt);
        assert_eq!(r.spin_map, v.spin_map().to_owned());
    }

    // --- STR ---
    {
        let mut raw = Vec::new();
        raw.push(0); // cont_flg
        raw.extend_from_slice(&1u32.to_le_bytes()); // test_num
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.extend_from_slice(&3u16.to_le_bytes()); // psr_ref
        raw.push(0); // test_flg
        cn(&mut raw, "LOG"); // log_typ
        cn(&mut raw, "TXT"); // test_txt
        cn(&mut raw, "ALM"); // alarm_id
        cn(&mut raw, "PRG"); // prog_txt
        cn(&mut raw, "RSL"); // rslt_txt
        raw.push(1); // z_val
        raw.push(0); // fmu_flg
        dn(&mut raw, 8, &[0xFF]); // mask_map
        dn(&mut raw, 8, &[0x0F]); // fal_map
        raw.extend_from_slice(&1000u64.to_le_bytes()); // cyc_cnt_t
        raw.extend_from_slice(&10u32.to_le_bytes()); // totf_cnt
        raw.extend_from_slice(&20u32.to_le_bytes()); // totl_cnt
        raw.extend_from_slice(&2000u64.to_le_bytes()); // cyc_base
        raw.extend_from_slice(&30u32.to_le_bytes()); // bit_base
        raw.extend_from_slice(&1u16.to_le_bytes()); // cond_cnt = 1
        raw.extend_from_slice(&1u16.to_le_bytes()); // lim_cnt = 1
        raw.push(2); // cyc_size
        raw.push(1); // pmr_size
        raw.push(4); // chn_size
        raw.push(4); // pat_size
        raw.push(2); // bit_size
        raw.push(1); // u1_size
        raw.push(2); // u2_size
        raw.push(4); // u3_size
        raw.push(2); // utx_size
        raw.extend_from_slice(&5u16.to_le_bytes()); // cap_bgn
        raw.extend_from_slice(&6u16.to_le_bytes()); // lim_indx[0]
        raw.extend_from_slice(&7u32.to_le_bytes()); // lim_spec[0]
        cn(&mut raw, "COND"); // cond_lst[0]
        raw.extend_from_slice(&1u16.to_le_bytes()); // cyc_cnt = 1
        raw.extend_from_slice(&8u16.to_le_bytes()); // cyc_ofst[0] (2 bytes)
        raw.extend_from_slice(&1u16.to_le_bytes()); // pmr_cnt = 1
        raw.push(9); // pmr_indx[0] (1 byte)
        raw.extend_from_slice(&1u16.to_le_bytes()); // chn_cnt = 1
        raw.extend_from_slice(&10u32.to_le_bytes()); // chn_num[0] (4 bytes)
        raw.extend_from_slice(&1u16.to_le_bytes()); // exp_cnt = 1
        raw.push(11); // exp_data[0]
        raw.extend_from_slice(&1u16.to_le_bytes()); // cap_cnt = 1
        raw.push(12); // cap_data[0]
        raw.extend_from_slice(&1u16.to_le_bytes()); // new_cnt = 1
        raw.push(13); // new_data[0]
        raw.extend_from_slice(&1u16.to_le_bytes()); // pat_cnt = 1
        raw.extend_from_slice(&14u32.to_le_bytes()); // pat_num[0] (4 bytes)
        raw.extend_from_slice(&1u16.to_le_bytes()); // bpos_cnt = 1
        raw.extend_from_slice(&15u16.to_le_bytes()); // bit_pos[0] (2 bytes)
        raw.extend_from_slice(&1u16.to_le_bytes()); // usr1_cnt = 1
        raw.push(16); // usr1[0] (1 byte)
        raw.extend_from_slice(&1u16.to_le_bytes()); // usr2_cnt = 1
        raw.extend_from_slice(&17u16.to_le_bytes()); // usr2[0] (2 bytes)
        raw.extend_from_slice(&1u16.to_le_bytes()); // usr3_cnt = 1
        raw.extend_from_slice(&18u32.to_le_bytes()); // usr3[0] (4 bytes)
        raw.extend_from_slice(&1u16.to_le_bytes()); // txt_cnt = 1
        raw.extend_from_slice(b"TX"); // user_txt[0] (2 bytes)
        let mut r = STR::new();
        r.read_from_bytes(&raw, &order);
        let v = STRView::new(&raw, &order);
        eq!(r, v;
            cont_flg, test_num, head_num, site_num, psr_ref, test_flg, z_val, fmu_flg,
            cyc_cnt_t, totf_cnt, totl_cnt, cyc_base, bit_base, cond_cnt, lim_cnt, cyc_size,
            pmr_size, chn_size, pat_size, bit_size, u1_size, u2_size, u3_size, utx_size, cap_bgn,
            lim_indx, lim_spec, cond_lst, cyc_cnt, cyc_ofst, pmr_cnt, pmr_indx, chn_cnt, chn_num,
            exp_cnt, exp_data, cap_cnt, cap_data, new_cnt, new_data, pat_cnt, pat_num, bpos_cnt,
            bit_pos, usr1_cnt, usr1, usr2_cnt, usr2, usr3_cnt, usr3, txt_cnt, user_txt
        );
        eq_str!(r, v; log_typ, test_txt, alarm_id, prog_txt, rslt_txt);
        assert_eq!(r.mask_map, v.mask_map().to_owned());
        assert_eq!(r.fal_map, v.fal_map().to_owned());
    }

    // --- BPS ---
    {
        let mut raw = Vec::new();
        cn(&mut raw, "BPS1"); // seq_name
        let mut r = BPS::new();
        r.read_from_bytes(&raw, &order);
        let v = BPSView::new(&raw, &order);
        eq_str!(r, v; seq_name);
    }

    // --- GDR ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&3u16.to_le_bytes()); // fld_cnt = 3
        raw.push(1); // V1::U1
        raw.push(5);
        raw.push(2); // V1::U2
        raw.extend_from_slice(&0x1234u16.to_le_bytes());
        raw.push(10); // V1::Cn
        cn(&mut raw, "X");
        let mut r = GDR::new();
        r.read_from_bytes(&raw, &order);
        let v = GDRView::new(&raw, &order);
        eq!(r, v; fld_cnt, gen_data);
    }

    // --- DTR ---
    {
        let mut raw = Vec::new();
        cn(&mut raw, "DTR1"); // text_dat
        let mut r = DTR::new();
        r.read_from_bytes(&raw, &order);
        let v = DTRView::new(&raw, &order);
        eq_str!(r, v; text_dat);
    }
}

// ---------------------------------------------------------------------------
// optional-field truncation
// ---------------------------------------------------------------------------

// Records with optional trailing fields (PTR/MPR): a buffer that ends right
// after the last mandatory field must make every optional field `None`, in both
// the eager parse and the zero-copy view.
#[test]
fn record_optional_field_truncation_test() {
    let order = ByteOrder::LittleEndian;

    // --- PTR: truncated after `alarm_id` ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&1u32.to_le_bytes()); // test_num
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(0); // test_flg
        raw.push(0); // parm_flg
        raw.extend_from_slice(&1.5f32.to_le_bytes()); // result
        cn(&mut raw, "TEST"); // test_txt
        cn(&mut raw, "ALM"); // alarm_id

        let mut r = PTR::new();
        r.read_from_bytes(&raw, &order);
        let v = PTRView::new(&raw, &order);

        assert_eq!(r.opt_flag, None);
        assert_eq!(r.opt_flag, v.opt_flag());
        assert_eq!(r.res_scal, v.res_scal());
        assert_eq!(r.llm_scal, v.llm_scal());
        assert_eq!(r.hlm_scal, v.hlm_scal());
        assert!(f32_opt_eq(r.lo_limit, v.lo_limit()));
        assert!(f32_opt_eq(r.hi_limit, v.hi_limit()));
        assert_eq!(r.units, v.units().map(|c| c.to_owned()));
        assert_eq!(r.c_resfmt, v.c_resfmt().map(|c| c.to_owned()));
        assert_eq!(r.c_llmfmt, v.c_llmfmt().map(|c| c.to_owned()));
        assert_eq!(r.c_hlmfmt, v.c_hlmfmt().map(|c| c.to_owned()));
        assert!(f32_opt_eq(r.lo_spec, v.lo_spec()));
        assert!(f32_opt_eq(r.hi_spec, v.hi_spec()));
    }

    // --- MPR: truncated after `alarm_id` ---
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&42u32.to_le_bytes()); // test_num
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(0); // test_flg
        raw.push(0); // parm_flg
        raw.extend_from_slice(&0u16.to_le_bytes()); // rtn_icnt = 0
        raw.extend_from_slice(&0u16.to_le_bytes()); // rslt_cnt = 0
        cn(&mut raw, "T1"); // test_txt
        cn(&mut raw, "A1"); // alarm_id

        let mut r = MPR::new();
        r.read_from_bytes(&raw, &order);
        let v = MPRView::new(&raw, &order);

        assert_eq!(r.opt_flag, None);
        assert_eq!(r.opt_flag, v.opt_flag());
        assert_eq!(r.res_scal, v.res_scal());
        assert_eq!(r.llm_scal, v.llm_scal());
        assert_eq!(r.hlm_scal, v.hlm_scal());
        assert!(f32_opt_eq(r.lo_limit, v.lo_limit()));
        assert!(f32_opt_eq(r.hi_limit, v.hi_limit()));
        assert!(f32_opt_eq(r.start_in, v.start_in()));
        assert!(f32_opt_eq(r.incr_in, v.incr_in()));
        assert_eq!(r.rtn_indx, v.rtn_indx());
        assert_eq!(r.units, v.units().map(|c| c.to_owned()));
        assert_eq!(r.units_in, v.units_in().map(|c| c.to_owned()));
        assert_eq!(r.c_resfmt, v.c_resfmt().map(|c| c.to_owned()));
        assert_eq!(r.c_llmfmt, v.c_llmfmt().map(|c| c.to_owned()));
        assert_eq!(r.c_hlmfmt, v.c_hlmfmt().map(|c| c.to_owned()));
        assert!(f32_opt_eq(r.lo_spec, v.lo_spec()));
        assert!(f32_opt_eq(r.hi_spec, v.hi_spec()));
    }

    // --- PTR: truncated partway through `lo_limit` (Option<R4>) ------------
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&1u32.to_le_bytes()); // test_num
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(0); // test_flg
        raw.push(0); // parm_flg
        raw.extend_from_slice(&1.5f32.to_le_bytes()); // result
        cn(&mut raw, "TEST"); // test_txt
        cn(&mut raw, "ALM"); // alarm_id
        raw.push(1); // opt_flag
        raw.push(2); // res_scal
        raw.push(3); // llm_scal
        raw.push(4); // hlm_scal
        let lo_limit_bytes = (-1.0f32).to_le_bytes();
        raw.extend_from_slice(&lo_limit_bytes[..2]); // lo_limit: only 2 of 4 bytes

        let mut r = PTR::new();
        r.read_from_bytes(&raw, &order);
        let v = PTRView::new(&raw, &order);

        // A partially-present optional field does not fit: eager sets it to
        // `None` and stops, and the view must report `None` too.
        assert_eq!(r.lo_limit, None);
        assert!(f32_opt_eq(r.lo_limit, v.lo_limit()));
        assert!(f32_opt_eq(r.hi_limit, v.hi_limit()));
    }

    // --- MPR: truncated partway through `rtn_indx` (Option<KxU2>) ----------
    {
        let mut raw = Vec::new();
        raw.extend_from_slice(&42u32.to_le_bytes()); // test_num
        raw.push(1); // head_num
        raw.push(2); // site_num
        raw.push(0); // test_flg
        raw.push(0); // parm_flg
        raw.extend_from_slice(&2u16.to_le_bytes()); // rtn_icnt = 2
        raw.extend_from_slice(&2u16.to_le_bytes()); // rslt_cnt = 2
        raw.push(0x21); // rtn_stat = [1, 2]
        raw.extend_from_slice(&1.0f32.to_le_bytes()); // rtn_rslt[0]
        raw.extend_from_slice(&2.0f32.to_le_bytes()); // rtn_rslt[1]
        cn(&mut raw, "T1"); // test_txt
        cn(&mut raw, "A1"); // alarm_id
        raw.push(1); // opt_flag
        raw.push(2); // res_scal
        raw.push(3); // llm_scal
        raw.push(4); // hlm_scal
        raw.extend_from_slice(&0.0f32.to_le_bytes()); // lo_limit
        raw.extend_from_slice(&10.0f32.to_le_bytes()); // hi_limit
        raw.extend_from_slice(&1.0f32.to_le_bytes()); // start_in
        raw.extend_from_slice(&2.0f32.to_le_bytes()); // incr_in
        raw.extend_from_slice(&1u16.to_le_bytes()); // rtn_indx: only 1 of 2 elements

        let mut r = MPR::new();
        r.read_from_bytes(&raw, &order);
        let v = MPRView::new(&raw, &order);

        assert_eq!(r.rtn_indx, None);
        assert_eq!(r.rtn_indx, v.rtn_indx());
        assert!(f32_opt_eq(r.lo_limit, v.lo_limit()));
        assert!(f32_opt_eq(r.hi_limit, v.hi_limit()));
    }
}

// ---------------------------------------------------------------------------
// string decoding
// ---------------------------------------------------------------------------

// `CnRef::as_str` borrows valid UTF-8 payloads zero-copy and falls back to an
// owned byte -> char (Latin-1) `String` for invalid UTF-8.
#[test]
fn record_cn_ref_str_test() {
    use std::borrow::Cow;

    let order = ByteOrder::LittleEndian;
    let mut raw = Vec::new();
    raw.push(1); // head_num
    raw.push(2); // site_num
    raw.push(b'P'); // test_typ
    raw.extend_from_slice(&100u32.to_le_bytes()); // test_num
    raw.extend_from_slice(&10u32.to_le_bytes()); // exec_cnt
    raw.extend_from_slice(&3u32.to_le_bytes()); // fail_cnt
    raw.extend_from_slice(&0u32.to_le_bytes()); // alrm_cnt
    raw.extend_from_slice(&[2, b'A', b'B']); // test_nam = "AB" (ASCII)
    raw.extend_from_slice(&[2, 0xC3, 0xA9]); // seq_name = "é" (valid UTF-8)
    raw.extend_from_slice(&[2, 0xB0, b'C']); // test_lbl = "°C" (invalid UTF-8)
    raw.push(0); // opt_flag
    raw.extend_from_slice(&1.5f32.to_le_bytes()); // test_tim
    raw.extend_from_slice(&(-2.0f32).to_le_bytes()); // test_min
    raw.extend_from_slice(&5.0f32.to_le_bytes()); // test_max
    raw.extend_from_slice(&12.0f32.to_le_bytes()); // tst_sums
    raw.extend_from_slice(&30.0f32.to_le_bytes()); // tst_sqrs

    let view = TSRView::new(&raw, &order);

    // ASCII: borrowed, zero-copy, and equal to the owned form.
    let name = view.test_nam().as_str();
    assert!(matches!(&name, Cow::Borrowed(_)));
    assert_eq!(name, "AB");
    assert_eq!(name.into_owned(), view.test_nam().to_owned());

    // Valid non-ASCII UTF-8: also borrowed, decoded as UTF-8, and `to_owned`
    // agrees with `as_str`.
    let seq = view.seq_name().as_str();
    assert!(matches!(&seq, Cow::Borrowed(_)));
    assert_eq!(seq, "é");
    assert_eq!(seq.into_owned(), view.seq_name().to_owned());

    // Invalid UTF-8: owned Latin-1 (byte -> char) string, matching `to_owned`.
    let lbl = view.test_lbl().as_str();
    assert!(matches!(&lbl, Cow::Owned(_)));
    assert_eq!(lbl, "\u{00B0}C");
    assert_eq!(lbl.into_owned(), view.test_lbl().to_owned());
}
