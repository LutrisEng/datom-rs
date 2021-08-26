// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{new_dynamic_connection, ConnectionError};

use crate::structs::{Connection, Database, Storage};

#[no_mangle]
pub extern "C" fn connect(storage: Box<Storage>) -> Box<Connection> {
    Box::new(new_dynamic_connection(storage.s).into())
}

#[no_mangle]
pub extern "C" fn disconnect(_: Box<Connection>) {}

#[no_mangle]
pub extern "C" fn db(conn: &'_ Connection) -> Option<Box<Database<'_>>> {
    let res: Result<Box<Database>, ConnectionError> = try {
        let db = conn.c.db()?;
        Box::new(db.into())
    };
    match res {
        Ok(d) => Some(d),
        Err(_) => {
            // update_last_connection_error(e.into())
            None
        }
    }
}
