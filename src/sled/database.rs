// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::HashSet;

use crate::{
    builtin_idents,
    serial::{
        avet_attribute_range, avet_attribute_value_range, deserialize_unknown,
        eavt_entity_attribute_range, eavt_entity_range, index_range,
    },
    Connection, Database, Datom, DatomType, Entity, EntityResult, Index, QueryError, Value, EID,
    ID,
};

use super::SledConnection;

/// A view of a sled-backed database
#[derive(Debug)]
pub struct SledDatabase<'connection> {
    pub(crate) connection: &'connection SledConnection,
    pub(crate) t: u64,
}

/// An iterator over [Datom]s in a sled-backed database
pub struct SledDatomIter {
    iter: sled::Iter,
    t: u64,
}

impl Iterator for SledDatomIter {
    type Item = Datom;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(Err(_)) => continue,
                Some(Ok((k, _))) => {
                    let bytes: &[u8] = &k;
                    let (datom, _) = deserialize_unknown(bytes)?;
                    if datom.t <= self.t {
                        return Some(datom);
                    }
                }
            }
        }
    }
}

impl DoubleEndedIterator for SledDatomIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next_back() {
                None => return None,
                Some(Err(_)) => continue,
                Some(Ok((k, _))) => {
                    let bytes: &[u8] = &k;
                    let (datom, _) = deserialize_unknown(bytes)?;
                    if datom.t <= self.t {
                        return Some(datom);
                    }
                }
            }
        }
    }
}

/// An iterator over attributes in a sled-backed database
pub struct SledAttributeIter {
    iter: SledDatomIter,
    seen: HashSet<ID>,
}

impl Iterator for SledAttributeIter {
    type Item = ID;

    fn next(&mut self) -> Option<Self::Item> {
        for datom in (&mut self.iter).rev() {
            let attr = datom.attribute;
            if !self.seen.contains(&attr) {
                self.seen.insert(attr);
                if datom.datom_type == DatomType::Addition {
                    return Some(attr);
                }
            }
        }
        None
    }
}

/// An [Entity] in a sled-backed database
#[derive(Debug, PartialEq)]
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
        if attribute == builtin_idents::id() {
            return Ok(Value::from(self.id).into());
        }
        let attribute_ent = db.entity(attribute.into())?;
        let is_repeated = !skip_cardinality
            && attribute_ent
                .get_with_options(builtin_idents::cardinality().into(), true, true)?
                .is_ref_to(&builtin_idents::cardinality_many());
        let attribute_type = {
            if skip_type {
                None
            } else {
                let attribute_type = attribute_ent.get_with_options(
                    builtin_idents::value_type().into(),
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
            EntityResult::Repeated(values.into_iter().map(EntityResult::from).collect())
        } else {
            db.datoms_for_entity_attribute(self.id, attribute)?
                .max_by(|a, b| a.t.cmp(&b.t))
                .map(|x| -> Result<EntityResult<Self>, QueryError> {
                    if x.datom_type == DatomType::Retraction {
                        Ok(EntityResult::NotFound)
                    } else if attribute_type == Some(builtin_idents::type_ref()) {
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
            let builtins = builtin_idents::get_builtin_entities();
            let builtin = builtins.get(&self.id);
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

    fn reverse_get_with_options(
        &self,
        _: EID,
        _: bool,
        _: bool,
    ) -> Result<EntityResult<Self>, QueryError> {
        todo!()
    }

    fn attributes(&self) -> Result<Self::AttributeIter, QueryError> {
        let db = self.connection.as_of(self.t)?;
        Ok(Self::AttributeIter {
            iter: db.datoms_for_entity(self.id)?,
            seen: HashSet::new(),
        })
    }
}

impl<'connection> Database<'connection> for SledDatabase<'connection> {
    type DatomIter = SledDatomIter;
    type Entity = SledEntity<'connection>;

    fn datoms(&self, index: Index) -> Result<Self::DatomIter, QueryError> {
        Ok(Self::DatomIter {
            iter: self.connection.db.range(index_range(index)),
            t: self.t,
        })
    }

    fn datoms_for_entity(&self, entity: ID) -> Result<Self::DatomIter, QueryError> {
        Ok(Self::DatomIter {
            iter: self.connection.db.range(eavt_entity_range(entity)),
            t: self.t,
        })
    }

    fn datoms_for_entity_attribute(
        &self,
        entity: ID,
        attribute: ID,
    ) -> Result<Self::DatomIter, QueryError> {
        Ok(Self::DatomIter {
            iter: self
                .connection
                .db
                .range(eavt_entity_attribute_range(entity, attribute)),
            t: self.t,
        })
    }

    fn datoms_for_attribute(&self, attribute: ID) -> Result<Self::DatomIter, QueryError> {
        Ok(Self::DatomIter {
            iter: self.connection.db.range(avet_attribute_range(attribute)),
            t: self.t,
        })
    }

    fn datoms_for_attribute_value(
        &self,
        attribute: ID,
        value: Value,
    ) -> Result<Self::DatomIter, QueryError> {
        Ok(Self::DatomIter {
            iter: self
                .connection
                .db
                .range(avet_attribute_value_range(attribute, value)),
            t: self.t,
        })
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
