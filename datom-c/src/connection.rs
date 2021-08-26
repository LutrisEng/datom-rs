// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{new_dynamic_connection, ConnectionError};

use crate::structs::{Connection, Database, Storage};

#[no_mangle]
pub extern "C" fn datom_connect(storage: Box<Storage>) -> Box<Connection> {
    Box::new(new_dynamic_connection(storage.s).into())
}

#[no_mangle]
pub extern "C" fn datom_disconnect(_: Box<Connection>) {}

#[no_mangle]
pub extern "C" fn datom_db(conn: &'_ Connection) -> Option<Box<Database<'_>>> {
    let res: Result<Box<Database>, ConnectionError> = (|| {
        let db = conn.c.db()?;
        Ok(Box::new(db.into()))
    })();
    match res {
        Ok(d) => Some(d),
        Err(_) => {
            // update_last_connection_error(e.into())
            None
        }
    }
}
