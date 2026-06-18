use std::{hint::black_box, str::FromStr};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

extern crate fixed_num;

criterion_group!(allocate, vector);
criterion_main!(allocate);

fn vector(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocate");

    for size in [100, 500, 1000, 10000, 100000, 1000000] {
        group.bench_with_input(BenchmarkId::new("f64", size), &size, |b, size| {
            const N: f64 = 0.123_456_789_101_112_13;
            b.iter(|| vec![black_box(N); black_box(*size)])
        });

        group.bench_with_input(BenchmarkId::new("rust_decimal", size), &size, |b, size| {
            let n = rust_decimal::Decimal::from_str("0.12345678910111213").unwrap();
            b.iter(|| vec![black_box(n); black_box(*size)])
        });

        group.bench_with_input(BenchmarkId::new("fixed_num", size), &size, |b, size| {
            let n = fixed_num::Dec19x19::from_str("0.12345678910111213").unwrap();
            b.iter(|| vec![black_box(n); black_box(*size)])
        });

        group.bench_with_input(BenchmarkId::new("fastnum128", size), &size, |b, size| {
            const N: fastnum::D128 = fastnum::dec128!(0.12345678910111213);
            b.iter(|| vec![black_box(N); black_box(*size)])
        });

        group.bench_with_input(BenchmarkId::new("BigDecimal", size), &size, |b, size| {
            let n = bigdecimal::BigDecimal::from_str("0.12345678910111213").unwrap();
            b.iter(|| vec![black_box(n.clone()); black_box(*size)])
        });
    }
    group.finish();
}
