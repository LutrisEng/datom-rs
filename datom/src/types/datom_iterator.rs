// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{serial::deserialize_unknown, storage::ItemIterator, Datom};

/// An iterator over [Datom]s
pub struct DatomIterator<'s> {
    iter: ItemIterator<'s>,
    t: u64,
}

impl<'s> DatomIterator<'s> {
    pub(crate) fn new(iter: ItemIterator<'s>, t: u64) -> Self {
        Self { iter, t }
    }
}

impl<'s> Iterator for DatomIterator<'s> {
    type Item = Datom;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                None => return None,
                Some(Err(_)) => continue,
                Some(Ok(k)) => {
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

impl<'s> DoubleEndedIterator for DatomIterator<'s> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next_back() {
                None => return None,
                Some(Err(_)) => continue,
                Some(Ok(k)) => {
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
