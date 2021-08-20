// SPDX-FileCopyrightText: 2021 Lutris Engineering, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use std::convert::TryInto;

use criterion::{
    black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion, Throughput,
};
use datom::{
    serial::{deserialize_unknown, serialize},
    Datom, DatomType, Index, ID,
};

fn serialize_f(b: &mut Bencher, d: &Datom) {
    b.iter(|| serialize(black_box(d), Index::EAVT))
}

fn deserialize_f(b: &mut Bencher, d: &[u8]) {
    b.iter(|| deserialize_unknown(black_box(d)))
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let string_datom = Datom {
        entity: ID::null(),
        attribute: ID::null(),
        value: "Hello, world! This is a datom!".into(),
        t: 0,
        datom_type: DatomType::Retraction,
    };
    let string_serialized = serialize(&string_datom, Index::EAVT);
    let integer_datom = Datom {
        entity: ID::null(),
        attribute: ID::null(),
        value: 123_456_789.into(),
        t: 0,
        datom_type: DatomType::Retraction,
    };
    let integer_serialized = serialize(&integer_datom, Index::EAVT);
    let decimal_datom = Datom {
        entity: ID::null(),
        attribute: ID::null(),
        value: 123_456_789.101112.try_into().unwrap(),
        t: 0,
        datom_type: DatomType::Retraction,
    };
    let decimal_serialized = serialize(&decimal_datom, Index::EAVT);
    let id_datom = Datom {
        entity: ID::null(),
        attribute: ID::null(),
        value: ID::null().into(),
        t: 0,
        datom_type: DatomType::Retraction,
    };
    let id_serialized = serialize(&id_datom, Index::EAVT);

    let mut group = c.benchmark_group("serial");

    group.throughput(Throughput::Bytes(string_serialized.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("serialize", "string"),
        &string_datom,
        serialize_f,
    );
    group.throughput(Throughput::Bytes(integer_serialized.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("serialize", "integer"),
        &integer_datom,
        serialize_f,
    );
    group.throughput(Throughput::Bytes(decimal_serialized.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("serialize", "decimal"),
        &decimal_datom,
        serialize_f,
    );
    group.throughput(Throughput::Bytes(id_serialized.len() as u64));
    group.bench_with_input(BenchmarkId::new("serialize", "id"), &id_datom, serialize_f);

    group.throughput(Throughput::Bytes(string_serialized.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("deserialize", "string"),
        string_serialized.as_slice(),
        deserialize_f,
    );
    group.throughput(Throughput::Bytes(integer_serialized.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("deserialize", "integer"),
        integer_serialized.as_slice(),
        deserialize_f,
    );
    group.throughput(Throughput::Bytes(decimal_serialized.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("deserialize", "decimal"),
        decimal_serialized.as_slice(),
        deserialize_f,
    );
    group.throughput(Throughput::Bytes(id_serialized.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("deserialize", "id"),
        id_serialized.as_slice(),
        deserialize_f,
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
