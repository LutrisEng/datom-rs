import Opaque from "ts-opaque";

export type ConnectionImpl = Opaque<'datomic-connection'>;

export function hello(): 'hello node';
export function version(): string;
export function new_connection(): ConnectionImpl;
export function connection_latest_t(connection: ConnectionImpl): number;
