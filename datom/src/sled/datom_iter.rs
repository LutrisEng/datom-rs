// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{serial::deserialize_unknown, Datom};

/// An iterator over [Datom]s in a sled-backed database
pub struct SledDatomIter {
    iter: sled::Iter,
    t: u64,
}

impl SledDatomIter {
    pub(crate) const fn new(iter: sled::Iter, t: u64) -> Self {
        Self { iter, t }
    }
}

impl Iterator for SledDatomIter {
    type Item = Datom;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(Err(_)) => continue,
                Some(Ok((k, _))) => {
                    let bytes: &[u8] = &k;
                    let (datom, _) = deserialize_unknown(bytes)?;
                    if datom.t <= self.t {
                        return Some(datom);
                    }
                }
            }
        }
    }
}

impl DoubleEndedIterator for SledDatomIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next_back() {
                None => return None,
                Some(Err(_)) => continue,
                Some(Ok((k, _))) => {
                    let bytes: &[u8] = &k;
                    let (datom, _) = deserialize_unknown(bytes)?;
                    if datom.t <= self.t {
                        return Some(datom);
                    }
                }
            }
        }
    }
}
