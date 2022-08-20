@rem SPDX-FileCopyrightText: 2022 Lutris, Inc
@rem SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
@rem SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

cbindgen --config cbindgen.toml --crate datom-c --output datom.h
