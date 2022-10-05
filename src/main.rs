mod stdf_file;
mod stdf_error;
mod stdf_types;
use stdf_types::StdfRecordType;
use std::env;
use std::time::Instant;

fn main() {
    let stdf_path: String;
    if let Some(fpath) = env::args().skip(1).next() { 
        stdf_path = fpath;
        println!("Input path: {}\n", stdf_path);
    } else {
        println!("no path\n");
        return;
    };
    let mut reader = match stdf_file::StdfReader::new(&stdf_path) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;}
    };
    
    let start_time = Instant::now();
    if let Ok(rec_list) = reader.read_all_records() {
        let elapsed = start_time.elapsed().as_millis();
        println!("Total records: {}, time elapsed {} ms\n", rec_list.len(), elapsed);
        for rec in rec_list.iter().filter(|x| x.is_type(StdfRecordType::RecMIR)) {
            println!("{:?}", rec);
        }
    }

}

