use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecordView};
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
    let ptr_test_name = env::args()
        .nth(2)
        .unwrap_or_else(|| "contiuity test".to_string());

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
    let rec_types = REC_PIR | REC_PTR | REC_MPR;
    // iterator starts from current file position,
    // if file hits EOF, it will NOT redirect to 0.
    for raw in reader
        .get_rawdata_iter()
        .map(|x| x.unwrap())
        .filter(|x| x.is_type(rec_types))
    {
        // build a view from raw data without parsing the whole record,
        // get corresponding field value by getter method.
        //
        // it can be more efficient than `stdf_example.rs` 
        // when you only need a few fields of a record.
        let rec_view: StdfRecordView = (&raw).into();
        match rec_view {
            StdfRecordView::PIR(_) => {
                dut_count += 1;
            }
            StdfRecordView::PTR(ptr_view) => {
                // println!(
                //     "[PTR] offset in file: {}, test num: {}, test name: [{}], test result: {}",
                //     raw.offset,
                //     ptr_view.test_num(),
                //     ptr_view.test_txt().as_str(),
                //     ptr_view.result()
                // );
                if ptr_view.test_txt().as_str() == ptr_test_name.as_str() {
                    continuity_rlt.push(ptr_view.result());
                }
            }
            StdfRecordView::MPR(mpr_view) => {
                if mpr_view.res_scal().is_none() {
                    println!("{:?}", mpr_view);
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
