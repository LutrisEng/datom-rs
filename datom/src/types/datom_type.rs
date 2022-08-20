// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

/// Whether a [datom](crate::Datom) is showing an addition or a retraction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DatomType {
    /// Adding an attribute value to an entity
    Addition,
    /// Removing an attribute value from an entity
    Retraction,
}

impl DatomType {
    /// Map the [DatomType] to its byte representation
    pub const fn byte(&self) -> u8 {
        match self {
            Self::Addition => 0,
            Self::Retraction => 1,
        }
    }

    /// Map byte representation to a [DatomType]
    pub const fn from_byte(b: u8) -> Self {
        match b {
            0 => Self::Addition,
            1 => Self::Retraction,
            _ => panic!("invalid datom type"),
        }
    }
}
