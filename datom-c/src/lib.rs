// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::ffi::CString;

#[allow(clippy::upper_case_acronyms)]
pub mod connection;
pub mod database;
pub mod misc;
pub mod sled;
pub mod structs;

#[no_mangle]
pub extern "C" fn datom_version() -> *mut i8 {
    CString::new(datom::version())
        .expect("Version contained an unexpected null")
        .into_raw()
}

/// # Safety
/// This takes a raw pointer and interprets it as a CString
#[no_mangle]
pub unsafe extern "C" fn datom_free_version(version: *mut i8) {
    drop(CString::from_raw(version));
}
