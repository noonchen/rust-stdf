//
// stdf_file_tests.rs
// Author: noonchen - chennoon233@foxmail.com
// Created Date: October 26th 2022
// -----
// Last Modified: Mon Aug 17 2026
// Modified By: noonchen
// -----
// Copyright (c) 2022 noonchen
//

use rand::prelude::*;
use rust_stdf::{
    stdf_file::*, stdf_record_type::*, ByteOrder, CompressType, RawDataElement, StdfRecord,
    StdfRecordView,
};
use std::{
    collections::HashMap,
    fs::{self, read_dir},
    io::{Cursor, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

fn get_test_stdf_files() -> Vec<PathBuf> {
    let mut test_folder = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_folder.push("demo_stdf");

    fn supported_ext(p: &Path) -> bool {
        let p = p.display().to_string();
        let file_ext = p.rsplit('.').next();
        match file_ext {
            None => false,
            Some(ext) => match ext {
                #[cfg(feature = "gzip")]
                "gz" => true,
                #[cfg(feature = "bzip")]
                "bz2" => true,
                #[cfg(feature = "zipfile")]
                "zip" => true,
                "stdf" => true,
                _ => false,
            },
        }
    }

    // list folder and get supported file paths
    read_dir(test_folder)
        .unwrap()
        .map(|ent| ent.unwrap().path().to_path_buf())
        .filter(|p| supported_ext(p))
        .collect::<Vec<PathBuf>>()
}

// ---------------------------------------------------------------------------
// helpers for synthetic in-memory STDF files
// ---------------------------------------------------------------------------

/// Serialize a record header (len/typ/sub) in the given byte order.
fn write_header(buf: &mut Vec<u8>, len: u16, typ: u8, sub: u8, order: ByteOrder) {
    match order {
        ByteOrder::LittleEndian => {
            buf.extend_from_slice(&len.to_le_bytes());
            buf.push(typ);
            buf.push(sub);
        }
        ByteOrder::BigEndian => {
            buf.extend_from_slice(&len.to_be_bytes());
            buf.push(typ);
            buf.push(sub);
        }
    }
}

/// Build a tiny valid STDF file: FAR + one MIR whose `setup_t` is 0x01020304.
///
/// The FAR `REC_LEN` is always serialized little-endian (2 or 512) so that
/// `StdfReader::from` can detect the file byte order before knowing it.
fn minimal_stdf(order: ByteOrder) -> Vec<u8> {
    let mut buf = Vec::new();
    let far_len = match order {
        ByteOrder::LittleEndian => 2u16,
        ByteOrder::BigEndian => 512u16,
    };
    buf.extend_from_slice(&far_len.to_le_bytes()); // FAR REC_LEN (always LE)
    buf.push(0); // typ = FAR
    buf.push(10); // sub = FAR
    buf.push(1); // cpu_type
    buf.push(4); // stdf_ver
    write_header(&mut buf, 4, 1, 10, order); // MIR, 4-byte body
    match order {
        ByteOrder::LittleEndian => buf.extend_from_slice(&0x01020304u32.to_le_bytes()),
        ByteOrder::BigEndian => buf.extend_from_slice(&0x01020304u32.to_be_bytes()),
    }
    buf
}

// ---------------------------------------------------------------------------
// demo files
// ---------------------------------------------------------------------------

#[test]
fn supported_stdf_file_test() {
    let stdf_file_list = get_test_stdf_files();
    assert_ne!(stdf_file_list.len(), 0);

    for file in stdf_file_list.iter() {
        let mut reader = StdfReader::new(file)
            .unwrap_or_else(|e| panic!("error when open {}: {e}", file.display()));

        let mut record_positions_list = Vec::with_capacity(2048);

        let mut rand_picked_record = Vec::with_capacity(2048);
        let mut rng = rand::thread_rng();
        let mut count = 0;

        for (ind, raw_rec) in reader.get_rawdata_iter().enumerate() {
            let raw_rec = raw_rec.unwrap();

            record_positions_list.push((
                raw_rec.header.get_type(),
                raw_rec.offset,
                raw_rec.raw_data.len(),
                raw_rec.byte_order,
            ));

            if count != 0 {
                count -= 1;
            } else {
                count = rng.gen_range(5..20);
                rand_picked_record.push((ind, StdfRecord::from(raw_rec)));
            }
        }

        assert!(record_positions_list[0].0 == REC_FAR);
        assert!(record_positions_list[record_positions_list.len() - 1].0 == REC_MRR);

        // try to read the data from file stream via info from `RawDataElement`
        if file.display().to_string().ends_with(".stdf") {
            let mut fp = fs::File::open(file).unwrap();
            for (ind, parsed_rec) in rand_picked_record.into_iter() {
                let (typ_code, offset, len, order) = record_positions_list[ind];
                let mut buffer = vec![0u8; len];

                fp.seek(SeekFrom::Start(offset))
                    .expect("unable seek to offset");
                match fp.read(&mut buffer) {
                    Ok(cnt) => {
                        if cnt != len {
                            panic!("cannot read expected count, this shouldn't happen");
                        }
                    }
                    Err(e) => panic!("{}", e),
                };
                let mut rec = StdfRecord::new(typ_code);
                rec.read_from_bytes(&buffer, &order);
                // check if it's the same record in the iteration.
                assert_eq!(parsed_rec, rec);
            }
        }
    }
}

// `get_record_iter` must produce exactly the same records as converting every
// `RawDataElement`, for every demo file (including compressed variants).
#[test]
fn record_iter_matches_rawdata_iter() {
    let files = get_test_stdf_files();
    assert_ne!(files.len(), 0);

    for file in &files {
        let mut reader =
            StdfReader::new(file).unwrap_or_else(|e| panic!("open {}: {e}", file.display()));
        let raw_records: Vec<StdfRecord> = reader
            .get_rawdata_iter()
            .map(|r| StdfRecord::from(r.unwrap()))
            .collect();

        let mut reader =
            StdfReader::new(file).unwrap_or_else(|e| panic!("open {}: {e}", file.display()));
        let parsed_records: Vec<StdfRecord> =
            reader.get_record_iter().map(|r| r.unwrap()).collect();

        assert_eq!(
            raw_records,
            parsed_records,
            "get_record_iter and get_rawdata_iter diverge for {}",
            file.display()
        );
        assert_eq!(raw_records.first().map(|r| r.get_type()), Some(REC_FAR));
        assert_eq!(raw_records.last().map(|r| r.get_type()), Some(REC_MRR));
    }
}

// The gzip/bzip2/zip variants of a demo file must decode to the same record
// sequence as the uncompressed original.
#[test]
fn compressed_files_match_uncompressed() {
    let mut by_base: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for p in get_test_stdf_files() {
        let base = p
            .file_name()
            .unwrap()
            .to_string_lossy()
            .split('.')
            .next()
            .unwrap()
            .to_string();
        by_base.entry(base).or_default().push(p);
    }
    assert!(!by_base.is_empty());

    let record_types = |p: &PathBuf| -> Vec<u64> {
        let mut reader = StdfReader::new(p).unwrap_or_else(|e| panic!("open {}: {e}", p.display()));
        reader
            .get_rawdata_iter()
            .map(|r| r.unwrap().header.get_type())
            .collect()
    };

    for (base, group) in &by_base {
        assert!(
            group.len() > 1,
            "only one variant of {base} present: {group:?}"
        );
        let reference = record_types(&group[0]);
        assert_eq!(reference.first(), Some(&REC_FAR));
        for p in &group[1..] {
            assert_eq!(
                reference,
                record_types(p),
                "{base}: {} decodes to a different record sequence",
                p.display()
            );
        }
    }
}

// ---------------------------------------------------------------------------
// StdfReader::from (stream constructor)
// ---------------------------------------------------------------------------

// The reader must detect the byte order from the FAR header and parse records
// in that byte order.
#[test]
fn stream_constructor_detects_endianness() {
    for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
        let mut reader = StdfReader::from(
            Cursor::new(minimal_stdf(order)),
            &CompressType::Uncompressed,
        )
        .unwrap();

        let raws: Vec<_> = reader.get_rawdata_iter().map(|r| r.unwrap()).collect();
        assert_eq!(raws.len(), 2);
        assert_eq!(raws[0].header.get_type(), REC_FAR);
        assert_eq!(raws[1].header.get_type(), REC_MIR);
        assert_eq!(raws[1].byte_order, order, "byte order not detected");

        let mut reader = StdfReader::from(
            Cursor::new(minimal_stdf(order)),
            &CompressType::Uncompressed,
        )
        .unwrap();
        let recs: Vec<_> = reader.get_record_iter().map(|r| r.unwrap()).collect();
        assert_eq!(recs.len(), 2);
        if let StdfRecord::MIR(mir) = &recs[1] {
            assert_eq!(mir.setup_t, 0x01020304, "MIR parsed in wrong byte order");
        } else {
            panic!("expected MIR, got {:?}", recs[1].get_type());
        }

        // view iterator: lending, so read one item at a time
        let mut reader = StdfReader::from(
            Cursor::new(minimal_stdf(order)),
            &CompressType::Uncompressed,
        )
        .unwrap();
        let mut view_iter = reader.get_rawdata_view_iter();
        let far = view_iter.next().unwrap().unwrap();
        assert_eq!(far.header.get_type(), REC_FAR);
        assert_eq!(far.byte_order, order);
        let mir = view_iter.next().unwrap().unwrap();
        assert_eq!(mir.header.get_type(), REC_MIR);
        assert_eq!(mir.byte_order, order, "byte order not detected");
        match StdfRecord::from(&mir) {
            StdfRecord::MIR(m) => {
                assert_eq!(m.setup_t, 0x01020304, "MIR parsed in wrong byte order")
            }
            other => panic!("expected MIR, got {:?}", other.get_type()),
        }
        assert!(view_iter.next().is_none());
    }
}

// ---------------------------------------------------------------------------
// error handling
// ---------------------------------------------------------------------------

#[test]
fn reader_rejects_invalid_inputs() {
    // empty stream: EOF while reading the FAR header
    let err = match StdfReader::from(Cursor::new(Vec::<u8>::new()), &CompressType::Uncompressed) {
        Ok(_) => panic!("empty stream should not open"),
        Err(e) => e,
    };
    assert_eq!(err.code, 4);

    // plausible little-endian length, but the first record is not FAR
    let err = match StdfReader::from(Cursor::new(vec![2, 0, 9, 9]), &CompressType::Uncompressed) {
        Ok(_) => panic!("non-FAR file should not open"),
        Err(e) => e,
    };
    assert_eq!(err.code, 1);

    // FAR record with an unrecognized REC_LEN: byte order cannot be determined
    let err = match StdfReader::from(Cursor::new(vec![7, 0, 0, 10]), &CompressType::Uncompressed) {
        Ok(_) => panic!("unknown endianness should not open"),
        Err(e) => e,
    };
    assert_eq!(err.code, 1);

    // opening a file that does not exist
    let err = match StdfReader::new("definitely/not/a/real/stdf/file.stdf") {
        Ok(_) => panic!("missing file should not open"),
        Err(e) => e,
    };
    assert_eq!(err.code, 3);
}

// A header that claims more body bytes than are present must surface as an IO
// error (code 3), not a silent clean EOF.
#[test]
fn truncated_body_yields_io_error() {
    // FAR followed by a record whose header claims 100 body bytes but only 3
    // are present.
    let truncated = || {
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_le_bytes());
        data.push(0);
        data.push(10); // FAR header
        data.push(1);
        data.push(4); // FAR body
        write_header(&mut data, 100, 1, 10, ByteOrder::LittleEndian);
        data.extend_from_slice(&[0u8; 3]);
        data
    };

    let mut reader =
        StdfReader::from(Cursor::new(truncated()), &CompressType::Uncompressed).unwrap();
    let raw_items: Vec<_> = reader.get_rawdata_iter().collect();
    assert_eq!(raw_items.len(), 2);
    assert!(raw_items[0].is_ok(), "FAR should parse");
    match &raw_items[1] {
        Err(e) => assert_eq!(e.code, 3, "rawdata iterator should report IO error"),
        Ok(_) => panic!("truncated body should error, got Ok"),
    }

    let mut reader =
        StdfReader::from(Cursor::new(truncated()), &CompressType::Uncompressed).unwrap();
    let rec_items: Vec<_> = reader.get_record_iter().collect();
    assert_eq!(rec_items.len(), 2);
    assert!(rec_items[0].is_ok(), "FAR should parse");
    match &rec_items[1] {
        Err(e) => assert_eq!(e.code, 3, "record iterator should report IO error"),
        Ok(_) => panic!("truncated body should error, got Ok"),
    }

    // view iterator: lending, so it can't `.collect()`; drive it manually
    let mut reader =
        StdfReader::from(Cursor::new(truncated()), &CompressType::Uncompressed).unwrap();
    let mut view_iter = reader.get_rawdata_view_iter();
    assert!(view_iter.next().unwrap().is_ok(), "FAR should parse");
    match view_iter.next() {
        Some(Err(e)) => assert_eq!(e.code, 3, "view iterator should report IO error"),
        Some(Ok(_)) => panic!("truncated body should error, got Ok"),
        None => panic!("truncated body should error, got None"),
    }
}

// ---------------------------------------------------------------------------
// iterator behaviour
// ---------------------------------------------------------------------------

// Once the stream is exhausted, a second iteration yields nothing; the
// iterator does not wrap around to the start of the file.
#[test]
fn iterator_stops_at_eof() {
    let mut reader = StdfReader::from(
        Cursor::new(minimal_stdf(ByteOrder::LittleEndian)),
        &CompressType::Uncompressed,
    )
    .unwrap();
    assert_eq!(reader.get_rawdata_iter().count(), 2);
    assert_eq!(reader.get_rawdata_iter().count(), 0);

    let mut reader = StdfReader::from(
        Cursor::new(minimal_stdf(ByteOrder::LittleEndian)),
        &CompressType::Uncompressed,
    )
    .unwrap();
    assert_eq!(reader.get_record_iter().count(), 2);
    assert_eq!(reader.get_record_iter().count(), 0);

    // view iterator: lending, so it can't `.count()`; drive it manually
    let mut reader = StdfReader::from(
        Cursor::new(minimal_stdf(ByteOrder::LittleEndian)),
        &CompressType::Uncompressed,
    )
    .unwrap();
    let mut count = 0;
    let mut view_iter = reader.get_rawdata_view_iter();
    while let Some(item) = view_iter.next() {
        item.unwrap();
        count += 1;
    }
    assert_eq!(count, 2);
    let mut view_iter = reader.get_rawdata_view_iter();
    assert!(view_iter.next().is_none());
}

// ---------------------------------------------------------------------------
// RawDataElement API
// ---------------------------------------------------------------------------

#[test]
fn rawdata_element_conversions() {
    let mut data = Vec::new();
    data.extend_from_slice(&2u16.to_le_bytes());
    data.push(0);
    data.push(10); // FAR header
    data.push(1);
    data.push(4); // FAR body
    write_header(&mut data, 4, 1, 10, ByteOrder::LittleEndian);
    data.extend_from_slice(&0x01020304u32.to_le_bytes()); // MIR body

    let mut reader = StdfReader::from(Cursor::new(data), &CompressType::Uncompressed).unwrap();
    for raw in reader.get_rawdata_iter() {
        let raw = raw.unwrap();
        let code = raw.header.get_type();

        // type checks
        assert!(raw.is_type(code));
        assert!(!raw.is_type(REC_INVALID));
        assert_eq!(raw.header.get_type(), code);

        // non-consuming conversions
        let rec: StdfRecord = (&raw).into();
        assert_eq!(rec.get_type(), code);
        let view: StdfRecordView = (&raw).into();
        assert_eq!(view.get_type(), code);

        // consuming conversion
        let rec_consumed: StdfRecord = raw.into();
        assert_eq!(rec_consumed, rec);
    }
}

// ---------------------------------------------------------------------------
// RawDataViewIter (buffer-reusing, borrowing raw iterator)
// ---------------------------------------------------------------------------

// The borrowing view iterator must yield the same records (type, offset and
// parsed content) as the owning `get_rawdata_iter`, for every demo file.
#[test]
fn view_iter_matches_rawdata_iter() {
    let files = get_test_stdf_files();
    assert_ne!(files.len(), 0);

    for file in &files {
        let mut reader =
            StdfReader::new(file).unwrap_or_else(|e| panic!("open {}: {e}", file.display()));
        let owned: Vec<(u64, u64, StdfRecord)> = reader
            .get_rawdata_iter()
            .map(|r| {
                let raw = r.unwrap();
                (raw.header.get_type(), raw.offset, StdfRecord::from(raw))
            })
            .collect();

        let mut reader =
            StdfReader::new(file).unwrap_or_else(|e| panic!("open {}: {e}", file.display()));
        let mut view_iter = reader.get_rawdata_view_iter();
        let mut i = 0;
        while let Some(item) = view_iter.next() {
            let raw = item.unwrap();
            let (typ, offset, ref rec) = owned[i];
            assert_eq!(
                raw.header.get_type(),
                typ,
                "type mismatch at #{i} in {}",
                file.display()
            );
            assert_eq!(
                raw.offset,
                offset,
                "offset mismatch at #{i} in {}",
                file.display()
            );
            let parsed: StdfRecord = (&raw).into();
            assert_eq!(
                &parsed,
                rec,
                "record mismatch at #{i} in {}",
                file.display()
            );
            i += 1;
        }
        assert_eq!(
            i,
            owned.len(),
            "view iterator length mismatch for {}",
            file.display()
        );
    }
}

// `RawDataElementView` type check and every conversion: to `StdfRecord`,
// `StdfRecordView`, and an owned `RawDataElement`.
#[test]
fn rawdata_element_view_conversions() {
    let mut data = Vec::new();
    data.extend_from_slice(&2u16.to_le_bytes());
    data.push(0);
    data.push(10); // FAR header
    data.push(1);
    data.push(4); // FAR body
    write_header(&mut data, 4, 1, 10, ByteOrder::LittleEndian);
    data.extend_from_slice(&0x01020304u32.to_le_bytes()); // MIR body

    let mut reader = StdfReader::from(Cursor::new(data), &CompressType::Uncompressed).unwrap();
    let mut iter = reader.get_rawdata_view_iter();
    while let Some(item) = iter.next() {
        let raw = item.unwrap();
        let code = raw.header.get_type();

        // type checks
        assert!(raw.is_type(code));
        assert!(!raw.is_type(REC_INVALID));

        // borrowed conversions
        let rec: StdfRecord = (&raw).into();
        assert_eq!(rec.get_type(), code);
        let view: StdfRecordView = (&raw).into();
        assert_eq!(view.get_type(), code);

        // copy into an owned RawDataElement that outlives the borrow
        let owned: RawDataElement = (&raw).into();
        assert_eq!(owned.offset, raw.offset);
        assert_eq!(owned.header, raw.header);
        assert_eq!(owned.raw_data, raw.raw_data);
        assert_eq!(owned.byte_order, raw.byte_order);
        assert_eq!(StdfRecord::from(&owned), rec);
    }
}
