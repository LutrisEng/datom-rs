// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::QueryError;

use crate::structs::{Database, Datoms, Entity, Index, ID};

#[no_mangle]
pub extern "C" fn datom_datoms<'s>(
    database: &'s Database,
    index: Index,
) -> Option<Box<Datoms<'s>>> {
    let res: Result<Box<Datoms<'s>>, QueryError> = try {
        let iter = database.d.datoms(index.into())?;
        Box::new(iter.into())
    };
    match res {
        Ok(d) => Some(d),
        Err(_) => {
            // update_last_query_error(e.into())
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn datom_entity<'s>(
    database: &'s Database,
    entity: Box<ID>,
) -> Option<Box<Entity<'s>>> {
    let res: Result<Box<Entity<'s>>, QueryError> = try {
        let e = database.d.entity(entity.i.into())?;
        Box::new(e.into())
    };
    match res {
        Ok(d) => Some(d),
        Err(_) => {
            // update_last_query_error(e.into())
            None
        }
    }
}
