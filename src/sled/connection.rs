// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::convert::TryInto;

use sled::Config;

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

    /**
    Create a connection to a temporary database. When the
    [SledConnection] is dropped, the temporary database will be
    removed from the disk. This is useful for tests.
    */
    pub fn connect_temp(uri: &str) -> Result<Self, ConnectionError> {
        let cfg = Config::new().path(uri).temporary(true);
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

    fn transact(
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
                let is_unique = attr_entity
                    .get_with_options(builtin_idents::unique().into(), true)?
                    == Some(Value::Boolean(true));
                let is_ref = attr_entity
                    .get_with_options(builtin_idents::value_type().into(), true)?
                    == Some(Value::ID(builtin_idents::type_ref()));
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
