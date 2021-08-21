// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::fmt::Debug;

use crate::{Datom, Index, QueryError, Value, EID, ID};

#[derive(Clone, PartialEq, Eq)]
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// An entity at a single point in time
pub trait Entity: Sized + PartialEq + Clone {
    /// An iterator over an entity's attributes
    type AttributeIter: Iterator<Item = ID>;

    /// Get the ID of this entity
    fn id(&self) -> &ID;

    /// Get the value of an attribute on this entity, with options
    fn get_with_options(
        &self,
        attribute: EID,
        skip_cardinality: bool,
        skip_type: bool,
    ) -> Result<EntityResult<Self>, QueryError>;
    /// Get the value of an attribute on this entity
    fn get(&self, attribute: EID) -> Result<EntityResult<Self>, QueryError> {
        self.get_with_options(attribute, false, false)
    }
    /// Get the entities with this entity as a value on an attribute
    /// (reverse lookup), with options
    fn reverse_get_with_options(
        &self,
        attribute: EID,
        skip_cardinality: bool,
        skip_type: bool,
    ) -> Result<EntityResult<Self>, QueryError>;
    /// Get the entities with this entity as a value on an attribute
    /// (reverse lookup)
    fn reverse_get(&self, attribute: EID) -> Result<EntityResult<Self>, QueryError> {
        self.reverse_get_with_options(attribute, false, false)
    }
    /// Get the attributes on this entity
    fn attributes(&self) -> Result<Self::AttributeIter, QueryError>;
}

/**
A snapshot of the database at a single point in time. That point in
time is referred to as the database's _basis-t_.
*/
pub trait Database<'connection> {
    /// An iterator over a database's datoms
    type DatomIter: Iterator<Item = Datom>;
    /// A single entity
    type Entity: Entity;

    /// Get all [datoms](crate::Datom) in the given index
    fn datoms(&self, index: Index) -> Result<Self::DatomIter, QueryError>;
    /// Get all [datoms](crate::Datom) in the
    /// [EAVT index](crate::Index::EAVT) for the given entity
    fn datoms_for_entity(&self, entity: ID) -> Result<Self::DatomIter, QueryError>;
    /// Get all [datoms](crate::Datom) in the
    /// [EAVT index](crate::Index::EAVT) for the given entity and attribute
    fn datoms_for_entity_attribute(
        &self,
        entity: ID,
        attribute: ID,
    ) -> Result<Self::DatomIter, QueryError>;
    /// Get all [datoms](crate::Datom) in the
    /// [AVET index](crate::Index::AVET) for the given attribute
    fn datoms_for_attribute(&self, attribute: ID) -> Result<Self::DatomIter, QueryError>;
    /// Get all [datoms](crate::Datom) in the
    /// [AVET index](crate::Index::AVET) for the given attribute and
    /// value
    fn datoms_for_attribute_value(
        &self,
        attribute: ID,
        value: Value,
    ) -> Result<Self::DatomIter, QueryError>;
    /// Get an entity
    fn entity(&self, entity: EID) -> Result<Self::Entity, QueryError>;
}
