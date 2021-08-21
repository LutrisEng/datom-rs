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
        match t {
            AttributeType::String => builtin_idents::type_string(),
            AttributeType::Integer => builtin_idents::type_integer(),
            AttributeType::Decimal => builtin_idents::type_decimal(),
            AttributeType::ID => builtin_idents::type_id(),
            AttributeType::Ref => builtin_idents::type_ref(),
            AttributeType::Boolean => builtin_idents::type_boolean(),
        }
    }
}

/// An imperative way to generate an attribute's schema
pub struct AttributeSchema {
    id: ID,
    ident: Option<String>,
    many: bool,
    value_type: Option<AttributeType>,
    doc: Option<String>,
    unique: bool,
    component: bool,
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
    fn default() -> Self {
        Self::new()
    }
}

impl Transactable for AttributeSchema {
    fn tx(&self) -> Transaction {
        let mut tx = Transaction::new();
        tx.add(self.id.into(), builtin_idents::id().into(), self.id.into());
        if let Some(ident) = self.ident.clone() {
            tx.add(self.id.into(), builtin_idents::ident().into(), ident.into());
        }
        if self.many {
            tx.add(
                self.id.into(),
                builtin_idents::cardinality().into(),
                builtin_idents::cardinality_many().into(),
            )
        }
        if let Some(t) = self.value_type {
            tx.add(
                self.id.into(),
                builtin_idents::value_type().into(),
                Value::ID(t.into()),
            );
        }
        if let Some(doc) = self.doc.clone() {
            tx.add(self.id.into(), builtin_idents::doc().into(), doc.into());
        }
        if self.unique {
            tx.add(self.id.into(), builtin_idents::unique().into(), true.into());
        }
        if self.component {
            tx.add(
                self.id.into(),
                builtin_idents::is_component().into(),
                true.into(),
            );
        }
        tx
    }
}
