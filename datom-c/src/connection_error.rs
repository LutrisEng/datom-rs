// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::cell::RefCell;

use datom::ConnectionError;

/// Network/disk errors
#[repr(C)]
#[derive(Clone, Copy)]
pub enum DatomConnectionError {
    /// No error
    None,
    /// There was invalid data in the data store
    InvalidData,
    /// There was an IO error
    IOError,
    /// There was an other error, possibly in the underlying data
    /// store
    Miscellaneous,
}

impl From<&ConnectionError> for DatomConnectionError {
    fn from(ce: &ConnectionError) -> Self {
        match ce {
            ConnectionError::InvalidData => Self::InvalidData,
            ConnectionError::IO(_) => Self::IOError,
            ConnectionError::Miscellaneous(_) => Self::Miscellaneous,
        }
    }
}

impl From<ConnectionError> for DatomConnectionError {
    fn from(ce: ConnectionError) -> Self {
        Self::from(&ce)
    }
}

thread_local! {
    static LAST_CONNECTION_ERROR: RefCell<Option<DatomConnectionError>> = RefCell::new(None);
}

/// Set the last [DatomConnectionError] thrown
pub fn update_last_connection_error(err: DatomConnectionError) {
    LAST_CONNECTION_ERROR.with(|prev| {
        *prev.borrow_mut() = Some(err);
    });
}

/// Get the last [DatomConnectionError] thrown
#[no_mangle]
pub extern "C" fn datom_last_connection_error() -> DatomConnectionError {
    LAST_CONNECTION_ERROR.with(|val| val.borrow().unwrap_or(DatomConnectionError::None))
}
