// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::fmt::Debug;

use chrono::Utc;

use crate::{
    builtin_idents,
    serial::{
        deserialize_tr, serialize_aevt, serialize_avet, serialize_eavt, serialize_tr,
        serialize_vaet, tr_range, vec_range_slice,
    },
    storage::Storage,
    ConnectionError, Database, Datom, EntityResult, Index, Transactable, Transaction,
    TransactionError, TransactionRecord, TransactionResult, Value, ID,
};

/// A persistent connection to a database
pub struct Connection<S: Storage> {
    pub(crate) storage: S,
    pub(crate) id: ID,
}

impl<S: Storage> PartialEq<Self> for Connection<S> {
    /// ```
    /// use datom::{backends::SledStorage, Connection};
    /// let storage1 = SledStorage::connect_temp()?;
    /// let storage2 = SledStorage::connect_temp()?;
    /// let conn1 = Connection::new(storage1);
    /// let conn2 = Connection::new(storage2);
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
        if let Some(res) = self.storage.range(vec_range_slice(&tr_range()))?.last() {
            let bytes = res?;
            deserialize_tr(&bytes).map_or(Err(ConnectionError::InvalidData), |tx| Ok(tx.t))
        } else {
            Ok(0)
        }
    }

    /// Fetch the t-value for the latest transaction
    pub const fn as_of(&self, t: u64) -> Result<Database<'_, S>, ConnectionError> {
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

    /// Run a transaction on the database
    pub fn transact_tx(
        &self,
        tx: Transaction,
    ) -> Result<TransactionResult<'_, S>, TransactionError> {
        let t_before = self.latest_t()?;
        let t = t_before + 1;
        let before = self.as_of(t_before)?;
        let data = tx.datoms(t, &before)?;
        let mut items: Vec<Vec<u8>> = vec![];
        for datom in data.iter() {
            items.push(serialize_eavt(datom));
            items.push(serialize_aevt(datom));
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
                items.push(serialize_avet(datom));
            }
            if is_ref {
                items.push(serialize_vaet(datom));
            }
        }
        items.push(serialize_tr(&TransactionRecord {
            t,
            timestamp: Utc::now(),
        }));
        self.storage.insert(&items).map_err(ConnectionError::from)?;
        Ok(TransactionResult {
            connection: self,
            before,
            after: self.as_of(t)?,
            data,
        })
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
    /// use datom::{Connection, backends::SledStorage};
    ///
    /// let storage = SledStorage::connect_temp()?;
    /// let conn = Connection::new(storage);
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
