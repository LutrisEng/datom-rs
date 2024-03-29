// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::str::FromStr;

use datom_bigdecimal::BigDecimal;
use edn_rs::Edn;
use num_bigint::BigInt;

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

    /// Create a fact from an EDN fact representation
    pub fn from_edn(edn: Edn) -> Result<Self, Box<dyn std::error::Error>> {
        let Edn::Vector(parts) = edn else {
            todo!("error");
        };
        let vec = parts.to_vec();
        if vec.len() != 3 {
            todo!("error");
        }
        let mut it = vec.into_iter();

        let entity_edn = it.next();
        let attribute_edn = it.next();
        let value_edn = it.next();

        let Some(Edn::Key(entity_keyword)) = entity_edn else {
            todo!("error");
        };
        let Some(Edn::Key(attribute_keyword)) = attribute_edn else {
            todo!("error");
        };

        let entity = EID::Ident(entity_keyword);
        let attribute = EID::Ident(attribute_keyword);

        let value = match value_edn {
            Some(Edn::Str(s)) => Value::String(s),
            Some(Edn::Int(i)) => Value::Integer(BigInt::from(i)),
            Some(Edn::Double(d)) => Value::Decimal(BigDecimal::from_str(&d.to_string())?),
            Some(Edn::Bool(b)) => Value::Boolean(b),
            _ => todo!("error"),
        };

        Ok(Self::Add(entity, attribute, value))
    }

    /// Generate an EDN representation from a fact
    pub fn to_edn(&self) -> String {
        let Self::Add(entity_eid, attribute_eid, value) = self else {
            todo!("error");
        };
        let EID::Ident(entity) = entity_eid else {
            todo!("error");
        };
        let EID::Ident(attribute) = attribute_eid else {
            todo!("error");
        };
        Edn::Vector(edn_rs::Vector::new(vec![
            Edn::Key(entity.to_string()),
            Edn::Key(attribute.to_string()),
            value.to_owned().into_edn(),
        ]))
        .to_string()
    }
}
