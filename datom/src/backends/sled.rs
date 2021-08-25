// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{env::temp_dir, ops::Range};

use sled::{transaction::ConflictableTransactionError, Config, Db};
use uuid::Uuid;

use crate::{
    storage::{Item, ItemIterator, Storage},
    StorageError, ID,
};

impl From<sled::Error> for StorageError {
    fn from(e: sled::Error) -> Self {
        Self::Miscellaneous(Box::new(e))
    }
}

impl From<sled::transaction::TransactionError> for StorageError {
    fn from(e: sled::transaction::TransactionError) -> Self {
        Self::Miscellaneous(Box::new(e))
    }
}

/// A storage backend backed by a [sled] database
pub struct SledStorage {
    db: Db,
    id: ID,
}

impl PartialEq<Self> for SledStorage {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Storage for SledStorage {
    fn range(&self, r: Range<&[u8]>) -> Result<ItemIterator<'_>, StorageError> {
        Ok(Box::new(
            self.db
                .range(r)
                .map(|r| r.map_err(|e| e.into()).map(|(k, _)| k.to_vec())),
        ))
    }

    fn insert(&self, i: Item) -> Result<(), StorageError> {
        self.db.insert(i, vec![])?;
        Ok(())
    }

    fn insert_many(&self, is: &[Item]) -> Result<(), StorageError> {
        self.db.transaction(|t| {
            for i in is {
                t.insert(i.as_slice(), vec![])?;
            }
            Ok::<(), ConflictableTransactionError>(())
        })?;
        Ok(())
    }
}

impl SledStorage {
    /// Create a connection to a temporary database. When the
    /// [SledStorage] is dropped, the temporary database will be
    /// removed from the disk. This is useful for tests.
    pub fn connect_temp() -> Result<Self, sled::Error> {
        let mut path = temp_dir();
        path.push(Uuid::new_v4().to_string());
        path.set_extension("db");
        let cfg = Config::new().path(path).temporary(true);
        let db = cfg.open()?;
        Ok(Self { db, id: ID::new() })
    }

    /// Create a connection to a database.
    pub fn connect(uri: &str) -> Result<Self, sled::Error> {
        let cfg = Config::new().path(uri);
        let db = cfg.open()?;
        Ok(Self { db, id: ID::new() })
    }
}
