// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::collections::HashSet;

use datom::{
    builtin_idents, prelude::*, AttributeSchema, AttributeType, EntityResult, Transaction, Value,
    EID, ID,
};

use datom::sled::*;
use datom_bigdecimal::BigDecimal;

#[test]
fn schema_entity_api() -> Result<(), Box<dyn std::error::Error>> {
    // Catch flakey tests early, since this test involves random IDs
    for _ in 1..5 {
        let conn = SledConnection::connect_temp()?;

        {
            let db = conn.db()?;
            let mut schema_tx = Transaction::new();
            schema_tx.add_many(
                ID::new().into(),
                [
                    ("db/ident".into(), "user/username".into()),
                    (
                        "db/value-type".into(),
                        Value::ID(EID::from("db.type/string").resolve(&db)?),
                    ),
                    (
                        "db/cardinality".into(),
                        Value::ID(EID::from("db.cardinality/one").resolve(&db)?),
                    ),
                    ("db/doc".into(), "The user's unique username".into()),
                    ("db/unique".into(), true.into()),
                ]
                .into(),
            );
            schema_tx.add_many(
                ID::new().into(),
                [
                    (builtin_idents::IDENT.into(), "user/admin?".into()),
                    (
                        builtin_idents::VALUE_TYPE.into(),
                        builtin_idents::TYPE_BOOLEAN.into(),
                    ),
                    (
                        builtin_idents::CARDINALITY.into(),
                        builtin_idents::CARDINALITY_ONE.into(),
                    ),
                ]
                .into(),
            );
            schema_tx.append(
                AttributeSchema::new()
                    .ident("user/first-name".to_string())
                    .value_type(AttributeType::String),
            );
            schema_tx.append(
                AttributeSchema::new()
                    .set_id(ID::new())
                    .ident("user/stripe-customer".to_string())
                    .value_type(AttributeType::Ref)
                    .component()
                    .unique(),
            );
            schema_tx.append(
                AttributeSchema::new()
                    .ident("user/friends".to_string())
                    .value_type(AttributeType::Ref)
                    .many(),
            );
            schema_tx.append(
                AttributeSchema::new()
                    .ident("user/balance".to_string())
                    .value_type(AttributeType::Decimal),
            );
            schema_tx.append(
                AttributeSchema::new()
                    .ident("user/repeated-numbers".to_string())
                    .value_type(AttributeType::Integer)
                    .many(),
            );
            conn.transact(schema_tx)?;
        }

        {
            let db = conn.db()?;
            let username_attribute = db.entity("user/username".into())?;
            assert_eq!(
                username_attribute.get("db/ident".into())?,
                EntityResult::Value("user/username".into()),
            );
            assert_eq!(
                username_attribute.get("db/value-type".into())?,
                builtin_idents::TYPE_STRING,
            );
            assert_eq!(
                username_attribute.get("db/cardinality".into())?,
                builtin_idents::CARDINALITY_ONE,
            );
            let admin_attribute = db.entity("user/admin?".into())?;
            assert_eq!(
                admin_attribute.get("db/ident".into())?,
                EntityResult::Value("user/admin?".into()),
            );
            assert_eq!(
                admin_attribute.get("db/value-type".into())?,
                builtin_idents::TYPE_BOOLEAN,
            );
            assert_eq!(
                admin_attribute.get("db/cardinality".into())?,
                builtin_idents::CARDINALITY_ONE,
            );
            let first_name_attribute = db.entity("user/first-name".into())?;
            assert_eq!(
                first_name_attribute.get("db/ident".into())?,
                EntityResult::Value("user/first-name".into()),
            );
            assert_eq!(
                first_name_attribute.get("db/value-type".into())?,
                builtin_idents::TYPE_STRING,
            );
            let friends_attribute = db.entity("user/friends".into())?;
            assert_eq!(
                friends_attribute.get("db/ident".into())?,
                EntityResult::Value("user/friends".into()),
            );
            assert_eq!(
                friends_attribute.get("db/value-type".into())?,
                builtin_idents::TYPE_REF,
            );
            assert_eq!(
                friends_attribute.get("db/cardinality".into())?,
                builtin_idents::CARDINALITY_MANY,
            );
        }

        let bal = BigDecimal::new(15042.into(), -2);

        {
            let mut user_tx = Transaction::new();
            user_tx.add_many(
                ID::new().into(),
                [
                    ("user/username".into(), "pmc".into()),
                    ("user/admin?".into(), true.into()),
                    ("user/first-name".into(), "Piper".into()),
                    ("user/balance".into(), bal.clone().into()),
                    ("user/repeated-numbers".into(), 1234.into()),
                ]
                .into(),
            );
            conn.transact(user_tx)?;
        }

        let db = conn.db()?;
        let admin = db.entity(EID::Unique(Box::new("user/username".into()), "pmc".into()))?;
        assert_eq!(
            admin.get("user/username".into())?,
            EntityResult::Value("pmc".into())
        );
        assert_eq!(
            admin.get("user/admin?".into())?,
            EntityResult::Value(true.into())
        );
        assert_eq!(
            admin.get("user/first-name".into())?,
            EntityResult::Value("Piper".into())
        );
        assert_eq!(
            admin.get("user/balance".into())?,
            EntityResult::Value(bal.clone().into())
        );
        assert_eq!(
            admin.get("user/repeated-numbers".into())?,
            EntityResult::Repeated(vec![EntityResult::Value(1234.into())])
        );
        assert_eq!(
            admin.get("db/id".into())?,
            EntityResult::Value(
                EID::Unique(Box::new("user/username".into()), "pmc".into())
                    .resolve(&db)?
                    .into()
            ),
        );

        {
            let mut not_admin_tx = Transaction::new();
            not_admin_tx.retract(
                EID::Unique(Box::new("user/username".into()), "pmc".into()),
                "user/admin?".into(),
            );
            conn.transact(not_admin_tx)?;
        }

        {
            let db = conn.db()?;
            let user = db.entity(EID::Unique(Box::new("user/username".into()), "pmc".into()))?;
            assert_eq!(
                user.get("user/username".into())?,
                EntityResult::Value("pmc".into())
            );
            assert_eq!(user.get("user/admin?".into())?, EntityResult::NotFound);
            assert_eq!(
                admin.get("user/admin?".into())?,
                EntityResult::Value(true.into())
            );
            assert_eq!(
                user.get("user/first-name".into())?,
                EntityResult::Value("Piper".into())
            );
            assert_eq!(
                user.get("user/balance".into())?,
                EntityResult::Value(bal.clone().into())
            );
        }

        {
            let db = conn.db()?;
            let user = db.entity(EID::Unique(Box::new("user/username".into()), "pmc".into()))?;
            let mut friend_tx = Transaction::new();
            let friend_id = ID::new();
            friend_tx.add_many(
                friend_id.into(),
                [
                    ("user/username".into(), "friend".into()),
                    ("user/friends".into(), user.id().to_owned().into()),
                ]
                .into(),
            );
            friend_tx.add(friend_id.into(), "user/friends".into(), 4321.into());
            conn.transact(friend_tx)?;
        }

        {
            let db = conn.db()?;
            let user = db.entity(EID::Unique(Box::new("user/username".into()), "pmc".into()))?;
            let friend = db.entity(EID::Unique(
                Box::new("user/username".into()),
                "friend".into(),
            ))?;
            assert_eq!(
                user.get("user/username".into())?,
                EntityResult::Value("pmc".into()),
            );
            assert_eq!(user.get("user/admin?".into())?, EntityResult::NotFound);
            assert_eq!(
                user.get("user/first-name".into())?,
                EntityResult::Value("Piper".into()),
            );
            assert_eq!(
                user.get("user/balance".into())?,
                EntityResult::Value(bal.clone().into())
            );
            assert_eq!(
                friend.get("user/username".into())?,
                EntityResult::Value("friend".into()),
            );
            let friends = friend.get("user/friends".into())?;
            if let EntityResult::Repeated(results) = friends {
                let friend_set: HashSet<EntityResult<SledEntity>> =
                    HashSet::from_iter(results.into_iter());
                assert_eq!(friend_set.len(), 2);
                assert!(friend_set.contains(&user.into()));
                assert!(friend_set.contains(&EntityResult::Value(4321.into())));
            } else {
                panic!();
            }
            assert_eq!(
                user.reverse_get("user/friends".into())?,
                EntityResult::Repeated(vec![friend.into()]),
            );
            let username = db.entity("user/username".into())?.id().to_owned();
            let first_name = db.entity("user/first-name".into())?.id().to_owned();
            let friends = db.entity("user/friends".into())?.id().to_owned();
            let balance = db.entity("user/balance".into())?.id().to_owned();
            let repeated_numbers = db.entity("user/repeated-numbers".into())?.id().to_owned();
            let mut user_attributes = [username, first_name, balance, repeated_numbers];
            user_attributes.sort_by_key(ID::to_string);
            user_attributes.reverse();
            let mut friend_attributes = vec![username, friends];
            friend_attributes.sort_by_key(ID::to_string);
            friend_attributes.reverse();
            assert_eq!(user.attributes()?.collect::<Vec<ID>>(), user_attributes);
            assert_eq!(friend.attributes()?.collect::<Vec<ID>>(), friend_attributes);
        }
    }

    Ok(())
}
