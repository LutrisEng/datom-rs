// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use crate::{builtin_idents, Transactable, Transaction, Value, ID};

/// The type of an attribute's values
#[derive(Clone, Copy)]
pub enum AttributeType {
    /// A [Value::String](crate::Value::String)
    String,
    /// A [Value::Integer](crate::Value::Integer)
    Integer,
    /// A [Value::Decimal](crate::Value::Decimal)
    Decimal,
    /// A [Value::ID](crate::Value::ID)
    ID,
    /// A [Value::ID](crate::Value::ID) referring to an entity
    Ref,
    /// A [Value::Boolean](crate::Value::Boolean)
    Boolean,
}

impl From<AttributeType> for ID {
    fn from(t: AttributeType) -> Self {
        use builtin_idents::*;
        match t {
            AttributeType::String => TYPE_STRING,
            AttributeType::Integer => TYPE_INTEGER,
            AttributeType::Decimal => TYPE_DECIMAL,
            AttributeType::ID => TYPE_ID,
            AttributeType::Ref => TYPE_REF,
            AttributeType::Boolean => TYPE_BOOLEAN,
        }
    }
}

/// An imperative way to generate an attribute's schema
#[derive(Clone)]
pub struct AttributeSchema {
    /// The attribute's ID
    pub id: ID,
    /// The attribute's unique identifier
    pub ident: Option<String>,
    /// Whether this attribute can store multiple values for an entity
    pub many: bool,
    /// What type values should be in this attribute
    pub value_type: Option<AttributeType>,
    /// A docstring for this attribute
    pub doc: Option<String>,
    /// Whether there can only be one entity of each value for this
    /// attribute
    pub unique: bool,
    /// Whether this attribute refers to a component
    pub component: bool,
}

impl AttributeSchema {
    /// Start generating an attribute's schema
    pub fn new() -> Self {
        Self {
            id: ID::new(),
            ident: None,
            many: false,
            value_type: None,
            doc: None,
            unique: false,
            component: false,
        }
    }

    /// Set a specific ID for an attribute
    pub const fn set_id(mut self, id: ID) -> Self {
        self.id = id;
        self
    }

    /// Set the attribute's ident
    #[allow(clippy::missing_const_for_fn)]
    pub fn ident(mut self, ident: String) -> Self {
        self.ident = Some(ident);
        self
    }

    /// Set the attribute's cardinality to many
    pub const fn many(mut self) -> Self {
        self.many = true;
        self
    }

    /// Set the attribute's value type
    pub const fn value_type(mut self, t: AttributeType) -> Self {
        self.value_type = Some(t);
        self
    }

    /// Set the attribute's docstring
    #[allow(clippy::missing_const_for_fn)]
    pub fn doc(mut self, doc: String) -> Self {
        self.doc = Some(doc);
        self
    }

    /// Set the attribute as unique
    pub const fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    /// Set the attribute as being a component reference
    pub const fn component(mut self) -> Self {
        self.value_type = Some(AttributeType::Ref);
        self.component = true;
        self
    }
}

impl Default for AttributeSchema {
    /// ```
    /// datom::AttributeSchema::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Transactable for AttributeSchema {
    fn tx(&self) -> Transaction {
        let mut tx = Transaction::new();
        tx.add(self.id.into(), builtin_idents::ID.into(), self.id.into());
        if let Some(ident) = self.ident.clone() {
            tx.add(self.id.into(), builtin_idents::IDENT.into(), ident.into());
        }
        if self.many {
            tx.add(
                self.id.into(),
                builtin_idents::CARDINALITY.into(),
                builtin_idents::CARDINALITY_MANY.into(),
            )
        }
        if let Some(t) = self.value_type {
            tx.add(
                self.id.into(),
                builtin_idents::VALUE_TYPE.into(),
                Value::ID(t.into()),
            );
        }
        if let Some(doc) = self.doc.clone() {
            tx.add(self.id.into(), builtin_idents::DOC.into(), doc.into());
        }
        if self.unique {
            tx.add(self.id.into(), builtin_idents::UNIQUE.into(), true.into());
        }
        if self.component {
            tx.add(
                self.id.into(),
                builtin_idents::IS_COMPONENT.into(),
                true.into(),
            );
        }
        tx
    }
}
