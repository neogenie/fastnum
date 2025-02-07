use std::str::FromStr;

use criterion::{black_box, BenchmarkId, Criterion};

macro_rules! macro_impl {
    ($group: ident, $bits: literal, $str: literal) => {{
        const STR: &'static str = $str;

        let size = STR.len();

        $group.bench_with_input(BenchmarkId::new("f64", size), &size, |b, _| {
            b.iter(|| black_box(f64::from_str(STR).unwrap()))
        });

        $group.bench_with_input(BenchmarkId::new("fastnum", size), &size, |b, _| {
            b.iter(|| {
                black_box(
                    fastnum::decimal::Decimal::<{ $bits / 64 }>::from_str(
                        STR,
                        fastnum::decimal::Context::default(),
                    )
                    .unwrap(),
                )
            })
        });

        $group.bench_with_input(BenchmarkId::new("bigdecimal", size), &size, |b, _| {
            b.iter(|| black_box(bigdecimal::BigDecimal::from_str(STR).unwrap()))
        });

        // $group.bench_with_input(BenchmarkId::new("rust_decimal", size),
        // &size, |b, _| {     b.iter(||
        // black_box(rust_decimal::Decimal::from_str(STR).unwrap())) });
    }};
}

pub fn from_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("from_str");

    macro_impl!(group, 64, "-1.0");
    macro_impl!(group, 64, "-1.23456789");
    macro_impl!(group, 64, "-1.23456789123456789");
    macro_impl!(group, 64, "-184467.44073709551615");
    macro_impl!(group, 128, "-184467.4407378463463374609551615");
    macro_impl!(group, 128, "3.4028236692093846346337460743176821145");
    macro_impl!(
        group,
        256,
        "-3.4028236692093846346337460743176821145554343232345"
    );
    macro_impl!(
        group,
        256,
        "-9.157920892373161954235709850086879078532699846656405640394573129639935"
    );
    macro_impl!(
        group,
        256,
        "-1.15792089237316195423570985008687907853269984665640564039457584007913129639935"
    );
    macro_impl!(group, 512, "-1.340780792994259709957402499820584612747936582059239337772356144372176403007354697680187429816");
    macro_impl!(group, 512, "-1.34078079299425970995740249982058461274793658205923933777235614437217640300735469768018742981669034276900318581864860");
    macro_impl!(group, 512, "-1.3407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569");
    macro_impl!(group, 512, "-1.3407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084095");

    group.finish();
}
