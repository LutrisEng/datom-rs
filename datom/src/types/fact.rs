// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{
    storage::Storage, Database, Datom, DatomType, EntityResult, TransactionError, Value, EID,
};

/**
A fact which hasn't yet been converted to a [Datom] (or set of
[Datom]s)
*/
#[derive(Clone, Debug)]
pub enum Fact {
    /// Adding an attribute value to an entity
    Add(
        /// Entity ID
        EID,
        /// Attribute ID
        EID,
        /// Value
        Value,
    ),
    /// Retracting a specific attribute value from an entity
    RetractValue(
        /// Entity ID
        EID,
        /// Attribute ID
        EID,
        /// Value
        Value,
    ),
    /// Retracting an attribute from an entity, no matter its value
    Retract(
        /// Entity ID
        EID,
        /// Attribute ID
        EID,
    ),
}

impl Fact {
    /// Convert this [Fact] into a [Datom], given a [Database]
    pub fn datom<S: Storage>(
        self,
        t: u64,
        db: &Database<'_, S>,
    ) -> Result<Datom, TransactionError> {
        match self {
            Self::Add(entity, attribute, value) => Ok(Datom {
                entity: entity.resolve(db)?,
                attribute: attribute.resolve(db)?,
                value,
                t,
                datom_type: DatomType::Addition,
            }),
            Self::RetractValue(entity, attribute, value) => Ok(Datom {
                entity: entity.resolve(db)?,
                attribute: attribute.resolve(db)?,
                value,
                t,
                datom_type: DatomType::Retraction,
            }),
            Self::Retract(entity, attribute) => {
                let entity = entity.resolve(db)?;
                let attribute = attribute.resolve(db)?;
                let value = db.entity(entity.into())?.get(attribute.into())?;
                if let EntityResult::Value(value) = value {
                    Ok(Datom {
                        entity,
                        attribute,
                        value,
                        t,
                        datom_type: DatomType::Retraction,
                    })
                } else {
                    Err(TransactionError::FailedToRetractRepeatedAttribute(
                        entity, attribute,
                    ))
                }
            }
        }
    }
}
