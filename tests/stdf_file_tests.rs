//
// stdf_file_tests.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 26th 2022
// -----
// Last Modified: Thu Oct 27 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use rust_stdf::{stdf_file::*, stdf_record_type::*};
use std::{fs::read_dir, path::PathBuf};

#[test]
fn supported_stdf_file_test() {
    let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_folder.push("tests");

    let supported_ext = |p: &PathBuf| p.ends_with(".stdf") | p.ends_with(".stdf.bz2") | p.ends_with(".stdf.gz");

    // list folder and get supported file paths
    let stdf_file_list = read_dir(test_folder)
        .ok()
        .unwrap()
        .map(|ent| ent.unwrap().path().to_path_buf())
        .filter(supported_ext)
        .collect::<Vec<PathBuf>>();

    for file in stdf_file_list.iter() {
        let mut reader = StdfReader::new(file).expect(&format!("error when open {}", file.display()));
        let type_list = reader.get_record_iter().map(|rec| rec.get_type()).collect::<Vec<_>>();
        assert!(type_list[0] == REC_FAR);
        assert!(type_list[ type_list.len() - 1 ] == REC_MRR);
    }
}
