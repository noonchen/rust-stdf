//
// stdf_to_xlsx.rs
//
// This example convert a STDF V4 to 
// xlsx with record name as sheet name.
//
// Requires feature "serialize"
//
// Author: noonchen - chennoon233@foxmail.com
// Created Date: November 16th 2022
// -----
// Last Modified: Wed Nov 16 2022
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};
use rust_xlsxwriter::{Workbook, Worksheet, XlsxError};
use serde_json;
use std::collections::HashMap;
use std::env;

fn main() -> Result<(), XlsxError> {
    let stdf_path: String;
    let xlsx_path: String;

    if let Some(fpath) = env::args().nth(1) {
        stdf_path = fpath;
        println!("Input stdf path: {}\n", stdf_path);
    } else {
        println!("no stdf path\n");
        return Ok(());
    };
    if let Some(fpath) = env::args().nth(2) {
        xlsx_path = fpath;
        println!("Input xlsx path: {}\n", xlsx_path);
    } else {
        println!("no xlsx path\n");
        return Ok(());
    };

    // create a xlsx
    let mut xlsx = Workbook::new();
    let bold_format = rust_xlsxwriter::Format::new().set_bold();

    // create a dictionary for worksheet, same stdf reocrd
    // will be written into a same sheet.
    let mut next_line_map = HashMap::with_capacity(40);

    // open stdf file and start reading
    let mut reader = StdfReader::new(&stdf_path).unwrap();
    for stdf_rec in reader.get_record_iter() {
        // if file is abnormally truncated, the last
        // `stdf_rec` will be an error.
        //
        // Invalid record in the middle of file stream
        // will not throw error, instead returning a
        // `StdfRecord::InvalidRec`.
        let stdf_rec = stdf_rec.unwrap();
        // use record name as hashmap key
        let rec_name = get_rec_name_from_code(stdf_rec.get_type());
        let field_names = get_fields_from_code(stdf_rec.get_type());
        // get sheet from workbook
        let sheet = match xlsx.worksheet_from_name(rec_name) {
            Ok(s) => s,
            Err(_) => {
                // create new if not exist
                let s = xlsx.add_worksheet();
                s.set_name(rec_name)?;
                // based on the record type, write the column header
                for (col, field) in field_names.iter().enumerate() {
                    s.write_string(0, col as u16, field, &bold_format)?;
                }
                s
            }
        };
        // get row + 1 for writing the new line
        let &mut row = next_line_map
            .entry(rec_name)
            .and_modify(|r| *r += 1)
            .or_insert(1);
        // serialize inner record, then write to sheet in field order
        match stdf_rec {
            // rec type 15
            StdfRecord::PTR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::MPR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::FTR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::STR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            // rec type 5
            StdfRecord::PIR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::PRR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            // rec type 2
            StdfRecord::WIR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::WRR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::WCR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            // rec type 50
            StdfRecord::GDR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::DTR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            // rec type 10
            StdfRecord::TSR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            // rec type 1
            StdfRecord::MIR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::MRR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::PCR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::HBR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::SBR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::PMR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::PGR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::PLR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::RDR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::SDR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::PSR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::NMR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::CNR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::SSR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::CDR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            // rec type 0
            StdfRecord::FAR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::ATR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::VUR(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            // rec type 20
            StdfRecord::BPS(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::EPS(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            // rec type 180: Reserved
            // rec type 181: Reserved
            StdfRecord::ReservedRec(r) => {
                let json = serde_json::to_value(&r).unwrap();
                write_json_to_sheet(json, field_names, sheet, row)?;
            }
            StdfRecord::InvalidRec(h) => {
                panic!("Invalid record found! {h:?}")
            }
        }
    }
    // save xlsx to path
    xlsx.save_to_path(std::path::Path::new(&xlsx_path))?;
    Ok(())
}

#[inline(always)]
fn get_fields_from_code(type_code: u64) -> &'static [&'static str] {
    match type_code {
        // rec type 15
        REC_PTR => rust_stdf::PTR::FIELD_NAMES_AS_ARRAY,
        REC_MPR => rust_stdf::MPR::FIELD_NAMES_AS_ARRAY,
        REC_FTR => rust_stdf::FTR::FIELD_NAMES_AS_ARRAY,
        REC_STR => rust_stdf::STR::FIELD_NAMES_AS_ARRAY,
        // rec type 5
        REC_PIR => rust_stdf::PIR::FIELD_NAMES_AS_ARRAY,
        REC_PRR => rust_stdf::PRR::FIELD_NAMES_AS_ARRAY,
        // rec type 2
        REC_WIR => rust_stdf::WIR::FIELD_NAMES_AS_ARRAY,
        REC_WRR => rust_stdf::WRR::FIELD_NAMES_AS_ARRAY,
        REC_WCR => rust_stdf::WCR::FIELD_NAMES_AS_ARRAY,
        // rec type 50
        REC_GDR => rust_stdf::GDR::FIELD_NAMES_AS_ARRAY,
        REC_DTR => rust_stdf::DTR::FIELD_NAMES_AS_ARRAY,
        // rec type 0
        REC_FAR => rust_stdf::FAR::FIELD_NAMES_AS_ARRAY,
        REC_ATR => rust_stdf::ATR::FIELD_NAMES_AS_ARRAY,
        REC_VUR => rust_stdf::VUR::FIELD_NAMES_AS_ARRAY,
        // rec type 1
        REC_MIR => rust_stdf::MIR::FIELD_NAMES_AS_ARRAY,
        REC_MRR => rust_stdf::MRR::FIELD_NAMES_AS_ARRAY,
        REC_PCR => rust_stdf::PCR::FIELD_NAMES_AS_ARRAY,
        REC_HBR => rust_stdf::HBR::FIELD_NAMES_AS_ARRAY,
        REC_SBR => rust_stdf::SBR::FIELD_NAMES_AS_ARRAY,
        REC_PMR => rust_stdf::PMR::FIELD_NAMES_AS_ARRAY,
        REC_PGR => rust_stdf::PGR::FIELD_NAMES_AS_ARRAY,
        REC_PLR => rust_stdf::PLR::FIELD_NAMES_AS_ARRAY,
        REC_RDR => rust_stdf::RDR::FIELD_NAMES_AS_ARRAY,
        REC_SDR => rust_stdf::SDR::FIELD_NAMES_AS_ARRAY,
        REC_PSR => rust_stdf::PSR::FIELD_NAMES_AS_ARRAY,
        REC_NMR => rust_stdf::NMR::FIELD_NAMES_AS_ARRAY,
        REC_CNR => rust_stdf::CNR::FIELD_NAMES_AS_ARRAY,
        REC_SSR => rust_stdf::SSR::FIELD_NAMES_AS_ARRAY,
        REC_CDR => rust_stdf::CDR::FIELD_NAMES_AS_ARRAY,
        // rec type 10
        REC_TSR => rust_stdf::TSR::FIELD_NAMES_AS_ARRAY,
        // rec type 20
        REC_BPS => rust_stdf::BPS::FIELD_NAMES_AS_ARRAY,
        REC_EPS => rust_stdf::EPS::FIELD_NAMES_AS_ARRAY,
        // rec type 180: Reserved
        // rec type 181: Reserved
        REC_RESERVE => rust_stdf::ReservedRec::FIELD_NAMES_AS_ARRAY,
        // not matched
        _ => &[""; 0],
    }
}

#[inline(always)]
fn write_json_to_sheet(
    json: serde_json::Value,
    field_names: &[&str],
    sheet: &mut Worksheet,
    row: u32,
) -> Result<(), XlsxError> {
    for (col, &field) in field_names.iter().enumerate() {
        let col = col as u16;
        let v = &json[field];
        match v {
            serde_json::Value::Number(n) => {
                sheet.write_number_only(row, col, n.as_f64().unwrap_or(f64::NAN))?
            }
            serde_json::Value::Null => sheet.write_string_only(row, col, "N/A")?,
            serde_json::Value::String(s) => sheet.write_string_only(row, col, s)?,
            _ => sheet.write_string_only(row, col, &v.to_string())?,
        };
    }
    Ok(())
}
