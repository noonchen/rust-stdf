use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};
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
    let ptr_test_name = env::args().nth(2).unwrap_or("contiuity test".to_string());

    let mut reader = match StdfReader::new(&stdf_path) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let start_time = Instant::now();

    // and put test result of PTR named
    // "continuity test" in a vector.
    let mut dut_count: u64 = 0;
    let mut continuity_rlt = vec![];

    // use type filter to work on certain types,
    // use `|` to combine multiple typs
    let rec_types = REC_PIR | REC_PTR;
    // iterator starts from current file position,
    // if file hits EOF, it will NOT redirect to 0.
    for rec in reader.get_record_iter().filter(|x| x.is_type(rec_types)) {
        match rec {
            StdfRecord::PIR(_) => {
                dut_count += 1;
            }
            StdfRecord::PTR(ref ptr_rec) => {
                if ptr_rec.test_txt == ptr_test_name {
                    continuity_rlt.push(ptr_rec.result);
                }
            }
            _ => {}
        }
    }
    let elapsed = start_time.elapsed().as_millis();
    println!(
        "Total duts {} \n {} result {:?}\n elapsed time {} ms",
        dut_count, ptr_test_name, continuity_rlt, elapsed
    );
}
