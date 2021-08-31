// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#![allow(missing_docs)]

use miette::Diagnostic;
use thiserror::Error;

use crate::{ConnectionError, StorageError, EID};

/// Errors during a [Database](crate::Database) query
#[derive(Error, Debug, Diagnostic)]
pub enum QueryError {
    #[error("the given EID `{0:?}` doesn't resolve to an entity")]
    #[diagnostic(code(datom::query::unresolved_eid))]
    UnresolvedEID(EID),

    #[error("there was an error with the underlying connection")]
    #[diagnostic(code(datom::connection))]
    ConnectionError(#[from] ConnectionError),
}

impl From<StorageError> for QueryError {
    fn from(se: StorageError) -> Self {
        Self::ConnectionError(se.into())
    }
}
