// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{ops::Range, sync::Arc};

use arc_swap::ArcSwap;
use rpds::RedBlackTreeSetSync;

use crate::{
    storage::{Item, ItemIterator, Storage},
    StorageError, ID,
};

/// A storage backend backed by a [RedBlackTreeSet]
pub struct RedBlackTreeSetStorage {
    set: ArcSwap<RedBlackTreeSetSync<Item>>,
    id: ID,
}

struct RedBlackTreeSetRangeIter {
    set: RedBlackTreeSetSync<Item>,
    last_front: Option<Item>,
    last_back: Option<Item>,
    start: Item,
    end: Item,
}

fn add_one(mut v: Vec<u8>) -> Vec<u8> {
    v.push(0);
    v
}

impl Iterator for RedBlackTreeSetRangeIter {
    type Item = Result<Item, StorageError>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self
            .last_front
            .clone()
            .map(add_one)
            .unwrap_or_else(|| self.start.clone());
        let end = self.last_back.clone().unwrap_or_else(|| self.end.clone());
        if start > end {
            None
        } else {
            let range = start..end;
            let mut it = self.set.range(range);
            let item = it.next()?.to_owned();
            self.last_front = Some(item.clone());
            Some(Ok(item))
        }
    }
}

impl DoubleEndedIterator for RedBlackTreeSetRangeIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let start = self
            .last_front
            .clone()
            .map(add_one)
            .unwrap_or_else(|| self.start.clone());
        let end = self.last_back.clone().unwrap_or_else(|| self.end.clone());
        if start > end {
            None
        } else {
            let range = start..end;
            let mut it = self.set.range(range).rev();
            let item = it.next()?.to_owned();
            self.last_back = Some(item.clone());
            Some(Ok(item))
        }
    }
}

impl Storage for RedBlackTreeSetStorage {
    fn range(&self, r: Range<&[u8]>) -> Result<ItemIterator<'_>, StorageError> {
        let set = (*self.set.load_full()).clone();
        Ok(Box::new(RedBlackTreeSetRangeIter {
            set,
            last_front: None,
            last_back: None,
            start: r.start.to_vec(),
            end: r.end.to_vec(),
        }))
    }

    fn insert(&self, is: &[Item]) -> Result<(), StorageError> {
        let set = (*self.set.load_full()).clone();
        let set = is.iter().fold(set, |s, i| s.insert(i.to_owned()));
        self.set.swap(Arc::new(set));
        Ok(())
    }

    fn id(&self) -> ID {
        self.id
    }
}

impl RedBlackTreeSetStorage {
    /// Create a new empty [RedBlackTreeSetStorage]
    pub fn new() -> Self {
        Self {
            set: ArcSwap::from_pointee(RedBlackTreeSetSync::new_sync()),
            id: ID::new(),
        }
    }
}

impl Default for RedBlackTreeSetStorage {
    fn default() -> Self {
        Self::new()
    }
}
