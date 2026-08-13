// Parity test: the zero-copy `PtrView` must return exactly the same values as
// the eager `PTR` parse for every PTR record in the demo files.

use rust_stdf::{stdf_file::*, stdf_record_type::*, PtrView, PTR};
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
