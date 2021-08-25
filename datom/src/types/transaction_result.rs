// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{storage::Storage, Connection, Database, Datom};

/**
The result of running a [Transaction](crate::Transaction) on a
[Connection]
*/
pub struct TransactionResult<'connection, S: Storage> {
    /**
    The [Connection] the [Transaction](crate::Transaction) was run
    on
    */
    pub connection: &'connection Connection<S>,
    /// The [database](crate::database::Database) before the transaction
    pub before: Database<'connection, S>,
    /// The [database](crate::database::Database) after the transaction
    pub after: Database<'connection, S>,
    /// The [datoms](crate::Datom) added to the database in the transaction
    pub data: Vec<Datom>,
}
