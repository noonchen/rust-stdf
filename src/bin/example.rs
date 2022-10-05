use rust_stdf::{stdf_record_type::*, StdfReader, StdfRecord};
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

    // let start_time = Instant::now();
    // if let Ok(rec_list) = reader.read_all_records() {
    //     let elapsed = start_time.elapsed().as_millis();
    //     println!("Total records: {}, time elapsed {} ms\n", rec_list.len(), elapsed);
    //     for rec in rec_list.iter().filter(|x| x.is_type(StdfRecordType::RecMIR)) {
    //         println!("{:?}", rec);
    //     }
    // }

    let start_time = Instant::now();
    let mut count = 0usize;
    let rec_t = REC_PIR | REC_PRR;
    for (ind, rec) in reader
        .get_record_iter()
        .filter(|x| x.is_type(rec_t))
        .enumerate()
    {
        count = ind + 1;
        match rec {
            StdfRecord::PIR(pir_rec) => {
                println!("x: {}, y: {}", pir_rec.head_num, pir_rec.site_num)
            }
            StdfRecord::PRR(prr_rec) => {
                println!("x: {}, y: {}", prr_rec.x_coord, prr_rec.y_coord)
            }
            _ => {}
        };
    }
    let elapsed = start_time.elapsed().as_millis();
    println!(
        "Total {:?} records: {}, time elapsed {} ms\n",
        rec_t, count, elapsed
    );
}
