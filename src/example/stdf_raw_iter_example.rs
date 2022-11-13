use rust_stdf::{stdf_file::*, stdf_record_type::*};
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

    let mut reader = match StdfReader::new(&stdf_path) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let start_time = Instant::now();

    let rec_types = REC_PTR;
    // if file hits EOF, it will NOT redirect to 0.
    for raw in reader
        .get_rawdata_iter()
        .map(|x| x.unwrap())
        .filter(|x| x.is_type(rec_types))
    {
        println!("{:?}", raw);
    }
    let elapsed = start_time.elapsed().as_millis();
    println!("elapsed time {} ms", elapsed);
}
