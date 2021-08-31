// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#![allow(missing_docs)]

use miette::Diagnostic;
use thiserror::Error;

use crate::{ConnectionError, QueryError, EID, ID};

/// Errors during a [Transaction](crate::Transaction)
#[derive(Error, Debug, Diagnostic)]
pub enum TransactionError {
    #[error(
        "the given attribute {1:?} doesn't have a value for entity {0:?}, and cannot be retracted"
    )]
    #[diagnostic(code(datom::transaction::retract_nonexistent_attribute), url(docsrs))]
    FailedToRetractNonexistentAttribute(ID, ID),

    #[error("the given attribute {1:?} is repeated, and cannot be retracted without specifying a specific value to retract")]
    #[diagnostic(code(datom::transaction::retract_repeated_attribute), url(docsrs))]
    FailedToRetractRepeatedAttribute(ID, ID),

    #[error("the given EID {0:?} doesn't resolve to an entity")]
    #[diagnostic(code(datom::transaction::unresolved_eid), url(docsrs))]
    UnresolvedEID(EID),

    #[error("a query executed during this transaction failed")]
    #[diagnostic(code(datom::query), url(docsrs))]
    QueryError(#[from] QueryError),

    #[error("there was an error with the underlying connection")]
    #[diagnostic(code(datom::connection), url(docsrs))]
    ConnectionError(#[from] ConnectionError),
}
