// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{
    ConnectionError, Database, Transactable, Transaction, TransactionError, TransactionResult,
};

/// A persistent connection to a database
pub trait Connection: Sized {
    /// The struct holding a point-in-time view of this connection
    type Database<'connection>: Database<'connection>;

    /// Create a connection through the given URI
    fn connect(uri: &str) -> Result<Self, ConnectionError>;
    /// Fetch the t-value for the latest transaction
    fn latest_t(&self) -> Result<u64, ConnectionError>;
    /// Get a [database](crate::database::Database) for the current
    /// point in time
    fn db(&self) -> Result<Self::Database<'_>, ConnectionError>;
    /// Get a [database](crate::database::Database) for a specific point
    /// in time
    fn as_of(&self, t: u64) -> Result<Self::Database<'_>, ConnectionError>;
    /// Run a transaction on the database
    fn transact_tx(
        &self,
        tx: Transaction,
    ) -> Result<TransactionResult<'_, Self, Self::Database<'_>>, TransactionError>;
    /// Transact a transactable on the database
    fn transact<T: Transactable>(
        &self,
        txable: T,
    ) -> Result<TransactionResult<'_, Self, Self::Database<'_>>, TransactionError> {
        self.transact_tx(txable.tx())
    }
}
