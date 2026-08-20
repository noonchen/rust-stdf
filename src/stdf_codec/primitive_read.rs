use super::primitives::*;

// data type functions
macro_rules! read_multi_byte_num {
    ($num_type:ty, $length:expr, $raw:ident, $pos:expr, $order:expr, $default:expr) => {{
        let pos_after_read = *$pos + $length;
        if pos_after_read <= $raw.len() {
            let mut tmp = [0u8; $length];
            tmp.copy_from_slice(&$raw[*$pos..pos_after_read]);
            *$pos = pos_after_read;
            match $order {
                ByteOrder::LittleEndian => <$num_type>::from_le_bytes(tmp),
                ByteOrder::BigEndian => <$num_type>::from_be_bytes(tmp),
            }
        } else {
            $default
        }
    }};
}

macro_rules! read_multi_element {
    ($count:expr, $func:ident($($arg:tt)+)) => {
        {
            let mut value = Vec::with_capacity($count as usize);
            for _ in 0..$count {
                value.push( $func($($arg)+) );
            }
            value
        }
    }
}

/// Read uint8 from byte array with offset "pos", compatible with B1, C1 and U1
#[inline(always)]
pub(crate) fn read_uint8(raw_data: &[u8], pos: &mut usize) -> u8 {
    if *pos < raw_data.len() {
        let value = (*raw_data)[*pos];
        *pos += 1;
        value
    } else {
        0
    }
}

/// Read U2 (u16) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_u2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U2 {
    read_multi_byte_num!(U2, 2, raw_data, pos, order, 0)
}

/// Read U4 (u32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U4 {
    read_multi_byte_num!(U4, 4, raw_data, pos, order, 0)
}

/// Read U8 (u64) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> U8 {
    read_multi_byte_num!(U8, 8, raw_data, pos, order, 0)
}

/// Read I1 (i8) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_i1(raw_data: &[u8], pos: &mut usize) -> I1 {
    if *pos < raw_data.len() {
        let value = (*raw_data)[*pos] as I1;
        *pos += 1;
        value
    } else {
        0
    }
}

/// Read I2 (i16) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_i2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> I2 {
    read_multi_byte_num!(I2, 2, raw_data, pos, order, 0)
}

/// Read I4 (i32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_i4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> I4 {
    read_multi_byte_num!(I4, 4, raw_data, pos, order, 0)
}

/// Read R4 (f32) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_r4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> R4 {
    read_multi_byte_num!(R4, 4, raw_data, pos, order, 0.0)
}

/// Read R8 (f64) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_r8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> R8 {
    read_multi_byte_num!(R8, 8, raw_data, pos, order, 0.0)
}

/// Read Cn (u8 + String) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_cn(raw_data: &[u8], pos: &mut usize) -> Cn {
    let count = read_uint8(raw_data, pos) as usize;
    let mut value = String::default();
    if count != 0 {
        let min_pos = std::cmp::min(*pos + count, raw_data.len());
        value = bytes_to_string(&raw_data[*pos..min_pos]);
        *pos = min_pos;
    }
    value
}

/// Read Sn (u16 + String) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> Sn {
    let count = read_u2(raw_data, pos, order) as usize;
    let mut value = String::default();
    if count != 0 {
        let min_pos = std::cmp::min(*pos + count, raw_data.len());
        value = bytes_to_string(&raw_data[*pos..min_pos]);
        *pos = min_pos;
    }
    value
}

/// Read Cf (String) from byte array with offset "pos", String length is provide by "f"
#[inline(always)]
pub(crate) fn read_cf(raw_data: &[u8], pos: &mut usize, f: u8) -> Cf {
    let mut value = String::default();
    if f != 0 {
        let pos_after_read = *pos + (f as usize);
        if pos_after_read <= raw_data.len() {
            // read count
            value = bytes_to_string(&raw_data[*pos..pos_after_read]);
            *pos = pos_after_read;
        } else {
            // read all
            value = bytes_to_string(&raw_data[*pos..]);
            *pos = raw_data.len();
        }
    }
    value
}

/// Read Bn (u8 + Vec<u8>) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_bn(raw_data: &[u8], pos: &mut usize) -> Bn {
    let count = read_uint8(raw_data, pos) as usize;
    if count != 0 {
        let min_pos = std::cmp::min(*pos + count, raw_data.len());
        let data_slice = &raw_data[*pos..min_pos];
        *pos = min_pos;
        let mut value = vec![0u8; data_slice.len()];
        value.copy_from_slice(data_slice);
        value
    } else {
        vec![0u8; 0]
    }
}

/// Read Dn (u16 + Vec<u8>) from byte array with offset "pos", u16 is bit counts
#[inline(always)]
pub(crate) fn read_dn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> Dn {
    let bitcount = read_u2(raw_data, pos, order);
    let bytecount = (bitcount as usize).div_ceil(8);
    if bytecount != 0 {
        let min_pos = std::cmp::min(*pos + bytecount, raw_data.len());
        let data_slice = &raw_data[*pos..min_pos];
        *pos = min_pos;
        let mut value = vec![0u8; data_slice.len()];
        value.copy_from_slice(data_slice);
        Dn {
            bit_count: bitcount,
            bit_data: value,
        }
    } else {
        Dn {
            bit_count: bitcount,
            bit_data: Vec::new(),
        }
    }
}

/// Read KxCn (Vec<Cn>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_cn(raw_data: &[u8], pos: &mut usize, k: u16) -> KxCn {
    read_multi_element!(k, read_cn(raw_data, pos))
}

/// Read KxSn (Vec<Sn>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_sn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxSn {
    read_multi_element!(k, read_sn(raw_data, pos, order))
}

/// Read KxCf (Vec<Cf>) from byte array with offset "pos", vector size is provide by "k", String size is "f"
#[inline(always)]
pub(crate) fn read_kx_cf(raw_data: &[u8], pos: &mut usize, k: u16, f: u8) -> KxCf {
    let mut value = Vec::with_capacity(k as usize);
    for _ in 0..k {
        value.push(read_cf(raw_data, pos, f));
    }
    value
}

/// Read KxU1 (Vec<u8>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u1(raw_data: &[u8], pos: &mut usize, k: u16) -> KxU1 {
    read_multi_element!(k, read_uint8(raw_data, pos))
}

/// Read KxU2 (Vec<u16>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u2(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU2 {
    read_multi_element!(k, read_u2(raw_data, pos, order))
}

/// Read KxU4 (Vec<u32>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU4 {
    read_multi_element!(k, read_u4(raw_data, pos, order))
}

/// Read KxU8 (Vec<u64>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_u8(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxU8 {
    read_multi_element!(k, read_u8(raw_data, pos, order))
}

/// Read KxUf (Vec<u8|u16|u32|u64>) from byte array with offset "pos", vector size is provide by "k", size of number is "f"
#[inline(always)]
pub(crate) fn read_kx_uf(
    raw_data: &[u8],
    pos: &mut usize,
    order: &ByteOrder,
    k: u16,
    f: u8,
) -> KxUf {
    if k != 0 {
        match f {
            1 => KxUf::F1(read_kx_u1(raw_data, pos, k)),
            2 => KxUf::F2(read_kx_u2(raw_data, pos, order, k)),
            4 => KxUf::F4(read_kx_u4(raw_data, pos, order, k)),
            8 => KxUf::F8(read_kx_u8(raw_data, pos, order, k)),
            _ => KxUf::F1(vec![0u8; 0]),
        }
    } else {
        KxUf::F1(vec![0u8; 0])
    }
}

/// Read KxR4 (Vec<f32>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_kx_r4(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> KxR4 {
    read_multi_element!(k, read_r4(raw_data, pos, order))
}

/// Read KxN1 (Vec<u8>) from byte array with offset "pos", vector size is provide by "k"
///
/// size of N1 = 4 bits, hence total bytes of k * N1 = k/2 + k%2
#[inline(always)]
pub(crate) fn read_kx_n1(raw_data: &[u8], pos: &mut usize, k: u16) -> KxN1 {
    if k != 0 {
        let bytecount = k / 2 + k % 2; // k = nibble counts, 1 byte = 2 nibble
        let mut value = Vec::with_capacity(k as usize);
        for i in 0..bytecount {
            let tmp = read_uint8(raw_data, pos);
            value.push(tmp & 0x0F);
            if (2 * i + 1) < k {
                value.push((tmp & 0xF0) >> 4);
            }
        }
        value
    } else {
        vec![0u8; 0]
    }
}

/// Read V1 (u8 + generic value) from byte array with offset "pos"
#[inline(always)]
pub(crate) fn read_v1(raw_data: &[u8], pos: &mut usize, order: &ByteOrder) -> V1 {
    let type_byte = if *pos < raw_data.len() {
        read_uint8(raw_data, pos)
    } else {
        0xF
    };

    match type_byte {
        0 => V1::B0,
        1 => V1::U1(read_uint8(raw_data, pos)),
        2 => V1::U2(read_u2(raw_data, pos, order)),
        3 => V1::U4(read_u4(raw_data, pos, order)),
        4 => V1::I1(read_i1(raw_data, pos)),
        5 => V1::I2(read_i2(raw_data, pos, order)),
        6 => V1::I4(read_i4(raw_data, pos, order)),
        7 => V1::R4(read_r4(raw_data, pos, order)),
        8 => V1::R8(read_r8(raw_data, pos, order)),
        10 => V1::Cn(read_cn(raw_data, pos)),
        11 => V1::Bn(read_bn(raw_data, pos)),
        12 => V1::Dn(read_dn(raw_data, pos, order)),
        13 => V1::N1(read_uint8(raw_data, pos) & 0x0F),
        _ => V1::Invalid,
    }
}

/// Read Vn (Vec<V1>) from byte array with offset "pos", vector size is provide by "k"
#[inline(always)]
pub(crate) fn read_vn(raw_data: &[u8], pos: &mut usize, order: &ByteOrder, k: u16) -> Vn {
    read_multi_element!(k, read_v1(raw_data, pos, order))
}
