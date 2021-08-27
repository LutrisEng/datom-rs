// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{backends::SledStorage, StorageError};

use crate::structs::{Storage, Str};

#[no_mangle]
pub extern "C" fn datom_sled_connect(path: Box<Str>) -> Option<Box<Storage>> {
    let res: Result<Box<Storage>, StorageError> = (|| {
        let storage = SledStorage::connect(&path.s)?;
        Ok(Box::new(storage.into()))
    })();
    match res {
        Ok(s) => Some(s),
        Err(_) => {
            // update_last_storage_error(e.into())
            None
        }
    }
}
