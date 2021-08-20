// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{prelude::*, Transaction, ID};

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
        assert_eq!(pmc_username_before_initial_tx, None);
        let pmc_username_after_initial_tx = pmc_after_initial_tx.get(username_attr.into())?;
        assert_eq!(pmc_username_after_initial_tx, Some("pmc".into()));

        let ztaylor_username_before_initial_tx =
            ztaylor_before_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_before_initial_tx, None);
        let ztaylor_username_after_initial_tx =
            ztaylor_after_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_after_initial_tx, Some("ztaylor54".into()));

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
        assert_eq!(pmc_username_before_initial_tx, None);
        let pmc_username_after_initial_tx = pmc_after_initial_tx.get(username_attr.into())?;
        assert_eq!(pmc_username_after_initial_tx, Some("pmc".into()));

        let ztaylor_username_before_initial_tx =
            ztaylor_before_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_before_initial_tx, None);
        let ztaylor_username_after_initial_tx =
            ztaylor_after_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_after_initial_tx, Some("ztaylor54".into()));

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
        assert_eq!(pmc_bio_before_add_pmc_bio_tx, None);
        let pmc_bio_after_add_pmc_bio_tx = pmc_after_add_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(
            pmc_bio_after_add_pmc_bio_tx,
            Some("Hi! I'm a person!".into())
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
        assert_eq!(pmc_username_before_initial_tx, None);
        let pmc_username_after_initial_tx = pmc_after_initial_tx.get(username_attr.into())?;
        assert_eq!(pmc_username_after_initial_tx, Some("pmc".into()));

        let ztaylor_username_before_initial_tx =
            ztaylor_before_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_before_initial_tx, None);
        let ztaylor_username_after_initial_tx =
            ztaylor_after_initial_tx.get(username_attr.into())?;
        assert_eq!(ztaylor_username_after_initial_tx, Some("ztaylor54".into()));

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
        assert_eq!(pmc_bio_before_add_pmc_bio_tx, None);
        let pmc_bio_after_add_pmc_bio_tx = pmc_after_add_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(
            pmc_bio_after_add_pmc_bio_tx,
            Some("Hi! I'm a person!".into())
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
            Some("Hi! I'm a person!".into())
        );
        let pmc_bio_after_retract_pmc_bio_tx = pmc_after_retract_pmc_bio_tx.get(bio_attr.into())?;
        assert_eq!(pmc_bio_after_retract_pmc_bio_tx, None);

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
