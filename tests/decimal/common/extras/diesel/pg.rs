macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D};
            use diesel::pg::data_types::PgNumeric;

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D};
            use diesel::pg::data_types::PgNumeric;

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };


    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (COMMON:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (UNSIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![] })]
        #[case($dec!(1), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![1] })]
        #[case($dec!(10), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![10] })]
        #[case($dec!(100), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![100] })]
        #[case($dec!(10000), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1] })]
        #[case($dec!(10001), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1, 1] })]
        #[case($dec!(1234), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![1234] })]
        #[case($dec!(12345), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1, 2345] })]
        #[case($dec!(12345678), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1234, 5678] })]
        #[case($dec!(100000000), PgNumeric::Positive { weight: 2, scale: 0, digits: vec![1] })]
        #[case($dec!(1.0), PgNumeric::Positive { weight: 0, scale: 1, digits: vec![1] })]
        #[case($dec!(1.1), PgNumeric::Positive { weight: 0, scale: 1, digits: vec![1, 1000] })]
        #[case($dec!(1.10), PgNumeric::Positive { weight: 0, scale: 2, digits: vec![1, 1000] })]
        #[case($dec!(100000000.0001), PgNumeric::Positive { weight: 2, scale: 4, digits: vec![1, 0, 0, 1] })]
        #[case($dec!(0.1), PgNumeric::Positive { weight: -1, scale: 1, digits: vec![1000] })]
        #[case($dec!(0.01), PgNumeric::Positive { weight: -1, scale: 2, digits: vec![100] })]
        #[case($dec!(0.012), PgNumeric::Positive { weight: -1, scale: 3, digits: vec![120] })]
        #[case($dec!(1.2345), PgNumeric::Positive { weight: 0, scale: 4, digits: vec![1, 2345] })]
        #[case($dec!(0.12345), PgNumeric::Positive { weight: -1, scale: 5, digits: vec![1234, 5000] })]
        #[case($dec!(0.01234), PgNumeric::Positive { weight: -1, scale: 5, digits: vec![0123, 4000] })]
        #[case($dec!(12345.67890), PgNumeric::Positive { weight: 1, scale: 5, digits: vec![1, 2345, 6789] })]
        #[case($dec!(0.00001234), PgNumeric::Positive { weight: -2, scale: 8, digits: vec![1234] })]
        #[case($dec!(123.456), PgNumeric::Positive { weight: 0, scale: 3, digits: vec![123, 4560] })]
        #[case($dec!(5000), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![5000] })]
        #[case($dec!(5e1), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![50] })]
        #[case($dec!(5e2), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![500] })]
        #[case($dec!(5e3), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![5000] })]
        #[case($dec!(5e4), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![5] })]
        #[case($dec!(5e5), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![50] })]
        #[case($dec!(50e2), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![5000] })]
        #[case($dec!(50.123e5), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![501, 2300] })]
        #[case($dec!(50.123000e6), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![5012, 3000] })]
        #[case($dec!(5000e2), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![50] })]
        #[case($dec!(5000e4), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![5000] })]
        #[case($dec!(1e4), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1] })]
        #[case($dec!(0.1000000000000000), PgNumeric::Positive { weight: -1, scale: 16, digits: vec![1000] })]
        #[case($dec!(0.00315937), PgNumeric::Positive { weight: -1, scale: 8, digits: vec![31, 5937] })]
        #[case($dec!(0.003159370000000000), PgNumeric::Positive { weight: -1, scale: 18, digits: vec![31, 5937] })]
        #[case($dec!(1e18) + $dec!(9e-16), PgNumeric::Positive { weight: 4, scale: 16, digits: vec![100, 0, 0, 0, 0, 0, 0, 0, 9] })]
        #[case($dec!(9e18) + $dec!(9e-16), PgNumeric::Positive { weight: 4, scale: 16, digits: vec![900, 0, 0, 0, 0, 0, 0, 0, 9] })]
        fn test_convert(#[case] d: $D, #[case] expected: PgNumeric) {
            let numeric: PgNumeric = d.try_into().unwrap();
            assert_eq!(numeric, expected);

            let decimal: $D = expected.try_into().unwrap();
            assert_eq!(decimal, d);
        }

        #[rstest(::trace)]
        fn test_nan() {
            let dec = $D::try_from(PgNumeric::NaN).unwrap();
            assert!(dec.is_nan());

            let num = PgNumeric::try_from(dec).unwrap();
            assert_eq!(num, PgNumeric::NaN);
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case(PgNumeric::Negative { weight: 0, scale: 0, digits: vec![1234] })]
        fn test_unsigned(#[case] numeric: PgNumeric) {
            let e = $D::try_from(numeric).unwrap_err();
            assert_eq!(
                e.to_string(),
                String::from(concat!(
                    "(fastnum) ",
                    stringify!($D),
                    " does not support negative values"
                ))
            );
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-1234), PgNumeric::Negative { weight: 0, scale: 0, digits: vec![1234] })]
        #[case($dec!(-12345678), PgNumeric::Negative { weight: 1, scale: 0, digits: vec![1234, 5678] })]
        #[case($dec!(-123.456), PgNumeric::Negative { weight: 0, scale: 3, digits: vec![123, 4560] })]
        #[case($dec!(-56.78), PgNumeric::Negative { weight: 0, scale: 2, digits: vec![56, 7800] })]
        fn test_convert_signed(#[case] d: $D, #[case] expected: PgNumeric) {
            let numeric: PgNumeric = d.try_into().unwrap();
            assert_eq!(numeric, expected);

            let decimal: $D = expected.try_into().unwrap();
            assert_eq!(decimal, d);
        }
    };
}

pub(crate) use test_impl;
