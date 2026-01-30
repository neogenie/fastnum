use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use num_traits::ToPrimitive;
use std::{hint::black_box, str::FromStr};

criterion_group!(to_f64, bench);
criterion_main!(to_f64);

macro_rules! macro_impl {
    ($group: ident, [$($bits: literal),*], $str: literal $($a: ident)?) => {{
        const STR: &'static str = $str;
        let size = STR.len();

        macro_impl!(@ $($a)? $group, [$($bits),*], STR, size);
    }};
    (@ $group: ident, [$($bits: literal),*], $str: ident, $size: ident) => {{
        let n = rust_decimal::Decimal::from_str($str).or_else(|_|
            rust_decimal::Decimal::from_scientific($str)
        ).unwrap();
        $group.bench_with_input(BenchmarkId::new("rust_decimal", $size), &$size, |b, _| {
            b.iter(|| black_box(n).to_f64().unwrap());
        });

        // fixed-num Dec19x19: only benchmark if value is within range
        if let Ok(n_fn) = fixed_num::Dec19x19::from_str($str) {
            $group.bench_with_input(BenchmarkId::new("fixed_num", $size), &$size, |b, _| {
                b.iter(|| f64::from(black_box(n_fn)))
            });
        }

        macro_impl!(@ A $group, [$($bits),*], $str, $size);
    }};
    (@ A $group: ident, [$($bits: literal),*], $str: ident, $size: ident) => {{
        $(
            let n = fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str(
                $str,
                fastnum::decimal::Context::default(),
            ).unwrap();

            $group.bench_with_input(
                BenchmarkId::new(concat!("fastnum", stringify!($bits)), $size),
                &$size,
                |b, _| {
                    b.iter(|| {
                        black_box(n).to_f64()
                    })
                },
            );
        )*

        $group.bench_with_input(BenchmarkId::new("bigdecimal", $size), &$size, |b, _| {
            let n = bigdecimal::BigDecimal::from_str($str).unwrap();
            b.iter(|| black_box(&n).to_f64())
        });
    }};
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("to_f64");

    macro_impl!(group, [64, 128], "1");
    macro_impl!(group, [64, 128], "0.1");
    macro_impl!(group, [64, 128], "-1.0");
    macro_impl!(group, [64, 128], "-0.05");
    macro_impl!(group, [64, 128], "123.456");
    macro_impl!(group, [64, 128], "0.000001");
    macro_impl!(group, [64, 128], "-1.23456789");
    macro_impl!(group, [64, 128], "1000000.123456789");
    macro_impl!(group, [64, 128], "1.234567891234567891");
    macro_impl!(group, [64, 128], "-1.234567891234567891");
    macro_impl!(group, [64, 128], "-184467.44073709551615");
    macro_impl!(group, [128], "-184467.4407378463463374609551615");
    macro_impl!(group, [128], "59890004.528405539691448211669921875");
    macro_impl!(group, [128], "0.340282366920938463463374607431768211455" A);
    // macro_impl!(
    //     group,
    //     [256],
    //     "-3.4028236692093846346337460743176821145554343232345" A
    // );
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
