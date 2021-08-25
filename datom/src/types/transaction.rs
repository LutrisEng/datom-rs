// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::HashMap;

use crate::{storage::Storage, Database, Datom, Fact, TransactionError, Value, EID};

/// A type which can be appended to a transaction
pub trait Transactable {
    /// Get a transaction to add this object to the database
    fn tx(&self) -> Transaction;
}

/// A set of facts which can be transacted into a database connection
#[derive(Clone, Debug)]
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

    /// Add many attribute values to an entity
    pub fn add_many(&mut self, entity: EID, attr_value_pairs: HashMap<EID, Value>) {
        for (attr, val) in attr_value_pairs {
            self.push_fact(Fact::Add(entity.clone(), attr, val));
        }
    }

    /// Retract a specific attribute value from an entity
    pub fn retract_value(&mut self, entity: EID, attribute: EID, value: Value) {
        self.push_fact(Fact::RetractValue(entity, attribute, value));
    }

    /// Retract an attribute from an entity, ignoring its value
    pub fn retract(&mut self, entity: EID, attribute: EID) {
        self.push_fact(Fact::Retract(entity, attribute))
    }

    /// Append a transactable to this transaction
    pub fn append<T: Transactable>(&mut self, txable: T) {
        self.facts.append(&mut txable.tx().facts);
    }

    /// Convert the [Transaction] to a set of [Datom]s
    pub fn datoms<'c, S: Storage>(
        &self,
        t: u64,
        db: &Database<'c, S>,
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

impl Transactable for Transaction {
    fn tx(&self) -> Transaction {
        self.to_owned()
    }
}
