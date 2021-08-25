// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use chrono::{DateTime, Utc};

/// The record of a past transaction
pub struct TransactionRecord {
    /// The t-value of this transaction
    pub t: u64,
    /// When this transaction was transacted
    pub timestamp: DateTime<Utc>,
}
