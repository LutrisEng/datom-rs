// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{
    fmt::{self, Debug, Formatter},
    hash::Hash,
};

use crate::{storage::Storage, Entity, Value, ID};

#[derive(Clone)]
/// The result of getting an attribute on an entity
pub enum EntityResult<'connection, S: Storage> {
    /// A value for that attribute wasn't found on this entity
    NotFound,
    /// A value for that attribute was found on this entity
    Value(Value),
    /// A value for that attribute was found on this entity, and it
    /// refers to another entity.
    Ref(Entity<'connection, S>),
    /// Possibly multiple values for that attribute were found on this
    /// entity
    Repeated(Vec<Self>),
}

impl<'connection, S: Storage> Debug for EntityResult<'connection, S> {
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

impl<'connection, S: Storage> EntityResult<'connection, S> {
    /// Check if the result is a reference to a specific ID
    pub fn is_ref_to(&self, id: &ID) -> bool {
        if let Self::Ref(e) = self {
            e.id() == id
        } else {
            false
        }
    }
}

impl<'connection, S: Storage> PartialEq<Self> for EntityResult<'connection, S> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::Ref(l0), Self::Ref(r0)) => l0 == r0,
            (Self::Repeated(l0), Self::Repeated(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'connection, S: Storage> Eq for EntityResult<'connection, S> {}

impl<'connection, S: Storage> Hash for EntityResult<'connection, S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl<'connection, S: Storage> PartialEq<Value> for EntityResult<'connection, S> {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Self::Value(s) => s == other,
            _ => false,
        }
    }
}

impl<'connection, S: Storage + PartialEq> PartialEq<Entity<'connection, S>>
    for EntityResult<'connection, S>
{
    fn eq(&self, other: &Entity<'connection, S>) -> bool {
        match self {
            Self::Ref(s) => s == other,
            _ => false,
        }
    }
}

impl<'connection, S: Storage> PartialEq<ID> for EntityResult<'connection, S> {
    fn eq(&self, other: &ID) -> bool {
        self.is_ref_to(other)
    }
}

impl<'connection, S: Storage + PartialEq> PartialEq<Vec<Self>> for EntityResult<'connection, S> {
    fn eq(&self, other: &Vec<Self>) -> bool {
        match self {
            Self::Repeated(s) => s == other,
            _ => false,
        }
    }
}

impl<'connection, S: Storage> From<Value> for EntityResult<'connection, S> {
    fn from(v: Value) -> Self {
        Self::Value(v)
    }
}

impl<'connection, S: Storage> From<Entity<'connection, S>> for EntityResult<'connection, S> {
    fn from(e: Entity<'connection, S>) -> Self {
        Self::Ref(e)
    }
}
