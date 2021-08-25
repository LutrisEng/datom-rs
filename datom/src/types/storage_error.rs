// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{error::Error, fmt, io};

/// An error in the underlying storage backend
#[derive(Debug)]
pub enum StorageError {
    /// An issue occurred related to concurrency.
    ConcurrencyError,
    /// Another error, caused by an error in the backend
    Miscellaneous(Box<dyn Error>),
}

impl From<io::Error> for StorageError {
    fn from(e: io::Error) -> Self {
        Self::Miscellaneous(Box::new(e))
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Error for StorageError {}
