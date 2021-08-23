// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{
    fmt::{self, Debug, Formatter},
    hash::Hash,
};

use crate::{Entity, Value, ID};

#[derive(Clone, PartialEq, Eq, Hash)]
/// The result of getting an attribute on an entity
pub enum EntityResult<E: Entity> {
    /// A value for that attribute wasn't found on this entity
    NotFound,
    /// A value for that attribute was found on this entity
    Value(Value),
    /// A value for that attribute was found on this entity, and it
    /// refers to another entity.
    Ref(E),
    /// Possibly multiple values for that attribute were found on this
    /// entity
    Repeated(Vec<Self>),
}

impl<E: Entity> Debug for EntityResult<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "NotFound")?,
            Self::Value(v) => {
                write!(f, "Value(")?;
                v.fmt(f)?;
                write!(f, ")")?;
            }
            Self::Ref(e) => {
                write!(f, "Ref(")?;
                e.id().fmt(f)?;
                write!(f, ")")?;
            }
            Self::Repeated(xs) => {
                write!(f, "Repeated(")?;
                for x in xs {
                    x.fmt(f)?;
                    write!(f, ", ")?;
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

impl<E: Entity> EntityResult<E> {
    /// Check if the result is a reference to a specific ID
    pub fn is_ref_to(&self, id: &ID) -> bool {
        if let Self::Ref(e) = self {
            e.id() == id
        } else {
            false
        }
    }
}

impl<E: Entity> PartialEq<Value> for EntityResult<E> {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Self::Value(s) => s == other,
            _ => false,
        }
    }
}

impl<E: Entity> PartialEq<E> for EntityResult<E> {
    fn eq(&self, other: &E) -> bool {
        match self {
            Self::Ref(s) => s == other,
            _ => false,
        }
    }
}

impl<E: Entity> PartialEq<ID> for EntityResult<E> {
    fn eq(&self, other: &ID) -> bool {
        self.is_ref_to(other)
    }
}

impl<E: Entity> PartialEq<Vec<Self>> for EntityResult<E> {
    fn eq(&self, other: &Vec<Self>) -> bool {
        match self {
            Self::Repeated(s) => s == other,
            _ => false,
        }
    }
}

impl<E: Entity> From<Value> for EntityResult<E> {
    fn from(v: Value) -> Self {
        Self::Value(v)
    }
}

impl<E: Entity> From<E> for EntityResult<E> {
    fn from(e: E) -> Self {
        Self::Ref(e)
    }
}
