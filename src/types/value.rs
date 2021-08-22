// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{hash::Hash, str::FromStr};

use datom_bigdecimal::{BigDecimal, ParseBigDecimalError};
use num_bigint::BigInt;

use crate::ID;

/// An attribute value.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    /// A basic string
    String(String),
    /// A basic integer, of arbitrary length
    Integer(BigInt),
    /// A basic decimal, of arbitrary length and precision
    Decimal(BigDecimal),
    /// An entity [ID]
    ID(ID),
    /// A basic boolean
    Boolean(bool),
}

impl Value {
    /// Serialize the [Value] to a [Vec<u8>]
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Value::String(str) => {
                let str_bytes = str.as_bytes();
                let mut v = vec![0; 1 + str_bytes.len()];
                v[0] = 0;
                (&mut v[1..str_bytes.len() + 1]).copy_from_slice(str_bytes);
                v
            }
            Value::Integer(int) => {
                let int_bytes = int.to_signed_bytes_be();
                let mut v = vec![0; 1 + int_bytes.len()];
                v[0] = 1;
                (&mut v[1..int_bytes.len() + 1]).copy_from_slice(&int_bytes);
                v
            }
            Value::Decimal(dec) => {
                let (i, e) = dec.as_bigint_and_exponent();
                let e_bytes = e.to_be_bytes();
                let i_bytes = i.to_signed_bytes_be();
                let mut v = vec![0; 1 + e_bytes.len() + i_bytes.len()];
                v[0] = 2;
                (&mut v[1..e_bytes.len() + 1]).copy_from_slice(&e_bytes);
                (&mut v[e_bytes.len() + 1..e_bytes.len() + i_bytes.len() + 1])
                    .copy_from_slice(&i_bytes);
                v
            }
            Value::ID(id) => {
                let id_bytes: [u8; 16] = id.into();
                let mut v = vec![0; 1 + id_bytes.len()];
                v[0] = 3;
                (&mut v[1..id_bytes.len() + 1]).copy_from_slice(&id_bytes);
                v
            }
            Value::Boolean(b) => {
                let byte = if *b { 1 } else { 0 };
                vec![4, byte]
            }
        }
    }

    /// Deserialize the [Value] from a byte slice
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let valtype = bytes[0];
        match valtype {
            0 => {
                let str = String::from_utf8_lossy(&bytes[1..]);
                Some(Self::String(str.to_string()))
            }
            1 => {
                let int = BigInt::from_signed_bytes_be(&bytes[1..]);
                Some(Self::Integer(int))
            }
            2 => {
                let scale_len = 0i64.to_be_bytes().len();
                if bytes.len() < scale_len + 1 {
                    return None;
                }
                let e = i64::from_be_bytes(bytes[1..scale_len + 1].try_into().ok()?);
                let i = BigInt::from_signed_bytes_be(&bytes[scale_len + 1..]);
                let dec = BigDecimal::new(i, e);
                Some(Self::Decimal(dec))
            }
            3 => {
                if bytes.len() != 17 {
                    return None;
                }
                let bytes: [u8; 16] = bytes[1..17].try_into().ok()?;
                Some(Self::ID(bytes.into()))
            }
            4 => Some(Self::Boolean(if bytes.len() != 2 {
                None
            } else {
                match bytes[1] {
                    0 => Some(false),
                    1 => Some(true),
                    _ => None,
                }
            }?)),
            _ => None,
        }
    }
}

impl From<String> for Value {
    fn from(str: String) -> Self {
        Self::String(str)
    }
}

impl From<&str> for Value {
    fn from(str: &str) -> Self {
        Self::String(str.to_owned())
    }
}

macro_rules! impl_value_from_int {
    ($T:ty) => {
        impl From<$T> for Value {
            #[inline]
            fn from(n: $T) -> Self {
                Self::Integer(BigInt::from(n))
            }
        }
    };
}

impl_value_from_int!(u8);
impl_value_from_int!(u16);
impl_value_from_int!(u32);
impl_value_from_int!(u64);
impl_value_from_int!(u128);
impl_value_from_int!(usize);

impl_value_from_int!(i8);
impl_value_from_int!(i16);
impl_value_from_int!(i32);
impl_value_from_int!(i64);
impl_value_from_int!(i128);
impl_value_from_int!(isize);

impl From<BigInt> for Value {
    #[inline]
    fn from(n: BigInt) -> Self {
        Self::Integer(n)
    }
}

impl From<&BigInt> for Value {
    #[inline]
    fn from(n: &BigInt) -> Self {
        Self::Integer(n.to_owned())
    }
}

macro_rules! impl_value_from_float {
    ($T:ty) => {
        impl TryFrom<$T> for Value {
            type Error = ParseBigDecimalError;
            #[inline]
            fn try_from(n: $T) -> Result<Self, Self::Error> {
                let str = n.to_string();
                Ok(Self::Decimal(BigDecimal::from_str(&str)?))
            }
        }
    };
}

impl_value_from_float!(f32);
impl_value_from_float!(f64);

impl From<BigDecimal> for Value {
    #[inline]
    fn from(n: BigDecimal) -> Self {
        Self::Decimal(n)
    }
}

impl From<ID> for Value {
    #[inline]
    fn from(id: ID) -> Self {
        Self::ID(id)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

#[cfg(test)]
mod tests {
    use crate::builtin_idents;

    use super::*;

    fn test(val: Value, constant: Option<Vec<u8>>) {
        // Ensure serialization is consistent
        assert_eq!(val.bytes(), val.bytes());
        // Ensure round-trip serialization/deserialization is exact
        assert_eq!(Value::from_bytes(&val.bytes()), Some(val.clone()));
        // Ensure serialization is stable
        if let Some(constant) = constant {
            assert_eq!(val.bytes(), constant);
        }
    }

    fn test_failure(bytes: &[u8]) {
        // Ensure deserialization fails
        assert_eq!(Value::from_bytes(bytes), None);
    }

    #[test]
    fn serialize_string() {
        test("".into(), Some(vec![0]));
        test(
            "Hello, world! This is a string.".into(),
            Some(vec![
                0, 72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33, 32, 84, 104, 105,
                115, 32, 105, 115, 32, 97, 32, 115, 116, 114, 105, 110, 103, 46,
            ]),
        );
        // No way I'm inlining the bytes for this
        test(include_str!("lipsum.txt").into(), None);
    }

    #[test]
    fn serialize_integer() {
        test(Value::from(0), Some(vec![1, 0]));
        test(Value::from(u8::MAX), Some(vec![1, 0, 255]));
        test(Value::from(u16::MAX), Some(vec![1, 0, 255, 255]));
        test(Value::from(u32::MAX), Some(vec![1, 0, 255, 255, 255, 255]));
        test(
            Value::from(u64::MAX),
            Some(vec![1, 0, 255, 255, 255, 255, 255, 255, 255, 255]),
        );
        test(
            Value::from(u128::MAX),
            Some(vec![
                1, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                255,
            ]),
        );
        test(Value::from(i8::MAX), Some(vec![1, 127]));
        test(Value::from(i16::MAX), Some(vec![1, 127, 255]));
        test(Value::from(i32::MAX), Some(vec![1, 127, 255, 255, 255]));
        test(
            Value::from(i64::MAX),
            Some(vec![1, 127, 255, 255, 255, 255, 255, 255, 255]),
        );
        test(
            Value::from(i128::MAX),
            Some(vec![
                1, 127, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            ]),
        );
        test(
            Value::from(BigInt::from(u128::MAX) + BigInt::from(u128::MAX)),
            Some(vec![
                1, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                254,
            ]),
        );
        test(
            Value::from((BigInt::from(u128::MAX) + BigInt::from(u128::MAX)) * -1),
            Some(vec![1, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2]),
        );
        test(
            Value::from(&(BigInt::from(u128::MAX) + BigInt::from(u128::MAX))),
            Some(vec![
                1, 1, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
                254,
            ]),
        );
    }

    #[test]
    fn serialize_decimal() {
        test(
            0.0.try_into().unwrap(),
            Some(vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        );
        test(
            (1.0 / 10.0 * 3.0).try_into().unwrap(),
            Some(vec![
                2, 0, 0, 0, 0, 0, 0, 0, 17, 106, 148, 215, 79, 67, 0, 4,
            ]),
        );
        test(
            BigDecimal::from(0).into(),
            Some(vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        );
        test_failure(&[2]);
        // PRs welcome for more test cases
    }

    #[test]
    fn serialize_id() {
        test(
            ID::null().into(),
            Some(vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        );
        test(
            builtin_idents::ID.into(),
            Some(vec![
                3, 248, 66, 64, 237, 25, 19, 74, 89, 129, 129, 207, 7, 40, 63, 192, 169,
            ]),
        );
        test(
            builtin_idents::IDENT.into(),
            Some(vec![
                3, 199, 224, 136, 63, 185, 11, 73, 49, 143, 245, 143, 95, 63, 236, 252, 202,
            ]),
        );
        test(
            builtin_idents::TYPE_ID.into(),
            Some(vec![
                3, 255, 140, 71, 76, 66, 79, 67, 230, 140, 184, 99, 9, 96, 8, 136, 79,
            ]),
        );
        test(ID::new().into(), None);
        test_failure(&[3]);
        test_failure(&[3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn serialize_bool() {
        // This one's easy
        test(true.into(), Some(vec![4, 1]));
        test(false.into(), Some(vec![4, 0]));
        test_failure(&[4, 3]);
        test_failure(&[4]);
        test_failure(&[4, 3, 0]);
        test_failure(&[4, 0, 0]);
    }

    #[test]
    fn serialize_invalid() {
        test_failure(&[5]);
        test_failure(&[255]);
    }
}
