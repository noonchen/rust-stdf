//
// stdf_file_tests.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 26th 2022
// -----
// Last Modified: Sat Oct 29 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use rust_stdf::{stdf_file::*, stdf_record_type::*};
use std::{fs::read_dir, path::PathBuf};

fn get_test_stdf_files() -> Vec<PathBuf> {
    let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_folder.push("tests");

    let supported_ext =
        |p: &PathBuf| p.ends_with(".stdf") | p.ends_with(".stdf.bz2") | p.ends_with(".stdf.gz");

    // list folder and get supported file paths
    read_dir(test_folder)
        .ok()
        .unwrap()
        .map(|ent| ent.unwrap().path().to_path_buf())
        .filter(supported_ext)
        .collect::<Vec<PathBuf>>()
}

#[test]
fn supported_stdf_file_test() {
    let stdf_file_list = get_test_stdf_files();

    for file in stdf_file_list.iter() {
        let mut reader =
            StdfReader::new(file).expect(&format!("error when open {}", file.display()));

        let mut record_positions_list = Vec::with_capacity(2048);

        for raw_rec in reader.get_rawdata_iter() {
            record_positions_list.push((
                raw_rec.type_code,
                raw_rec.offset,
                raw_rec.raw_data.len(),
                raw_rec.byte_order.clone(),
            ));
        }

        assert!(record_positions_list[0].0 == REC_FAR);
        assert!(record_positions_list[record_positions_list.len() - 1].0 == REC_MRR);
    }
}
