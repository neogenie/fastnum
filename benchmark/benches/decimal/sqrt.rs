use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;

criterion_group!(sqrt, bench);
criterion_main!(sqrt);

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sqrt");

    let x = fastnum::dec128!(3);
    let x_f64 = 3.0_f64;
    let x_bd = bigdecimal::BigDecimal::try_from(3.0_f64).unwrap();
    let x_rd = rust_decimal::Decimal::try_from(3.0_f64).unwrap();
    let x_fn = fixed_num::Dec19x19::from(3_i32);

    group.bench_with_input(BenchmarkId::new("f64", "3"), &x_f64, |bench, x| {
        bench.iter(|| black_box(x).sqrt())
    });

    group.bench_with_input(BenchmarkId::new("rust_decimal", "3"), &x_rd, |bench, x| {
        use rust_decimal::MathematicalOps;
        bench.iter(|| black_box(x).sqrt())
    });

    group.bench_with_input(BenchmarkId::new("fixed_num", "3"), &x_fn, |bench, x| {
        use fixed_num::ops::UncheckedSqrt;
        bench.iter(|| black_box(*x).unchecked_sqrt())
    });

    group.bench_with_input(BenchmarkId::new("fastnum", "3"), &x, |bench, x| {
        bench.iter(|| black_box(x).sqrt())
    });

    group.bench_with_input(BenchmarkId::new("bigdecimal", "3"), &x_bd, |bench, x| {
        bench.iter(|| black_box(x).sqrt().unwrap())
    });

    group.finish();
}
