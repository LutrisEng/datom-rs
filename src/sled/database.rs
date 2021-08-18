// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::HashSet;

use crate::{
    builtin_idents,
    serial::{
        avet_attribute_range, deserialize_unknown, eavt_entity_attribute_range, eavt_entity_range,
    },
    Connection, Database, Datom, DatomType, Entity, Index, QueryError, Value, EID, ID,
};

use super::SledConnection;

/// A view of a sled-backed database
pub struct SledDatabase<'connection> {
    pub(crate) connection: &'connection SledConnection,
    pub(crate) t: u64,
}

/// An iterator over [Datom]s in a sled-backed database
pub struct SledDatomIter {
    iter: sled::Iter,
    t: u64,
    index: Index,
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
                    let (datom, index) = deserialize_unknown(bytes)?;
                    if index != self.index {
                        return None;
                    } else if datom.t <= self.t {
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
                    let (datom, index) = deserialize_unknown(bytes)?;
                    if index != self.index {
                        return None;
                    } else if datom.t <= self.t {
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
pub struct SledEntity<'connection> {
    pub(crate) connection: &'connection SledConnection,
    pub(crate) t: u64,
    pub(crate) id: ID,
}

impl<'connection> Entity for SledEntity<'connection> {
    type AttributeIter = SledAttributeIter;

    fn get_with_options(
        &self,
        attribute: EID,
        skip_cardinality: bool,
    ) -> Result<Option<Value>, QueryError> {
        let db = self.connection.as_of(self.t)?;
        let attribute = attribute.resolve(&db)?;
        let attribute_ent = db.entity(attribute.into())?;
        let is_repeated = !skip_cardinality
            && attribute_ent.get_with_options(builtin_idents::cardinality().into(), true)?
                == Some(Value::ID(builtin_idents::cardinality_many()));
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
            if values.is_empty() {
                None
            } else {
                Some(Value::Repeated(values))
            }
        } else {
            db.datoms_for_entity_attribute(self.id, attribute)?
                .max_by(|a, b| a.t.cmp(&b.t))
                .map(|x| {
                    if x.datom_type == DatomType::Retraction {
                        None
                    } else {
                        Some(x.value)
                    }
                })
                .flatten()
        };
        Ok(result.or_else(|| {
            // Get values for builtin entities
            let builtins = builtin_idents::get_builtin_entities();
            let builtin = builtins.get(&self.id)?;
            let val = builtin.get(&attribute)?;
            Some(val.to_owned())
        }))
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
            iter: self.connection.db.iter(),
            t: self.t,
            index,
        })
    }

    fn datoms_for_entity(&self, entity: ID) -> Result<Self::DatomIter, QueryError> {
        Ok(Self::DatomIter {
            iter: self.connection.db.range(eavt_entity_range(entity)),
            t: self.t,
            index: Index::EAVT,
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
            index: Index::EAVT,
        })
    }

    fn datoms_for_attribute(&self, attribute: ID) -> Result<Self::DatomIter, QueryError> {
        Ok(Self::DatomIter {
            iter: self.connection.db.range(avet_attribute_range(attribute)),
            t: self.t,
            index: Index::EAVT,
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
