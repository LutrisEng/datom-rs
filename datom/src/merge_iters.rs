// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::BTreeMap;

/**
n sorted iterators, merged
```
use datom::MergeIters;
let first: [u64; 4] = [1, 76, 2754, 53326];
let second: [u64; 3] = [53, 62, 431];
let third: [u64; 3] = [63235, 5322663, 634463436];
let mut iters = [first.iter().cloned(), second.iter().cloned(), third.iter().cloned()];
let merged = MergeIters::new(&mut iters);
let results: Vec<u64> = merged.collect();
assert_eq!(vec![1, 53, 62, 76, 431, 2754, 53326, 63235, 5322663, 634463436], results);
```
*/
pub struct MergeIters<'i, T: Ord, I: Iterator<Item = T>> {
    iterators: &'i mut [I],
    map: BTreeMap<T, usize>,
}

impl<'i, T: Ord, I: Iterator<Item = T>> Iterator for MergeIters<'i, T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (val, from_iterator) = self.map.pop_first()?;
        if let Some(v) = self.iterators[from_iterator].next() {
            self.map.insert(v, from_iterator);
        }
        Some(val)
    }
}

impl<'i, T: Ord, I: Iterator<Item = T>> MergeIters<'i, T, I> {
    /// Merge n sorted iterators
    pub fn new(iterators: &'i mut [I]) -> Self {
        let mut map = BTreeMap::new();
        for (i, iterator) in iterators.iter_mut().enumerate() {
            if let Some(v) = iterator.next() {
                map.insert(v, i);
            }
        }
        MergeIters { iterators, map }
    }
}
