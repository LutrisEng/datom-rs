#!/bin/bash
# SPDX-FileCopyrightText: 2022 Lutris, Inc
# SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
# SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

set -euxo pipefail

cbindgen --config cbindgen.toml --crate datom-c --output datom.h
