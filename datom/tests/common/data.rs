// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::HashSet;

use datom::{
    builtin_idents, storage::Storage, Connection, Database, DynamicConnection, EntityResult,
    Transactable, Transaction, Value, ID,
};
use miette::DiagnosticResult;
use num_bigint::BigInt;
use once_cell::sync::Lazy;

struct User {
    id: ID,
    username: Option<String>,
    admin: Option<bool>,
    stripe_customer: Option<ID>,
    friends: Vec<Value>,
    repeated_numbers: Vec<BigInt>,
}

impl Transactable for &User {
    fn tx(&self) -> Transaction {
        let mut tx = Transaction::new();
        let id = self.id;
        if let Some(username) = &self.username {
            tx.add(id.into(), "user/username".into(), username.as_str().into());
        }
        if let Some(admin) = self.admin {
            tx.add(id.into(), "user/admin?".into(), admin.into());
        }
        if let Some(customer) = self.stripe_customer {
            tx.add(id.into(), "user/stripe-customer".into(), customer.into());
        }
        for friend in self.friends.iter() {
            tx.add(id.into(), "user/friends".into(), friend.to_owned());
        }
        for number in self.repeated_numbers.iter() {
            tx.add(id.into(), "user/repeated-numbers".into(), number.into());
        }
        tx
    }
}

static USERS: Lazy<Vec<User>> = Lazy::new(|| {
    [User {
        id: ID::new(),
        username: Some("pmc".into()),
        admin: Some(true),
        stripe_customer: None,
        friends: vec![],
        repeated_numbers: vec![1234.into(), 5678.into()],
    }]
    .into()
});

pub fn transact_users(conn: &DynamicConnection) -> DiagnosticResult<()> {
    let mut tx = Transaction::new();
    for user in USERS.iter() {
        tx.append(user);
    }
    conn.transact(tx)?;
    Ok(())
}

pub fn db_users_transacted_properly<S: Storage>(db: &Database<'_, S>) -> DiagnosticResult<()> {
    for user in USERS.iter() {
        let user_ent = db.entity(user.id.into())?;
        assert_eq!(user_ent.id(), &user.id);
        assert_eq!(
            user_ent.get(builtin_idents::ID.into())?,
            EntityResult::Value(user.id.into())
        );
        if let Some(username) = &user.username {
            assert_eq!(
                user_ent.get("user/username".into())?,
                EntityResult::Value(username.as_str().into())
            );
        }
        if let Some(admin) = user.admin {
            assert_eq!(
                user_ent.get("user/admin?".into())?,
                EntityResult::Value(admin.into())
            );
        }
        if let Some(stripe_customer) = user.stripe_customer {
            assert!(user_ent
                .get("user/stripe-customer".into())?
                .is_ref_to(&stripe_customer));
        }
        let mut friends = HashSet::new();
        if let EntityResult::Repeated(results) = user_ent.get("user/friends".into())? {
            for res in results.into_iter() {
                friends.insert(res);
            }
        } else {
            panic!();
        }
        for friend in user.friends.iter() {
            if let Value::ID(id) = friend {
                assert!(friends.contains(&EntityResult::Ref(db.entity(id.to_owned().into())?)));
            } else {
                assert!(friends.contains(&EntityResult::Value(friend.to_owned())));
            }
        }
        let mut numbers = HashSet::new();
        if let EntityResult::Repeated(results) = user_ent.get("user/repeated-numbers".into())? {
            for res in results.into_iter() {
                numbers.insert(res);
            }
        } else {
            panic!();
        }
        for number in user.repeated_numbers.iter() {
            assert!(numbers.contains(&EntityResult::Value(number.into())));
        }
    }
    Ok(())
}

pub fn users_transacted_properly<S: Storage>(conn: &Connection<S>) -> DiagnosticResult<()> {
    let db = conn.db()?;
    db_users_transacted_properly(&db)
}
