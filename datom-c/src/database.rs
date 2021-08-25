// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::sled::SledDatabase;

/// Destroy a connection to a sled-backed database view
///
/// # Safety
///
/// db must be a valid, non-null [SledDatabase] created by
/// [datom_sled_db](crate::c::datom_sled_db).
#[no_mangle]
pub unsafe extern "C" fn datom_sled_db_destroy(db: *mut SledDatabase) {
    Box::from_raw(db);
}
