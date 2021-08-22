// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{Datom, Entity, Index, QueryError, Value, EID, ID};

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
    /// Get all [datoms](crate::Datom) in the
    /// [VAET index](crate::Index::VAET) for the given value and
    /// attribute
    fn datoms_for_value_attribute(
        &self,
        value: Value,
        attribute: ID,
    ) -> Result<Self::DatomIter, QueryError>;
    /// Get an entity
    fn entity(&self, entity: EID) -> Result<Self::Entity, QueryError>;
}