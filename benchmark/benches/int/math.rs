use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fastnum::{U128, U64};

criterion_group!(math, bench /*mul_digit*/);
criterion_main!(math);

#[allow(dead_code)]
fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("math");

    let a_u64 = 9_223_372_036_854_775_808u64;
    let b_u64 = 9_223_372_036_854_775_807u64;

    assert_eq!(u64::MAX, a_u64 + b_u64);

    let a_f64 = fastnum::u64!(9223372036854775808);
    let b_f64 = fastnum::u64!(9223372036854775807);

    let a_u128 = 170_141_183_460_469_231_731_687_303_715_884_105_728u128;
    let b_u128 = 170_141_183_460_469_231_731_687_303_715_884_105_727u128;

    assert_eq!(u128::MAX, a_u128 + b_u128);

    let a_f128 = fastnum::u128!(170141183460469231731687303715884105728);
    let b_f128 = fastnum::u128!(170141183460469231731687303715884105727);

    group.bench_with_input(
        BenchmarkId::new("+", "u64"),
        &(a_u64, b_u64),
        |bench, &(a, b)| bench.iter(|| black_box(a) + black_box(b)),
    );

    group.bench_with_input(
        BenchmarkId::new("+", "U64"),
        &(a_f64, b_f64),
        |bench, &(a, b)| bench.iter(|| black_box(a) + black_box(b)),
    );

    group.bench_with_input(
        BenchmarkId::new("+ (checked)", "u64"),
        &(a_u64, b_u64),
        |bench, &(a, b)| bench.iter(|| black_box(a).checked_add(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("+ (checked)", "U64"),
        &(a_f64, b_f64),
        |bench, &(a, b)| bench.iter(|| black_box(a).checked_add(black_box(b))),
    );


    group.bench_with_input(
        BenchmarkId::new("+", "u128"),
        &(a_u128, b_u128),
        |bench, &(a, b)| bench.iter(|| black_box(a) + black_box(b)),
    );

    group.bench_with_input(
        BenchmarkId::new("+", "U128"),
        &(a_f128, b_f128),
        |bench, &(a, b)| bench.iter(|| black_box(a) + black_box(b)),
    );

    group.bench_with_input(
        BenchmarkId::new("+ (checked)", "u128"),
        &(a_u128, b_u128),
        |bench, &(a, b)| bench.iter(|| black_box(a).checked_add(black_box(b))),
    );

    group.bench_with_input(
        BenchmarkId::new("+ (checked)", "U128"),
        &(a_f128, b_f128),
        |bench, &(a, b)| bench.iter(|| black_box(a).checked_add(black_box(b))),
    );


    // group.bench_with_input(
    //     BenchmarkId::new("u128", "-"),
    //     &(a_u128, b_u128),
    //     |bench, &(a, b)| bench.iter(|| a.checked_sub(black_box(b))),
    // );
    //
    // group.bench_with_input(
    //     BenchmarkId::new("u128", "*"),
    //     &(a_u128, b_u128),
    //     |bench, &(a, b)| bench.iter(|| a.overflowing_mul(black_box(b))),
    // );
    //
    // group.bench_with_input(
    //     BenchmarkId::new("fastnum", "+"),
    //     &(a_f, b_f),
    //     |bench, &(a, b)| bench.iter(|| a.checked_add(black_box(b))),
    // );
    //
    // group.bench_with_input(
    //     BenchmarkId::new("fastnum", "-"),
    //     &(a_f, b_f),
    //     |bench, &(a, b)| bench.iter(|| a.checked_sub(black_box(b))),
    // );
    //
    // group.bench_with_input(
    //     BenchmarkId::new("fastnum", "*"),
    //     &(a_f, b_f),
    //     |bench, &(a, b)| bench.iter(|| a.overflowing_mul(black_box(b))),
    // );

    group.finish();
}

#[allow(dead_code)]
fn add_digit(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_digit");

    for power in [
        1,
        10,
        100,
        1000,
        10000000000000,
        100000000000000,
        1000000000000000,
        1000000000000000000,
        10000000000000000000,
    ] {
        let x = power;
        let y = 5555555555555555555_u64;
        group.bench_with_input(BenchmarkId::new("u64", power), &(x, y), |b, (x, y)| {
            b.iter(|| black_box(x) + black_box(*y))
        });

        let x = U64::from_u64(power);
        group.bench_with_input(BenchmarkId::new("U64", power), &(x, y), |b, (x, y)| {
            b.iter(|| black_box(x).strict_add_digit(black_box(*y)))
        });
    }

    for power in [
        1,
        1000,
        1000000,
        1000000000,
        1000000000000,
        1000000000000000000000000000000000000,
        100000000000000000000000000000000000000,
    ] {
        let x = power;
        let y = 5555555555555555555_u64;
        group.bench_with_input(BenchmarkId::new("u128", power), &(x, y), |b, (x, y)| {
            b.iter(|| black_box(x) + black_box(*y) as u128)
        });

        let x = U128::from_u128(power).unwrap();
        group.bench_with_input(BenchmarkId::new("U128", power), &(x, y), |b, (x, y)| {
            b.iter(|| black_box(x).strict_add_digit(black_box(*y)))
        });
    }

    group.finish();
}

#[allow(dead_code)]
fn mul_digit(c: &mut Criterion) {
    let mut group = c.benchmark_group("mul_digit");

    for power in [
        1,
        10,
        100,
        1000,
        1000000000,
        10000000000000,
        100000000000000000,
    ] {
        let x = power;
        let y = 11_u64;
        group.bench_with_input(BenchmarkId::new("u64", power), &(x, y), |b, (x, y)| {
            b.iter(|| black_box(x) * black_box(*y))
        });

        let x = U64::from_u64(power);
        group.bench_with_input(BenchmarkId::new("U64", power), &(x, y), |b, (x, y)| {
            b.iter(|| black_box(x).strict_mul_digit(black_box(*y)))
        });
    }

    for power in [
        1,
        1000,
        1000000,
        1000000000000,
        1000000000000000000000,
        10000000000000000000000000000000000000,
    ] {
        let x = power;
        let y = 11_u64;
        group.bench_with_input(BenchmarkId::new("u128", power), &(x, y), |b, (x, y)| {
            b.iter(|| black_box(x) * black_box(*y) as u128)
        });

        let x = U128::from_u128(power).unwrap();
        group.bench_with_input(BenchmarkId::new("U128", power), &(x, y), |b, (x, y)| {
            b.iter(|| black_box(x).strict_mul_digit(black_box(*y)))
        });
    }

    group.finish();
}
