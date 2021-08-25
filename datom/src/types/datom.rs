// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{DatomType, Value, ID};

/**
A _datom_, or a single fact at a single point in time. Short for
_data atom_.
*/
#[derive(Clone, Debug, PartialEq)]
pub struct Datom {
    /// The entity this [Datom] is attached to
    pub entity: ID,
    /// The attribute this [Datom] is setting on the entity
    pub attribute: ID,
    /// The value for the attribute
    pub value: Value,
    /// The t-value for the transaction which introduced this [Datom]
    pub t: u64,
    /// Whether this [Datom] is adding or retracting data
    pub datom_type: DatomType,
}
