// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

mod common;

use common::{
    data::{db_users_transacted_properly, transact_users, users_transacted_properly},
    schema::{schema_transacted_properly, with_connection},
};
use datom::{EntityResult, Transaction, TransactionError, EID};
use miette::DiagnosticResult;

#[test]
fn schema_only() -> DiagnosticResult<()> {
    with_connection(|conn| schema_transacted_properly(&conn))
}

#[test]
fn users() -> DiagnosticResult<()> {
    with_connection(|conn| {
        schema_transacted_properly(&conn)?;
        transact_users(&conn)?;
        users_transacted_properly(&conn)?;
        Ok(())
    })
}

#[test]
fn retract_repeated_value() -> DiagnosticResult<()> {
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

#[test]
fn database_is_persistent() -> DiagnosticResult<()> {
    with_connection(|conn| {
        schema_transacted_properly(&conn)?;
        transact_users(&conn)?;
        users_transacted_properly(&conn)?;

        let before = conn.db()?;
        db_users_transacted_properly(&before)?;

        let user = before.entity(EID::unique("user/username".into(), "pmc".into()))?;
        assert_eq!(
            user.get("user/admin?".into())?,
            EntityResult::Value(true.into())
        );

        let mut tx = Transaction::new();
        tx.add(
            EID::unique("user/username".into(), "pmc".into()),
            "user/admin?".into(),
            false.into(),
        );
        conn.transact(tx)?;

        let after = conn.db()?;
        db_users_transacted_properly(&before)?;

        let user = before.entity(EID::unique("user/username".into(), "pmc".into()))?;
        assert_eq!(
            user.get("user/admin?".into())?,
            EntityResult::Value(true.into())
        );

        let user = after.entity(EID::unique("user/username".into(), "pmc".into()))?;
        assert_eq!(
            user.get("user/admin?".into())?,
            EntityResult::Value(false.into())
        );

        Ok(())
    })
}
