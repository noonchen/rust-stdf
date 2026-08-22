//! Demonstrates [`StdfWriter`].
//!
//! Two modes:
//!
//! - `stdf_write_example <out.stdf>` builds a tiny STDF file from typed records
//!   (each is validated on write).
//! - `stdf_write_example <in.stdf> <out.stdf>` does a byte-exact filter/copy:
//!   stream `in.stdf` and re-emit every record except PTRs, using zero-copy
//!   borrowed views (no decode/re-encode).

use rust_stdf::{
    stdf_file::{StdfReader, StdfWriter},
    ByteOrder, FAR, PTR,
};
use std::env;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, out] => build_demo(out),
        [_, input, out] => filter_copy(input, out),
        _ => {
            eprintln!("usage:\n  stdf_write_example <out.stdf>\n  stdf_write_example <in.stdf> <out.stdf>");
        }
    }
}

/// Build a small file from typed records. `write_record` validates each record
/// before any bytes are emitted.
fn build_demo(out_path: &str) {
    let file = File::create(out_path).expect("create output");
    // Wrap in BufWriter, mirroring how StdfReader wraps its input in BufReader.
    let mut writer = StdfWriter::new(BufWriter::new(file), ByteOrder::LittleEndian);

    let mut far = FAR::new();
    far.cpu_type = 2;
    far.stdf_ver = 4;
    writer.write_record(&far).expect("write FAR");

    let mut ptr = PTR::new();
    ptr.test_num = 1000;
    ptr.head_num = 1;
    ptr.site_num = 1;
    ptr.test_flg = [0x00];
    ptr.parm_flg = [0x00];
    ptr.result = 1.234;
    ptr.test_txt = "continuity".to_string();
    // Trailing `Option` fields are left `None` (the valid truncated form).
    writer.write_record(&ptr).expect("write PTR");

    // BufWriter must be flushed (or dropped) before the file is complete.
    writer.flush().expect("flush");
    println!("wrote {out_path}");
}

/// Byte-exact filter/copy using zero-copy views: keep everything except PTR.
fn filter_copy(in_path: &str, out_path: &str) {
    let mut reader = StdfReader::new(in_path).expect("open input");

    // PTR is record type (15, 10).
    const PTR_TYP: u8 = 15;
    const PTR_SUB: u8 = 10;

    // The writer's byte order must match the file's; take it from the first
    // record view so the passthrough byte-order guard is satisfied.
    let mut writer: Option<StdfWriter<BufWriter<File>>> = None;
    let mut kept = 0usize;
    let mut dropped = 0usize;

    let mut iter = reader.get_rawdata_view_iter();
    while let Some(item) = iter.next() {
        let view = item.expect("read record");

        let w = writer.get_or_insert_with(|| {
            let file = File::create(out_path).expect("create output");
            StdfWriter::new(BufWriter::new(file), view.byte_order)
        });

        if view.header.typ == PTR_TYP && view.header.sub == PTR_SUB {
            dropped += 1;
            continue;
        }
        // Re-emit the borrowed bytes verbatim.
        w.write_raw_view(&view).expect("write record");
        kept += 1;
    }

    if let Some(mut w) = writer {
        w.flush().expect("flush");
    }
    println!("{out_path}: kept {kept} record(s), dropped {dropped} PTR(s)");
}
