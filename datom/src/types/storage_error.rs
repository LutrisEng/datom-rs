// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#![allow(missing_docs)]

use std::{error, io};

use miette::Diagnostic;
use thiserror::Error;

/// An error in the underlying storage backend
#[derive(Error, Debug, Diagnostic)]
pub enum StorageError {
    #[error("an error occurred related to concurrency")]
    #[diagnostic(code(datom::storage::concurrency), url(docsrs))]
    ConcurrencyError,

    #[error("an I/O error occurred")]
    #[diagnostic(code(datom::storage::io), url(docsrs))]
    IOError(#[from] io::Error),

    #[error("an unknown error occurred")]
    #[diagnostic(code(datom::storage::misc), url(docsrs))]
    Miscellaneous(#[from] Box<dyn error::Error + Send + Sync + 'static>),
}
