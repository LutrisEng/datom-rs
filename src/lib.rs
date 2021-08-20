// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

#![feature(generic_associated_types, map_first_last)]
#![deny(missing_docs)]
#![warn(clippy::nursery, clippy::cargo)]
#![doc(
    html_logo_url = "https://avatars.githubusercontent.com/u/85201395?s=200&v=4",
    html_favicon_url = "https://lutris.engineering/favicon-32x32.png"
)]

/*!
An open-source database inspired by Datomic.

```
// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>
```

Currently built on top of [sled], a modern embedded database, and is
only suitable for embedded usage. Future multi-peer support is
planned.

If you aren't already familiar with Datomic, it might help to look
over [Datomic's excellent documentation]. To the user, datom-rs
functions extremely similarly to Datomic.

```
use datom::{
    prelude::*,
    sled::SledConnection,
    Transaction, ID, Value,
};

// Create a temporary database
let conn = SledConnection::connect_temp()?;

// Create an ID to use for the username attribute
let username = ID::new();
// Create an ID to use for the user's entity
let user = ID::new();

// Create a transaction setting the username attribute on the user entity to "pmc"
let mut tx = Transaction::new();
tx.add(user.into(), username.into(), "pmc".into());
// Execute the transaction using the connection
conn.transact(tx)?;

// Get a view of the database in the current point in time
let db = conn.db()?;
// Get the value of the username attribute on the user entity
if let Some(Value::String(u)) = db.entity(user.into())?.get(username.into())? {
    println!("The user's username is {}.", u);
}
# assert_eq!(db.entity(user.into())?.get(username.into())?, Some("pmc".into()));
# Ok::<(), Box<dyn std::error::Error>>(())
```

[sled]: https://sled.rs
[Datomic's excellent documentation]: https://docs.datomic.com/on-prem/overview/architecture.html
*/

mod connection;
pub use connection::*;

mod database;
pub use database::*;

mod types;
pub use types::*;

/// Serialization/deserialization functions
pub mod serial;

/// IDs for the built-in attributes and other idents
pub mod builtin_idents;

mod merge_iters;
pub use merge_iters::*;

/// [sled](https://sled.rs) storage backend
#[cfg(feature = "sled")]
pub mod sled;

/// C bindings
pub mod c;

/// The main traits which should be in scope when using datom-rs
pub mod prelude {
    pub use crate::{Connection, Database, Entity};
}
