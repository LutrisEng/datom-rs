// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::ops::Range;

use crate::{StorageError, ID};

/// A serialized datom
pub type Item = Vec<u8>;

/// An iterator over a sorted set of datoms
pub type ItemIterator<'s> = Box<dyn DoubleEndedIterator<Item = Result<Item, StorageError>> + 's>;

/// A [std::collections::BTreeSet<Datom>]-like storage backend
pub trait Storage: Send + Sync {
    /// Get all items within this range
    fn range(&self, r: Range<&[u8]>) -> Result<ItemIterator<'_>, StorageError>;

    /// Insert many new items into the backend (in one transaction, if possible)
    fn insert(&self, is: &[Item]) -> Result<(), StorageError>;

    /// Get a unique ID for this instance
    fn id(&self) -> ID;
}

impl<S: Storage + ?Sized> Storage for Box<S> {
    fn range(&self, r: Range<&[u8]>) -> Result<ItemIterator<'_>, StorageError> {
        (**self).range(r)
    }

    fn insert(&self, is: &[Item]) -> Result<(), StorageError> {
        (**self).insert(is)
    }

    fn id(&self) -> ID {
        (**self).id()
    }
}

/// A storage backend which persists its state. This may take the form
/// of a file on disk, a remote database, or something else.
pub trait DurableStorage: Storage {}
