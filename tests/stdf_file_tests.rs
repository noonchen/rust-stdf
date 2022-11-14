//
// stdf_file_tests.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 26th 2022
// -----
// Last Modified: Mon Nov 14 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use rand::prelude::*;
use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};
use std::{
    fs::{self, read_dir},
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

fn get_test_stdf_files() -> Vec<PathBuf> {
    let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_folder.push("demo_stdf");

    fn supported_ext(p: &PathBuf) -> bool {
        let p = p.display().to_string();
        let file_ext = p.rsplit('.').next();
        match file_ext {
            None => false,
            Some(ext) => match ext {
                #[cfg(feature = "gzip")]
                "gz" => true,
                #[cfg(feature = "bzip")]
                "bz2" => true,
                "stdf" => true,
                _ => false,
            },
        }
    }

    // list folder and get supported file paths
    read_dir(test_folder)
        .unwrap()
        .map(|ent| ent.unwrap().path().to_path_buf())
        .filter(supported_ext)
        .collect::<Vec<PathBuf>>()
}

#[test]
fn supported_stdf_file_test() {
    let stdf_file_list = get_test_stdf_files();
    assert_ne!(stdf_file_list.len(), 0);

    for file in stdf_file_list.iter() {
        let mut reader =
            StdfReader::new(file).expect(&format!("error when open {}", file.display()));

        let mut record_positions_list = Vec::with_capacity(2048);

        let mut rand_picked_record = Vec::with_capacity(2048);
        let mut rng = rand::thread_rng();
        let mut count = 0;

        for (ind, raw_rec) in reader.get_rawdata_iter().enumerate() {
            let raw_rec = raw_rec.unwrap();

            record_positions_list.push((
                raw_rec.header.get_type(),
                raw_rec.offset,
                raw_rec.raw_data.len(),
                raw_rec.byte_order.clone(),
            ));

            if count != 0 {
                count -= 1;
            } else {
                count = rng.gen_range(5..20);
                rand_picked_record.push((ind, StdfRecord::from(raw_rec)));
            }
        }

        assert!(record_positions_list[0].0 == REC_FAR);
        assert!(record_positions_list[record_positions_list.len() - 1].0 == REC_MRR);

        // try to read the data from file stream via info from `RawDataElement`
        if file.display().to_string().ends_with(".stdf") {
            let mut fp = fs::File::open(file).unwrap();
            for (ind, parsed_rec) in rand_picked_record.into_iter() {
                let (typ_code, offset, len, order) = record_positions_list[ind];
                let mut buffer = vec![0u8; len];

                fp.seek(SeekFrom::Start(offset))
                    .expect("unable seek to offset");
                match fp.read(&mut buffer) {
                    Ok(cnt) => {
                        if cnt != len {
                            panic!("cannot read expected count, this shouldn't happen");
                        }
                    }
                    Err(e) => panic!("{}", e),
                };
                let mut rec = StdfRecord::new(typ_code);
                rec.read_from_bytes(&buffer, &order);
                // check if it's the same record in the iteration.
                assert_eq!(parsed_rec, rec);
            }
        }
    }
}
