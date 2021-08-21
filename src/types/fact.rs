// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{Database, Datom, DatomType, Entity, EntityResult, TransactionError, Value, EID};

/**
A fact which hasn't yet been converted to a [Datom] (or set of
[Datom]s)
*/
#[derive(Clone)]
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
    pub fn datom<'c, D: Database<'c>>(self, t: u64, db: &D) -> Result<Datom, TransactionError> {
        match self {
            Fact::Add(entity, attribute, value) => Ok(Datom {
                entity: entity.resolve(db)?,
                attribute: attribute.resolve(db)?,
                value,
                t,
                datom_type: DatomType::Addition,
            }),
            Fact::RetractValue(entity, attribute, value) => Ok(Datom {
                entity: entity.resolve(db)?,
                attribute: attribute.resolve(db)?,
                value,
                t,
                datom_type: DatomType::Retraction,
            }),
            Fact::Retract(entity, attribute) => {
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
