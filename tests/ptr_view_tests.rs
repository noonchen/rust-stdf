// Parity test: the zero-copy `PTRView` must return exactly the same values as
// the eager `PTR` parse for every PTR record in the demo files.

use rust_stdf::{stdf_file::*, stdf_record_type::*, ByteOrder, MPRView, PTRView, TSRView, MPR, PTR, TSR};
use std::path::PathBuf;

fn demo_files() -> Vec<PathBuf> {
    let mut folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    folder.push("demo_stdf");
    std::fs::read_dir(folder)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().map(|e| e == "stdf").unwrap_or(false))
        .collect()
}

fn f32_opt_eq(a: Option<f32>, b: Option<f32>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => x.to_bits() == y.to_bits(),
        (None, None) => true,
        _ => false,
    }
}

#[test]
fn ptr_view_matches_eager_parse() {
    let files = demo_files();
    assert_ne!(files.len(), 0, "no demo .stdf files found");

    let mut ptr_seen = 0usize;

    for file in &files {
        let mut reader = StdfReader::new(file)
            .unwrap_or_else(|e| panic!("open {}: {e}", file.display()));

        for raw in reader.get_rawdata_iter() {
            let raw = raw.unwrap();
            if raw.header.get_type() != REC_PTR {
                continue;
            }
            ptr_seen += 1;

            // eager parse
            let mut eager = PTR::new();
            eager.read_from_bytes(&raw.raw_data, &raw.byte_order);

            // zero-copy view over the same bytes
            let view = PTRView::new(&raw.raw_data, raw.byte_order);

            assert_eq!(eager.test_num, view.test_num());
            assert_eq!(eager.head_num, view.head_num());
            assert_eq!(eager.site_num, view.site_num());
            assert_eq!(eager.test_flg, view.test_flg());
            assert_eq!(eager.parm_flg, view.parm_flg());
            assert_eq!(eager.result.to_bits(), view.result().to_bits());
            assert_eq!(eager.test_txt, view.test_txt().to_cn());
            assert_eq!(eager.alarm_id, view.alarm_id().to_cn());
            assert_eq!(eager.opt_flag, view.opt_flag());
            assert_eq!(eager.res_scal, view.res_scal());
            assert_eq!(eager.llm_scal, view.llm_scal());
            assert_eq!(eager.hlm_scal, view.hlm_scal());
            assert!(f32_opt_eq(eager.lo_limit, view.lo_limit()));
            assert!(f32_opt_eq(eager.hi_limit, view.hi_limit()));
            assert_eq!(eager.units, view.units().map(|c| c.to_cn()));
            assert_eq!(eager.c_resfmt, view.c_resfmt().map(|c| c.to_cn()));
            assert_eq!(eager.c_llmfmt, view.c_llmfmt().map(|c| c.to_cn()));
            assert_eq!(eager.c_hlmfmt, view.c_hlmfmt().map(|c| c.to_cn()));
            assert!(f32_opt_eq(eager.lo_spec, view.lo_spec()));
            assert!(f32_opt_eq(eager.hi_spec, view.hi_spec()));
        }
    }

    assert_ne!(ptr_seen, 0, "no PTR records encountered in demo files");
}

fn f32_vec_eq(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

/// The `MPRView` getters (including the variable-length `Kx*` arrays whose
/// length comes from earlier count fields) must return exactly what the eager
/// `MPR` parse produces over the same bytes. A synthetic little-endian record
/// is used so the array paths are always exercised.
#[test]
fn mpr_view_matches_eager_parse() {
    let mut raw: Vec<u8> = Vec::new();
    raw.extend_from_slice(&42u32.to_le_bytes()); // test_num
    raw.push(1); // head_num
    raw.push(2); // site_num
    raw.push(0); // test_flg
    raw.push(0); // parm_flg
    raw.extend_from_slice(&2u16.to_le_bytes()); // rtn_icnt = 2
    raw.extend_from_slice(&2u16.to_le_bytes()); // rslt_cnt = 2
    raw.push(0x21); // rtn_stat: 2 nibble-packed states (1 byte)
    raw.extend_from_slice(&1.0f32.to_le_bytes()); // rtn_rslt[0]
    raw.extend_from_slice(&2.0f32.to_le_bytes()); // rtn_rslt[1]
    raw.extend_from_slice(&[2, b'T', b'1']); // test_txt = "T1"
    raw.push(0); // alarm_id = ""
    raw.push(0); // opt_flag
    raw.push(0); // res_scal
    raw.push(0); // llm_scal
    raw.push(0); // hlm_scal
    raw.extend_from_slice(&0.0f32.to_le_bytes()); // lo_limit
    raw.extend_from_slice(&10.0f32.to_le_bytes()); // hi_limit
    raw.extend_from_slice(&0.0f32.to_le_bytes()); // start_in
    raw.extend_from_slice(&0.0f32.to_le_bytes()); // incr_in
    raw.extend_from_slice(&1u16.to_le_bytes()); // rtn_indx[0]
    raw.extend_from_slice(&2u16.to_le_bytes()); // rtn_indx[1]
    raw.extend_from_slice(&[1, b'V']); // units = "V"
    raw.push(0); // units_in = ""
    raw.push(0); // c_resfmt = ""
    raw.push(0); // c_llmfmt = ""
    raw.push(0); // c_hlmfmt = ""
    raw.extend_from_slice(&0.0f32.to_le_bytes()); // lo_spec
    raw.extend_from_slice(&100.0f32.to_le_bytes()); // hi_spec

    let order = ByteOrder::LittleEndian;

    let mut eager = MPR::new();
    eager.read_from_bytes(&raw, &order);

    let view = MPRView::new(&raw, order);

    assert_eq!(eager.test_num, view.test_num());
    assert_eq!(eager.head_num, view.head_num());
    assert_eq!(eager.site_num, view.site_num());
    assert_eq!(eager.test_flg, view.test_flg());
    assert_eq!(eager.parm_flg, view.parm_flg());
    assert_eq!(eager.rtn_icnt, view.rtn_icnt());
    assert_eq!(eager.rslt_cnt, view.rslt_cnt());
    assert_eq!(eager.rtn_stat, view.rtn_stat());
    assert!(f32_vec_eq(&eager.rtn_rslt, &view.rtn_rslt()));
    assert_eq!(eager.test_txt, view.test_txt().to_cn());
    assert_eq!(eager.alarm_id, view.alarm_id().to_cn());
    assert_eq!(eager.opt_flag, view.opt_flag());
    assert_eq!(eager.res_scal, view.res_scal());
    assert_eq!(eager.llm_scal, view.llm_scal());
    assert_eq!(eager.hlm_scal, view.hlm_scal());
    assert!(f32_opt_eq(eager.lo_limit, view.lo_limit()));
    assert!(f32_opt_eq(eager.hi_limit, view.hi_limit()));
    assert!(f32_opt_eq(eager.start_in, view.start_in()));
    assert!(f32_opt_eq(eager.incr_in, view.incr_in()));
    assert_eq!(eager.rtn_indx, view.rtn_indx());
    assert_eq!(eager.units, view.units().map(|c| c.to_cn()));
    assert_eq!(eager.units_in, view.units_in().map(|c| c.to_cn()));
    assert_eq!(eager.c_resfmt, view.c_resfmt().map(|c| c.to_cn()));
    assert_eq!(eager.c_llmfmt, view.c_llmfmt().map(|c| c.to_cn()));
    assert_eq!(eager.c_hlmfmt, view.c_hlmfmt().map(|c| c.to_cn()));
    assert!(f32_opt_eq(eager.lo_spec, view.lo_spec()));
    assert!(f32_opt_eq(eager.hi_spec, view.hi_spec()));

    // sanity: the arrays were actually populated
    assert_eq!(eager.rtn_stat.len(), 2);
    assert_eq!(eager.rtn_rslt.len(), 2);
    assert_eq!(eager.rtn_indx.as_deref().map(<[u16]>::len), Some(2));
}

// A TSR record truncated right after `site_num`: every field with a
// `smart_default` sentinel (`test_typ`, `exec_cnt`, `fail_cnt`, `alrm_cnt`) is
// absent from the buffer, so both the eager parse and the zero-copy view must
// fall back to that sentinel instead of returning 0.
#[test]
fn tsr_view_returns_default_when_truncated() {
    let order = ByteOrder::LittleEndian;
    // Only head_num + site_num present; everything after is missing.
    let raw = vec![1u8, 2u8];

    let mut eager = TSR::new();
    eager.read_from_bytes(&raw, &order);

    let view = TSRView::new(&raw, order);

    // present fields
    assert_eq!(view.head_num(), 1);
    assert_eq!(view.site_num(), 2);
    assert_eq!(eager.head_num, view.head_num());
    assert_eq!(eager.site_num, view.site_num());

    // absent fields fall back to the smart_default sentinels
    assert_eq!(view.test_typ(), ' ');
    assert_eq!(view.exec_cnt(), 4_294_967_295);
    assert_eq!(view.fail_cnt(), 4_294_967_295);
    assert_eq!(view.alrm_cnt(), 4_294_967_295);

    // eager parse honors the same sentinels ...
    assert_eq!(eager.test_typ, ' ');
    assert_eq!(eager.exec_cnt, 4_294_967_295);
    assert_eq!(eager.fail_cnt, 4_294_967_295);
    assert_eq!(eager.alrm_cnt, 4_294_967_295);

    // ... and the two paths agree
    assert_eq!(eager.test_typ, view.test_typ());
    assert_eq!(eager.exec_cnt, view.exec_cnt());
    assert_eq!(eager.fail_cnt, view.fail_cnt());
    assert_eq!(eager.alrm_cnt, view.alrm_cnt());

    // a field without a sentinel default still reads as 0 when absent
    assert_eq!(view.test_num(), 0);
    assert_eq!(eager.test_num, 0);
}

// Full TSR buffer: the zero-copy view must match the eager parse field-by-field.
#[test]
fn tsr_view_matches_eager_parse() {
    let order = ByteOrder::LittleEndian;
    let mut raw = Vec::new();
    raw.push(1); // head_num
    raw.push(2); // site_num
    raw.push(b'P'); // test_typ = 'P'
    raw.extend_from_slice(&100u32.to_le_bytes()); // test_num
    raw.extend_from_slice(&10u32.to_le_bytes()); // exec_cnt
    raw.extend_from_slice(&3u32.to_le_bytes()); // fail_cnt
    raw.extend_from_slice(&0u32.to_le_bytes()); // alrm_cnt
    raw.extend_from_slice(&[3, b'V', b'D', b'D']); // test_nam = "VDD"
    raw.push(0); // seq_name = ""
    raw.extend_from_slice(&[2, b'L', b'1']); // test_lbl = "L1"
    raw.push(0); // opt_flag
    raw.extend_from_slice(&1.5f32.to_le_bytes()); // test_tim
    raw.extend_from_slice(&(-2.0f32).to_le_bytes()); // test_min
    raw.extend_from_slice(&5.0f32.to_le_bytes()); // test_max
    raw.extend_from_slice(&12.0f32.to_le_bytes()); // tst_sums
    raw.extend_from_slice(&30.0f32.to_le_bytes()); // tst_sqrs

    let mut eager = TSR::new();
    eager.read_from_bytes(&raw, &order);

    let view = TSRView::new(&raw, order);

    assert_eq!(eager.head_num, view.head_num());
    assert_eq!(eager.site_num, view.site_num());
    assert_eq!(eager.test_typ, view.test_typ());
    assert_eq!(eager.test_num, view.test_num());
    assert_eq!(eager.exec_cnt, view.exec_cnt());
    assert_eq!(eager.fail_cnt, view.fail_cnt());
    assert_eq!(eager.alrm_cnt, view.alrm_cnt());
    assert_eq!(eager.test_nam, view.test_nam().to_cn());
    assert_eq!(eager.seq_name, view.seq_name().to_cn());
    assert_eq!(eager.test_lbl, view.test_lbl().to_cn());
    assert_eq!(eager.opt_flag, view.opt_flag());
    assert!(f32_opt_eq(Some(eager.test_tim), Some(view.test_tim())));
    assert!(f32_opt_eq(Some(eager.test_min), Some(view.test_min())));
    assert!(f32_opt_eq(Some(eager.test_max), Some(view.test_max())));
    assert!(f32_opt_eq(Some(eager.tst_sums), Some(view.tst_sums())));
    assert!(f32_opt_eq(Some(eager.tst_sqrs), Some(view.tst_sqrs())));

    // sanity: values actually parsed
    assert_eq!(view.test_typ(), 'P');
    assert_eq!(view.exec_cnt(), 10);
}
