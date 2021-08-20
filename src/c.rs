// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{cell::RefCell, ffi::CStr, os::raw::c_char};

use crate::{
    sled::{SledConnection, SledDatabase},
    Connection, DatomConnectionError, DatomTransactionResult, Transaction, Value, EID, ID,
};

thread_local! {
    static LAST_CONNECTION_ERROR: RefCell<Option<DatomConnectionError>> = RefCell::new(None);
}

fn update_last_connection_error(err: DatomConnectionError) {
    LAST_CONNECTION_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(err);
    });
}

/// Get the last DatomConnectionError thrown
#[no_mangle]
pub extern "C" fn datom_last_connection_error() -> DatomConnectionError {
    LAST_CONNECTION_ERROR.with(|val| val.borrow().unwrap_or(DatomConnectionError::None))
}

/**
Create a connection to a sled-backed database at the given path

# Safety

path must be a NULL-terminated string. You must call
[datom_sled_disconnect] when you are done with the SledConnection.
Returns NULL on an error - check [datom_last_connection_error] to
get the error code.
*/
#[no_mangle]
pub unsafe extern "C" fn datom_sled_connect(path: *const c_char) -> Option<Box<SledConnection>> {
    let path_cstr = CStr::from_ptr(path);
    let path_str = path_cstr
        .to_str()
        .map_err(|_| DatomConnectionError::Miscellaneous);
    match path_str {
        Ok(path_str) => {
            let maybe_conn = SledConnection::connect(path_str);
            match maybe_conn {
                Ok(conn) => Some(Box::new(conn)),
                Err(e) => {
                    update_last_connection_error(e.into());
                    None
                }
            }
        }
        Err(e) => {
            update_last_connection_error(e);
            None
        }
    }
}

/// Create an EID object from a 16-byte ID.
#[no_mangle]
pub extern "C" fn datom_eid_id(id: &[u8; 16]) -> Box<EID> {
    Box::new(EID::Resolved(ID::from(*id)))
}

/**
Create an EID object from a string ident.

# Safety

ident must be a NULL-terminated string.
*/
#[no_mangle]
pub unsafe extern "C" fn datom_eid_ident(ident: *const c_char) -> Option<Box<EID>> {
    let ident_cstr = CStr::from_ptr(ident);
    let ident_str = ident_cstr.to_str();
    match ident_str {
        Ok(ident_str) => Some(Box::new(EID::Ident(ident_str.to_owned()))),
        Err(_) => None,
    }
}

/**
Create an EID object from an attribute and value. Consumes attr and
value.
*/
#[no_mangle]
pub extern "C" fn datom_eid_unique(attr: Box<EID>, value: Box<Value>) -> Box<EID> {
    Box::new(EID::Unique(attr, *value))
}

/**
Destroy an EID object which wasn't consumed

# Safety

eid must be a valid, non-null [EID] created by [datom_eid_id],
[datom_eid_ident], or [datom_eid_unique].
*/
#[no_mangle]
pub unsafe extern "C" fn datom_eid_destroy(eid: *mut EID) {
    Box::from_raw(eid);
}

/// Create a transaction object
#[no_mangle]
pub extern "C" fn datom_transaction_create() -> Box<Transaction> {
    Box::new(Transaction::new())
}

/**
Add an attribute value to an entity in a transaction

Consumes entity, attribute, and value.

# Safety

transaction must be a valid, non-null [Transaction] created by
[datom_transaction_create].
*/
#[no_mangle]
pub extern "C" fn datom_transaction_add(
    transaction: &mut Transaction,
    entity: Box<EID>,
    attribute: Box<EID>,
    value: Box<Value>,
) {
    transaction.add(*entity, *attribute, *value);
}

/**
Retract a specific attribute value from an entity in a transaction

Consumes entity, attribute, and value.

# Safety

transaction must be a valid, non-null [Transaction] created by
[datom_transaction_create].
*/
#[no_mangle]
pub extern "C" fn datom_transaction_retract_value(
    transaction: &mut Transaction,
    entity: Box<EID>,
    attribute: Box<EID>,
    value: Box<Value>,
) {
    transaction.retract_value(*entity, *attribute, *value);
}

/**
Retract an attribute from an entity, ignoring its value, in a
transaction

Consumes entity and attribute.

# Safety

transaction must be a valid, non-null [Transaction] created by
[datom_transaction_create].
*/
#[no_mangle]
pub extern "C" fn datom_transaction_retract(
    transaction: &mut Transaction,
    entity: Box<EID>,
    attribute: Box<EID>,
) {
    transaction.retract(*entity, *attribute);
}

/**
Destroy a transaction object which wasn't consumed

# Safety

transaction must be a valid, non-null [Transaction] created by
[datom_transaction_create].
*/
#[no_mangle]
pub unsafe extern "C" fn datom_transaction_destroy(transaction: *mut Transaction) {
    Box::from_raw(transaction);
}

/**
Run a transaction on a sled-backed database. Consumes transaction.

# Safety

conn must be a valid, non-null [SledConnection] created by
[datom_sled_connect]. transaction must be a valid, non-null
[Transaction] created by [datom_transaction_create]. You must destroy
the return value (if non-NULL) after you are done.
*/
#[no_mangle]
pub extern "C" fn datom_sled_transact(
    conn: &SledConnection,
    transaction: Box<Transaction>,
) -> Option<Box<DatomTransactionResult<'_, SledConnection, SledDatabase<'_>>>> {
    let res = conn.transact(*transaction);
    match res {
        Ok(result) => Some(Box::new(result.into())),
        Err(_) => None,
    }
}

/**
Destroy a transaction result

# Safety

res must be a valid, non-null [DatomTransactionResult] created by
[datom_sled_transact].
*/
#[no_mangle]
pub unsafe extern "C" fn datom_sled_transaction_result_destroy<'c>(
    res: *mut DatomTransactionResult<'c, SledConnection, SledDatabase<'c>>,
) {
    Box::from_raw(res);
}

/**
Destroy a connection to a sled-backed database

# Safety

conn must be a valid, non-null [SledConnection] created by
[datom_sled_connect].
*/
#[no_mangle]
pub unsafe extern "C" fn datom_sled_disconnect(conn: *mut SledConnection) {
    Box::from_raw(conn);
}
