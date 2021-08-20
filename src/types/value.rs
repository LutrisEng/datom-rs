// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{collections::HashSet, convert::TryInto, hash::Hash, str::FromStr};

use datom_bigdecimal::BigDecimal;
use num_bigint::BigInt;

use crate::ID;

/// An attribute value.
#[derive(Clone, Debug, Eq)]
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
    /**
    A collection of values. Cannot be transacted, only retrieved
    from a query or [Entity](crate::Entity)
    */
    Repeated(HashSet<Self>),
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
            Value::Repeated(_) => {
                vec![255]
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
                let e = i64::from_be_bytes(
                    bytes[0..0i64.to_be_bytes().len()]
                        .try_into()
                        .expect("not enough bytes"),
                );
                let i = BigInt::from_signed_bytes_be(&bytes[0i64.to_be_bytes().len()..]);
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

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Decimal(l0), Self::Decimal(r0)) => l0 == r0,
            (Self::ID(l0), Self::ID(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Repeated(l0), Self::Repeated(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Repeated(hs) => {
                state.write_u8(255);
                state.write(b"repeated");
                for item in hs {
                    item.hash(state);
                }
            }
            _ => {
                state.write(&self.bytes());
            }
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
        impl From<$T> for Value {
            #[inline]
            fn from(n: $T) -> Self {
                let str = n.to_string();
                Self::Decimal(BigDecimal::from_str(&str).unwrap())
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

impl From<&BigDecimal> for Value {
    #[inline]
    fn from(n: &BigDecimal) -> Self {
        Self::Decimal(n.to_owned())
    }
}

impl From<ID> for Value {
    #[inline]
    fn from(id: ID) -> Self {
        Self::ID(id)
    }
}

impl From<&ID> for Value {
    #[inline]
    fn from(id: &ID) -> Self {
        Self::ID(id.to_owned())
    }
}
