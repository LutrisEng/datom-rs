// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::{Value, ID as TID};

/// An entity's [ID]. This is a virtual attribute, and isn't actually
/// stored.
pub const ID: TID = TID::from_u128(329992551406372030633500533120122732713u128);

/// An entity's alias, usually used for the attribute schema.
pub const IDENT: TID = TID::from_u128(265682209113858765770461024079827500234u128);

/// Whether an attribute associates one value or multiple values with an
/// entity
pub const CARDINALITY: TID = TID::from_u128(110064945635332807503383834157761461043u128);

/// The intended type for an attribute. Note that this is not
/// (currently) strictly checked.
pub const VALUE_TYPE: TID = TID::from_u128(276059213908560386420175049892299151374u128);

/// A documentation string for an entity
pub const DOC: TID = TID::from_u128(303289866496710859530474533904741988829u128);

/// Whether there can only be one entity per value for this attribute
pub const UNIQUE: TID = TID::from_u128(307615836394596470679724073561969695989);

/// Whether the entity referred to in this [type_ref](self::type_ref) attribute is a
/// sub-component. When you retract an entity, all sub-components will
/// also be retracted.
pub const IS_COMPONENT: TID = TID::from_u128(308724514559417715856375983930347810391u128);

/// A value for the [CARDINALITY](self::CARDINALITY) attribute
pub const CARDINALITY_ONE: TID = TID::from_u128(143444949937465711736574828873158396909u128);

/// A value for the [CARDINALITY](self::CARDINALITY) attribute

pub const CARDINALITY_MANY: TID = TID::from_u128(11338831660433813835424721536043447369u128);

/// A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute

pub const TYPE_STRING: TID = TID::from_u128(301439516182801820546961599694687577507u128);

/// A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
pub const TYPE_INTEGER: TID = TID::from_u128(183876393307651966214416059730593095u128);

/// A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
pub const TYPE_DECIMAL: TID = TID::from_u128(297077785792417755741249562058415972414u128);

/// A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
pub const TYPE_ID: TID = TID::from_u128(339681506578292470250558610134765439055u128);

/// A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
pub const TYPE_REF: TID = TID::from_u128(149893903729185565330222631892178876560u128);

/// A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
pub const TYPE_BOOLEAN: TID = TID::from_u128(149893903729185565330222631892178876560u128);

/// The data behind a built-in entity
pub type BuiltinEntity = HashMap<TID, Value>;

/// The type of the data behind all built-in entities
pub type BuiltinEntities = HashMap<TID, BuiltinEntity>;

/// The type of the data behind all built-in entities, by ident
pub type BuiltinEntitiesByIdent = HashMap<String, BuiltinEntity>;

/// The data behind all built-in entities
pub static BUILTIN_ENTITIES: Lazy<BuiltinEntities> = Lazy::new(|| {
    let mut entities = BuiltinEntities::new();
    entities.insert(ID, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, ID.into());
        entity.insert(IDENT, Value::from("db/id"));
        entity.insert(VALUE_TYPE, Value::from(TYPE_ID));
        entity.insert(CARDINALITY, Value::from(CARDINALITY_ONE));
        entity
    });
    entities.insert(IDENT, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, IDENT.into());
        entity.insert(IDENT, Value::from("db/ident"));
        entity.insert(UNIQUE, Value::from(true));
        entity.insert(VALUE_TYPE, Value::from(TYPE_STRING));
        entity.insert(CARDINALITY, Value::from(CARDINALITY_ONE));
        entity
    });
    entities.insert(CARDINALITY, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, CARDINALITY.into());
        entity.insert(IDENT, Value::from("db/cardinality"));
        entity.insert(VALUE_TYPE, Value::from(TYPE_REF));
        entity.insert(CARDINALITY, Value::from(CARDINALITY_ONE));
        entity
    });
    entities.insert(VALUE_TYPE, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, VALUE_TYPE.into());
        entity.insert(IDENT, Value::from("db/value-type"));
        entity.insert(VALUE_TYPE, Value::from(TYPE_REF));
        entity.insert(CARDINALITY, Value::from(CARDINALITY_ONE));
        entity
    });
    entities.insert(DOC, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, DOC.into());
        entity.insert(IDENT, Value::from("db/doc"));
        entity.insert(VALUE_TYPE, Value::from(TYPE_STRING));
        entity.insert(CARDINALITY, Value::from(CARDINALITY_ONE));
        entity
    });
    entities.insert(UNIQUE, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, UNIQUE.into());
        entity.insert(IDENT, Value::from("db/unique"));
        entity.insert(VALUE_TYPE, Value::from(TYPE_BOOLEAN));
        entity.insert(CARDINALITY, Value::from(CARDINALITY_ONE));
        entity
    });
    entities.insert(IS_COMPONENT, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, IS_COMPONENT.into());
        entity.insert(IDENT, Value::from("db/is-component"));
        entity.insert(VALUE_TYPE, Value::from(TYPE_BOOLEAN));
        entity.insert(CARDINALITY, Value::from(CARDINALITY_ONE));
        entity
    });
    entities.insert(CARDINALITY_ONE, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, CARDINALITY_ONE.into());
        entity.insert(IDENT, Value::from("db.cardinality/one"));
        entity
    });
    entities.insert(CARDINALITY_MANY, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, CARDINALITY_MANY.into());
        entity.insert(IDENT, Value::from("db.cardinality/many"));
        entity
    });
    entities.insert(TYPE_STRING, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, TYPE_STRING.into());
        entity.insert(IDENT, Value::from("db.type/string"));
        entity
    });
    entities.insert(TYPE_INTEGER, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, TYPE_INTEGER.into());
        entity.insert(IDENT, Value::from("db.type/integer"));
        entity
    });
    entities.insert(TYPE_DECIMAL, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, TYPE_DECIMAL.into());
        entity.insert(IDENT, Value::from("db.type/decimal"));
        entity
    });
    entities.insert(TYPE_ID, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, TYPE_ID.into());
        entity.insert(IDENT, Value::from("db.type/id"));
        entity
    });
    entities.insert(TYPE_REF, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, TYPE_REF.into());
        entity.insert(IDENT, Value::from("db.type/ref"));
        entity
    });
    entities.insert(TYPE_BOOLEAN, {
        let mut entity = BuiltinEntity::new();
        entity.insert(ID, TYPE_BOOLEAN.into());
        entity.insert(IDENT, Value::from("db.type/boolean"));
        entity
    });
    entities
});

/// The data behind all built-in entities, by ident
pub static BUILTIN_ENTITIES_BY_IDENT: Lazy<BuiltinEntitiesByIdent> = Lazy::new(|| {
    let mut m = HashMap::new();
    for e in BUILTIN_ENTITIES.values() {
        if let Some(Value::String(ident)) = e.get(&IDENT) {
            m.insert(ident.to_owned(), e.to_owned());
        }
    }
    m
});

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn no_duplicates() {
        let mut set = HashSet::new();
        let ids = [
            CARDINALITY,
            CARDINALITY_MANY,
            CARDINALITY_ONE,
            DOC,
            IDENT,
            IS_COMPONENT,
            TYPE_DECIMAL,
            TYPE_ID,
            TYPE_INTEGER,
            TYPE_REF,
            TYPE_STRING,
            UNIQUE,
            VALUE_TYPE,
        ];
        for id in ids {
            assert!(!set.contains(&id));
            set.insert(id);
        }
    }
}
