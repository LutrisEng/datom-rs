// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{env::temp_dir, fmt::Debug};

use sled::Config;
use uuid::Uuid;

use crate::{
    builtin_idents, serial::serialize, Connection, ConnectionError, Database, Datom, Entity,
    EntityResult, Index, Transaction, TransactionError, TransactionResult, Value, ID,
};

use super::SledDatabase;

/// A persistent [Connection] to a sled-backed database
pub struct SledConnection {
    pub(crate) db: sled::Db,
    pub(crate) id: ID,
}

impl PartialEq<Self> for SledConnection {
    /// ```
    /// use datom::sled::*;
    /// let conn1 = SledConnection::connect_temp()?;
    /// let conn2 = SledConnection::connect_temp()?;
    /// let conn1r = &conn1;
    /// let conn2r = &conn2;
    ///
    /// assert_eq!(&conn1, &conn1);
    /// assert_eq!(&conn1, conn1r);
    /// assert_eq!(conn1r, &conn1);
    /// assert_eq!(conn1r, conn1r);
    ///
    /// assert_ne!(&conn1, &conn2);
    /// assert_ne!(&conn1, conn2r);
    /// assert_ne!(conn1r, &conn2);
    /// assert_ne!(conn1r, conn2r);
    /// assert_ne!(&conn2, &conn1);
    /// assert_ne!(&conn2, conn1r);
    /// assert_ne!(conn2r, &conn1);
    /// assert_ne!(conn2r, conn1r);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SledConnection {}

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
        Ok(Self { db, id: ID::new() })
    }
}

impl Connection for SledConnection {
    type Database<'a> = SledDatabase<'a>;

    /// ```
    /// use datom::{prelude::*, sled::*};
    ///
    /// let mut path = std::env::temp_dir();
    /// path.push(uuid::Uuid::new_v4().to_string());
    /// path.set_extension("db");
    /// {
    ///     let conn = SledConnection::connect(path.to_str().ok_or(datom::ConnectionError::InvalidData)?)?;
    ///     let db = conn.db()?;
    ///     // ...
    /// }
    /// std::fs::remove_dir_all(path)?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn connect(uri: &str) -> Result<Self, ConnectionError> {
        let cfg = Config::new().path(uri);
        let db = cfg.open()?;
        Ok(Self { db, id: ID::new() })
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
                    attr_entity.get_with_options(builtin_idents::UNIQUE.into(), true, true)?;
                let type_value =
                    attr_entity.get_with_options(builtin_idents::VALUE_TYPE.into(), true, true)?;
                let is_unique = {
                    if let EntityResult::Value(Value::Boolean(x)) = unique_value {
                        x
                    } else {
                        false
                    }
                };
                let is_ref = {
                    if let EntityResult::Value(Value::ID(id)) = type_value {
                        id == builtin_idents::TYPE_REF
                    } else {
                        false
                    }
                };
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

impl Debug for SledConnection {
    /// ```
    /// use datom::sled::*;
    ///
    /// let conn = SledConnection::connect_temp()?;
    /// println!("{:#?}", conn);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SledConnection")
            .field("id", &self.id)
            .field(
                "EAVT",
                &(|| {
                    Ok::<Vec<Datom>, Box<dyn std::error::Error>>(
                        self.db()?.datoms(Index::EAVT)?.collect(),
                    )
                })(),
            )
            .field(
                "AEVT",
                &(|| {
                    Ok::<Vec<Datom>, Box<dyn std::error::Error>>(
                        self.db()?.datoms(Index::AEVT)?.collect(),
                    )
                })(),
            )
            .field(
                "AVET",
                &(|| {
                    Ok::<Vec<Datom>, Box<dyn std::error::Error>>(
                        self.db()?.datoms(Index::AVET)?.collect(),
                    )
                })(),
            )
            .field(
                "VAET",
                &(|| {
                    Ok::<Vec<Datom>, Box<dyn std::error::Error>>(
                        self.db()?.datoms(Index::VAET)?.collect(),
                    )
                })(),
            )
            .finish()
    }
}
