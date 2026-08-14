// Parity test: the zero-copy `PtrView` must return exactly the same values as
// the eager `PTR` parse for every PTR record in the demo files.

use rust_stdf::{stdf_file::*, stdf_record_type::*, ByteOrder, MprView, PtrView, MPR, PTR};
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
            let view = PtrView::new(&raw.raw_data, raw.byte_order);

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

/// The `MprView` getters (including the variable-length `Kx*` arrays whose
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

    let view = MprView::new(&raw, order);

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
