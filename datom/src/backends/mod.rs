// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#[cfg(feature = "sled")]
mod sled;
#[cfg(feature = "sled")]
pub use self::sled::SledStorage;

#[cfg(feature = "redblacktreeset")]
mod redblacktreeset;
#[cfg(feature = "redblacktreeset")]
pub use self::redblacktreeset::RedBlackTreeSetStorage;
