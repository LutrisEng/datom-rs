// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{error::Error, fmt, io};

/// Network/disk errors
#[derive(Debug)]
pub enum ConnectionError {
    /// There was invalid data in the data store
    InvalidData,
    /// There was an IO error
    IO(io::Error),
    /// There was an other error, possibly in the underlying data store
    Miscellaneous(Box<dyn std::error::Error>),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

impl Error for ConnectionError {}

impl From<io::Error> for ConnectionError {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<sled::Error> for ConnectionError {
    fn from(e: sled::Error) -> Self {
        Self::Miscellaneous(Box::new(e))
    }
}