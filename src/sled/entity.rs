// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::HashSet;

use crate::{
    builtin_idents, Connection, Database, Datom, DatomType, Entity, EntityResult, QueryError,
    Value, EID, ID,
};

use super::{SledAttributeIter, SledConnection};

/// An [Entity] in a sled-backed database
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SledEntity<'connection> {
    pub(crate) connection: &'connection SledConnection,
    pub(crate) t: u64,
    pub(crate) id: ID,
}

impl<'connection> Entity for SledEntity<'connection> {
    type AttributeIter = SledAttributeIter;

    fn id(&self) -> &ID {
        &self.id
    }

    fn get_with_options(
        &self,
        attribute: EID,
        skip_cardinality: bool,
        skip_type: bool,
    ) -> Result<EntityResult<Self>, QueryError> {
        let db = self.connection.as_of(self.t)?;
        let attribute = attribute.resolve(&db)?;
        if attribute == builtin_idents::ID {
            return Ok(Value::from(self.id).into());
        }
        let attribute_ent = db.entity(attribute.into())?;
        let is_repeated = !skip_cardinality
            && attribute_ent
                .get_with_options(builtin_idents::CARDINALITY.into(), true, false)?
                .is_ref_to(&builtin_idents::CARDINALITY_MANY);
        let attribute_type = {
            if skip_type {
                None
            } else {
                let attribute_type = attribute_ent.get_with_options(
                    builtin_idents::VALUE_TYPE.into(),
                    true,
                    true,
                )?;
                if let EntityResult::Value(Value::ID(t)) = attribute_type {
                    Some(t)
                } else {
                    None
                }
            }
        };
        let result = if is_repeated {
            let datoms = db.datoms_for_entity_attribute(self.id, attribute)?;
            // The index is sorted in EAVT order, so for a given value
            // all additions and retractions will be in time-order.
            let mut values = HashSet::new();
            for datom in datoms {
                if datom.datom_type == DatomType::Retraction {
                    values.remove(&datom.value);
                } else {
                    values.insert(datom.value);
                }
            }
            let res: Result<Vec<EntityResult<Self>>, QueryError> = values
                .into_iter()
                .map(|v| {
                    if attribute_type == Some(builtin_idents::TYPE_REF) {
                        if let Value::ID(id) = v {
                            Ok(EntityResult::Ref(db.entity(id.into())?))
                        } else {
                            Ok(EntityResult::Value(v))
                        }
                    } else {
                        Ok(EntityResult::from(v))
                    }
                })
                .collect();
            EntityResult::Repeated(res?)
        } else {
            db.datoms_for_entity_attribute(self.id, attribute)?
                .max_by(|a, b| a.t.cmp(&b.t))
                .map(|x| -> Result<EntityResult<Self>, QueryError> {
                    if x.datom_type == DatomType::Retraction {
                        Ok(EntityResult::NotFound)
                    } else if attribute_type == Some(builtin_idents::TYPE_REF) {
                        if let Value::ID(id) = x.value {
                            Ok(EntityResult::Ref(db.entity(id.into())?))
                        } else {
                            Ok(EntityResult::Value(x.value))
                        }
                    } else {
                        Ok(EntityResult::Value(x.value))
                    }
                })
                .unwrap_or(Ok(EntityResult::NotFound))?
        };
        if let EntityResult::NotFound = result {
            let builtin = builtin_idents::BUILTIN_ENTITIES.get(&self.id);
            if let Some(builtin) = builtin {
                let val = builtin.get(&attribute);
                if let Some(val) = val {
                    Ok(val.to_owned().into())
                } else {
                    Ok(EntityResult::NotFound)
                }
            } else {
                Ok(EntityResult::NotFound)
            }
        } else {
            Ok(result)
        }
    }

    fn reverse_get(&self, attribute: EID) -> Result<EntityResult<Self>, QueryError> {
        let db = self.connection.as_of(self.t)?;
        let attribute = attribute.resolve(&db)?;
        let datoms = db.datoms_for_value_attribute(self.id().to_owned().into(), attribute)?;
        let datoms: Vec<Datom> = datoms.collect();
        // The index is sorted in AVET order, so for a given entity
        // all additions and retractions will be in time-order.
        let mut entities = HashSet::new();
        for datom in datoms {
            if datom.datom_type == DatomType::Retraction {
                entities.remove(&datom.entity);
            } else {
                entities.insert(datom.entity);
            }
        }
        let res: Result<Vec<EntityResult<Self>>, QueryError> = entities
            .into_iter()
            .map(|id| Ok(EntityResult::Ref(db.entity(id.into())?)))
            .collect();
        Ok(EntityResult::Repeated(res?))
    }

    fn attributes(&self) -> Result<Self::AttributeIter, QueryError> {
        let db = self.connection.as_of(self.t)?;
        Ok(SledAttributeIter::new(db.datoms_for_entity(self.id)?))
    }
}
