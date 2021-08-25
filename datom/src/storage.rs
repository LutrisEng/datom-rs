// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::ops::Range;

use crate::StorageError;

/// A serialized datom
pub type Item = Vec<u8>;

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
