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

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D};

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

        #[rstest(::trace)]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.1), $dec!(340282366920938463463374607431768211454.9))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.5), $dec!(340282366920938463463374607431768211454.5))]
        #[case($dec!(170141183460469231731687303715884105727), $dec!(0.1), $dec!(170141183460469231731687303715884105726.9))]
        #[case($dec!(170141183460469231731687303715884105727), $dec!(0.5), $dec!(170141183460469231731687303715884105726.5))]
        fn test_sub_256(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a -= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }
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

        #[rstest(::trace)]
        #[case($dec!(-340282366920938463463374607431768211455), $dec!(-0.1), $dec!(-340282366920938463463374607431768211454.9))]
        #[case($dec!(-340282366920938463463374607431768211455), $dec!(-0.5), $dec!(-340282366920938463463374607431768211454.5))]
        #[case($dec!(-170141183460469231731687303715884105727), $dec!(-0.1), $dec!(-170141183460469231731687303715884105726.9))]
        #[case($dec!(-170141183460469231731687303715884105727), $dec!(-0.5), $dec!(-170141183460469231731687303715884105726.5))]
        fn test_sub_256_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a -= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($D::from(u128::MAX), $dec!(0.1), $D::from(u128::MAX))]
        #[case($D::from(u128::MAX), $dec!(0.5), $D::from(u128::MAX))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.5), $dec!(340282366920938463463374607431768211455))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.1), $dec!(340282366920938463463374607431768211455))]
        #[case($dec!(34028236692093846346337460743176821145), $dec!(0.01), $dec!(34028236692093846346337460743176821145.0))]
        #[case($dec!(1), $dec!(0.000000000000000000000000000000000000001), $dec!(100000000000000000000000000000000000000e-38))]
        #[case($dec!(1), $dec!(0.0000000000000000000000000000000000000001), $dec!(100000000000000000000000000000000000000e-38))]
        fn test_sub_128_inexact(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;
            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }

        #[rstest(::trace)]
        #[case($dec!(184467440737e3380), $dec!(0), $dec!(184467440737000000000000000000000000000e3353))]
        fn test_sub_128_clamped(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;
            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            assert!(!res.is_op_inexact());
            assert!(res.is_op_clamped());
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0), $dec!(0))]
        #[case($dec!(1), $dec!(1), $dec!(0))]
        #[case($dec!(1), $dec!(0), $dec!(1))]
        #[case($dec!(2), $dec!(1), $dec!(1))]
        #[case($dec!(2), $dec!(2), $dec!(0))]
        #[case($dec!(3), $dec!(2), $dec!(1))]
        #[case($dec!(11), $dec!(1), $dec!(10))]
        #[case($dec!(10), $dec!(1), $dec!(9))]
        #[case($dec!(9), $dec!(1), $dec!(8))]
        #[case($dec!(5.75), $dec!(3.3), $dec!(2.45))]
        #[case($dec!(0.7), $dec!(0.3), $dec!(0.4))]
        #[case($dec!(1.3), $dec!(0.3), $dec!(1.0))]
        #[case($dec!(1.25), $dec!(1.25), $dec!(0.00))]
        #[case($dec!(1.0), $dec!(0), $dec!(1.0))]
        #[case($dec!(1), $dec!(0.75), $dec!(0.25))]
        #[case($dec!(12.34), $dec!(1.234), $dec!(11.106))]
        #[case($dec!(1234e6), $dec!(1234e-6), $dec!(1233999999.998766))]
        #[case($dec!(85616001e4), $dec!(0), $dec!(856160010000))]
        #[case($dec!(11), $dec!(1), $dec!(10))]
        #[case($dec!(10), $dec!(1), $dec!(9))]
        #[case($dec!(9), $dec!(1), $dec!(8))]
        #[case($dec!(1), $dec!(1), $dec!(0))]
        #[case($dec!(5.75), $dec!(3.3), $dec!(2.45))]
        #[case($dec!(0.7), $dec!(0.3), $dec!(0.4))]
        #[case($dec!(1.3), $dec!(0.3), $dec!(1.0))]
        #[case($dec!(1.25), $dec!(1.25), $dec!(0.00))]
        // ------------

        #[case($dec!(10.23456789), $dec!(10.23456789), $dec!(0E-8))]
        #[case($dec!(10.23456790), $dec!(10.23456789), $dec!(1E-8))]
        #[case($dec!(10.23456791), $dec!(10.23456789), $dec!(2E-8))]
        #[case($dec!(10.23456792), $dec!(10.23456789), $dec!(3E-8))]
        #[case($dec!(10.23456793), $dec!(10.23456789), $dec!(4E-8))]
        #[case($dec!(10.23456794), $dec!(10.23456789), $dec!(5E-8))]
        #[case($dec!(10.23456786), $dec!(10.23456786), $dec!(0E-8))]
        #[case($dec!(10.23456787), $dec!(10.23456786), $dec!(1E-8))]
        #[case($dec!(10.23456788), $dec!(10.23456786), $dec!(2E-8))]
        #[case($dec!(10.23456789), $dec!(10.23456786), $dec!(3E-8))]
        #[case($dec!(10.23456790), $dec!(10.23456786), $dec!(4E-8))]
        #[case($dec!(10.23456791), $dec!(10.23456786), $dec!(5E-8))]
        #[case($dec!(1), $dec!(0.999999999), $dec!(1E-9))]
        // ------------
        #[case($dec!(0.9998), $dec!(0.0000), $dec!(0.9998))]
        #[case($dec!(0.9998), $dec!(0.0001), $dec!(0.9997))]
        #[case($dec!(0.9998), $dec!(0.0002), $dec!(0.9996))]
        #[case($dec!(0.9998), $dec!(0.0003), $dec!(0.9995))]
        // ------------
        fn test_sub(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a -= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($D::NAN, $dec!(1))]
        #[case($dec!(1), $D::NAN)]
        #[case($D::NAN, $D::NAN)]
        #[case($D::NAN, $D::INFINITY)]
        #[case($D::INFINITY, $D::NAN)]
        #[case($D::INFINITY, $D::INFINITY)]
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_sub_nan(#[case] a: $D, #[case] b: $D) {
            let _ = a - b;
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(2))]
        #[case($dec!(0.003), $dec!(0.3))]
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_sub_negative_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a - b;
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($D::from(i128::MAX), $dec!(0.1), $D::from(i128::MAX))]
        #[case($D::from(i128::MAX), $dec!(0.5), $D::from(i128::MAX))]
        #[case($dec!(0.000000001), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.000001), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.1), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.4), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.49), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.499999), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.499999999), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.5), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.500000001), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.500001), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211455))]
        #[case($dec!(0.5001), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        #[case($dec!(0.51), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        #[case($dec!(0.6), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        #[case($dec!(0.9), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        #[case($dec!(0.99999), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        #[case($dec!(0.999999999), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        #[case($dec!(1.000000001), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        #[case($dec!(1.00001), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        #[case($dec!(1.1), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        fn test_sub_128_inexact_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0), $dec!(0))]
        #[case($dec!(1), $dec!(2), $dec!(-1))]
        #[case($dec!(-0), $dec!(0), $dec!(-0))]
        #[case($dec!(0), $dec!(-0), $dec!(0))]
        #[case($dec!(0), $dec!(1), $dec!(-1))]
        #[case($dec!(1), $dec!(-1), $dec!(2))]
        #[case($dec!(1), $dec!(-2), $dec!(3))]
        #[case($dec!(-1), $dec!(0), $dec!(-1))]
        #[case($dec!(-1), $dec!(1), $dec!(-2))]
        #[case($dec!(-1), $dec!(2), $dec!(-3))]
        #[case($dec!(2), $dec!(-1), $dec!(3))]
        #[case($dec!(2), $dec!(-2), $dec!(4))]
        #[case($dec!(2), $dec!(3), $dec!(-1))]
        #[case($dec!(2), $dec!(-3), $dec!(5))]
        #[case($dec!(-2), $dec!(1), $dec!(-3))]
        #[case($dec!(-2), $dec!(2), $dec!(-4))]
        #[case($dec!(-3), $dec!(2), $dec!(-5))]
        #[case($dec!(-2), $dec!(3), $dec!(-5))]
        #[case($dec!(-1), $dec!(0.75), $dec!(-1.75))]
        #[case($dec!(0), $dec!(2), $dec!(-2))]
        #[case($dec!(3), $dec!(-2), $dec!(5))]
        #[case($dec!(-9), $dec!(1), $dec!(-10))]
        #[case($dec!(-10), $dec!(1), $dec!(-11))]
        #[case($dec!(-11), $dec!(1), $dec!(-12))]
        #[case($dec!(5), $dec!(-3), $dec!(8))]
        #[case($dec!(-5), $dec!(-3), $dec!(-2))]
        #[case($dec!(-3), $dec!(-5), $dec!(2))]
        #[case($dec!(-7), $dec!(2.5), $dec!(-9.5))]
        #[case($dec!(0.003), $dec!(0.3), $dec!(-0.297))]
        #[case($dec!(12.34), $dec!(-1.234), $dec!(13.574))]
        #[case($dec!(1234e-6), $dec!(1234e6), $dec!(-1233999999.998766))]
        #[case($dec!(712911676e-6), $dec!(4856259269250829), $dec!(-4856259269250116.088324))]
        #[case($dec!(0), $dec!(5207.07672), $dec!(-520707672e-5))]
        #[case($dec!(99291289e5), $dec!(0), $dec!(9929128900000))]
        #[case($dec!(0.7051277471570131), $dec!(1), $dec!(-0.2948722528429869))]
        #[case($dec!(40686030.22763836), $dec!(-10), $dec!(40686040.22763836))]
        #[case($dec!(0), $dec!(1), $dec!(-1))]
        #[case($dec!(-1), $dec!(1), $dec!(-2))]
        #[case($dec!(-9), $dec!(1), $dec!(-10))]
        #[case($dec!(-10), $dec!(1), $dec!(-11))]
        #[case($dec!(-11), $dec!(1), $dec!(-12))]
        #[case($dec!(5), $dec!(-3), $dec!(8))]
        #[case($dec!(-5), $dec!(-3), $dec!(-2))]
        #[case($dec!(-7), $dec!(2.5), $dec!(-9.5))]
        #[case($D::INFINITY, $D::NEG_INFINITY, $D::INFINITY)]
        #[case($D::NEG_INFINITY, $D::INFINITY, $D::NEG_INFINITY)]
        #[case($dec!(1), $D::INFINITY, $D::NEG_INFINITY)]
        #[case($dec!(1000), $D::INFINITY, $D::NEG_INFINITY)]
        #[case($D::INFINITY, $dec!(1000), $D::INFINITY)]
        #[case($D::NEG_INFINITY, $dec!(1000), $D::NEG_INFINITY)]
        // ---------
        #[case($dec!(10.23456784), $dec!(10.23456789), $dec!(-5E-8))]
        #[case($dec!(10.23456785), $dec!(10.23456789), $dec!(-4E-8))]
        #[case($dec!(10.23456786), $dec!(10.23456789), $dec!(-3E-8))]
        #[case($dec!(10.23456787), $dec!(10.23456789), $dec!(-2E-8))]
        #[case($dec!(10.23456788), $dec!(10.23456789), $dec!(-1E-8))]
        #[case($dec!(10.23456781), $dec!(10.23456786), $dec!(-5E-8))]
        #[case($dec!(10.23456782), $dec!(10.23456786), $dec!(-4E-8))]
        #[case($dec!(10.23456783), $dec!(10.23456786), $dec!(-3E-8))]
        #[case($dec!(10.23456784), $dec!(10.23456786), $dec!(-2E-8))]
        #[case($dec!(10.23456785), $dec!(10.23456786), $dec!(-1E-8))]
        #[case($dec!(0), $dec!(1231234567456789), $dec!(-1231234567456789))]
        // ---------
        #[case($dec!(0.999999999), $dec!(1), $dec!(-1E-9))]
        #[case($dec!(-10.23456780), $dec!(-10.23456786), $dec!(6E-8))]
        #[case($dec!(-10.23456790), $dec!(-10.23456786), $dec!(-4E-8))]
        #[case($dec!(-10.23456791), $dec!(-10.23456786), $dec!(-5E-8))]
        // ---------
        #[case($dec!(0.9998), $dec!(-0.0000), $dec!(0.9998))]
        #[case($dec!(0.9998), $dec!(-0.0001), $dec!(0.9999))]
        #[case($dec!(0.9998), $dec!(-0.0002), $dec!(1.0000))]
        #[case($dec!(0.9998), $dec!(-0.0003), $dec!(1.0001))]
        #[case($dec!(1), $dec!(340282366920938463463374607431768211455), $dec!(-340282366920938463463374607431768211454))]
        fn test_sub_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a - b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a -= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($D::NAN, $dec!(-1))]
        #[case($dec!(-1), $D::NAN)]
        #[case($D::NEG_INFINITY, $D::NEG_INFINITY)]
        #[case($D::NAN, $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, $D::NAN)]
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_sub_nan_signed(#[case] a: $D, #[case] b: $D) {
            let _ = a - b;
        }
    };
}

pub(crate) use test_impl;
