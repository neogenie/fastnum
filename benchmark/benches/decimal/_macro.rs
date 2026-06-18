macro_rules! benchmark_op {
    ($op: tt, $case: literal, $group: ident, [$($bits: literal),*], $a: literal, $b: literal $($arb: ident)?) => {{
        benchmark_op!(@ $($arb)? $op, $case, $group, [$($bits),*], $a, $b);
    }};
    (@ $op: tt, $case: literal, $group: ident, [$($bits: literal),*], $a: literal, $b: literal) => {{
        let a_f64 = f64::from_str($a).unwrap();
        let b_f64 = f64::from_str($b).unwrap();

        $group.bench_with_input(
            BenchmarkId::new("f64", $case),
            &(a_f64, b_f64),
            |bench, (a, b)| bench.iter(|| black_box(*a) $op black_box(*b)),
        );

        let a_rd = rust_decimal::Decimal::from_str($a).unwrap();
        let b_rd = rust_decimal::Decimal::from_str($b).unwrap();

        $group.bench_with_input(
            BenchmarkId::new("rust_decimal", $case),
            &(a_rd, b_rd),
            |bench, (a, b)| bench.iter(|| black_box(*a) $op black_box(*b)),
        );

        // fixed-num Dec19x19: only benchmarks values within its range (±17_014_118_346_046_923_173.168...)
        if let (Ok(a_fn), Ok(b_fn)) = (
            fixed_num::Dec19x19::from_str($a),
            fixed_num::Dec19x19::from_str($b),
        ) {
            $group.bench_with_input(
                BenchmarkId::new("fixed_num", $case),
                &(a_fn, b_fn),
                |bench, (a, b)| bench.iter(|| black_box(*a) $op black_box(*b)),
            );
        }

        benchmark_op!(@ A $op, $case, $group, [$($bits),*], $a, $b);
    }};
    (@ A $op: tt, $case: literal, $group: ident, [$($bits: literal),*], $a: literal, $b: literal) => {{

        benchmark_op!(@ F $op, $case, $group, [$($bits),*], $a, $b);

        let a_bd = bigdecimal::BigDecimal::from_str($a).unwrap();
        let b_bd = bigdecimal::BigDecimal::from_str($b).unwrap();

        $group.bench_with_input(
            BenchmarkId::new("bigdecimal", $case),
            &(a_bd, b_bd),
            |bench, (a, b)| bench.iter(|| black_box(a) $op black_box(b)),
        );
    }};
    (@ F $op: tt, $case: literal, $group: ident, [$($bits: literal),*], $a: literal, $b: literal) => {{
        $(
            let ctx = fastnum::decimal::Context::default();

            let a = fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str($a, ctx).unwrap();
            let b = fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str($b, ctx).unwrap();

            $group.bench_with_input(
                BenchmarkId::new(concat!("fastnum", stringify!($bits)), $case),
                &(a, b),
                |bench, (a, b)| bench.iter(|| black_box(*a) $op black_box(*b)),
            );
        )*
    }};
}

pub(super) use benchmark_op;
