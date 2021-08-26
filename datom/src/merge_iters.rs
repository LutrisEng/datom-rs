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
let iters = vec![first.iter().cloned(), second.iter().cloned(), third.iter().cloned()];
let merged = MergeIters::new(iters);
let results: Vec<u64> = merged.map(|x| x.0).collect();
assert_eq!(vec![1, 53, 62, 76, 431, 2754, 53326, 63235, 5322663, 634463436], results);
```
*/
pub struct MergeIters<T: Ord, I: Iterator<Item = T>> {
    iterators: Vec<I>,
    front_map: Option<BTreeMap<T, usize>>,
    back_map: Option<BTreeMap<T, usize>>,
}

impl<T: Ord, I: Iterator<Item = T>> Iterator for MergeIters<T, I> {
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let map = if let Some(map) = &mut self.front_map {
            map
        } else {
            let mut map = BTreeMap::new();
            for (i, iterator) in self.iterators.iter_mut().enumerate() {
                if let Some(v) = iterator.next() {
                    map.insert(v, i);
                }
            }
            self.front_map = Some(map);
            if let Some(map) = &mut self.front_map {
                map
            } else {
                panic!()
            }
        };
        let (val, from_iterator) = map.pop_first()?;
        if let Some(v) = self.iterators[from_iterator].next() {
            map.insert(v, from_iterator);
        }
        Some((val, from_iterator))
    }
}

impl<T: Ord, I: Iterator<Item = T> + DoubleEndedIterator> DoubleEndedIterator for MergeIters<T, I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let map = if let Some(map) = &mut self.back_map {
            map
        } else {
            let mut map = BTreeMap::new();
            for (i, iterator) in self.iterators.iter_mut().enumerate() {
                if let Some(v) = iterator.next_back() {
                    map.insert(v, i);
                }
            }
            self.back_map = Some(map);
            if let Some(map) = &mut self.back_map {
                map
            } else {
                panic!()
            }
        };
        let (val, from_iterator) = map.pop_last()?;
        if let Some(v) = self.iterators[from_iterator].next_back() {
            map.insert(v, from_iterator);
        }
        Some((val, from_iterator))
    }
}

impl<T: Ord, I: Iterator<Item = T>> MergeIters<T, I> {
    /// Merge n sorted iterators
    pub fn new(iterators: Vec<I>) -> Self {
        Self {
            iterators,
            front_map: None,
            back_map: None,
        }
    }
}
