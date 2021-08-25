// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::fmt::Debug;

use crate::{
    builtin_idents, serial::serialize, storage::Storage, ConnectionError, Database, Datom,
    EntityResult, Index, Transactable, Transaction, TransactionError, TransactionResult, Value, ID,
};

/// A persistent connection to a database
pub struct Connection<S: Storage> {
    pub(crate) storage: S,
    pub(crate) id: ID,
}

impl<S: Storage> PartialEq<Self> for Connection<S> {
    /// ```
    /// use datom::sled::*;
    /// let conn1 = SledConnection::connect_temp()?;
    /// let conn2 = SledConnection::connect_temp()?;
    /// let conn1r = &conn1;
    /// let conn2r = &conn2;
    ///
    /// assert_eq!(&conn1, &conn1);
    /// assert_eq!(&conn1, conn1r);
    /// assert_eq!(conn1r, &conn1);
    /// assert_eq!(conn1r, conn1r);
    ///
    /// assert_ne!(&conn1, &conn2);
    /// assert_ne!(&conn1, conn2r);
    /// assert_ne!(conn1r, &conn2);
    /// assert_ne!(conn1r, conn2r);
    /// assert_ne!(&conn2, &conn1);
    /// assert_ne!(&conn2, conn1r);
    /// assert_ne!(conn2r, &conn1);
    /// assert_ne!(conn2r, conn1r);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<S: Storage> Eq for Connection<S> {}

/// A connection which uses a dynamically dispatched storage backend
pub type DynamicConnection = Connection<Box<dyn Storage>>;

/// Create a new connection which uses a dynamically dispatched storage
/// backend
pub fn new_dynamic_connection<S: Storage + 'static>(storage: S) -> DynamicConnection {
    Connection::new(Box::new(storage))
}

impl<S: Storage> Connection<S> {
    /// Create a new connection from a storage backend
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            id: ID::new(),
        }
    }

    /// Fetch the t-value for the latest transaction
    pub fn latest_t(&self) -> Result<u64, ConnectionError> {
        Ok(0)
    }

    fn set_t(&self, _: u64) -> Result<(), ConnectionError> {
        todo!()
    }

    /// Fetch the t-value for the latest transaction
    pub fn as_of(&self, t: u64) -> Result<Database<'_, S>, ConnectionError> {
        Ok(Database {
            connection: self,
            t,
        })
    }

    /// Get a [database](crate::database::Database) for the current
    /// point in time
    pub fn db(&self) -> Result<Database<'_, S>, ConnectionError> {
        self.as_of(self.latest_t()?)
    }

    /// Get a [database](crate::database::Database) for a specific point
    /// in time
    pub fn insert(&self, datom: &Datom, index: Index) -> Result<(), ConnectionError> {
        self.storage
            .insert(serialize(datom, index))
            .map_err(|e| e.into())
    }

    /// Run a transaction on the database
    pub fn transact_tx(
        &self,
        tx: Transaction,
    ) -> Result<TransactionResult<'_, S>, TransactionError> {
        let res = {
            let t_before = self.latest_t()?;
            let t = t_before + 1;
            let before = self.as_of(t_before)?;
            let data = tx.datoms(t, &before)?;
            for datom in data.iter() {
                self.insert(datom, Index::EAVT)?;
                self.insert(datom, Index::AEVT)?;
                let attr_entity = before.entity(datom.attribute.into())?;
                let unique_value =
                    attr_entity.get_with_options(builtin_idents::UNIQUE.into(), true, true)?;
                let type_value =
                    attr_entity.get_with_options(builtin_idents::VALUE_TYPE.into(), true, true)?;
                let is_unique = {
                    if let EntityResult::Value(Value::Boolean(x)) = unique_value {
                        x
                    } else {
                        false
                    }
                };
                let is_ref = {
                    if let EntityResult::Value(Value::ID(id)) = type_value {
                        id == builtin_idents::TYPE_REF
                    } else {
                        false
                    }
                };
                if is_unique {
                    self.insert(datom, Index::AVET)?;
                }
                if is_ref {
                    self.insert(datom, Index::VAET)?;
                }
            }
            self.set_t(t)?;
            Ok(TransactionResult {
                connection: self,
                before,
                after: self.as_of(t)?,
                data,
            })
        };
        res
    }

    /// Transact a transactable on the database
    pub fn transact<T: Transactable>(
        &self,
        txable: T,
    ) -> Result<TransactionResult<'_, S>, TransactionError> {
        self.transact_tx(txable.tx())
    }
}

impl<S: Storage> Debug for Connection<S> {
    /// ```
    /// use datom::sled::*;
    ///
    /// let conn = SledConnection::connect_temp()?;
    /// println!("{:#?}", conn);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("id", &self.id)
            .field(
                "EAVT",
                &(|| {
                    Ok::<Vec<Datom>, Box<dyn std::error::Error>>(
                        self.db()?.datoms(Index::EAVT)?.collect(),
                    )
                })(),
            )
            .field(
                "AEVT",
                &(|| {
                    Ok::<Vec<Datom>, Box<dyn std::error::Error>>(
                        self.db()?.datoms(Index::AEVT)?.collect(),
                    )
                })(),
            )
            .field(
                "AVET",
                &(|| {
                    Ok::<Vec<Datom>, Box<dyn std::error::Error>>(
                        self.db()?.datoms(Index::AVET)?.collect(),
                    )
                })(),
            )
            .field(
                "VAET",
                &(|| {
                    Ok::<Vec<Datom>, Box<dyn std::error::Error>>(
                        self.db()?.datoms(Index::VAET)?.collect(),
                    )
                })(),
            )
            .finish()
    }
}
