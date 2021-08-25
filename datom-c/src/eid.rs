// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{ffi::CStr, os::raw::c_char};

use datom::{Value, EID, ID};

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
