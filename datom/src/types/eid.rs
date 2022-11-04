// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::cmp::Ordering;

use crate::{builtin_idents, storage::Storage, Database, Datom, QueryError, Value, ID};

/**
An un-resolved entity [ID], which can be used to resolve entities by
[ident](crate::builtin_idents::IDENT) or
[unique](crate::builtin_idents::UNIQUE) attribute
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EID {
    /// A resolved entity [ID]
    Resolved(ID),
    /// Resolve an entity by its [ident](crate::builtin_idents::IDENT)
    Ident(String),
    /// Resolve an entity by a static
    /// [ident](crate::builtin_idents::IDENT)
    InternedIdent(&'static str),
    /// Resolve an entity by a [unique](crate::builtin_idents::UNIQUE)
    /// attribute
    Unique(
        /// [unique](crate::builtin_idents::UNIQUE) attribute
        Box<Self>,
        /// Value
        Value,
    ),
}

fn by_t(a: &Datom, b: &Datom) -> Ordering {
    a.t.cmp(&b.t)
}

impl EID {
    /**
    Create an [EID] to resolve an entity by a
    [unique](crate::builtin_idents::UNIQUE) attribute
    */
    pub fn unique(eid: Self, val: Value) -> Self {
        Self::Unique(eid.into(), val)
    }

    /// Resolve this [EID] into its [ID] according to a [Database]
    pub fn resolve<'c, S: Storage>(&self, db: &Database<'c, S>) -> Result<ID, QueryError> {
        match self {
            Self::Resolved(id) => Ok(*id),
            Self::Ident(ident_str) => {
                if let Some(entity) = builtin_idents::BUILTIN_ENTITIES_BY_IDENT.get(ident_str) {
                    if let Some(Value::ID(id)) = entity.get(&builtin_idents::ID) {
                        return Ok(id.to_owned());
                    }
                }
                let ident_val = Value::from(ident_str.as_str());
                db.datoms_for_attribute_value(builtin_idents::IDENT, ident_val)?
                    .max_by(by_t)
                    .map(|datom| datom.entity)
                    .ok_or_else(|| QueryError::UnresolvedEID(self.clone()))
            }
            Self::InternedIdent(ident_str) => Self::Ident(ident_str.to_string()).resolve(db),
            Self::Unique(attr_eid, val) => {
                let attr_id = attr_eid.resolve(db)?;
                db.datoms_for_attribute_value(attr_id, val.to_owned())?
                    .max_by(by_t)
                    .map(|datom| datom.entity)
                    .ok_or_else(|| QueryError::UnresolvedEID(self.clone()))
            }
        }
    }
}

impl From<ID> for EID {
    fn from(id: ID) -> Self {
        Self::Resolved(id)
    }
}

impl From<String> for EID {
    fn from(ident: String) -> Self {
        Self::Ident(ident)
    }
}

impl From<&'static str> for EID {
    fn from(ident: &'static str) -> Self {
        Self::InternedIdent(ident)
    }
}
