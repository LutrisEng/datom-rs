// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{cmp::Ordering, ops::Range};

use crate::{
    merge_iters::MergeIters,
    storage::{DurableStorage, Item, ItemIterator, Storage},
    StorageError, ID,
};

/// A storage backend backed by two other storage backends
/// Inserts are sent to both backends.
/// Reads come from both backends, and any items which are read from B
/// but not found in A will persisted to A in a future update.
pub struct TieredStorage<A: Storage, B: Storage> {
    a: A,
    b: B,
    id: ID,
}

impl<A: Storage, B: Storage> TieredStorage<A, B> {
    /// Create a new tiered storage from two other storages
    pub fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            id: ID::new(),
        }
    }
}

impl PartialEq for StorageError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Miscellaneous(_), Self::Miscellaneous(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for StorageError {}

impl PartialOrd for StorageError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StorageError {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (StorageError::ConcurrencyError, StorageError::ConcurrencyError) => Ordering::Equal,
            (StorageError::ConcurrencyError, StorageError::Miscellaneous(_)) => Ordering::Less,
            (StorageError::Miscellaneous(_), StorageError::ConcurrencyError) => Ordering::Greater,
            (StorageError::Miscellaneous(_), StorageError::Miscellaneous(_)) => Ordering::Equal,
        }
    }
}

impl<A: Storage, B: Storage> Storage for TieredStorage<A, B> {
    fn range(&self, r: Range<&[u8]>) -> Result<ItemIterator<'_>, StorageError> {
        let merged = MergeIters::new(self.a.range(r.clone())?, self.b.range(r.clone())?);
        Ok(Box::new(merged.map(|x| x.0)))
    }

    fn insert(&self, is: &[Item]) -> Result<(), StorageError> {
        self.a.insert(is)?;
        self.b.insert(is)?;
        Ok(())
    }

    fn id(&self) -> ID {
        self.id
    }
}

impl<A: Storage, B: DurableStorage> DurableStorage for TieredStorage<A, B> {}
