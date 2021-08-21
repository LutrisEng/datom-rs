// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{
    serial::{
        avet_attribute_range, avet_attribute_value_range, eavt_entity_attribute_range,
        eavt_entity_range, index_range, vaet_value_attribute_range,
    },
    Database, Index, QueryError, Value, EID, ID,
};

use super::{SledConnection, SledDatomIter, SledEntity};

/// A view of a sled-backed database
#[derive(Debug)]
pub struct SledDatabase<'connection> {
    pub(crate) connection: &'connection SledConnection,
    pub(crate) t: u64,
}

impl<'connection> Database<'connection> for SledDatabase<'connection> {
    type DatomIter = SledDatomIter;
    type Entity = SledEntity<'connection>;

    fn datoms(&self, index: Index) -> Result<Self::DatomIter, QueryError> {
        Ok(SledDatomIter::new(
            self.connection.db.range(index_range(index)),
            self.t,
        ))
    }

    fn datoms_for_entity(&self, entity: ID) -> Result<Self::DatomIter, QueryError> {
        Ok(SledDatomIter::new(
            self.connection.db.range(eavt_entity_range(entity)),
            self.t,
        ))
    }

    fn datoms_for_entity_attribute(
        &self,
        entity: ID,
        attribute: ID,
    ) -> Result<Self::DatomIter, QueryError> {
        Ok(SledDatomIter::new(
            self.connection
                .db
                .range(eavt_entity_attribute_range(entity, attribute)),
            self.t,
        ))
    }

    fn datoms_for_attribute(&self, attribute: ID) -> Result<Self::DatomIter, QueryError> {
        Ok(SledDatomIter::new(
            self.connection.db.range(avet_attribute_range(attribute)),
            self.t,
        ))
    }

    fn datoms_for_attribute_value(
        &self,
        attribute: ID,
        value: Value,
    ) -> Result<Self::DatomIter, QueryError> {
        Ok(SledDatomIter::new(
            self.connection
                .db
                .range(avet_attribute_value_range(attribute, value)),
            self.t,
        ))
    }

    fn datoms_for_value_attribute(
        &self,
        value: Value,
        attribute: ID,
    ) -> Result<Self::DatomIter, QueryError> {
        Ok(SledDatomIter::new(
            self.connection
                .db
                .range(vaet_value_attribute_range(value, attribute)),
            self.t,
        ))
    }

    fn entity(&self, entity: EID) -> Result<Self::Entity, QueryError> {
        let entity = entity.resolve(self)?;
        Ok(Self::Entity {
            connection: self.connection,
            t: self.t,
            id: entity,
        })
    }
}

/// C bindings
#[cfg(feature = "c")]
pub mod c {
    use super::SledDatabase;

    /// Destroy a connection to a sled-backed database view
    ///
    /// # Safety
    ///
    /// db must be a valid, non-null [SledDatabase] created by
    /// [datom_sled_db](crate::c::datom_sled_db).
    #[no_mangle]
    pub unsafe extern "C" fn datom_sled_db_destroy(db: *mut SledDatabase) {
        Box::from_raw(db);
    }
}
