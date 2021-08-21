// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{
    builtin_idents, prelude::*, AttributeSchema, AttributeType, EntityResult, Transaction, Value,
    EID, ID,
};

use datom::sled::*;

#[test]
fn entity_api() -> Result<(), Box<dyn std::error::Error>> {
    let conn = SledConnection::connect_temp()?;
    let username_attr = ID::new();
    let bio_attr = ID::new();
    let mut both_attrs = vec![username_attr, bio_attr];
    both_attrs.sort();
    both_attrs.reverse();

    let pmc_ent = ID::new();
    let ztaylor_ent = ID::new();

    let mut initial_tx = Transaction::new();
    initial_tx.add(pmc_ent.into(), username_attr.into(), "pmc".into());
    initial_tx.add(ztaylor_ent.into(), username_attr.into(), "ztaylor54".into());
    let initial_tx_result = conn.transact(initial_tx)?;

    let before_initial_tx = &initial_tx_result.before;
    let after_initial_tx = &initial_tx_result.after;
    let pmc_before_initial_tx = before_initial_tx.entity(pmc_ent.into())?;
    let pmc_after_initial_tx = after_initial_tx.entity(pmc_ent.into())?;
    let ztaylor_before_initial_tx = before_initial_tx.entity(pmc_ent.into())?;
    let ztaylor_after_initial_tx = after_initial_tx.entity(ztaylor_ent.into())?;

    {
        let pmc_username_before_initial_tx = pmc_before_initial_tx.get(username_attr.into())?;
        assert_eq!(pmc_username_before_initial_tx, EntityResult::NotFound);
        let pmc_username_after_initial_tx = pmc_after_initial_tx.get(username_attr.into())?;
        assert_eq!(
            pmc_username_after_initial_tx,
            EntityResult::Value("pmc".into())
        );

        let ztaylor_username_before_initial_tx =
            ztaylor_before_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_before_initial_tx, EntityResult::NotFound);
        let ztaylor_username_after_initial_tx =
            ztaylor_after_initial_tx.get(username_attr.into())?;
        assert_eq!(
            ztaylor_username_after_initial_tx,
            EntityResult::Value("ztaylor54".into())
        );

        assert_eq!(
            pmc_before_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![]
        );
        assert_eq!(
            pmc_after_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![username_attr]
        );
        assert_eq!(
            ztaylor_before_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![]
        );
        assert_eq!(
            ztaylor_after_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![username_attr]
        );
    }

    let mut add_pmc_bio_tx = Transaction::new();
    add_pmc_bio_tx.add(pmc_ent.into(), bio_attr.into(), "Hi! I'm a person!".into());
    let add_pmc_bio_tx_result = conn.transact(add_pmc_bio_tx)?;

    {
        // Ensure this all still works
        let pmc_username_before_initial_tx = pmc_before_initial_tx.get(username_attr.into())?;
        assert_eq!(pmc_username_before_initial_tx, EntityResult::NotFound);
        let pmc_username_after_initial_tx = pmc_after_initial_tx.get(username_attr.into())?;
        assert_eq!(
            pmc_username_after_initial_tx,
            EntityResult::Value("pmc".into())
        );

        let ztaylor_username_before_initial_tx =
            ztaylor_before_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_before_initial_tx, EntityResult::NotFound);
        let ztaylor_username_after_initial_tx =
            ztaylor_after_initial_tx.get(username_attr.into())?;
        assert_eq!(
            ztaylor_username_after_initial_tx,
            EntityResult::Value("ztaylor54".into())
        );

        assert_eq!(
            pmc_before_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![]
        );
        assert_eq!(
            pmc_after_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![username_attr]
        );
        assert_eq!(
            ztaylor_before_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![]
        );
        assert_eq!(
            ztaylor_after_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![username_attr]
        );
    }

    let after_add_pmc_bio_tx = &add_pmc_bio_tx_result.after;
    let pmc_before_add_pmc_bio_tx = &pmc_after_initial_tx;
    let pmc_after_add_pmc_bio_tx = after_add_pmc_bio_tx.entity(pmc_ent.into())?;

    {
        let pmc_bio_before_add_pmc_bio_tx = pmc_before_add_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(pmc_bio_before_add_pmc_bio_tx, EntityResult::NotFound);
        let pmc_bio_after_add_pmc_bio_tx = pmc_after_add_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(
            pmc_bio_after_add_pmc_bio_tx,
            EntityResult::Value("Hi! I'm a person!".into())
        );

        assert_eq!(
            pmc_before_add_pmc_bio_tx.attributes()?.collect::<Vec<ID>>(),
            vec![username_attr]
        );
        assert_eq!(
            pmc_after_add_pmc_bio_tx.attributes()?.collect::<Vec<ID>>(),
            both_attrs
        );
    }

    let mut retract_pmc_bio_tx = Transaction::new();
    retract_pmc_bio_tx.retract(pmc_ent.into(), bio_attr.into());
    let retract_pmc_bio_result = conn.transact(retract_pmc_bio_tx)?;

    {
        // Ensure this all still works
        let pmc_username_before_initial_tx = pmc_before_initial_tx.get(username_attr.into())?;
        assert_eq!(pmc_username_before_initial_tx, EntityResult::NotFound);
        let pmc_username_after_initial_tx = pmc_after_initial_tx.get(username_attr.into())?;
        assert_eq!(
            pmc_username_after_initial_tx,
            EntityResult::Value("pmc".into())
        );

        let ztaylor_username_before_initial_tx =
            ztaylor_before_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_before_initial_tx, EntityResult::NotFound);
        let ztaylor_username_after_initial_tx =
            ztaylor_after_initial_tx.get(username_attr.into())?;
        assert_eq!(
            ztaylor_username_after_initial_tx,
            EntityResult::Value("ztaylor54".into())
        );

        assert_eq!(
            pmc_before_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![]
        );
        assert_eq!(
            pmc_after_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![username_attr]
        );
        assert_eq!(
            ztaylor_before_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![]
        );
        assert_eq!(
            ztaylor_after_initial_tx.attributes()?.collect::<Vec<ID>>(),
            vec![username_attr]
        );
    }

    {
        // Ensure this all still works
        let pmc_bio_before_add_pmc_bio_tx = pmc_before_add_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(pmc_bio_before_add_pmc_bio_tx, EntityResult::NotFound);
        let pmc_bio_after_add_pmc_bio_tx = pmc_after_add_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(
            pmc_bio_after_add_pmc_bio_tx,
            EntityResult::Value("Hi! I'm a person!".into())
        );

        assert_eq!(
            pmc_before_add_pmc_bio_tx.attributes()?.collect::<Vec<ID>>(),
            vec![username_attr]
        );
        assert_eq!(
            pmc_after_add_pmc_bio_tx.attributes()?.collect::<Vec<ID>>(),
            both_attrs
        );
    }

    let after_retract_pmc_bio_tx = &retract_pmc_bio_result.after;
    let pmc_before_retract_pmc_bio_tx = &pmc_after_add_pmc_bio_tx;
    let pmc_after_retract_pmc_bio_tx = after_retract_pmc_bio_tx.entity(pmc_ent.into())?;

    {
        let pmc_bio_before_retract_pmc_bio_tx =
            pmc_before_retract_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(
            pmc_bio_before_retract_pmc_bio_tx,
            EntityResult::Value("Hi! I'm a person!".into())
        );
        let pmc_bio_after_retract_pmc_bio_tx = pmc_after_retract_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(pmc_bio_after_retract_pmc_bio_tx, EntityResult::NotFound);

        assert_eq!(
            pmc_before_retract_pmc_bio_tx
                .attributes()?
                .collect::<Vec<ID>>(),
            both_attrs
        );
        assert_eq!(
            pmc_after_retract_pmc_bio_tx
                .attributes()?
                .collect::<Vec<ID>>(),
            vec![username_attr]
        );
    }

    Ok(())
}

#[test]
fn schema_entity_api() -> Result<(), Box<dyn std::error::Error>> {
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
                ("db/unique".into(), true.into()),
            ]
            .into(),
        );
        schema_tx.add_many(
            ID::new().into(),
            [
                (builtin_idents::ident().into(), "user/admin?".into()),
                (
                    builtin_idents::value_type().into(),
                    builtin_idents::type_boolean().into(),
                ),
                (
                    builtin_idents::cardinality().into(),
                    builtin_idents::cardinality_one().into(),
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
                .ident("user/friends".to_string())
                .value_type(AttributeType::Ref)
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
            builtin_idents::type_string(),
        );
        assert_eq!(
            username_attribute.get("db/cardinality".into())?,
            builtin_idents::cardinality_one(),
        );
        let admin_attribute = db.entity("user/admin?".into())?;
        assert_eq!(
            admin_attribute.get("db/ident".into())?,
            EntityResult::Value("user/admin?".into()),
        );
        assert_eq!(
            admin_attribute.get("db/value-type".into())?,
            builtin_idents::type_boolean(),
        );
        assert_eq!(
            admin_attribute.get("db/cardinality".into())?,
            builtin_idents::cardinality_one(),
        );
        let first_name_attribute = db.entity("user/first-name".into())?;
        assert_eq!(
            first_name_attribute.get("db/ident".into())?,
            EntityResult::Value("user/first-name".into()),
        );
        assert_eq!(
            first_name_attribute.get("db/value-type".into())?,
            builtin_idents::type_string(),
        );
        let friends_attribute = db.entity("user/friends".into())?;
        assert_eq!(
            friends_attribute.get("db/ident".into())?,
            EntityResult::Value("user/friends".into()),
        );
        assert_eq!(
            friends_attribute.get("db/value-type".into())?,
            builtin_idents::type_ref(),
        );
        assert_eq!(
            friends_attribute.get("db/cardinality".into())?,
            builtin_idents::cardinality_many(),
        );
    }

    {
        let mut user_tx = Transaction::new();
        user_tx.add_many(
            ID::new().into(),
            [
                ("user/username".into(), "pmc".into()),
                ("user/admin?".into(), true.into()),
                ("user/first-name".into(), "Piper".into()),
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
    }

    {
        let db = conn.db()?;
        let user = db.entity(EID::Unique(Box::new("user/username".into()), "pmc".into()))?;
        let mut friend_tx = Transaction::new();
        friend_tx.add_many(
            ID::new().into(),
            [
                ("user/username".into(), "friend".into()),
                ("user/friends".into(), user.id().into()),
            ]
            .into(),
        );
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
            friend.get("user/username".into())?,
            EntityResult::Value("friend".into()),
        );
        assert_eq!(
            friend.get("user/friends".into())?,
            EntityResult::Repeated(vec![user.clone().into()]),
        );
        // assert_eq!(
        //     user.reverse_get("user/friends".into())?,
        //     EntityResult::Repeated(vec![EntityResult::Ref(friend)]),
        // );
    }

    Ok(())
}
