// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#[cfg(feature = "redblacktreeset")]
use datom::backends::RedBlackTreeSetStorage;
#[cfg(feature = "sled")]
use datom::backends::SledStorage;
use datom::{
    backends::TieredStorage, builtin_idents, new_dynamic_connection, AttributeSchema,
    AttributeType, DynamicConnection, EntityResult, Transaction,
};
use once_cell::sync::Lazy;

static ATTRIBUTES: Lazy<Vec<AttributeSchema>> = Lazy::new(|| {
    [
        AttributeSchema::new()
            .ident("user/username".into())
            .value_type(AttributeType::String)
            .doc("The user's unique username".into())
            .unique(),
        AttributeSchema::new()
            .ident("user/admin?".into())
            .value_type(AttributeType::Boolean),
        AttributeSchema::new()
            .ident("user/stripe-customer".into())
            .value_type(AttributeType::Ref)
            .component()
            .unique(),
        AttributeSchema::new()
            .ident("user/friends".into())
            .value_type(AttributeType::Ref)
            .many(),
        AttributeSchema::new()
            .ident("user/repeated-numbers".into())
            .value_type(AttributeType::Integer)
            .many(),
    ]
    .into()
});

pub fn transact_schema(conn: &DynamicConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = Transaction::new();
    for attr in ATTRIBUTES.iter() {
        tx.append(attr.to_owned());
    }
    conn.transact(tx)?;
    Ok(())
}

#[cfg(feature = "sled")]
pub fn sled_connection_with_schema() -> Result<DynamicConnection, Box<dyn std::error::Error>> {
    let storage = SledStorage::connect_temp()?;
    let conn = new_dynamic_connection(storage);
    transact_schema(&conn)?;
    Ok(conn)
}

#[cfg(feature = "redblacktreeset")]
pub fn redblacktreeset_connection_with_schema(
) -> Result<DynamicConnection, Box<dyn std::error::Error>> {
    let storage = RedBlackTreeSetStorage::new();
    let conn = new_dynamic_connection(storage);
    transact_schema(&conn)?;
    Ok(conn)
}

#[cfg(all(feature = "sled", feature = "redblacktreeset"))]
pub fn tiered_connection_with_schema() -> Result<DynamicConnection, Box<dyn std::error::Error>> {
    let a = SledStorage::connect_temp()?;
    let b = RedBlackTreeSetStorage::new();
    let storage = TieredStorage::new(a, b);
    let conn = new_dynamic_connection(storage);
    transact_schema(&conn)?;
    Ok(conn)
}

pub fn with_connection<F: Fn(DynamicConnection) -> Result<(), Box<dyn std::error::Error>>>(
    f: F,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "sled")]
    f(sled_connection_with_schema()?)?;
    #[cfg(feature = "redblacktreeset")]
    f(redblacktreeset_connection_with_schema()?)?;
    #[cfg(all(feature = "sled", feature = "redblacktreeset"))]
    f(tiered_connection_with_schema()?)?;
    Ok(())
}

pub fn schema_transacted_properly(
    conn: &DynamicConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = conn.db()?;
    for attr in ATTRIBUTES.iter() {
        let attr_ent = db.entity(attr.id.into())?;
        assert_eq!(attr_ent.id(), &attr.id);
        assert_eq!(
            attr_ent.get(builtin_idents::ID.into())?,
            EntityResult::Value(attr.id.into())
        );
        if let Some(ident) = &attr.ident {
            assert_eq!(
                attr_ent.get(builtin_idents::IDENT.into())?,
                EntityResult::Value(ident.as_str().into())
            );
        }
        if attr.many {
            assert!(attr_ent
                .get(builtin_idents::CARDINALITY.into())?
                .is_ref_to(&builtin_idents::CARDINALITY_MANY));
        }
        if let Some(t) = attr.value_type {
            assert!(attr_ent
                .get(builtin_idents::VALUE_TYPE.into())?
                .is_ref_to(&t.into()));
        }
        if let Some(doc) = &attr.doc {
            assert_eq!(
                attr_ent.get(builtin_idents::DOC.into())?,
                EntityResult::Value(doc.as_str().into())
            );
        }
        if attr.unique {
            assert_eq!(
                attr_ent.get(builtin_idents::UNIQUE.into())?,
                EntityResult::Value(true.into())
            );
        }
        if attr.component {
            assert_eq!(
                attr_ent.get(builtin_idents::IS_COMPONENT.into())?,
                EntityResult::Value(true.into())
            );
        }
    }
    Ok(())
}
