// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#![feature(once_cell)]

mod common;

use common::{
    data::{transact_users, users_transacted_properly},
    schema::{connection_with_schema, schema_transacted_properly},
};

#[test]
fn schema_only() -> Result<(), Box<dyn std::error::Error>> {
    let conn = connection_with_schema()?;
    schema_transacted_properly(&conn)?;
    Ok(())
}

#[test]
fn users() -> Result<(), Box<dyn std::error::Error>> {
    let conn = connection_with_schema()?;
    schema_transacted_properly(&conn)?;
    transact_users(&conn)?;
    users_transacted_properly(&conn)?;
    Ok(())
}
