//
// atdf_example.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 7th 2022
// -----
// Last Modified: Wed Oct 26 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use rust_stdf::atdf_file::*;
use std::env;
use std::time::Instant;

fn main() {
    let stdf_path: String;
    if let Some(fpath) = env::args().nth(1) {
        stdf_path = fpath;
        println!("Input path: {}\n", stdf_path);
    } else {
        println!("no path\n");
        return;
    };

    let mut reader = match AtdfReader::new(&stdf_path) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let start_time = Instant::now();

    for rec in reader.get_record_iter() {
        // println!("{:?}", rec);
        println!("{}", rec.to_atdf_string());
    }
    let elapsed = start_time.elapsed().as_millis();
    println!("elapsed time {:?} ms", elapsed);
}
