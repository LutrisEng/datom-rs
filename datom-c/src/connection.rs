// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{new_dynamic_connection, ConnectionError, TransactionError};

use crate::structs::{Connection, Database, Storage, Transaction, TransactionResult};

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

#[no_mangle]
pub extern "C" fn datom_as_of(conn: &'_ Connection, t: u64) -> Option<Box<Database<'_>>> {
    let res: Result<Box<Database>, ConnectionError> = (|| {
        let db = conn.c.as_of(t)?;
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

#[no_mangle]
pub extern "C" fn datom_latest_t(conn: &Connection) -> u64 {
    let res: Result<u64, ConnectionError> = conn.c.latest_t();
    match res {
        Ok(t) => t,
        Err(_) => {
            // update_last_connection_error(e.into())
            u64::MAX
        }
    }
}

#[no_mangle]
pub extern "C" fn datom_transact(
    conn: &Connection,
    tx: Box<Transaction>,
) -> Option<Box<TransactionResult>> {
    let res: Result<Box<TransactionResult>, TransactionError> = (|| {
        let r = conn.c.transact_tx(tx.t)?;
        Ok(Box::new(r.into()))
    })();
    match res {
        Ok(r) => Some(r),
        Err(_) => {
            // update_last_transaction_error(e.into())
            None
        }
    }
}
