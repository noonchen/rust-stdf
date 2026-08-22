use rust_stdf::{
    stdf_file::{StdfReader, StdfWriter},
    stdf_record_type::*,
    *,
};
use std::io::Cursor;

fn write_owned(rec: &StdfRecord, order: ByteOrder) -> Vec<u8> {
    let mut out = Vec::new();
    let mut writer = StdfWriter::new(&mut out, order);
    writer.write_stdf_record(rec).unwrap();
    out
}

fn read_one(bytes: &[u8], order: ByteOrder) -> StdfRecord {
    StdfRecord::read_from_bytes_with_header(bytes, &order).unwrap()
}

/// The header length must equal the number of payload bytes actually written.
fn assert_frame_length_matches(bytes: &[u8], order: ByteOrder) {
    assert!(bytes.len() >= 4);
    let declared = match order {
        ByteOrder::LittleEndian => u16::from_le_bytes([bytes[0], bytes[1]]),
        ByteOrder::BigEndian => u16::from_be_bytes([bytes[0], bytes[1]]),
    };
    assert_eq!(
        declared as usize + 4,
        bytes.len(),
        "REC_LEN does not match the number of payload bytes written"
    );
}

fn far_record() -> FAR {
    let mut far = FAR::new();
    far.cpu_type = 2;
    far.stdf_ver = 4;
    far
}

#[test]
fn write_record_roundtrip_far_ptr_both_orders() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let far = far_record();
        let mut far_bytes = Vec::new();
        let mut writer = StdfWriter::new(&mut far_bytes, order);
        writer.write_record(&far).unwrap();
        assert_eq!(far_bytes.len(), 6);
        assert_eq!(read_one(&far_bytes, order), StdfRecord::FAR(far.clone()));

        let mut ptr = PTR::new();
        ptr.test_num = 7;
        ptr.head_num = 1;
        ptr.site_num = 2;
        ptr.test_flg = [0x80];
        ptr.parm_flg = [0];
        ptr.result = 1.25;
        ptr.test_txt = "continuity".to_string();
        ptr.alarm_id = "ALARM".to_string();
        // Trailing options are all `None`, which is the valid truncation form.
        let original = StdfRecord::PTR(ptr.clone());

        let mut bytes = Vec::new();
        let mut writer = StdfWriter::new(&mut bytes, order);
        writer.write_record(&ptr).unwrap();

        assert_frame_length_matches(&bytes, order);
        assert_eq!(read_one(&bytes, order), original);
    }
}

#[test]
fn all_known_record_defaults_roundtrip() {
    let codes = [
        REC_FAR, REC_ATR, REC_VUR, REC_MIR, REC_MRR, REC_PCR, REC_HBR, REC_SBR, REC_PMR, REC_PGR,
        REC_PLR, REC_RDR, REC_SDR, REC_PSR, REC_NMR, REC_CNR, REC_SSR, REC_CDR, REC_WIR, REC_WRR,
        REC_WCR, REC_PIR, REC_PRR, REC_TSR, REC_PTR, REC_MPR, REC_FTR, REC_STR, REC_BPS, REC_EPS,
        REC_GDR, REC_DTR,
    ];

    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        for code in codes {
            let mut rec = StdfRecord::new(code);
            if code == REC_STR {
                // `KxUf::default()` is `F1`, so zero-count STR records must
                // declare width 1 for their KxUf fields to be writable.
                if let StdfRecord::STR(str) = &mut rec {
                    str.cyc_size = 1;
                    str.pmr_size = 1;
                    str.chn_size = 1;
                    str.pat_size = 1;
                    str.bit_size = 1;
                    str.u1_size = 1;
                    str.u2_size = 1;
                    str.u3_size = 1;
                }
            }
            let bytes = write_owned(&rec, order);
            assert_frame_length_matches(&bytes, order);
            let parsed = read_one(&bytes, order);
            assert_eq!(parsed, rec, "roundtrip mismatch for {code:#x} ({order:?})");

            // View passthrough must be byte-exact, not decode/re-encode.
            let view = StdfRecordView::read_from_bytes_with_header(&bytes, &order).unwrap();
            let mut passthrough = Vec::new();
            let mut w = StdfWriter::new(&mut passthrough, order);
            w.write_stdf_record_view(&view).unwrap();
            assert_eq!(
                passthrough, bytes,
                "view passthrough mismatch for {code:#x}"
            );
        }
    }
}

#[test]
fn write_vn_generic_data_roundtrip() {
    let mut gdr = GDR::new();
    gdr.gen_data = vec![
        V1::B0,
        V1::U1(0xAB),
        V1::U2(0x1234),
        V1::U4(0x89AB_CDEF),
        V1::I1(-1),
        V1::I2(-1234),
        V1::I4(-123_456),
        V1::R4(1.5),
        V1::R8(-2.25),
        V1::Cn("GEN".to_string()),
        V1::Bn(vec![1, 2, 3]),
        V1::Dn(Dn {
            bit_count: 10,
            bit_data: vec![0xFF, 0x03],
        }),
        V1::N1(0x0F),
    ];
    gdr.fld_cnt = gdr.gen_data.len() as u16;

    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let mut bytes = Vec::new();
        let mut writer = StdfWriter::new(&mut bytes, order);
        writer.write_record(&gdr).unwrap();
        assert_frame_length_matches(&bytes, order);
        assert_eq!(read_one(&bytes, order), StdfRecord::GDR(gdr.clone()));
    }
}

#[test]
fn typed_write_validation_failures() {
    let mut out = Vec::new();
    let mut writer = StdfWriter::new(&mut out, ByteOrder::LittleEndian);

    // Cn length cap.
    let mut atr = ATR::new();
    atr.cmd_line = "x".repeat(256);
    assert_eq!(
        writer.write_record(&atr).unwrap_err().kind(),
        StdfErrorKind::InvalidLength
    );

    // Sn length cap.
    let mut cnr = CNR::new();
    cnr.cell_nam = "x".repeat(u16::MAX as usize + 1);
    assert_eq!(
        writer.write_record(&cnr).unwrap_err().kind(),
        StdfErrorKind::InvalidLength
    );

    // Bn length cap.
    let mut prr = PRR::new();
    prr.part_fix = vec![0u8; u8::MAX as usize + 1];
    assert_eq!(
        writer.write_record(&prr).unwrap_err().kind(),
        StdfErrorKind::InvalidLength
    );

    // Kx count alignment.
    let mut mpr = MPR::new();
    mpr.rtn_icnt = 1;
    mpr.rtn_stat = Vec::new();
    assert_eq!(
        writer.write_record(&mpr).unwrap_err().kind(),
        StdfErrorKind::CountMismatch
    );

    // KxN1 elements must fit in one nibble.
    let mut mpr = MPR::new();
    mpr.rtn_icnt = 1;
    mpr.rtn_stat = vec![0x10];
    assert_eq!(
        writer.write_record(&mpr).unwrap_err().kind(),
        StdfErrorKind::InvalidValue
    );

    // KxSn element length cap.
    let mut cdr = CDR::new();
    cdr.lst_cnt = 1;
    cdr.cell_lst = vec!["x".repeat(u16::MAX as usize + 1)];
    assert_eq!(
        writer.write_record(&cdr).unwrap_err().kind(),
        StdfErrorKind::InvalidLength
    );

    // Dn bit_data length must be ceil(bit_count / 8).
    let mut ftr = FTR::new();
    ftr.fail_pin = Dn {
        bit_count: 10,
        bit_data: vec![0xFF],
    };
    assert_eq!(
        writer.write_record(&ftr).unwrap_err().kind(),
        StdfErrorKind::InvalidLength
    );

    // No Some after None in the trailing optional run.
    let mut ptr = PTR::new();
    ptr.lo_limit = None;
    ptr.hi_limit = Some(1.0);
    assert_eq!(
        writer.write_record(&ptr).unwrap_err().kind(),
        StdfErrorKind::InvalidOptionalOrder
    );

    // C1 must fit in one byte.
    let mut tsr = TSR::new();
    tsr.test_typ = char::from_u32(0x100).unwrap();
    assert_eq!(
        writer.write_record(&tsr).unwrap_err().kind(),
        StdfErrorKind::InvalidValue
    );

    // KxCf elements must be exactly the declared fixed width.
    let mut str = STR::new();
    str.cyc_size = 1;
    str.pmr_size = 1;
    str.chn_size = 1;
    str.pat_size = 1;
    str.bit_size = 1;
    str.u1_size = 1;
    str.u2_size = 1;
    str.u3_size = 1;
    str.utx_size = 3;
    str.txt_cnt = 1;
    str.user_txt = vec!["AB".to_string()];
    assert_eq!(
        writer.write_record(&str).unwrap_err().kind(),
        StdfErrorKind::WidthMismatch
    );

    // V1::Invalid is not serializable.
    let mut gdr = GDR::new();
    gdr.fld_cnt = 1;
    gdr.gen_data = vec![V1::Invalid];
    assert_eq!(
        writer.write_record(&gdr).unwrap_err().kind(),
        StdfErrorKind::InvalidValue
    );

    // Writing failed validations leaves no partial output.
    assert!(out.is_empty());
}

#[test]
fn raw_passthrough_is_byte_exact_and_order_guarded() {
    let order = ByteOrder::BigEndian;
    let raw_data = [0xAA, 0xBB, 0xCC];
    let raw = RawDataElement {
        offset: 0,
        header: RecordHeader {
            len: raw_data.len() as u16,
            typ: 180,
            sub: 11,
        },
        raw_data: raw_data.to_vec(),
        byte_order: order,
    };

    let mut out = Vec::new();
    {
        let mut writer = StdfWriter::new(&mut out, order);
        writer.write_raw(&raw).unwrap();
    }
    assert_eq!(out, [0, 3, 180, 11, 0xAA, 0xBB, 0xCC]);

    let view = RawDataElementView {
        offset: 0,
        header: raw.header,
        raw_data: &raw_data,
        byte_order: order,
    };
    out.clear();
    {
        let mut writer = StdfWriter::new(&mut out, order);
        writer.write_raw_view(&view).unwrap();
    }
    assert_eq!(out, [0, 3, 180, 11, 0xAA, 0xBB, 0xCC]);

    let mut le_writer = StdfWriter::new(Vec::new(), ByteOrder::LittleEndian);
    let err = le_writer.write_raw(&raw).unwrap_err();
    assert_eq!(err.kind(), StdfErrorKind::ByteOrderMismatch);
}

#[test]
fn record_view_passthrough_is_byte_exact() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let mut source = Vec::new();
        {
            let mut w = StdfWriter::new(&mut source, order);
            w.write_record(&far_record()).unwrap();
        }
        let view = StdfRecordView::read_from_bytes_with_header(&source, &order).unwrap();

        let mut out = Vec::new();
        let mut w = StdfWriter::new(&mut out, order);
        w.write_stdf_record_view(&view).unwrap();
        assert_eq!(out, source);

        // Known record types are re-encoded into the writer's byte order when
        // it differs from the view's, so no byte order match is required.
        if order == ByteOrder::BigEndian {
            let mut le_source = Vec::new();
            {
                let mut le_src_w = StdfWriter::new(&mut le_source, ByteOrder::LittleEndian);
                le_src_w.write_record(&far_record()).unwrap();
            }
            let mut le_out = Vec::new();
            let mut le_w = StdfWriter::new(&mut le_out, ByteOrder::LittleEndian);
            le_w.write_stdf_record_view(&view).unwrap();
            assert_eq!(le_out, le_source);
        }

        // EPS is the zero-length enum special case.
        let eps_source = [0u8, 0, 20, 20];
        let eps_view = StdfRecordView::read_from_bytes_with_header(&eps_source, &order).unwrap();
        let mut eps_out = Vec::new();
        let mut eps_w = StdfWriter::new(&mut eps_out, order);
        eps_w.write_stdf_record_view(&eps_view).unwrap();
        assert_eq!(eps_out, eps_source);
    }
}

#[test]
fn reserved_unknown_write_roundtrip() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let reserved = ReservedRec {
            typ: 180,
            sub: 10,
            byte_order: order,
            raw_data: vec![0x10, 0x20],
        };
        let rec = StdfRecord::ReservedRec(reserved.clone());
        let bytes = write_owned(&rec, order);
        let mut expected = Vec::new();
        match order {
            ByteOrder::LittleEndian => expected.extend_from_slice(&2u16.to_le_bytes()),
            ByteOrder::BigEndian => expected.extend_from_slice(&2u16.to_be_bytes()),
        }
        expected.extend_from_slice(&[180, 10, 0x10, 0x20]);
        assert_eq!(bytes, expected);
        assert_eq!(read_one(&bytes, order), rec);

        // Reserved/unknown view passthrough is also self-framing and byte-exact.
        let view = StdfRecordView::read_from_bytes_with_header(&bytes, &order).unwrap();
        let mut passthrough = Vec::new();
        let mut w = StdfWriter::new(&mut passthrough, order);
        w.write_stdf_record_view(&view).unwrap();
        assert_eq!(passthrough, bytes);

        // Opaque reserved/unknown payloads are already-encoded bytes, so a
        // writer configured for the opposite byte order must reject them with
        // a byte-order mismatch rather than silently corrupting the file.
        let opposite = match order {
            ByteOrder::LittleEndian => ByteOrder::BigEndian,
            ByteOrder::BigEndian => ByteOrder::LittleEndian,
        };
        let mut cross = Vec::new();
        let mut cross_writer = StdfWriter::new(&mut cross, opposite);
        let err = cross_writer.write_stdf_record(&rec).unwrap_err();
        assert_eq!(err.kind(), StdfErrorKind::ByteOrderMismatch);
        assert!(cross.is_empty());

        let mut cross_view = Vec::new();
        let mut cross_writer = StdfWriter::new(&mut cross_view, opposite);
        let err = cross_writer.write_stdf_record_view(&view).unwrap_err();
        assert_eq!(err.kind(), StdfErrorKind::ByteOrderMismatch);
        assert!(cross_view.is_empty());

        let unknown = ReservedRec {
            typ: 99,
            sub: 7,
            byte_order: order,
            raw_data: vec![9, 8, 7],
        };
        let rec = StdfRecord::UnknownRec(unknown.clone());
        let bytes = write_owned(&rec, order);
        assert_eq!(read_one(&bytes, order), StdfRecord::UnknownRec(unknown));

        let view = StdfRecordView::read_from_bytes_with_header(&bytes, &order).unwrap();
        let mut passthrough = Vec::new();
        let mut w = StdfWriter::new(&mut passthrough, order);
        w.write_stdf_record_view(&view).unwrap();
        assert_eq!(passthrough, bytes);
    }
}

#[test]
fn str_and_mpr_complex_arrays_roundtrip() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let mut str = STR::new();
        str.mask_map = Dn {
            bit_count: 10,
            bit_data: vec![0xFF, 0x03],
        };
        str.fal_map = Dn {
            bit_count: 4,
            bit_data: vec![0x0F],
        };
        str.cond_cnt = 1;
        str.cond_lst = vec!["COND=1".to_string()];
        str.lim_cnt = 1;
        str.lim_indx = vec![7];
        str.lim_spec = vec![8];
        str.cyc_size = 2;
        str.cyc_cnt = 2;
        str.cyc_ofst = KxUf::F2(vec![10, 20]);
        str.pmr_size = 1;
        str.pmr_cnt = 1;
        str.pmr_indx = KxUf::F1(vec![3]);
        str.chn_size = 4;
        str.chn_cnt = 1;
        str.chn_num = KxUf::F4(vec![4]);
        str.exp_cnt = 2;
        str.exp_data = vec![5, 6];
        str.cap_cnt = 2;
        str.cap_data = vec![7, 8];
        str.new_cnt = 2;
        str.new_data = vec![9, 10];
        str.pat_size = 2;
        str.pat_cnt = 2;
        str.pat_num = KxUf::F2(vec![11, 12]);
        str.bit_size = 4;
        str.bpos_cnt = 2;
        str.bit_pos = KxUf::F4(vec![13, 14]);
        str.u1_size = 2;
        str.usr1_cnt = 1;
        str.usr1 = KxUf::F2(vec![15]);
        str.u2_size = 4;
        str.usr2_cnt = 1;
        str.usr2 = KxUf::F4(vec![16]);
        str.u3_size = 8;
        str.usr3_cnt = 1;
        str.usr3 = KxUf::F8(vec![17]);
        str.utx_size = 3;
        str.txt_cnt = 2;
        str.user_txt = vec!["AB ".to_string(), "CD ".to_string()];
        let original = StdfRecord::STR(str.clone());

        let mut bytes = Vec::new();
        let mut writer = StdfWriter::new(&mut bytes, order);
        writer.write_record(&str).unwrap();
        assert_frame_length_matches(&bytes, order);
        assert_eq!(read_one(&bytes, order), original);

        let mut mpr = MPR::new();
        mpr.rtn_icnt = 2;
        mpr.rslt_cnt = 2;
        mpr.rtn_stat = vec![1, 2];
        mpr.rtn_rslt = vec![1.5, -2.25];
        mpr.test_txt = "MPR".to_string();
        mpr.alarm_id = "AL".to_string();
        mpr.opt_flag = Some([0x80]);
        mpr.res_scal = Some(1);
        mpr.llm_scal = Some(2);
        mpr.hlm_scal = Some(3);
        mpr.lo_limit = Some(-1.0);
        mpr.hi_limit = Some(2.0);
        mpr.start_in = Some(0.5);
        mpr.incr_in = Some(0.25);
        mpr.rtn_indx = Some(vec![3, 4]);
        let original = StdfRecord::MPR(mpr.clone());

        bytes.clear();
        let mut writer = StdfWriter::new(&mut bytes, order);
        writer.write_record(&mpr).unwrap();
        assert_frame_length_matches(&bytes, order);
        assert_eq!(read_one(&bytes, order), original);

        let mut cdr = CDR::new();
        cdr.lst_cnt = 2;
        cdr.cell_lst = vec!["SCAN1".to_string(), "SCAN2".to_string()];
        let original = StdfRecord::CDR(cdr.clone());

        bytes.clear();
        let mut writer = StdfWriter::new(&mut bytes, order);
        writer.write_record(&cdr).unwrap();
        assert_frame_length_matches(&bytes, order);
        assert_eq!(read_one(&bytes, order), original);
    }
}

#[test]
fn width_validation_and_record_too_large() {
    let mut str = STR::new();
    str.cyc_size = 2;
    str.cyc_cnt = 0;
    str.cyc_ofst = KxUf::F1(Vec::new());
    let mut writer = StdfWriter::new(Vec::new(), ByteOrder::LittleEndian);
    assert_eq!(
        writer.write_record(&str).unwrap_err().kind(),
        StdfErrorKind::WidthMismatch
    );

    let raw = RawDataElement {
        offset: 0,
        header: RecordHeader {
            len: 0,
            typ: 180,
            sub: 10,
        },
        raw_data: vec![0u8; u16::MAX as usize + 1],
        byte_order: ByteOrder::LittleEndian,
    };
    let mut out = Vec::new();
    let mut writer = StdfWriter::new(&mut out, ByteOrder::LittleEndian);
    assert_eq!(
        writer.write_raw(&raw).unwrap_err().kind(),
        StdfErrorKind::RecordTooLarge
    );
    assert!(out.is_empty());
}

#[test]
fn raw_view_iter_filter_copy_is_byte_exact() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let mut far_bytes = Vec::new();
        let mut mir_bytes = Vec::new();
        let mut ptr_bytes = Vec::new();
        {
            let far = far_record();
            let mut writer = StdfWriter::new(&mut far_bytes, order);
            writer.write_record(&far).unwrap();

            let mut mir = MIR::new();
            mir.lot_id = "LOT123".to_string();
            let mut writer = StdfWriter::new(&mut mir_bytes, order);
            writer.write_record(&mir).unwrap();

            let mut ptr = PTR::new();
            ptr.test_num = 42;
            let mut writer = StdfWriter::new(&mut ptr_bytes, order);
            writer.write_record(&ptr).unwrap();
        }

        let mut source = Vec::new();
        source.extend_from_slice(&far_bytes);
        source.extend_from_slice(&mir_bytes);
        source.extend_from_slice(&ptr_bytes);

        // Plan §12 usage: read borrowed raw views and write back only kept records.
        let mut reader =
            StdfReader::from(Cursor::new(source), &CompressType::Uncompressed).unwrap();
        let mut out = Vec::new();
        let mut writer = StdfWriter::new(&mut out, order);
        let mut iter = reader.get_rawdata_view_iter();
        while let Some(item) = iter.next() {
            let raw_view = item.unwrap();
            if raw_view.header.typ == 15 && raw_view.header.sub == 10 {
                continue; // drop PTR
            }
            writer.write_raw_view(&raw_view).unwrap();
        }

        let mut expected = Vec::new();
        expected.extend_from_slice(&far_bytes);
        expected.extend_from_slice(&mir_bytes);
        assert_eq!(out, expected);
    }
}

#[test]
fn writer_reader_stream_roundtrip() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let mut buf = Vec::new();
        {
            let mut writer = StdfWriter::new(&mut buf, order);
            let mut far = far_record();
            far.stdf_ver = 4;
            writer.write_record(&far).unwrap();

            let mut mir = MIR::new();
            mir.setup_t = 0x0102_0304;
            mir.lot_id = "LOT123".to_string();
            writer.write_record(&mir).unwrap();
        }

        let mut reader = StdfReader::from(Cursor::new(buf), &CompressType::Uncompressed).unwrap();
        let records = reader
            .get_record_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(records.len(), 2);
        assert!(matches!(records[0], StdfRecord::FAR(_)));
        assert!(matches!(records[1], StdfRecord::MIR(_)));
    }
}
