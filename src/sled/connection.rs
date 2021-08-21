// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::env::temp_dir;

use sled::Config;
use uuid::Uuid;

use crate::{
    builtin_idents, serial::serialize, Connection, ConnectionError, Database, Datom, Entity, Index,
    Transaction, TransactionError, TransactionResult, Value,
};

use super::SledDatabase;

/// A persistent [Connection] to a sled-backed database
pub struct SledConnection {
    pub(crate) db: sled::Db,
}

const LATEST_T: [u8; 1] = [255u8];
const TRANSACTOR_LOCK: [u8; 1] = [254u8];

impl SledConnection {
    fn insert(&self, datom: &Datom, index: Index) -> Result<(), ConnectionError> {
        self.db.insert(serialize(datom, index), vec![])?;
        Ok(())
    }

    fn set_t(&self, t: u64) -> Result<(), ConnectionError> {
        self.db.insert(LATEST_T, &t.to_be_bytes())?;
        Ok(())
    }

    fn lock_transactor(&self) -> Result<(), ConnectionError> {
        while self
            .db
            .compare_and_swap(
                TRANSACTOR_LOCK,
                Option::<&'static str>::None,
                Some("Locked"),
            )?
            .is_err()
        {
            // Wait for the transactor to unlock
        }
        Ok(())
    }

    fn unlock_transactor(&self) -> Result<(), ConnectionError> {
        self.db.remove(TRANSACTOR_LOCK)?;
        Ok(())
    }

    /// Create a connection to a temporary database. When the
    /// [SledConnection] is dropped, the temporary database will be
    /// removed from the disk. This is useful for tests.
    pub fn connect_temp() -> Result<Self, ConnectionError> {
        let mut path = temp_dir();
        path.push(Uuid::new_v4().to_string());
        path.set_extension("db");
        let cfg = Config::new().path(path).temporary(true);
        let db = cfg.open()?;
        Ok(Self { db })
    }
}

impl Connection for SledConnection {
    type Database<'a> = SledDatabase<'a>;

    fn connect(uri: &str) -> Result<Self, ConnectionError> {
        let cfg = Config::new().path(uri);
        let db = cfg.open()?;
        Ok(Self { db })
    }

    fn latest_t(&self) -> Result<u64, ConnectionError> {
        match self.db.get(LATEST_T)? {
            Some(t_bytes) => Ok(u64::from_be_bytes(
                t_bytes[..]
                    .try_into()
                    .map_err(|_| ConnectionError::InvalidData)?,
            )),
            None => Ok(0),
        }
    }

    fn as_of(&self, t: u64) -> Result<Self::Database<'_>, ConnectionError> {
        Ok(SledDatabase {
            connection: self,
            t,
        })
    }

    fn db(&self) -> Result<Self::Database<'_>, ConnectionError> {
        self.as_of(self.latest_t()?)
    }

    fn transact_tx(
        &self,
        tx: Transaction,
    ) -> Result<TransactionResult<'_, Self, Self::Database<'_>>, TransactionError> {
        self.lock_transactor()?;
        let res = {
            let t_before = self.latest_t()?;
            let t = t_before + 1;
            let before = self.as_of(t_before)?;
            let data = tx.datoms(t, &before)?;
            for datom in data.iter() {
                self.insert(datom, Index::EAVT)?;
                self.insert(datom, Index::AEVT)?;
                let attr_entity = before.entity(datom.attribute.into())?;
                let unique_value =
                    attr_entity.get_with_options(builtin_idents::unique().into(), true)?;
                let type_value =
                    attr_entity.get_with_options(builtin_idents::value_type().into(), true)?;
                let is_unique = unique_value == Some(Value::Boolean(true));
                let is_ref = type_value == Some(Value::ID(builtin_idents::type_ref()));
                if is_unique {
                    self.insert(datom, Index::AVET)?;
                }
                if is_ref {
                    self.insert(datom, Index::VAET)?;
                }
            }
            self.set_t(t)?;
            Ok(TransactionResult {
                connection: self,
                before,
                after: self.as_of(t)?,
                data,
            })
        };
        self.unlock_transactor()?;
        res
    }
}

/// C bindings
pub mod c {
    use std::{ffi::CStr, os::raw::c_char};

    use crate::{
        c::{update_last_connection_error, DatomConnectionError},
        sled::SledDatabase,
        types::c::DatomTransactionResult,
        Connection, Transaction,
    };

    use super::SledConnection;

    /// Create a connection to a sled-backed database at the given path
    ///
    /// # Safety
    ///
    /// path must be a NULL-terminated string. You must call
    /// [datom_sled_disconnect] when you are done with the
    /// SledConnection. Returns NULL on an error - check
    /// [datom_last_connection_error] to get the error code.
    #[no_mangle]
    pub unsafe extern "C" fn datom_sled_connect(
        path: *const c_char,
    ) -> Option<Box<SledConnection>> {
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
}
