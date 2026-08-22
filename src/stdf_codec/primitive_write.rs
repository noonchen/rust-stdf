use super::primitives::*;
use crate::stdf_error::{StdfError, StdfErrorKind};
use std::io::Write;

/// Base single-byte writer. Named to mirror the reader's sole one-byte leaf
/// `read_uint8`, which is the name the derive macro maps `U1`/`C1`/`B1` fields
/// to; the `write_b1`/`write_c1` wrappers delegate here.
#[inline(always)]
pub(crate) fn write_uint8<W: Write>(w: &mut W, v: U1) -> Result<(), StdfError> {
    w.write_all(&[v])?;
    Ok(())
}

#[inline(always)]
pub(crate) fn write_b1<W: Write>(w: &mut W, v: &B1) -> Result<(), StdfError> {
    write_uint8(w, v[0])
}

#[inline(always)]
pub(crate) fn write_c1<W: Write>(w: &mut W, v: C1) -> Result<(), StdfError> {
    write_uint8(w, v as U1)
}

/// Write a fixed-width scalar in the requested byte order.
///
/// Taking the pre-encoded `le`/`be` arrays keeps this generic over width via a
/// const `N` without a numeric trait bound. Both conversions appear at the call
/// site, but each is a `bswap`/no-op and `#[inline(always)]` lets the optimizer
/// drop the unused one, so there is no real overhead.
#[inline(always)]
fn write_ordered<W: Write, const N: usize>(
    w: &mut W,
    le: [u8; N],
    be: [u8; N],
    order: &ByteOrder,
) -> Result<(), StdfError> {
    match order {
        ByteOrder::LittleEndian => w.write_all(&le)?,
        ByteOrder::BigEndian => w.write_all(&be)?,
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn write_u2<W: Write>(w: &mut W, v: U2, order: &ByteOrder) -> Result<(), StdfError> {
    write_ordered(w, v.to_le_bytes(), v.to_be_bytes(), order)
}

#[inline(always)]
pub(crate) fn write_u4<W: Write>(w: &mut W, v: U4, order: &ByteOrder) -> Result<(), StdfError> {
    write_ordered(w, v.to_le_bytes(), v.to_be_bytes(), order)
}

#[inline(always)]
pub(crate) fn write_u8<W: Write>(w: &mut W, v: U8, order: &ByteOrder) -> Result<(), StdfError> {
    write_ordered(w, v.to_le_bytes(), v.to_be_bytes(), order)
}

#[inline(always)]
pub(crate) fn write_i1<W: Write>(w: &mut W, v: I1) -> Result<(), StdfError> {
    write_uint8(w, v as U1)
}

#[inline(always)]
pub(crate) fn write_i2<W: Write>(w: &mut W, v: I2, order: &ByteOrder) -> Result<(), StdfError> {
    write_ordered(w, v.to_le_bytes(), v.to_be_bytes(), order)
}

#[inline(always)]
pub(crate) fn write_i4<W: Write>(w: &mut W, v: I4, order: &ByteOrder) -> Result<(), StdfError> {
    write_ordered(w, v.to_le_bytes(), v.to_be_bytes(), order)
}

#[inline(always)]
pub(crate) fn write_r4<W: Write>(w: &mut W, v: R4, order: &ByteOrder) -> Result<(), StdfError> {
    write_ordered(w, v.to_le_bytes(), v.to_be_bytes(), order)
}

#[inline(always)]
pub(crate) fn write_r8<W: Write>(w: &mut W, v: R8, order: &ByteOrder) -> Result<(), StdfError> {
    write_ordered(w, v.to_le_bytes(), v.to_be_bytes(), order)
}

#[inline(always)]
pub(crate) fn write_cn<W: Write>(w: &mut W, v: &Cn) -> Result<(), StdfError> {
    write_uint8(w, v.len() as U1)?;
    w.write_all(v.as_bytes())?;
    Ok(())
}

#[inline(always)]
pub(crate) fn write_sn<W: Write>(w: &mut W, v: &Sn, order: &ByteOrder) -> Result<(), StdfError> {
    write_u2(w, v.len() as U2, order)?;
    w.write_all(v.as_bytes())?;
    Ok(())
}

#[inline(always)]
pub(crate) fn write_bn<W: Write>(w: &mut W, v: &Bn) -> Result<(), StdfError> {
    write_uint8(w, v.len() as U1)?;
    w.write_all(v)?;
    Ok(())
}

#[inline(always)]
pub(crate) fn write_dn<W: Write>(w: &mut W, v: &Dn, order: &ByteOrder) -> Result<(), StdfError> {
    write_u2(w, v.bit_count, order)?;
    w.write_all(&v.bit_data)?;
    Ok(())
}

#[inline(always)]
pub(crate) fn write_cf<W: Write>(w: &mut W, v: &Cf) -> Result<(), StdfError> {
    w.write_all(v.as_bytes())?;
    Ok(())
}

pub(crate) fn write_kx_cn<W: Write>(w: &mut W, v: &KxCn) -> Result<(), StdfError> {
    for s in v {
        write_cn(w, s)?;
    }
    Ok(())
}

pub(crate) fn write_kx_sn<W: Write>(
    w: &mut W,
    v: &KxSn,
    order: &ByteOrder,
) -> Result<(), StdfError> {
    for s in v {
        write_sn(w, s, order)?;
    }
    Ok(())
}

pub(crate) fn write_kx_cf<W: Write>(w: &mut W, v: &KxCf) -> Result<(), StdfError> {
    for s in v {
        write_cf(w, s)?;
    }
    Ok(())
}

pub(crate) fn write_kx_u1<W: Write>(w: &mut W, v: &KxU1) -> Result<(), StdfError> {
    for x in v {
        write_uint8(w, *x)?;
    }
    Ok(())
}

pub(crate) fn write_kx_u2<W: Write>(
    w: &mut W,
    v: &KxU2,
    order: &ByteOrder,
) -> Result<(), StdfError> {
    for x in v {
        write_u2(w, *x, order)?;
    }
    Ok(())
}

pub(crate) fn write_kx_u4<W: Write>(
    w: &mut W,
    v: &KxU4,
    order: &ByteOrder,
) -> Result<(), StdfError> {
    for x in v {
        write_u4(w, *x, order)?;
    }
    Ok(())
}

pub(crate) fn write_kx_u8<W: Write>(
    w: &mut W,
    v: &KxU8,
    order: &ByteOrder,
) -> Result<(), StdfError> {
    for x in v {
        write_u8(w, *x, order)?;
    }
    Ok(())
}

pub(crate) fn write_kx_r4<W: Write>(
    w: &mut W,
    v: &KxR4,
    order: &ByteOrder,
) -> Result<(), StdfError> {
    for x in v {
        write_r4(w, *x, order)?;
    }
    Ok(())
}

/// Write a `KxUf` variable-width unsigned array. Mirrors the reader's
/// `read_kx_uf`, which is the name the derive macro maps `KxUf` fields to.
pub(crate) fn write_kx_uf<W: Write>(
    w: &mut W,
    v: &KxUf,
    order: &ByteOrder,
) -> Result<(), StdfError> {
    match v {
        KxUf::F1(v) => write_kx_u1(w, v),
        KxUf::F2(v) => write_kx_u2(w, v, order),
        KxUf::F4(v) => write_kx_u4(w, v, order),
        KxUf::F8(v) => write_kx_u8(w, v, order),
    }
}

pub(crate) fn write_kx_n1<W: Write>(w: &mut W, v: &KxN1) -> Result<(), StdfError> {
    let mut iter = v.iter();
    while let Some(lo) = iter.next() {
        let mut byte = *lo & 0x0F;
        if let Some(hi) = iter.next() {
            byte |= (*hi & 0x0F) << 4;
        }
        write_uint8(w, byte)?;
    }
    Ok(())
}

pub(crate) fn v1_payload_len(v: &V1) -> usize {
    match v {
        V1::B0 => 1,
        V1::U1(_) => 2,
        V1::U2(_) => 3,
        V1::U4(_) => 5,
        V1::I1(_) => 2,
        V1::I2(_) => 3,
        V1::I4(_) => 5,
        V1::R4(_) => 5,
        V1::R8(_) => 9,
        V1::Cn(s) => 2 + s.len(),
        V1::Bn(b) => 2 + b.len(),
        V1::Dn(d) => 3 + d.bit_data.len(),
        V1::N1(_) => 2,
        V1::Invalid => 0,
    }
}

pub(crate) fn vn_payload_len(v: &Vn) -> usize {
    v.iter().map(v1_payload_len).sum()
}

pub(crate) fn write_v1<W: Write>(w: &mut W, v: &V1, order: &ByteOrder) -> Result<(), StdfError> {
    match v {
        V1::B0 => write_uint8(w, 0),
        V1::U1(x) => {
            write_uint8(w, 1)?;
            write_uint8(w, *x)
        }
        V1::U2(x) => {
            write_uint8(w, 2)?;
            write_u2(w, *x, order)
        }
        V1::U4(x) => {
            write_uint8(w, 3)?;
            write_u4(w, *x, order)
        }
        V1::I1(x) => {
            write_uint8(w, 4)?;
            write_i1(w, *x)
        }
        V1::I2(x) => {
            write_uint8(w, 5)?;
            write_i2(w, *x, order)
        }
        V1::I4(x) => {
            write_uint8(w, 6)?;
            write_i4(w, *x, order)
        }
        V1::R4(x) => {
            write_uint8(w, 7)?;
            write_r4(w, *x, order)
        }
        V1::R8(x) => {
            write_uint8(w, 8)?;
            write_r8(w, *x, order)
        }
        V1::Cn(s) => {
            write_uint8(w, 10)?;
            write_cn(w, s)
        }
        V1::Bn(b) => {
            write_uint8(w, 11)?;
            write_bn(w, b)
        }
        V1::Dn(d) => {
            write_uint8(w, 12)?;
            write_dn(w, d, order)
        }
        V1::N1(x) => {
            write_uint8(w, 13)?;
            write_uint8(w, *x)
        }
        V1::Invalid => Err(StdfError::new(
            StdfErrorKind::InvalidValue,
            "V1::Invalid is not serializable",
        )),
    }
}

pub(crate) fn write_vn<W: Write>(w: &mut W, v: &Vn, order: &ByteOrder) -> Result<(), StdfError> {
    for item in v {
        write_v1(w, item, order)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdf_codec::{
        read_bn, read_cf, read_cn, read_dn, read_i1, read_i2, read_i4, read_kx_cf, read_kx_cn,
        read_kx_n1, read_kx_r4, read_kx_sn, read_kx_u1, read_kx_u2, read_kx_u4, read_kx_u8,
        read_kx_uf, read_r4, read_r8, read_sn, read_u2, read_u4, read_u8, read_uint8, read_vn,
    };

    fn read_one<T>(
        order: &ByteOrder,
        raw: &[u8],
        f: impl FnOnce(&[u8], &mut usize, &ByteOrder) -> T,
    ) -> T {
        let mut pos = 0;
        f(raw, &mut pos, order)
    }

    fn write_one<W: std::io::Write>(w: &mut W, f: impl FnOnce(&mut W) -> Result<(), StdfError>) {
        f(w).unwrap()
    }

    #[test]
    fn scalar_write_read_roundtrip() {
        for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
            let mut buf = Vec::new();
            write_uint8(&mut buf, 0xAB).unwrap();
            write_u2(&mut buf, 0x1234, &order).unwrap();
            write_u4(&mut buf, 0x89AB_CDEF, &order).unwrap();
            write_u8(&mut buf, 0x0123_4567_89AB_CDEF, &order).unwrap();
            write_i1(&mut buf, -7).unwrap();
            write_i2(&mut buf, -1234, &order).unwrap();
            write_i4(&mut buf, -123_456, &order).unwrap();
            write_r4(&mut buf, 1.5, &order).unwrap();
            write_r8(&mut buf, -2.25, &order).unwrap();
            write_b1(&mut buf, &[0x80]).unwrap();
            write_c1(&mut buf, 'A').unwrap();

            let mut pos = 0;
            assert_eq!(read_uint8(&buf, &mut pos), 0xAB);
            assert_eq!(read_u2(&buf, &mut pos, &order), 0x1234);
            assert_eq!(read_u4(&buf, &mut pos, &order), 0x89AB_CDEF);
            assert_eq!(read_u8(&buf, &mut pos, &order), 0x0123_4567_89AB_CDEF);
            assert_eq!(read_i1(&buf, &mut pos), -7);
            assert_eq!(read_i2(&buf, &mut pos, &order), -1234);
            assert_eq!(read_i4(&buf, &mut pos, &order), -123_456);
            assert_eq!(read_r4(&buf, &mut pos, &order), 1.5);
            assert_eq!(read_r8(&buf, &mut pos, &order), -2.25);
            assert_eq!([read_uint8(&buf, &mut pos)], [0x80]);
            assert_eq!(read_uint8(&buf, &mut pos) as char, 'A');
            assert_eq!(pos, buf.len());
        }
    }

    #[test]
    fn variable_write_read_roundtrip() {
        for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
            let mut buf = Vec::new();
            write_cn(&mut buf, &"Test".to_string()).unwrap();
            write_sn(&mut buf, &"Sn data".to_string(), &order).unwrap();
            write_bn(&mut buf, &vec![1, 2, 3]).unwrap();
            write_dn(
                &mut buf,
                &Dn {
                    bit_count: 10,
                    bit_data: vec![0xFF, 0x03],
                },
                &order,
            )
            .unwrap();
            write_cf(&mut buf, &"fixed".to_string()).unwrap();

            let mut pos = 0;
            assert_eq!(read_cn(&buf, &mut pos), "Test");
            assert_eq!(read_sn(&buf, &mut pos, &order), "Sn data");
            assert_eq!(read_bn(&buf, &mut pos), vec![1, 2, 3]);
            assert_eq!(
                read_dn(&buf, &mut pos, &order),
                Dn {
                    bit_count: 10,
                    bit_data: vec![0xFF, 0x03],
                }
            );
            assert_eq!(read_cf(&buf, &mut pos, 5), "fixed");
            assert_eq!(pos, buf.len());
        }
    }

    #[test]
    fn kx_write_read_roundtrip() {
        for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
            let mut buf = Vec::new();
            write_kx_cn(&mut buf, &vec!["A".to_string(), "BC".to_string()]).unwrap();
            write_kx_sn(&mut buf, &vec!["DE".to_string(), "FGH".to_string()], &order).unwrap();
            write_kx_cf(&mut buf, &vec!["AB".to_string(), "CD".to_string()]).unwrap();
            write_kx_u1(&mut buf, &vec![1, 2]).unwrap();
            write_kx_u2(&mut buf, &vec![3, 4], &order).unwrap();
            write_kx_u4(&mut buf, &vec![5, 6], &order).unwrap();
            write_kx_u8(&mut buf, &vec![7, 8], &order).unwrap();
            write_kx_r4(&mut buf, &vec![1.5, -2.5], &order).unwrap();
            write_kx_n1(&mut buf, &vec![0x0A, 0x0B, 0x0C]).unwrap();
            write_kx_uf(&mut buf, &KxUf::F2(vec![9, 10]), &order).unwrap();

            let mut pos = 0;
            assert_eq!(
                read_kx_cn(&buf, &mut pos, 2),
                vec!["A".to_string(), "BC".to_string()]
            );
            assert_eq!(
                read_kx_sn(&buf, &mut pos, &order, 2),
                vec!["DE".to_string(), "FGH".to_string()]
            );
            assert_eq!(
                read_kx_cf(&buf, &mut pos, 2, 2),
                vec!["AB".to_string(), "CD".to_string()]
            );
            assert_eq!(read_kx_u1(&buf, &mut pos, 2), vec![1, 2]);
            assert_eq!(read_kx_u2(&buf, &mut pos, &order, 2), vec![3, 4]);
            assert_eq!(read_kx_u4(&buf, &mut pos, &order, 2), vec![5, 6]);
            assert_eq!(read_kx_u8(&buf, &mut pos, &order, 2), vec![7, 8]);
            assert_eq!(read_kx_r4(&buf, &mut pos, &order, 2), vec![1.5, -2.5]);
            assert_eq!(read_kx_n1(&buf, &mut pos, 3), vec![0x0A, 0x0B, 0x0C]);
            assert_eq!(
                read_kx_uf(&buf, &mut pos, &order, 2, 2),
                KxUf::F2(vec![9, 10])
            );
            assert_eq!(pos, buf.len());
        }
    }

    #[test]
    fn vn_write_read_roundtrip() {
        for order in [ByteOrder::LittleEndian, ByteOrder::BigEndian] {
            let vn = vec![
                V1::B0,
                V1::U1(1),
                V1::U2(2),
                V1::U4(3),
                V1::I1(-1),
                V1::I2(-2),
                V1::I4(-3),
                V1::R4(1.25),
                V1::R8(-2.5),
                V1::Cn("Cn".to_string()),
                V1::Bn(vec![4, 5]),
                V1::Dn(Dn {
                    bit_count: 10,
                    bit_data: vec![0xFF, 0x03],
                }),
                V1::N1(0x0F),
            ];
            let mut buf = Vec::new();
            write_vn(&mut buf, &vn, &order).unwrap();

            let mut pos = 0;
            assert_eq!(read_vn(&buf, &mut pos, &order, vn.len() as u16), vn);
            assert_eq!(pos, buf.len());
            assert_eq!(vn_payload_len(&vn), buf.len());
        }
    }

    #[test]
    fn leaf_helpers_write_to_scratch() {
        // Smoke-test the generic helper shapes used above. The read-one helper
        // intentionally keeps `pos` internal so each leaf test is self-contained.
        let order = ByteOrder::LittleEndian;
        let mut buf = Vec::new();
        write_one(&mut buf, |w| write_u2(w, 0x0102, &order));
        let pos = 0;
        let value = read_one(&order, &buf, read_u2);
        assert_eq!(value, 0x0102);
        assert_eq!(pos, 0);
    }
}
