//
// record_serialize_test.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: November 14th 2022
// -----
// Last Modified: Mon Nov 14 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

#[cfg(feature = "serialize")]
use rust_stdf::{stdf_record_type::*, *};
#[cfg(feature = "serialize")]
use serde_json::{self, json};

#[test]
#[cfg(feature = "serialize")]
fn record_ser_test() {
    // check upper case
    match StdfRecord::new(REC_FAR) {
        StdfRecord::FAR(r) => {
            let json = serde_json::to_value(&r).unwrap();
            assert_eq!(json["CPU_TYPE"], json!(0));
            assert_eq!(json["cpu_type"], serde_json::Value::Null);
        }
        _ => {}
    }

    // check GDR
    let gdr_rec = StdfRecord::GDR(GDR {
        fld_cnt: 3,
        gen_data: vec![
            V1::Cn("test".to_string()),
            V1::Bn(vec![1, 2, 3, 4, 5, 6, 7]),
            V1::N1(8),
        ],
    });
    match gdr_rec {
        StdfRecord::GDR(r) => {
            let json = serde_json::to_value(&r).unwrap();
            assert_eq!(json["FLD_CNT"], json!(3));
            assert_eq!(json["GEN_DATA"][0]["Cn"], json!("test"));
            assert_eq!(json["GEN_DATA"][1]["Bn"], json!(vec![1, 2, 3, 4, 5, 6, 7]));
            assert_eq!(json["GEN_DATA"][2]["N1"], json!(8));
        }
        _ => {}
    }
}
