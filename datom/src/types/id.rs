// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use uuid::Uuid;

/**
An entity ID

These IDs are better known as Squuids - a [UUID](Uuid) v4 with the most
significant 32 bits overwritten with seconds since epoch. Squuids
allow for more efficient indexing of the IDs, since they will end up
approximately monotonically increasing on a large scale.

Since attributes are entities themselves, these are also attribute
IDs.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ID(Uuid);

impl ID {
    /// Generate a new [ID] from the current time.
    pub fn new() -> Self {
        let now = SystemTime::now();
        let since_epoch = now
            .duration_since(UNIX_EPOCH)
            .expect("Must be run after 1970 (large ask, I know)")
            .as_secs() as u32;
        let since_epoch_be = since_epoch.to_be_bytes();
        let mut uuid_bytes = Uuid::new_v4().as_bytes().to_owned();
        uuid_bytes[..since_epoch_be.len()].clone_from_slice(&since_epoch_be[..]);
        Self(Uuid::from_bytes(uuid_bytes))
    }

    /// Get the null [UUID](Uuid), which is all zeroes.
    pub const fn null() -> Self {
        Self(Uuid::nil())
    }

    /// A const equivalent to [From<u128>::from]
    ///
    /// ```
    /// use datom::ID;
    /// assert_eq!(ID::from_u128(0), ID::null());
    /// ```
    pub const fn from_u128(x: u128) -> Self {
        Self(Uuid::from_u128(x))
    }
}

impl Default for ID {
    /// ```
    /// datom::ID::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for ID {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s)?))
    }
}

impl ToString for ID {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<u128> for ID {
    /// ```
    /// use datom::ID;
    /// assert_eq!(ID::from_u128(0), ID::null());
    /// ```
    fn from(n: u128) -> Self {
        Self(Uuid::from_u128(n))
    }
}

impl From<ID> for u128 {
    /// ```
    /// use datom::ID;
    /// assert_eq!(0u128, ID::null().into());
    /// ```
    fn from(val: ID) -> Self {
        val.0.as_u128()
    }
}

impl From<[u8; 16]> for ID {
    fn from(bytes: [u8; 16]) -> Self {
        Self(Uuid::from_bytes(bytes))
    }
}

impl From<&ID> for [u8; 16] {
    fn from(id: &ID) -> Self {
        id.0.as_bytes().to_owned()
    }
}

impl From<ID> for [u8; 16] {
    fn from(id: ID) -> Self {
        <[u8; 16]>::from(&id)
    }
}
