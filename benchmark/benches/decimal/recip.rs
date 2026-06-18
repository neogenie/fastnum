use std::{hint::black_box, str::FromStr};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(recip, bench);
criterion_main!(recip);

macro_rules! macro_impl {
    ($group: ident, $bits: literal, $a: literal) => {{
        let ctx = fastnum::decimal::Context::default();

        let a = fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str($a, ctx).unwrap();

        let size = a.digits_count();

        let a_f64 = f64::from_str($a).unwrap();

        let a_bd = bigdecimal::BigDecimal::from_str($a).unwrap();

        let a_rd = rust_decimal::Decimal::from_str($a).unwrap();

        $group.bench_with_input(BenchmarkId::new("f64", size), &a_f64, |bench, a| {
            bench.iter(|| black_box(*a).recip())
        });

        $group.bench_with_input(BenchmarkId::new("rust_decimal", size), &a_rd, |bench, a| {
            bench.iter(|| {
                rust_decimal::Decimal::ONE
                    .checked_div(black_box(*a))
                    .unwrap()
            })
        });

        // fixed-num Dec19x19: only benchmark if value is within range
        // Dec19x19 doesn't have recip(), use 1/x via division
        if let Ok(a_fn) = fixed_num::Dec19x19::from_str($a) {
            let one_fn = fixed_num::Dec19x19::from(1_i32);
            $group.bench_with_input(
                BenchmarkId::new("fixed_num", size),
                &(one_fn, a_fn),
                |bench, (one, a)| bench.iter(|| black_box(*one) / black_box(*a)),
            );
        }

        $group.bench_with_input(BenchmarkId::new("fastnum", size), &a, |bench, a| {
            bench.iter(|| black_box(*a).recip())
        });

        $group.bench_with_input(BenchmarkId::new("bigdecimal", size), &a_bd, |bench, a| {
            bench.iter(|| black_box(a).inverse())
        });
    }};
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("1/a");

    macro_impl!(group, 128, "789.012");
    macro_impl!(group, 128, "12.345");
    macro_impl!(group, 128, "500549251119075878721813");

    group.finish();
}
