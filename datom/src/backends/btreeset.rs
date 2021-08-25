// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{collections::BTreeSet, ops::Range};

use crate::{
    storage::{Item, ItemIterator, Storage},
    StorageError, ID,
};

/// A storage backend backed by a [BTreeSet]
pub struct BTreeSetStorage {
    set: BTreeSet<Item>,
    id: ID,
}

impl Storage for BTreeSetStorage {
    fn range(&self, r: Range<&[u8]>) -> Result<ItemIterator<'_>, StorageError> {
        Ok(Box::new(
            self.set
                .range(r.start.to_vec()..r.end.to_vec())
                .map(|x| Ok(x.to_owned())),
        ))
    }

    fn insert(&self, _: Item) -> Result<(), StorageError> {
        // self.set.insert(i);
        // Ok(())
        todo!()
    }

    fn insert_many(&self, _: &[Item]) -> Result<(), StorageError> {
        // for i in is {
        //     self.set.insert(i.to_owned());
        // }
        // Ok(())
        todo!()
    }

    fn id(&self) -> ID {
        self.id
    }
}

impl BTreeSetStorage {
    /// Create a new empty [BTreeSetStorage]
    pub fn new() -> Self {
        Self {
            set: BTreeSet::new(),
            id: ID::new(),
        }
    }
}

impl Default for BTreeSetStorage {
    fn default() -> Self {
        Self::new()
    }
}
