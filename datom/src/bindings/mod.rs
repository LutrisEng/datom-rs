// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

/// Bindings to the C programming language
#[cfg(feature = "c")]
pub mod c;

/// Bindings to the Java programming language and JVM ecosystem
#[cfg(feature = "java")]
pub mod java;
