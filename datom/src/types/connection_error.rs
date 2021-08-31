// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#![allow(missing_docs)]

use miette::Diagnostic;
use thiserror::Error;

use crate::StorageError;

/// Network/disk errors
#[derive(Error, Debug, Diagnostic)]
pub enum ConnectionError {
    #[error("there was invalid data in the data store")]
    #[diagnostic(code(datom::connection::invalid_data))]
    InvalidData,

    #[error("there was an error in the underlying storage backend")]
    #[diagnostic(code(datom::storage))]
    Storage(#[from] StorageError),
}
