use std::str::FromStr;

use criterion::{black_box, BenchmarkId, Criterion};

use bigdecimal::BigDecimal;
use fastnum::{dec128, D128};

pub fn vector(c: &mut Criterion) {
    let mut group = c.benchmark_group("Allocate");

    for size in [100, 500, 1000, 10000, 100000, 1000000] {
        group.bench_with_input(BenchmarkId::new("f64", size), &size, |b, size| {
            const N: f64 = 0.123_456_789_101_112_13;
            b.iter(|| black_box(vec![N; *size]))
        });

        group.bench_with_input(BenchmarkId::new("D128", size), &size, |b, size| {
            const N: D128 = dec128!(0.12345678910111213);
            b.iter(|| black_box(vec![N; *size]))
        });

        group.bench_with_input(BenchmarkId::new("BigDecimal", size), &size, |b, size| {
            let n = BigDecimal::from_str("0.12345678910111213").unwrap();
            b.iter(|| black_box(vec![n.clone(); *size]))
        });
    }
    group.finish();
}
