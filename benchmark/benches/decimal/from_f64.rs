use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(from_f64, bench);
criterion_main!(from_f64);

macro_rules! macro_impl {
    ($group: ident, [$($bits: literal),*], $f: expr $(, $a: ident)?) => {{
        let str = format!("{:?}", $f);
        let size = str.len();

        macro_impl!(@ $($a)? $group, [$($bits),*], $f, size);
    }};
    (@ $group: ident, [$($bits: literal),*], $f: expr, $size: ident) => {{
        $group.bench_with_input(BenchmarkId::new("rust_decimal", $size), &$size, |b, _| {
            b.iter(|| rust_decimal::Decimal::from_f64_retain(black_box($f)).unwrap())
        });

        // fixed-num Dec19x19: try_from for f64
        if let Ok(_) = fixed_num::Dec19x19::try_from($f) {
            $group.bench_with_input(BenchmarkId::new("fixed_num", $size), &$size, |b, _| {
                b.iter(|| fixed_num::Dec19x19::try_from(black_box($f)).unwrap())
            });
        }

        macro_impl!(@ A $group, [$($bits),*], $f, $size);
    }};
    (@ A $group: ident, [$($bits: literal),*], $f: expr, $size: ident) => {{
        $(
            $group.bench_with_input(
                BenchmarkId::new(concat!("fastnum", stringify!($bits)), $size),
                &$size,
                |b, _| {
                    b.iter(|| {
                        fastnum::decimal::Decimal::<{ $bits / 64 }>::from_f64(black_box($f))
                    })
                },
            );
        )*

        $group.bench_with_input(BenchmarkId::new("bigdecimal", $size), &$size, |b, _| {
            b.iter(||bigdecimal::BigDecimal::try_from(black_box($f)).unwrap())
        });
    }};
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_f64");

    macro_impl!(group, [64, 128], 1_f64);
    macro_impl!(group, [64, 128], -1.0);
    macro_impl!(group, [64, 128], -0.05);
    macro_impl!(group, [64, 128], 123.456);
    macro_impl!(group, [64, 128], 0.1);
    macro_impl!(group, [64, 128], std::f64::consts::SQRT_2);
    macro_impl!(group, [64, 128], std::f64::consts::PI);
    macro_impl!(group, [64, 128], 1.732050807568877293527446341505872367_f64);

    group.finish();
}
