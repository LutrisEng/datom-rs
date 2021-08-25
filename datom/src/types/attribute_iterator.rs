// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::HashSet;

use crate::{DatomIterator, DatomType, QueryError, ID};

/// An iterator over attributes in a sled-backed database
pub struct AttributeIterator<'d> {
    iter: DatomIterator<'d>,
    seen: HashSet<ID>,
}

impl<'d> AttributeIterator<'d> {
    pub(crate) fn new(iter: DatomIterator<'d>) -> Result<Self, QueryError> {
        Ok(Self {
            iter,
            seen: HashSet::new(),
        })
    }
}

impl<'s> Iterator for AttributeIterator<'s> {
    type Item = ID;

    fn next(&mut self) -> Option<Self::Item> {
        for datom in (&mut self.iter).rev() {
            let attr = datom.attribute;
            if !self.seen.contains(&attr) {
                self.seen.insert(attr);
                if datom.datom_type == DatomType::Addition {
                    return Some(attr);
                }
            }
        }
        None
    }
}
