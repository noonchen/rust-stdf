//! Demonstrates editing records while streaming with [`StdfWriter`].
//!
//! `stdf_edit_example <in.stdf> <out.stdf> [test_num]` copies `in.stdf` to
//! `out.stdf`, adding `0.5` to the result of every PTR whose test number matches
//! `test_num` (default 1000). All other records (and PTRs with a different test
//! number) are re-emitted byte-for-byte from their borrowed views, so only the
//! edited records are decoded and re-encoded.

use rust_stdf::{
    stdf_file::{StdfReader, StdfWriter},
    stdf_record_type::*,
    StdfRecordView, PTR,
};
use std::env;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (input, out, test_num) = match args.as_slice() {
        [_, input, out] => (input, out, 1000u32),
        [_, input, out, test_num] => match test_num.parse() {
            Ok(n) => (input, out, n),
            Err(_) => {
                eprintln!("error: test_num must be a non-negative integer");
                return;
            }
        },
        _ => {
            eprintln!("usage: stdf_edit_example <in.stdf> <out.stdf> [test_num]");
            return;
        }
    };

    if let Err(e) = edit(input, out, test_num) {
        eprintln!("error: {e}");
    }
}

fn edit(in_path: &str, out_path: &str, test_num: u32) -> Result<(), rust_stdf::StdfError> {
    let mut reader = StdfReader::new(in_path)?;
    let file = File::create(out_path).expect("create output");
    // The writer's byte order must match the file being copied.
    let mut writer = StdfWriter::new(BufWriter::new(file), reader.get_byte_order());

    let mut copied = 0usize;
    let mut edited = 0usize;

    let mut iter = reader.get_rawdata_view_iter();
    while let Some(item) = iter.next() {
        let raw = item?;

        // Copy every non-PTR record unchanged.
        if !raw.is_type(REC_PTR) {
            writer.write_raw_view(&raw)?;
            copied += 1;
            continue;
        }

        let rec_view: StdfRecordView = (&raw).into();
        if let StdfRecordView::PTR(ptr) = rec_view {
            // Copy PTRs whose test number does not match unchanged.
            if ptr.test_num() != test_num {
                writer.write_raw_view(&raw)?;
                copied += 1;
                continue;
            }
            // Otherwise add 0.5 to the result and write the edited record.
            let mut rec: PTR = ptr.to_owned();
            rec.result += 0.5;
            writer.write_record(&rec)?;
            edited += 1;
        }
    }

    // BufWriter must be flushed (or dropped) before the file is complete.
    writer.flush()?;
    println!(
        "{out_path}: copied {copied} record(s), edited {edited} PTR(s) with test_num {test_num}"
    );
    Ok(())
}
