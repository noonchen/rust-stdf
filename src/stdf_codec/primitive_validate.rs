use super::primitives::*;
use crate::stdf_error::StdfErrorKind;

#[inline]
pub(crate) fn validate_c1(v: C1) -> Result<(), StdfErrorKind> {
    if (v as u32) <= u8::MAX as u32 {
        Ok(())
    } else {
        Err(StdfErrorKind::InvalidValue)
    }
}

#[inline]
pub(crate) fn validate_cn(v: &Cn) -> Result<(), StdfErrorKind> {
    if v.len() <= u8::MAX as usize {
        Ok(())
    } else {
        Err(StdfErrorKind::InvalidLength)
    }
}

#[inline]
pub(crate) fn validate_sn(v: &Sn) -> Result<(), StdfErrorKind> {
    if v.len() <= u16::MAX as usize {
        Ok(())
    } else {
        Err(StdfErrorKind::InvalidLength)
    }
}

#[inline]
pub(crate) fn validate_bn(v: &Bn) -> Result<(), StdfErrorKind> {
    if v.len() <= u8::MAX as usize {
        Ok(())
    } else {
        Err(StdfErrorKind::InvalidLength)
    }
}

#[inline]
pub(crate) fn validate_dn(v: &Dn) -> Result<(), StdfErrorKind> {
    let expected = (v.bit_count as usize).div_ceil(8);
    if v.bit_data.len() == expected {
        Ok(())
    } else {
        Err(StdfErrorKind::InvalidLength)
    }
}

#[inline]
pub(crate) fn validate_n1(v: U1) -> Result<(), StdfErrorKind> {
    if v <= 0x0F {
        Ok(())
    } else {
        Err(StdfErrorKind::InvalidValue)
    }
}

#[inline]
pub(crate) fn validate_kx_count(actual: usize, k: usize) -> Result<(), StdfErrorKind> {
    if actual == k {
        Ok(())
    } else {
        Err(StdfErrorKind::CountMismatch)
    }
}

pub(crate) fn validate_kx_n1(v: &KxN1, k: usize) -> Result<(), StdfErrorKind> {
    validate_kx_count(v.len(), k)?;
    for n in v {
        validate_n1(*n)?;
    }
    Ok(())
}

pub(crate) fn validate_kx_cn(v: &KxCn, k: usize) -> Result<(), StdfErrorKind> {
    validate_kx_count(v.len(), k)?;
    for s in v {
        validate_cn(s)?;
    }
    Ok(())
}

pub(crate) fn validate_kx_sn(v: &KxSn, k: usize) -> Result<(), StdfErrorKind> {
    validate_kx_count(v.len(), k)?;
    for s in v {
        validate_sn(s)?;
    }
    Ok(())
}

pub(crate) fn validate_kx_uf(v: &KxUf, k: usize, f: usize) -> Result<(), StdfErrorKind> {
    let actual_k = match v {
        KxUf::F1(v) => v.len(),
        KxUf::F2(v) => v.len(),
        KxUf::F4(v) => v.len(),
        KxUf::F8(v) => v.len(),
    };
    validate_kx_count(actual_k, k)?;

    let actual_f = match v {
        KxUf::F1(_) => 1,
        KxUf::F2(_) => 2,
        KxUf::F4(_) => 4,
        KxUf::F8(_) => 8,
    };
    if actual_f == f {
        Ok(())
    } else {
        Err(StdfErrorKind::WidthMismatch)
    }
}

pub(crate) fn validate_kx_cf(v: &KxCf, k: usize, f: usize) -> Result<(), StdfErrorKind> {
    validate_kx_count(v.len(), k)?;
    for s in v {
        if s.len() != f {
            return Err(StdfErrorKind::WidthMismatch);
        }
    }
    Ok(())
}

pub(crate) fn validate_vn(v: &Vn, k: usize) -> Result<(), StdfErrorKind> {
    validate_kx_count(v.len(), k)?;
    for item in v {
        match item {
            V1::B0
            | V1::U1(_)
            | V1::U2(_)
            | V1::U4(_)
            | V1::I1(_)
            | V1::I2(_)
            | V1::I4(_)
            | V1::R4(_)
            | V1::R8(_) => {}
            V1::Cn(s) => validate_cn(s)?,
            V1::Bn(b) => validate_bn(b)?,
            V1::Dn(d) => validate_dn(d)?,
            V1::N1(n) => validate_n1(*n)?,
            V1::Invalid => return Err(StdfErrorKind::InvalidValue),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c1_and_n1_ranges() {
        assert!(validate_c1('A').is_ok());
        assert!(validate_c1('\u{FF}').is_ok());
        assert!(validate_c1('\u{100}').is_err());
        assert!(validate_n1(0).is_ok());
        assert!(validate_n1(0x0F).is_ok());
        assert!(validate_n1(0x10).is_err());
    }

    #[test]
    fn string_and_byte_caps() {
        assert!(validate_cn(&"x".repeat(u8::MAX as usize)).is_ok());
        assert!(validate_cn(&"x".repeat(u8::MAX as usize + 1)).is_err());
        assert!(validate_sn(&"x".repeat(u16::MAX as usize)).is_ok());
        assert!(validate_sn(&"x".repeat(u16::MAX as usize + 1)).is_err());
        assert!(validate_bn(&vec![0u8; u8::MAX as usize]).is_ok());
        assert!(validate_bn(&vec![0u8; u8::MAX as usize + 1]).is_err());
    }

    #[test]
    fn dn_bit_data_length() {
        assert!(validate_dn(&Dn {
            bit_count: 0,
            bit_data: vec![],
        })
        .is_ok());
        assert!(validate_dn(&Dn {
            bit_count: 1,
            bit_data: vec![0xFF],
        })
        .is_ok());
        assert!(validate_dn(&Dn {
            bit_count: 9,
            bit_data: vec![0xFF, 0x01],
        })
        .is_ok());
        assert!(validate_dn(&Dn {
            bit_count: 9,
            bit_data: vec![0xFF],
        })
        .is_err());
    }

    #[test]
    fn kx_counts_and_elements() {
        assert!(validate_kx_count(0, 0).is_ok());
        assert!(validate_kx_count(1, 1).is_ok());
        assert!(validate_kx_count(0, 1).is_err());

        assert!(validate_kx_n1(&vec![], 0).is_ok());
        assert!(validate_kx_n1(&vec![0x0F, 0x00], 2).is_ok());
        assert!(validate_kx_n1(&vec![0x10], 1).is_err());
        assert!(validate_kx_n1(&vec![0], 2).is_err());

        assert!(validate_kx_cn(&vec!["ab".to_string()], 1).is_ok());
        assert!(validate_kx_cn(&vec!["ab".to_string()], 2).is_err());

        assert!(validate_kx_sn(&vec!["ab".to_string()], 1).is_ok());
        assert!(validate_kx_sn(&vec!["ab".to_string()], 2).is_err());
    }

    #[test]
    fn kx_uf_width_and_cf_fixed_width() {
        assert!(validate_kx_uf(&KxUf::F1(vec![]), 0, 1).is_ok());
        assert!(validate_kx_uf(&KxUf::F2(vec![1, 2]), 2, 2).is_ok());
        assert!(validate_kx_uf(&KxUf::F2(vec![1, 2]), 2, 4).is_err());
        assert!(validate_kx_uf(&KxUf::F4(vec![1]), 1, 4).is_ok());
        assert!(validate_kx_uf(&KxUf::F8(vec![1]), 1, 8).is_ok());

        assert!(validate_kx_cf(&vec![], 0, 3).is_ok());
        assert!(validate_kx_cf(&vec!["abc".to_string()], 1, 3).is_ok());
        assert!(validate_kx_cf(&vec!["ab".to_string()], 1, 3).is_err());
    }

    #[test]
    fn vn_recursive_validation() {
        assert!(validate_vn(&vec![V1::B0, V1::U1(1), V1::U2(2), V1::U4(3)], 4,).is_ok());
        assert!(validate_vn(
            &vec![
                V1::Cn("x".to_string()),
                V1::Bn(vec![1]),
                V1::Dn(Dn {
                    bit_count: 8,
                    bit_data: vec![0xFF],
                }),
                V1::N1(0x0F),
            ],
            4,
        )
        .is_ok());
        assert!(validate_vn(&vec![V1::Invalid], 1).is_err());
        assert!(validate_vn(&vec![V1::N1(0x10)], 1).is_err());
        assert!(validate_vn(
            &vec![V1::Dn(Dn {
                bit_count: 8,
                bit_data: vec![],
            })],
            1,
        )
        .is_err());
        assert!(validate_vn(&vec![V1::U1(1)], 2).is_err());
    }
}
