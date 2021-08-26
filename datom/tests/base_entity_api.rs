// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#![feature(once_cell)]

mod common;

#[cfg(feature = "redblacktreeset")]
use common::schema::redblacktreeset_connection_with_schema;
#[cfg(feature = "sled")]
use common::schema::sled_connection_with_schema;
use common::{
    data::{transact_users, users_transacted_properly},
    schema::{schema_transacted_properly, tiered_connection_with_schema},
};
use datom::{DynamicConnection, EntityResult, Transaction, TransactionError, EID};

fn schema_only(conn: DynamicConnection) -> Result<(), Box<dyn std::error::Error>> {
    schema_transacted_properly(&conn)?;
    Ok(())
}

#[test]
fn test_schema_only() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "sled")]
    schema_only(sled_connection_with_schema()?)?;
    #[cfg(feature = "redblacktreeset")]
    schema_only(redblacktreeset_connection_with_schema()?)?;
    #[cfg(all(feature = "sled", feature = "redblacktreeset"))]
    schema_only(tiered_connection_with_schema()?)?;
    Ok(())
}

fn users(conn: DynamicConnection) -> Result<(), Box<dyn std::error::Error>> {
    schema_transacted_properly(&conn)?;
    transact_users(&conn)?;
    users_transacted_properly(&conn)?;
    Ok(())
}

#[test]
fn test_users() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "sled")]
    users(sled_connection_with_schema()?)?;
    #[cfg(feature = "redblacktreeset")]
    users(redblacktreeset_connection_with_schema()?)?;
    #[cfg(all(feature = "sled", feature = "redblacktreeset"))]
    users(tiered_connection_with_schema()?)?;
    Ok(())
}

fn retract_repeated_value(conn: DynamicConnection) -> Result<(), Box<dyn std::error::Error>> {
    schema_transacted_properly(&conn)?;
    transact_users(&conn)?;
    users_transacted_properly(&conn)?;

    let mut tx = Transaction::new();
    tx.retract_value(
        EID::unique("user/username".into(), "pmc".into()),
        "user/repeated-numbers".into(),
        5678.into(),
    );
    conn.transact(tx)?;

    let db = conn.db()?;
    let user = db.entity(EID::unique("user/username".into(), "pmc".into()))?;
    assert_eq!(
        user.get("user/repeated-numbers".into())?,
        EntityResult::Repeated(vec![EntityResult::Value(1234.into())])
    );

    let mut tx = Transaction::new();
    tx.retract(
        EID::unique("user/username".into(), "pmc".into()),
        "user/repeated-numbers".into(),
    );
    if let Err(TransactionError::FailedToRetractRepeatedAttribute(_, _)) = conn.transact(tx) {
        // Good, we should get this error!
    } else {
        panic!();
    }

    Ok(())
}

#[test]
fn test_retract_repeated_value() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "sled")]
    retract_repeated_value(sled_connection_with_schema()?)?;
    #[cfg(feature = "redblacktreeset")]
    retract_repeated_value(redblacktreeset_connection_with_schema()?)?;
    #[cfg(all(feature = "sled", feature = "redblacktreeset"))]
    retract_repeated_value(tiered_connection_with_schema()?)?;
    Ok(())
}
