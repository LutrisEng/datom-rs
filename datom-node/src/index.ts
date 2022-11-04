// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

import * as datom from '../index.node';

export class Connection {
    #impl: datom.ConnectionImpl

    constructor() {
        this.#impl = datom.new_connection();
    }

    latestT(): number {
        return datom.connection_latest_t(this.#impl);
    }
}

export function hello(): string {
    return datom.hello();
}

export const VERSION = datom.version();
