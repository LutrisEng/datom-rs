// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::{collections::HashMap, lazy::SyncLazy};

use crate::{Value, ID as TID};

/**
An entity's [ID]. This is a virtual attribute, and isn't actually stored.
*/

pub static ID: TID = TID::from_u128(329992551406372030633500533120122732713u128);

/**
An entity's alias, usually used for the attribute schema.
*/

pub static IDENT: TID = TID::from_u128(265682209113858765770461024079827500234u128);

/**
Whether an attribute associates one value or multiple values with an
entity
*/

pub static CARDINALITY: TID = TID::from_u128(110064945635332807503383834157761461043u128);

/**
The intended type for an attribute. Note that this is not
(currently) strictly checked.
*/

pub static VALUE_TYPE: TID = TID::from_u128(276059213908560386420175049892299151374u128);

/**
A documentation string for an entity
*/

pub static DOC: TID = TID::from_u128(303289866496710859530474533904741988829u128);

/**
Whether there can only be one entity per value for this attribute
```
*/

pub static UNIQUE: TID = TID::from_u128(307615836394596470679724073561969695989);

/**
Whether the entity referred to in this [type_ref](self::type_ref) attribute is a
sub-component. When you retract an entity, all sub-components will
also be retracted.
*/

pub static IS_COMPONENT: TID = TID::from_u128(308724514559417715856375983930347810391u128);

/**
A value for the [CARDINALITY](self::CARDINALITY) attribute
```
*/

pub static CARDINALITY_ONE: TID = TID::from_u128(143444949937465711736574828873158396909u128);

/**
A value for the [CARDINALITY](self::CARDINALITY) attribute
```
*/

pub static CARDINALITY_MANY: TID = TID::from_u128(11338831660433813835424721536043447369u128);

/**
A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
```
*/

pub static TYPE_STRING: TID = TID::from_u128(301439516182801820546961599694687577507u128);

/**
A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
```
*/

pub static TYPE_INTEGER: TID = TID::from_u128(183876393307651966214416059730593095u128);

/**
A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
```
*/

pub static TYPE_DECIMAL: TID = TID::from_u128(297077785792417755741249562058415972414u128);

/**
A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
```
*/

pub static TYPE_ID: TID = TID::from_u128(339681506578292470250558610134765439055u128);

/**
A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
```
*/

pub static TYPE_REF: TID = TID::from_u128(149893903729185565330222631892178876560u128);

/**
A value for the [VALUE_TYPE](self::VALUE_TYPE) attribute
*/

pub static TYPE_BOOLEAN: TID = TID::from_u128(149893903729185565330222631892178876560u128);

/// The data behind a built-in entity
pub type BuiltinEntity = HashMap<TID, Value>;

/// The type of the data behind all built-in entities
pub type BuiltinEntities = HashMap<TID, BuiltinEntity>;

/// The type of the data behind all built-in entities, by ident
pub type BuiltinEntitiesByIdent = HashMap<String, BuiltinEntity>;

/// The data behind all built-in entities
pub static BUILTIN_ENTITIES: SyncLazy<BuiltinEntities> = SyncLazy::new(|| {
    [
        (
            ID,
            [
                (ID, ID.into()),
                (IDENT, Value::from("db/id")),
                (VALUE_TYPE, Value::from(TYPE_ID)),
                (CARDINALITY, Value::from(CARDINALITY_ONE)),
            ]
            .into(),
        ),
        (
            IDENT,
            [
                (ID, IDENT.into()),
                (IDENT, Value::from("db/ident")),
                (UNIQUE, Value::from(true)),
                (VALUE_TYPE, Value::from(TYPE_STRING)),
                (CARDINALITY, Value::from(CARDINALITY_ONE)),
            ]
            .into(),
        ),
        (
            CARDINALITY,
            [
                (ID, CARDINALITY.into()),
                (IDENT, Value::from("db/cardinality")),
                (VALUE_TYPE, Value::from(TYPE_REF)),
                (CARDINALITY, Value::from(CARDINALITY_ONE)),
            ]
            .into(),
        ),
        (
            VALUE_TYPE,
            [
                (ID, VALUE_TYPE.into()),
                (IDENT, Value::from("db/value-type")),
                (VALUE_TYPE, Value::from(TYPE_REF)),
                (CARDINALITY, Value::from(CARDINALITY_ONE)),
            ]
            .into(),
        ),
        (
            DOC,
            [
                (ID, DOC.into()),
                (IDENT, Value::from("db/doc")),
                (VALUE_TYPE, Value::from(TYPE_STRING)),
                (CARDINALITY, Value::from(CARDINALITY_ONE)),
            ]
            .into(),
        ),
        (
            UNIQUE,
            [
                (ID, UNIQUE.into()),
                (IDENT, Value::from("db/unique")),
                (VALUE_TYPE, Value::from(TYPE_BOOLEAN)),
                (CARDINALITY, Value::from(CARDINALITY_ONE)),
            ]
            .into(),
        ),
        (
            IS_COMPONENT,
            [
                (ID, IS_COMPONENT.into()),
                (IDENT, Value::from("db/is-component")),
                (VALUE_TYPE, Value::from(TYPE_BOOLEAN)),
                (CARDINALITY, Value::from(CARDINALITY_ONE)),
            ]
            .into(),
        ),
        (
            CARDINALITY_ONE,
            [
                (ID, CARDINALITY_ONE.into()),
                (IDENT, Value::from("db.cardinality/one")),
            ]
            .into(),
        ),
        (
            CARDINALITY_MANY,
            [
                (ID, CARDINALITY_MANY.into()),
                (IDENT, Value::from("db.cardinality/many")),
            ]
            .into(),
        ),
        (
            TYPE_STRING,
            [
                (ID, TYPE_STRING.into()),
                (IDENT, Value::from("db.type/string")),
            ]
            .into(),
        ),
        (
            TYPE_INTEGER,
            [
                (ID, TYPE_INTEGER.into()),
                (IDENT, Value::from("db.type/integer")),
            ]
            .into(),
        ),
        (
            TYPE_DECIMAL,
            [
                (ID, TYPE_DECIMAL.into()),
                (IDENT, Value::from("db.type/decimal")),
            ]
            .into(),
        ),
        (
            TYPE_ID,
            [(ID, TYPE_ID.into()), (IDENT, Value::from("db.type/id"))].into(),
        ),
        (
            TYPE_REF,
            [(ID, TYPE_REF.into()), (IDENT, Value::from("db.type/ref"))].into(),
        ),
        (
            TYPE_BOOLEAN,
            [
                (ID, TYPE_BOOLEAN.into()),
                (IDENT, Value::from("db.type/boolean")),
            ]
            .into(),
        ),
    ]
    .into()
});

/// The data behind all built-in entities, by ident
pub static BUILTIN_ENTITIES_BY_IDENT: SyncLazy<BuiltinEntitiesByIdent> = SyncLazy::new(|| {
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
