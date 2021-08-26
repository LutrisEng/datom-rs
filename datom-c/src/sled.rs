// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::backends::SledStorage;

use crate::structs::{Storage, Str};

#[no_mangle]
pub extern "C" fn datom_sled_connect(path: Box<Str>) -> Option<Box<Storage>> {
    let maybe_storage = SledStorage::connect(&path.s);
    match maybe_storage {
        Ok(storage) => Some(Box::new(storage.into())),
        Err(_) => {
            // update_last_storage_error(e.into())
            None
        }
    }
}
