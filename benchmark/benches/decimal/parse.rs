use std::{hint::black_box, str::FromStr};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

criterion_group!(parse, from_str);
criterion_main!(parse);

macro_rules! macro_impl {
    ($group: ident, [$($bits: literal),*], $str: literal $($a: ident)?) => {{
        const STR: &'static str = $str;
        let size = STR.len();

        macro_impl!(@ $($a)? $group, [$($bits),*], STR, size);
    }};
    (@ $group: ident, [$($bits: literal),*], $str: ident, $size: ident) => {{
        $group.bench_with_input(BenchmarkId::new("f64", $size), &$size, |b, _| {
            b.iter(|| f64::from_str(black_box($str)).unwrap())
        });

        $group.bench_with_input(BenchmarkId::new("rust_decimal", $size), &$size, |b, _| {
            b.iter(|| rust_decimal::Decimal::from_str(black_box($str)).unwrap())
        });

        // fixed-num Dec19x19: only benchmark if value is within range
        if let Ok(_) = fixed_num::Dec19x19::from_str($str) {
            $group.bench_with_input(BenchmarkId::new("fixed_num", $size), &$size, |b, _| {
                b.iter(|| fixed_num::Dec19x19::from_str(black_box($str)).unwrap())
            });
        }

        macro_impl!(@ A $group, [$($bits),*], $str, $size);
    }};
    (@ A $group: ident, [$($bits: literal),*], $str: ident, $size: ident) => {{
        $(
            $group.bench_with_input(
                BenchmarkId::new(concat!("fastnum", stringify!($bits)), $size),
                &$size,
                |b, _| {
                    b.iter(|| {
                        fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str(
                            black_box($str),
                            fastnum::decimal::Context::default(),
                        )
                        .unwrap()
                    })
                },
            );
        )*

        $group.bench_with_input(BenchmarkId::new("bigdecimal", $size), &$size, |b, _| {
            b.iter(||bigdecimal::BigDecimal::from_str(black_box($str)).unwrap())
        });
    }};
}

fn from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_str");

    macro_impl!(group, [64, 128], "1");
    macro_impl!(group, [64, 128], "1.0");
    macro_impl!(group, [64, 128], "-1.0");
    macro_impl!(group, [64, 128], "-0.05");
    macro_impl!(group, [64, 128], "123.456");
    macro_impl!(group, [64, 128], "0.000001");
    macro_impl!(group, [64, 128], "-1.23456789");
    macro_impl!(group, [64, 128], "1000000.123456789");
    macro_impl!(group, [64, 128], "1234567891234567891");
    macro_impl!(group, [64, 128], "1.234567891234567891");
    macro_impl!(group, [64, 128], "-1.234567891234567891");
    macro_impl!(group, [64, 128], "-184467.44073709551615");
    macro_impl!(group, [128], "-184467.4407378463463374609551615");
    macro_impl!(group, [128], "3.4028236692093846346337460743176821145" A);
    macro_impl!(
        group,
        [256],
        "-3.4028236692093846346337460743176821145554343232345" A
    );
    // macro_impl!(
    //     group,
    //     256,
    //     "-9.157920892373161954235709850086879078532699846656405640394573129639935"
    // );
    // macro_impl!(
    //     group,
    //     256,
    //     "-1.15792089237316195423570985008687907853269984665640564039457584007913129639935"
    // );
    // macro_impl!(group, 512,
    // "-1.340780792994259709957402499820584612747936582059239337772356144372176403007354697680187429816"
    // ); macro_impl!(group, 512,
    // "-1.34078079299425970995740249982058461274793658205923933777235614437217640300735469768018742981669034276900318581864860"
    // ); macro_impl!(group, 512,
    // "-1.3407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569"
    // ); macro_impl!(group, 512,
    // "-1.3407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095"
    // );

    group.finish();
}
