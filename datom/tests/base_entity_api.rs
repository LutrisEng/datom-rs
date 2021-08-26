// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#![feature(once_cell)]

mod common;

use common::{
    data::{transact_users, users_transacted_properly},
    schema::{schema_transacted_properly, with_connection},
};
use datom::{EntityResult, Transaction, TransactionError, EID};

#[test]
fn schema_only() -> Result<(), Box<dyn std::error::Error>> {
    with_connection(|conn| schema_transacted_properly(&conn))
}

#[test]
fn users() -> Result<(), Box<dyn std::error::Error>> {
    with_connection(|conn| {
        schema_transacted_properly(&conn)?;
        transact_users(&conn)?;
        users_transacted_properly(&conn)?;
        Ok(())
    })
}

#[test]
fn retract_repeated_value() -> Result<(), Box<dyn std::error::Error>> {
    with_connection(|conn| {
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
    })
}
