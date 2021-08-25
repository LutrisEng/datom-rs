// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{error::Error, fmt};

use crate::{storage::StorageError, ConnectionError, EID};

/// Errors during a [Database](crate::Database) query
#[derive(Debug)]
pub enum QueryError {
    /// The given EID doesn't resolve to an entity
    UnresolvedEID(EID),
    /// There was an error with the underlying connection
    ConnectionError(ConnectionError),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Error for QueryError {}

impl From<ConnectionError> for QueryError {
    fn from(ce: ConnectionError) -> Self {
        Self::ConnectionError(ce)
    }
}

impl From<StorageError> for QueryError {
    fn from(se: StorageError) -> Self {
        Self::ConnectionError(se.into())
    }
}
