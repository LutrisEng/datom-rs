// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{ffi::CStr, os::raw::c_char};

use crate::{
    sled::{SledConnection, SledDatabase},
    Connection, Transaction,
};

use super::{
    connection_error::{update_last_connection_error, DatomConnectionError},
    transaction_result::DatomTransactionResult,
};

/// Create a connection to a sled-backed database at the given path
///
/// # Safety
///
/// path must be a NULL-terminated string. You must call
/// [datom_sled_disconnect] when you are done with the
/// SledConnection. Returns NULL on an error - check
/// [datom_last_connection_error] to get the error code.
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

/// Destroy a connection to a sled-backed database
///
/// # Safety
///
/// conn must be a valid, non-null [SledConnection] created by
/// [datom_sled_connect].
#[no_mangle]
pub unsafe extern "C" fn datom_sled_disconnect(conn: *mut SledConnection) {
    Box::from_raw(conn);
}

/// Run a transaction on a sled-backed database. Consumes
/// transaction.
///
/// # Safety
///
/// conn must be a valid, non-null [SledConnection] created by
/// [datom_sled_connect]. transaction must be a valid, non-null
/// [Transaction] created by [datom_transaction_create]. You must
/// destroy the return value (if non-NULL) after you are done.
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

/// Get a [database](SledDatabase) for the current point in time
#[no_mangle]
pub extern "C" fn datom_sled_db(conn: &SledConnection) -> Option<Box<SledDatabase>> {
    let res = conn.db();
    match res {
        Ok(db) => Some(Box::new(db)),
        Err(e) => {
            update_last_connection_error(e.into());
            None
        }
    }
}

/// Get a [database](SledDatabase) for a specific point in time
#[no_mangle]
pub extern "C" fn datom_sled_as_of(conn: &SledConnection, t: u64) -> Option<Box<SledDatabase>> {
    let res = conn.as_of(t);
    match res {
        Ok(db) => Some(Box::new(db)),
        Err(e) => {
            update_last_connection_error(e.into());
            None
        }
    }
}
