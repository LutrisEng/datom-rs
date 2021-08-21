// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::HashMap;

use crate::{Value, ID};

/**
An entity's [ID]. This is a virtual attribute, and isn't actually stored.

```
let id = datom::builtin_idents::id();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn id() -> ID {
    329992551406372030633500533120122732713u128.into()
}

/**
An entity's alias, usually used for the attribute schema.

```
let id = datom::builtin_idents::ident();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn ident() -> ID {
    265682209113858765770461024079827500234u128.into()
}

/**
Whether an attribute associates one value or multiple values with an
entity

```
let id = datom::builtin_idents::cardinality();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn cardinality() -> ID {
    110064945635332807503383834157761461043u128.into()
}

/**
The intended type for an attribute. Note that this is not
(currently) strictly checked.

```
let id = datom::builtin_idents::value_type();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn value_type() -> ID {
    276059213908560386420175049892299151374u128.into()
}

/**
A documentation string for an entity

```
let id = datom::builtin_idents::doc();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn doc() -> ID {
    303289866496710859530474533904741988829u128.into()
}

/**
Whether there can only be one entity per value for this attribute

```
let id = datom::builtin_idents::unique();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn unique() -> ID {
    307615836394596470679724073561969695989.into()
}

/**
Whether the entity referred to in this [type_ref](self::type_ref) attribute is a
sub-component. When you retract an entity, all sub-components will
also be retracted.

```
let id = datom::builtin_idents::is_component();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn is_component() -> ID {
    308724514559417715856375983930347810391u128.into()
}

/**
A value for the [cardinality](self::cardinality) attribute

```
let id = datom::builtin_idents::cardinality_one();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn cardinality_one() -> ID {
    143444949937465711736574828873158396909u128.into()
}

/**
A value for the [cardinality](self::cardinality) attribute

```
let id = datom::builtin_idents::cardinality_many();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn cardinality_many() -> ID {
    11338831660433813835424721536043447369u128.into()
}

/**
A value for the [value_type](self::value_type) attribute

```
let id = datom::builtin_idents::type_string();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn type_string() -> ID {
    301439516182801820546961599694687577507u128.into()
}

/**
A value for the [value_type](self::value_type) attribute

```
let id = datom::builtin_idents::type_integer();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn type_integer() -> ID {
    183876393307651966214416059730593095u128.into()
}

/**
A value for the [value_type](self::value_type) attribute

```
let id = datom::builtin_idents::type_decimal();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn type_decimal() -> ID {
    297077785792417755741249562058415972414u128.into()
}

/**
A value for the [value_type](self::value_type) attribute

```
let id = datom::builtin_idents::type_id();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn type_id() -> ID {
    339681506578292470250558610134765439055u128.into()
}

/**
A value for the [value_type](self::value_type) attribute

```
let id = datom::builtin_idents::type_ref();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn type_ref() -> ID {
    149893903729185565330222631892178876560u128.into()
}

/**
A value for the [value_type](self::value_type) attribute

```
let id = datom::builtin_idents::type_ref();
let as_int: u128 = id.into();
let as_bytes: [u8; 16] = id.into();
assert_eq!(id, as_int.into());
assert_eq!(id, as_bytes.into());
```
*/
#[inline]
pub fn type_boolean() -> ID {
    149893903729185565330222631892178876560u128.into()
}

/// The data behind a built-in entity
pub type BuiltinEntity = HashMap<ID, Value>;

/// The data behind all built-in entities
pub type BuiltinEntities = HashMap<ID, BuiltinEntity>;

/// Get the data behind the built-in entities
pub fn get_builtin_entities() -> BuiltinEntities {
    [
        (
            id(),
            [
                (id(), id().into()),
                (ident(), Value::from("db/id")),
                (value_type(), Value::from(type_id())),
                (cardinality(), Value::from(cardinality_one())),
            ]
            .into(),
        ),
        (
            ident(),
            [
                (id(), ident().into()),
                (ident(), Value::from("db/ident")),
                (unique(), Value::from(true)),
                (value_type(), Value::from(type_string())),
                (cardinality(), Value::from(cardinality_one())),
            ]
            .into(),
        ),
        (
            cardinality(),
            [
                (id(), cardinality().into()),
                (ident(), Value::from("db/cardinality")),
                (value_type(), Value::from(type_ref())),
                (cardinality(), Value::from(cardinality_one())),
            ]
            .into(),
        ),
        (
            value_type(),
            [
                (id(), value_type().into()),
                (ident(), Value::from("db/value-type")),
                (value_type(), Value::from(type_ref())),
                (cardinality(), Value::from(cardinality_one())),
            ]
            .into(),
        ),
        (
            doc(),
            [
                (id(), doc().into()),
                (ident(), Value::from("db/doc")),
                (value_type(), Value::from(type_string())),
                (cardinality(), Value::from(cardinality_one())),
            ]
            .into(),
        ),
        (
            unique(),
            [
                (id(), unique().into()),
                (ident(), Value::from("db/unique")),
                (value_type(), Value::from(type_boolean())),
                (cardinality(), Value::from(cardinality_one())),
            ]
            .into(),
        ),
        (
            is_component(),
            [
                (id(), is_component().into()),
                (ident(), Value::from("db/is-component")),
                (value_type(), Value::from(type_boolean())),
                (cardinality(), Value::from(cardinality_one())),
            ]
            .into(),
        ),
        (
            cardinality_one(),
            [
                (id(), cardinality_one().into()),
                (ident(), Value::from("db.cardinality/one")),
            ]
            .into(),
        ),
        (
            cardinality_many(),
            [
                (id(), cardinality_many().into()),
                (ident(), Value::from("db.cardinality/many")),
            ]
            .into(),
        ),
        (
            type_string(),
            [
                (id(), type_string().into()),
                (ident(), Value::from("db.type/string")),
            ]
            .into(),
        ),
        (
            type_integer(),
            [
                (id(), type_integer().into()),
                (ident(), Value::from("db.type/integer")),
            ]
            .into(),
        ),
        (
            type_decimal(),
            [
                (id(), type_decimal().into()),
                (ident(), Value::from("db.type/decimal")),
            ]
            .into(),
        ),
        (
            type_id(),
            [
                (id(), type_id().into()),
                (ident(), Value::from("db.type/id")),
            ]
            .into(),
        ),
        (
            type_ref(),
            [
                (id(), type_ref().into()),
                (ident(), Value::from("db.type/ref")),
            ]
            .into(),
        ),
        (
            type_boolean(),
            [
                (id(), type_boolean().into()),
                (ident(), Value::from("db.type/boolean")),
            ]
            .into(),
        ),
    ]
    .into()
}

/// Get a built-in attribute ID by its ident
///
/// ```
/// use datom::builtin_idents;
/// assert_eq!(builtin_idents::builtin_by_ident("db/id"), Some(builtin_idents::id()));
/// assert_eq!(builtin_idents::builtin_by_ident("db/ident"), Some(builtin_idents::ident()));
/// assert_eq!(builtin_idents::builtin_by_ident("db/value-type"), Some(builtin_idents::value_type()));
/// assert_eq!(builtin_idents::builtin_by_ident("not a built-in ident"), None);
/// ```
pub fn builtin_by_ident(ident_str: &str) -> Option<ID> {
    let builtins = get_builtin_entities();
    builtins
        .values()
        .find(|ent| ent.get(&ident()) == Some(&ident_str.into()))
        .and_then(|ent| ent.get(&id()))
        .and_then(|id| {
            if let Value::ID(id) = id {
                Some(id.to_owned())
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn no_duplicates() {
        let mut set = HashSet::new();
        let ids = [
            cardinality(),
            cardinality_many(),
            cardinality_one(),
            doc(),
            ident(),
            is_component(),
            type_decimal(),
            type_id(),
            type_integer(),
            type_ref(),
            type_string(),
            unique(),
            value_type(),
        ];
        for id in ids {
            assert!(!set.contains(&id));
            set.insert(id);
        }
    }
}
