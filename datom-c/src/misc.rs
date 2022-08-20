// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::ffi::CStr;

use libc::c_char;

use crate::structs::Str;

/// # Safety
/// str must be a valid, NULL-terminated string.
#[no_mangle]
pub unsafe extern "C" fn datom_string(str: *const c_char) -> Option<Box<Str>> {
    let cstr = CStr::from_ptr(str);
    cstr.to_str()
        .ok()
        .map(|s| Box::new(Str { s: s.to_string() }))
}
