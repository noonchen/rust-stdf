# rust-stdf

[Documentation](https://docs.rs/rust-stdf/)

A Rust library for reading and writing STDF V4 and V4-2007 test data logs.

```
# Cargo.toml
[dependencies]
rust-stdf = "1.1.0"
```
## Features

Available features:

- `gzip`: Read gzip-compressed (`.gz`) files with `flate2`.
- `bzip`: Read bzip2-compressed (`.bz2`) files with `bzip2`.
- `zipfile`: Read zip archives with `zip`.
- `atdf`: Read ATDF files and convert STDF to ATDF. This feature is under development.
- `serialize`: Serialize STDF records with `serde`.

> [!NOTE]
> `zipfile` contains unsafe Rust. The reader opens only the first unencrypted file in an archive.

The `gzip` and `bzip` features are enabled by default. Disable default features to choose them explicitly:

```
rust-stdf = { version="1.1.0", default-features = false, features = ["gzip", ...]}
```

---

## Reading STDF

This example iterates over records in an STDF V4 file. More examples are available in the [example folder](https://github.com/noonchen/rust-stdf/tree/main/example).

```rust
use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecord};

fn main() {
    let stdf_path = "demo_file.stdf";   // "demo_file.stdf.gz" "demo_file.stdf.bz2"
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
        .map(|x| x.unwrap())
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

### Choosing an Iterator

`StdfReader` provides three ways to read records. Each starts at the current file position:

- `get_record_iter()` returns fully parsed, owned `StdfRecord` values. It implements `Iterator`.
- `get_rawdata_iter()` returns unparsed `RawDataElement` values with owned payloads. It implements `Iterator`.
- `get_rawdata_view_iter()` returns unparsed `RawDataElementView` values that borrow a reused buffer. It avoids heap allocation, but does not implement `Iterator`; call `next()` to advance it.

When you need only a few fields, convert a raw view to a zero-copy `StdfRecordView`. Its field getters parse data on demand instead of parsing the entire record:

```rust
use rust_stdf::{stdf_file::*, stdf_record_type::*, StdfRecordView};

let mut reader = StdfReader::new("demo_file.stdf").unwrap();

// Read two unparsed records with get_rawdata_iter().
let mut count = 0;
for raw in reader
    .get_rawdata_iter()
    .map(|x| x.unwrap())
    .filter(|x| x.is_type(REC_PTR))
{
    if count >= 2 { break; }

    // Convert an owned raw record to either representation,
    // or send it to another thread.
    let _rec: StdfRecord = (&raw).into();
    let _view: StdfRecordView = (&raw).into();

    count += 1;
}

// Continue reading with get_rawdata_view_iter().
let mut iter = reader.get_rawdata_view_iter();
while let Some(item) = iter.next() {
    // This is not a standard iterator, so it has no map/filter methods.
    let raw = item.unwrap();
    if !raw.is_type(REC_PTR) {
        continue;
    }
    // The raw/view is valid only within this scope.
    let rec_view: StdfRecordView = (&raw).into();
    // It can be converted into StdfRecord and RawDataElement
    // if you need it live longer.

    // This iterator is the most efficient option.
    if let StdfRecordView::PTR(ptr) = rec_view {
        println!("{}", ptr.result());
    }
}
```

## Writing STDF

Use `StdfWriter` from `stdf_file` to write STDF bytes. Typed records are validated before writing. When using `BufWriter`, call `flush()` or drop the writer when finished. See [`example/stdf_write_example.rs`](https://github.com/noonchen/rust-stdf/tree/main/example/stdf_write_example.rs).

`StdfWriter` provides these methods:

- `write_record`: Writes a validated typed record such as `FAR` or `PTR`. It does not accept reserved or unknown records.
- `write_stdf_record`: Writes any owned `StdfRecord`, including `ReservedRec` and `UnknownRec`.
- `write_stdf_record_view`: Writes any borrowed `StdfRecordView`. Known records are re-encoded when their byte order differs from the writer's.
- `write_raw` and `write_raw_view`: Write an encoded `RawDataElement` or `RawDataElementView` without changing its bytes.

```rust
use rust_stdf::{stdf_file::StdfWriter, ByteOrder, FAR, PTR};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), rust_stdf::StdfError> {
    let file = File::create("out.stdf").unwrap();
    let mut writer = StdfWriter::new(BufWriter::new(file), ByteOrder::LittleEndian);

    let mut far = FAR::new();
    far.cpu_type = 2;
    far.stdf_ver = 4;
    writer.write_record(&far)?;

    let mut ptr = PTR::new();
    ptr.test_num = 1000;
    ptr.head_num = 1;
    ptr.site_num = 1;
    ptr.result = 1.234;
    ptr.test_txt = "continuity".to_string();
    writer.write_record(&ptr)?;

    writer.flush()?;
    Ok(())
}
```

For a byte-exact filter or copy, such as removing selected record types, read borrowed views with `get_rawdata_view_iter()` and write them with `write_raw_view`. This avoids decoding and re-encoding. Match `err.kind()` to handle errors such as `ByteOrderMismatch` and `RecordTooLarge`. The following example edits selected records while copying the rest verbatim; see [`example/stdf_edit_example.rs`](https://github.com/noonchen/rust-stdf/tree/main/example/stdf_edit_example.rs).

```rust
use rust_stdf::{
    stdf_file::{StdfReader, StdfWriter},
    stdf_record_type::*,
    StdfRecordView, PTR,
};
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), rust_stdf::StdfError> {
    let mut reader = StdfReader::new("in.stdf").unwrap();
    let file = File::create("out.stdf").unwrap();
    let mut writer = StdfWriter::new(BufWriter::new(file), reader.get_byte_order());

    let mut iter = reader.get_rawdata_view_iter();
    while let Some(item) = iter.next() {
        let raw = item.unwrap();

        // Copy every non-PTR record unchanged.
        if !raw.is_type(REC_PTR) {
            writer.write_raw_view(&raw)?;
            continue;
        }

        let rec_view: StdfRecordView = (&raw).into();
        if let StdfRecordView::PTR(ptr) = rec_view {
            // Copy PTRs whose test number is not 1000 unchanged.
            if ptr.test_num() != 1000 {
                writer.write_raw_view(&raw)?;
                continue;
            }
            // Otherwise add 0.5 to the result and write the edited record.
            let mut rec: PTR = ptr.to_owned();
            rec.result += 0.5;
            writer.write_record(&rec)?;
        }
    }

    writer.flush()?;
    Ok(())
}
```
