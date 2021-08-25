// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{error::Error, fmt};

use crate::{ConnectionError, QueryError, EID, ID};

/// Errors during a [Transaction](crate::Transaction)
#[derive(Debug)]
pub enum TransactionError {
    /// The given attribute doesn't have a value, and cannot be
    /// retracted
    FailedToRetractNonexistentAttribute(ID, ID),
    /// The given attribute is repeated, and cannot be retracted without
    /// specifying a specific value to retract.
    FailedToRetractRepeatedAttribute(ID, ID),
    /// The given EID doesn't resolve to an entity
    UnresolvedEID(EID),
    /// A query executed during this transaction failed
    QueryError(QueryError),
    /// There was an error with the underlying connection
    ConnectionError(ConnectionError),
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Error for TransactionError {}

impl From<QueryError> for TransactionError {
    fn from(qe: QueryError) -> Self {
        match qe {
            QueryError::UnresolvedEID(eid) => Self::UnresolvedEID(eid),
            _ => Self::QueryError(qe),
        }
    }
}

impl From<ConnectionError> for TransactionError {
    fn from(ce: ConnectionError) -> Self {
        Self::ConnectionError(ce)
    }
}
