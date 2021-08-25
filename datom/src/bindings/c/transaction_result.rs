// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{
    sled::{SledConnection, SledDatabase},
    Connection, Database, TransactionResult,
};

/// The result of running a [Transaction](crate::Transaction) on a
/// [Connection]; for C bindings
#[repr(C)]
pub struct DatomTransactionResult<'connection, C: Connection, D: Database<'connection>> {
    /**
    The [Connection] the [Transaction](crate::Transaction) was run
    on
    */
    pub connection: &'connection C,
    /// The [database](crate::database::Database) before the transaction
    pub before: Box<D>,
    /// The [database](crate::database::Database) after the transaction
    pub after: Box<D>,
}

impl<'c, C: Connection, D: Database<'c>> From<TransactionResult<'c, C, D>>
    for DatomTransactionResult<'c, C, D>
{
    fn from(tr: TransactionResult<'c, C, D>) -> Self {
        DatomTransactionResult {
            connection: tr.connection,
            before: Box::new(tr.before),
            after: Box::new(tr.after),
        }
    }
}

/// Destroy a transaction result
///
/// # Safety
///
/// res must be a valid, non-null [DatomTransactionResult] created by
/// [datom_sled_transact](crate::c::datom_sled_transact).
#[no_mangle]
pub unsafe extern "C" fn datom_sled_transaction_result_destroy<'c>(
    res: *mut DatomTransactionResult<'c, SledConnection, SledDatabase<'c>>,
) {
    Box::from_raw(res);
}
