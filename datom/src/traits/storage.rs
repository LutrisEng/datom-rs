// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{error::Error, fmt, io, ops::Range};

/// A serialized datom
pub type Item = Vec<u8>;

/// An error in the underlying storage backend
#[derive(Debug)]
pub enum StorageError {
    /// An issue occurred related to concurrency.
    ConcurrencyError,
    /// Another error, caused by an error in the backend
    Miscellaneous(Box<dyn Error>),
}

impl From<io::Error> for StorageError {
    fn from(e: io::Error) -> Self {
        Self::Miscellaneous(Box::new(e))
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Error for StorageError {}

/// An iterator over a sorted set of datoms
pub type ItemIterator<'s> = Box<dyn DoubleEndedIterator<Item = Result<Item, StorageError>> + 's>;

/// A [std::collections::BTreeSet<Datom>]-like storage backend
pub trait Storage: Sync + Send + PartialEq {
    /// Get all items within this range
    fn range(&self, r: Range<&[u8]>) -> Result<ItemIterator<'_>, StorageError>;

    /// Insert a new item into the backend
    fn insert(&self, i: Item) -> Result<(), StorageError>;

    /// Insert many new items into the backend (in one transaction, if possible)
    fn insert_many(&self, is: &[Item]) -> Result<(), StorageError>;
}
