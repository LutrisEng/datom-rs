#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2022 Lutris, Inc
# SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
# SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

set -euxo pipefail

rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add x86_64-pc-windows-gnu
