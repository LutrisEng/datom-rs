// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{Transaction, Value, EID};

/// Create a transaction object
#[no_mangle]
pub extern "C" fn datom_transaction_create() -> Box<Transaction> {
    Box::new(Transaction::new())
}

/// Add an attribute value to an entity in a transaction
///
/// Consumes entity, attribute, and value.
///
/// # Safety
///
/// transaction must be a valid, non-null [Transaction] created by
/// [datom_transaction_create].
#[no_mangle]
pub extern "C" fn datom_transaction_add(
    transaction: &mut Transaction,
    entity: Box<EID>,
    attribute: Box<EID>,
    value: Box<Value>,
) {
    transaction.add(*entity, *attribute, *value);
}

/// Retract a specific attribute value from an entity in a transaction
///
/// Consumes entity, attribute, and value.
///
/// # Safety
///
/// transaction must be a valid, non-null [Transaction] created by
/// [datom_transaction_create].
#[no_mangle]
pub extern "C" fn datom_transaction_retract_value(
    transaction: &mut Transaction,
    entity: Box<EID>,
    attribute: Box<EID>,
    value: Box<Value>,
) {
    transaction.retract_value(*entity, *attribute, *value);
}

/// Retract an attribute from an entity, ignoring its value, in a
/// transaction
///
/// Consumes entity and attribute.
///
/// # Safety
///
/// transaction must be a valid, non-null [Transaction] created by
/// [datom_transaction_create].
#[no_mangle]
pub extern "C" fn datom_transaction_retract(
    transaction: &mut Transaction,
    entity: Box<EID>,
    attribute: Box<EID>,
) {
    transaction.retract(*entity, *attribute);
}

/// Destroy a transaction object which wasn't consumed
///
/// # Safety
///
/// transaction must be a valid, non-null [Transaction] created by
/// [datom_transaction_create].
#[no_mangle]
pub unsafe extern "C" fn datom_transaction_destroy(transaction: *mut Transaction) {
    Box::from_raw(transaction);
}
