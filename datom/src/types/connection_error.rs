// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{error::Error, fmt};

use crate::StorageError;

/// Network/disk errors
#[derive(Debug)]
pub enum ConnectionError {
    /// There was invalid data in the data store
    InvalidData,
    /// There was an error in the underlying storage backend
    Storage(StorageError),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Error for ConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConnectionError::Storage(e) => Some(e),
            _ => None,
        }
    }
}

impl From<StorageError> for ConnectionError {
    fn from(e: StorageError) -> Self {
        Self::Storage(e)
    }
}
