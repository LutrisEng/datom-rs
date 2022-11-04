// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

const datom = require('../dist/index');

test('can require the module', () => {
    require('../dist/index');
});

test('hello world works properly', () => {
    expect(datom.hello()).toBe('hello node');
});

test('version returns the current version', () => {
    expect(datom.VERSION).toBe('0.1.1-pre4');
});

test('can create a connection', () => {
    const connection = new datom.Connection();
    expect(connection.latestT()).toBe(0);
});
