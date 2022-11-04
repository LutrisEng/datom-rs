// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

import Opaque from "ts-opaque";

export type ConnectionImpl = Opaque<'datomic-connection'>;

export function hello(): 'hello node';
export function version(): string;
export function new_connection(): ConnectionImpl;
export function connection_latest_t(connection: ConnectionImpl): number;
