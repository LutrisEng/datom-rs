// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{
    serial::{
        avet_attribute_value_range, eavt_entity_attribute_range, eavt_entity_range, index_range,
        range_slice, vaet_value_attribute_range, vec_range_slice,
    },
    storage::Storage,
    Connection, DatomIterator, Entity, Index, QueryError, Value, EID, ID,
};

/// A view of a database at a specific point in time
#[derive(Debug)]
pub struct Database<'connection, S: Storage> {
    pub(crate) connection: &'connection Connection<S>,
    pub(crate) t: u64,
}

impl<'connection, S: Storage> Database<'connection, S> {
    /// Get all [datoms](crate::Datom) in the given index
    pub fn datoms(&self, index: Index) -> Result<DatomIterator<'connection>, QueryError> {
        Ok(DatomIterator::new(
            self.connection
                .storage
                .range(range_slice(&index_range(index)))?,
            self.t,
        ))
    }

    /// Get all [datoms](crate::Datom) in the
    /// [EAVT index](crate::Index::EAVT) for the given entity
    pub fn datoms_for_entity(&self, entity: ID) -> Result<DatomIterator<'connection>, QueryError> {
        Ok(DatomIterator::new(
            self.connection
                .storage
                .range(range_slice(&eavt_entity_range(entity)))?,
            self.t,
        ))
    }

    /// Get all [datoms](crate::Datom) in the
    /// [EAVT index](crate::Index::EAVT) for the given entity and
    /// attribute
    pub fn datoms_for_entity_attribute(
        &self,
        entity: ID,
        attribute: ID,
    ) -> Result<DatomIterator<'connection>, QueryError> {
        Ok(DatomIterator::new(
            self.connection
                .storage
                .range(range_slice(&eavt_entity_attribute_range(entity, attribute)))?,
            self.t,
        ))
    }

    /// Get all [datoms](crate::Datom) in the
    /// [AVET index](crate::Index::AVET) for the given attribute and
    /// value
    pub fn datoms_for_attribute_value(
        &self,
        attribute: ID,
        value: Value,
    ) -> Result<DatomIterator<'connection>, QueryError> {
        Ok(DatomIterator::new(
            self.connection
                .storage
                .range(vec_range_slice(&avet_attribute_value_range(
                    attribute, value,
                )))?,
            self.t,
        ))
    }

    /// Get all [datoms](crate::Datom) in the
    /// [VAET index](crate::Index::VAET) for the given value and
    /// attribute
    pub fn datoms_for_value_attribute(
        &self,
        value: Value,
        attribute: ID,
    ) -> Result<DatomIterator<'connection>, QueryError> {
        Ok(DatomIterator::new(
            self.connection
                .storage
                .range(vec_range_slice(&vaet_value_attribute_range(
                    value, attribute,
                )))?,
            self.t,
        ))
    }

    /// Get an entity
    pub fn entity(&self, entity: EID) -> Result<Entity<'connection, S>, QueryError> {
        let entity = entity.resolve(self)?;
        Ok(Entity {
            connection: self.connection,
            t: self.t,
            id: entity,
        })
    }
}
