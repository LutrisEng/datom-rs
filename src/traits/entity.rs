// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::hash::Hash;

use crate::{EntityResult, QueryError, EID, ID};

/// An entity at a single point in time
pub trait Entity: Sized + PartialEq + Eq + Clone + Hash {
    /// An iterator over an entity's attributes
    type AttributeIter: Iterator<Item = ID>;

    /// Get the ID of this entity
    fn id(&self) -> &ID;

    /// Get the value of an attribute on this entity, with options
    fn get_with_options(
        &self,
        attribute: EID,
        skip_cardinality: bool,
        skip_type: bool,
    ) -> Result<EntityResult<Self>, QueryError>;
    /// Get the value of an attribute on this entity
    fn get(&self, attribute: EID) -> Result<EntityResult<Self>, QueryError> {
        self.get_with_options(attribute, false, false)
    }
    /// Get the entities with this entity as a value on an attribute
    /// (reverse lookup)
    fn reverse_get(&self, attribute: EID) -> Result<EntityResult<Self>, QueryError>;
    /// Get the attributes on this entity
    fn attributes(&self) -> Result<Self::AttributeIter, QueryError>;
}
