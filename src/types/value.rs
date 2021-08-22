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
                let e = i64::from_be_bytes(
                    bytes[1..scale_len + 1]
                        .try_into()
                        .expect("not enough bytes"),
                );
                let i = BigInt::from_signed_bytes_be(&bytes[scale_len + 1..]);
                let dec = BigDecimal::new(i, e);
                Some(Self::Decimal(dec))
            }
            3 => {
                let bytes: [u8; 16] = bytes[1..17].try_into().expect("not enough bytes");
                Some(Self::ID(bytes.into()))
            }
            4 => Some(Self::Boolean(match bytes[1] {
                0 => Some(false),
                1 => Some(true),
                _ => None,
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
