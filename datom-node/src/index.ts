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
