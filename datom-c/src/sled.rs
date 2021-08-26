// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::ffi::CStr;

use datom::{backends::SledStorage, ConnectionError};
use libc::c_char;

use crate::structs::Storage;

/// # Safety
/// path must be a valid, NULL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn sled_connect(path: *const c_char) -> Option<Box<Storage>> {
    let path_cstr = CStr::from_ptr(path);
    let path_str = path_cstr.to_str().map_err(|_| ConnectionError::InvalidData);
    match path_str {
        Ok(path_str) => {
            let maybe_storage = SledStorage::connect(path_str);
            match maybe_storage {
                Ok(storage) => Some(Box::new(storage.into())),
                Err(_) => {
                    // update_last_storage_error(e.into())
                    None
                }
            }
        }
        Err(_) => {
            // update_last_storage_error(e.into())
            None
        }
    }
}
