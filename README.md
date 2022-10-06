# rust-stdf
 
A Rust STDF library for process STDF datalogs of Version V4 and V4-2007.

Supported compression:
 - Uncompressed
 - Gzip (.gz)

## Example

```rust
use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};

fn main() {
    let stdf_path = "demo_file.stdf";
    let mut reader = match StdfReader::new(&stdf_path) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // we will count total DUT# in the file
    // and put test result of PTR named
    // "continuity test" in a vector.
    let mut dut_count: u64 = 0;
    let mut continuity_rlt = vec![];

    // use type filter to work on certain types,
    // use `|` to combine multiple typs
    let rec_types = REC_PIR | REC_PTR;
    // iterator starts from current file position,
    // if file hits EOF, it will NOT redirect to 0.
    for rec in reader
        .get_record_iter()
        .filter(|x| x.is_type(rec_types))
    {
        match rec {
            StdfRecord::PIR(_) => {dut_count += 1;}
            StdfRecord::PTR(ref ptr_rec) => {
                if ptr_rec.test_txt == "continuity test" {
                    continuity_rlt.push(ptr_rec.result);
                }
            }
            _ => {}
        }
    }
    println!("Total duts {} \n continuity result {:?}",
            dut_count,
            continuity_rlt);
}
```