// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{Database, Datom, Fact, TransactionError, Value, EID};

/// A set of facts which can be transacted into a database connection
pub struct Transaction {
    facts: Vec<Fact>,
}

impl Transaction {
    /// Create a new empty [Transaction]
    pub const fn new() -> Self {
        Self { facts: vec![] }
    }

    /// Add a raw [Fact]
    pub fn push_fact(&mut self, fact: Fact) {
        self.facts.push(fact)
    }

    /// Add an attribute value to an entity
    pub fn add(&mut self, entity: EID, attribute: EID, value: Value) {
        self.push_fact(Fact::Add(entity, attribute, value));
    }

    /// Retract a specific attribute value from an entity
    pub fn retract_value(&mut self, entity: EID, attribute: EID, value: Value) {
        self.push_fact(Fact::RetractValue(entity, attribute, value));
    }

    /// Retract an attribute from an entity, ignoring its value
    pub fn retract(&mut self, entity: EID, attribute: EID) {
        self.push_fact(Fact::Retract(entity, attribute))
    }

    /// Convert the [Transaction] to a set of [Datom]s
    pub fn datoms<'c, D: Database<'c>>(
        &self,
        t: u64,
        db: &D,
    ) -> Result<Vec<Datom>, TransactionError> {
        self.facts
            .iter()
            .map(|f| f.to_owned().datom(t, db))
            .collect()
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new()
    }
}
