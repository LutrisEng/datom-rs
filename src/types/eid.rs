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

/// C bindings
pub mod c {
    use std::{ffi::CStr, os::raw::c_char};

    use crate::{Value, EID, ID};

    /// Create an EID object from a 16-byte ID.
    #[no_mangle]
    pub extern "C" fn datom_eid_id(id: &[u8; 16]) -> Box<EID> {
        Box::new(EID::Resolved(ID::from(*id)))
    }

    /// Create an EID object from a string ident.
    ///
    /// # Safety
    ///
    /// ident must be a NULL-terminated string.
    #[no_mangle]
    pub unsafe extern "C" fn datom_eid_ident(ident: *const c_char) -> Option<Box<EID>> {
        let ident_cstr = CStr::from_ptr(ident);
        let ident_str = ident_cstr.to_str();
        match ident_str {
            Ok(ident_str) => Some(Box::new(EID::Ident(ident_str.to_owned()))),
            Err(_) => None,
        }
    }

    /// Create an EID object from an attribute and value. Consumes attr
    /// and value.
    #[no_mangle]
    pub extern "C" fn datom_eid_unique(attr: Box<EID>, value: Box<Value>) -> Box<EID> {
        Box::new(EID::Unique(attr, *value))
    }

    /// Destroy an EID object which wasn't consumed
    ///
    /// # Safety
    ///
    /// eid must be a valid, non-null [EID] created by [datom_eid_id],
    /// [datom_eid_ident], or [datom_eid_unique].
    #[no_mangle]
    pub unsafe extern "C" fn datom_eid_destroy(eid: *mut EID) {
        Box::from_raw(eid);
    }
}
