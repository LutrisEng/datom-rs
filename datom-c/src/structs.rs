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
