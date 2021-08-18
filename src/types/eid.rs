// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{builtin_idents, Database, Index, QueryError, Value, ID};

/**
An un-resolved entity [ID], which can be used to resolve entities by
[ident](crate::builtin_idents::ident) or
[unique](crate::builtin_idents::unique) attribute
*/
#[derive(Clone, Debug)]
pub enum EID {
    /// A resolved entity [ID]
    Resolved(ID),
    /// Resolve an entity by its [ident](crate::builtin_idents::ident)
    Ident(String),
    /**
    Resolve an entity by a [unique](crate::builtin_idents::unique)
    attribute
    */
    Unique(
        /// [unique](crate::builtin_idents::unique) attribute
        Box<EID>,
        /// Value
        Value,
    ),
}

impl EID {
    /**
    Create an [EID] to resolve an entity by a
    [unique](crate::builtin_idents::unique) attribute
    */
    pub fn unique(eid: Self, val: Value) -> Self {
        Self::Unique(eid.into(), val)
    }

    /// Resolve this [EID] into its [ID] according to a [Database]
    pub fn resolve<'c, D: Database<'c>>(&self, db: &D) -> Result<ID, QueryError> {
        match self {
            Self::Resolved(id) => Ok(*id),
            Self::Ident(ident_str) => {
                if let Some(id) = builtin_idents::builtin_by_ident(ident_str) {
                    return Ok(id);
                }
                let ident = builtin_idents::ident();
                let ident_val = Value::from(ident_str.as_str());
                db.datoms(Index::VAET)?
                    .find(|datom| datom.attribute == ident && datom.value == ident_val)
                    .map(|datom| datom.entity)
                    .ok_or_else(|| QueryError::UnresolvedEID(self.clone()))
            }
            Self::Unique(attr_eid, val) => {
                let attr_id = attr_eid.resolve(db)?;
                db.datoms_for_attribute(attr_id)?
                    .find(|datom| &datom.value == val)
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
