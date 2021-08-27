// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::DynamicConnection;

pub struct Str {
    pub(crate) s: String,
}

impl From<String> for Str {
    fn from(s: String) -> Self {
        Str { s }
    }
}

impl From<Str> for String {
    fn from(s: Str) -> Self {
        s.s
    }
}

impl<'a> From<&'a Str> for &'a str {
    fn from(s: &'a Str) -> Self {
        s.s.as_str()
    }
}

pub struct Storage {
    pub(crate) s: Box<dyn datom::storage::Storage>,
}

impl<T: datom::storage::Storage + 'static> From<T> for Storage {
    fn from(s: T) -> Self {
        Storage { s: Box::new(s) }
    }
}

impl From<Storage> for Box<dyn datom::storage::Storage> {
    fn from(s: Storage) -> Self {
        s.s
    }
}

impl<'a> From<&'a Storage> for &'a dyn datom::storage::Storage {
    fn from(s: &'a Storage) -> Self {
        &s.s
    }
}

pub struct Connection {
    pub(crate) c: DynamicConnection,
}

impl From<DynamicConnection> for Connection {
    fn from(c: DynamicConnection) -> Self {
        Connection { c }
    }
}

impl From<Connection> for DynamicConnection {
    fn from(c: Connection) -> Self {
        c.c
    }
}

impl<'a> From<&'a Connection> for &'a DynamicConnection {
    fn from(c: &'a Connection) -> Self {
        &c.c
    }
}

pub struct Database<'c> {
    pub(crate) d: datom::Database<'c, Box<dyn datom::storage::Storage>>,
}

impl<'c> From<datom::Database<'c, Box<dyn datom::storage::Storage>>> for Database<'c> {
    fn from(d: datom::Database<'c, Box<dyn datom::storage::Storage>>) -> Self {
        Database { d }
    }
}

impl<'c> From<Database<'c>> for datom::Database<'c, Box<dyn datom::storage::Storage>> {
    fn from(d: Database<'c>) -> Self {
        d.d
    }
}

impl<'c, 'a> From<&'a Database<'c>> for &'a datom::Database<'c, Box<dyn datom::storage::Storage>> {
    fn from(d: &'a Database<'c>) -> Self {
        &d.d
    }
}

pub struct Transaction {
    pub(crate) t: datom::Transaction,
}

impl From<datom::Transaction> for Transaction {
    fn from(t: datom::Transaction) -> Self {
        Transaction { t }
    }
}

impl From<Transaction> for datom::Transaction {
    fn from(t: Transaction) -> Self {
        t.t
    }
}

impl<'a> From<&'a Transaction> for &'a datom::Transaction {
    fn from(t: &'a Transaction) -> Self {
        &t.t
    }
}

pub struct TransactionResult<'s> {
    pub(crate) r: datom::TransactionResult<'s, Box<dyn datom::storage::Storage>>,
}

impl<'s> From<datom::TransactionResult<'s, Box<dyn datom::storage::Storage>>>
    for TransactionResult<'s>
{
    fn from(r: datom::TransactionResult<'s, Box<dyn datom::storage::Storage>>) -> Self {
        TransactionResult { r }
    }
}

impl<'s> From<TransactionResult<'s>>
    for datom::TransactionResult<'s, Box<dyn datom::storage::Storage>>
{
    fn from(r: TransactionResult<'s>) -> Self {
        r.r
    }
}

impl<'s, 'a> From<&'a TransactionResult<'s>>
    for &'a datom::TransactionResult<'s, Box<dyn datom::storage::Storage>>
{
    fn from(r: &'a TransactionResult<'s>) -> Self {
        &r.r
    }
}

pub struct Datoms<'s> {
    pub(crate) d: datom::DatomIterator<'s>,
}

impl<'s> From<datom::DatomIterator<'s>> for Datoms<'s> {
    fn from(d: datom::DatomIterator<'s>) -> Self {
        Datoms { d }
    }
}

impl<'s> From<Datoms<'s>> for datom::DatomIterator<'s> {
    fn from(d: Datoms<'s>) -> Self {
        d.d
    }
}

impl<'s, 'a> From<&'a Datoms<'s>> for &'a datom::DatomIterator<'s> {
    fn from(d: &'a Datoms<'s>) -> Self {
        &d.d
    }
}

#[repr(C)]
pub enum Index {
    EAVT,
    AEVT,
    AVET,
    VAET,
}

impl From<datom::Index> for Index {
    fn from(i: datom::Index) -> Self {
        use datom::Index as DIndex;
        match i {
            DIndex::EAVT => Index::EAVT,
            DIndex::AEVT => Index::AEVT,
            DIndex::AVET => Index::AVET,
            DIndex::VAET => Index::VAET,
        }
    }
}

impl From<Index> for datom::Index {
    fn from(i: Index) -> Self {
        use datom::Index as DIndex;
        match i {
            Index::EAVT => DIndex::EAVT,
            Index::AEVT => DIndex::AEVT,
            Index::AVET => DIndex::AVET,
            Index::VAET => DIndex::VAET,
        }
    }
}

pub struct Entity<'c> {
    pub(crate) e: datom::Entity<'c, Box<dyn datom::storage::Storage>>,
}

impl<'c> From<datom::Entity<'c, Box<dyn datom::storage::Storage>>> for Entity<'c> {
    fn from(e: datom::Entity<'c, Box<dyn datom::storage::Storage>>) -> Self {
        Entity { e }
    }
}

impl<'c> From<Entity<'c>> for datom::Entity<'c, Box<dyn datom::storage::Storage>> {
    fn from(e: Entity<'c>) -> Self {
        e.e
    }
}

impl<'c, 'a> From<&'a Entity<'c>> for &'a datom::Entity<'c, Box<dyn datom::storage::Storage>> {
    fn from(e: &'a Entity<'c>) -> Self {
        &e.e
    }
}

pub struct ID {
    pub(crate) i: datom::ID,
}

impl From<datom::ID> for ID {
    fn from(i: datom::ID) -> Self {
        ID { i }
    }
}

impl From<ID> for datom::ID {
    fn from(i: ID) -> Self {
        i.i
    }
}

impl<'a> From<&'a ID> for &'a datom::ID {
    fn from(i: &'a ID) -> Self {
        &i.i
    }
}
