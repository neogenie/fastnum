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
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.5), $dec!(340282366920938463463374607431768211455.5))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.05), $dec!(340282366920938463463374607431768211455.05))]
        fn test_add_256(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a + b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a += b;
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
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.5), $dec!(34028236692093846346337460743176821146e1))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(0.1), $dec!(340282366920938463463374607431768211455))]
        #[case($dec!(34028236692093846346337460743176821145), $dec!(0.01), $dec!(34028236692093846346337460743176821145.0))]
        #[case($dec!(34028236692093846346337460743176821145), $dec!(0.05), $dec!(34028236692093846346337460743176821145.1))]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(340282366920938463463374607431768211455), $dec!(68056473384187692692674921486353642291e1))]
        #[case($dec!(0.340282366920938463463374607431768211455), $dec!(0.340282366920938463463374607431768211455), $dec!(0.68056473384187692692674921486353642291))]
        #[case($dec!(23.6666666666666666666666666666666666667), $dec!(10.6666666666666666666666666666666666668), $dec!(34.333333333333333333333333333333333334))]
        fn test_add_inexact(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a + b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }

        #[rstest(::trace)]
        #[case($dec!(184467440737e3380), $dec!(0), $dec!(184467440737000000000000000000000000000e3353))]
        fn test_add_clamped(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a + b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());

            assert!(!res.is_op_inexact());
            assert!(res.is_op_clamped());
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0), $dec!(0))]
        #[case($dec!(0), $dec!(0.00), $dec!(0.00))]
        #[case($dec!(0), $dec!(1), $dec!(1))]
        #[case($dec!(1), $dec!(0), $dec!(1))]
        #[case($dec!(1), $dec!(1), $dec!(2))]
        #[case($dec!(2), $dec!(1), $dec!(3))]
        #[case($dec!(3), $dec!(1), $dec!(4))]
        #[case($dec!(2), $dec!(3), $dec!(5))]
        #[case($dec!(4), $dec!(1), $dec!(5))]
        #[case($dec!(5), $dec!(1), $dec!(6))]
        #[case($dec!(6), $dec!(1), $dec!(7))]
        #[case($dec!(7), $dec!(1), $dec!(8))]
        #[case($dec!(8), $dec!(1), $dec!(9))]
        #[case($dec!(9), $dec!(1), $dec!(10))]
        #[case($dec!(10), $dec!(0.00), $dec!(10.00))]
        #[case($dec!(12.34), $dec!(1.234), $dec!(13.574))]
        #[case($dec!(5.75), $dec!(3.3), $dec!(9.05))]
        #[case($dec!(0.7), $dec!(0.3), $dec!(1.0))]
        #[case($dec!(1.25), $dec!(1.25), $dec!(2.50))]
        #[case($dec!(1.23456789), $dec!(1.00000000), $dec!(2.23456789))]
        #[case($dec!(1.23456789), $dec!(1.00000011), $dec!(2.23456800))]
        #[case($dec!(0.9998), $dec!(0.0000), $dec!(0.9998))]
        #[case($dec!(0.9998), $dec!(0.0001), $dec!(0.9999))]
        #[case($dec!(0.9998), $dec!(0.0002), $dec!(1.0000))]
        #[case($dec!(0.9998), $dec!(0.0003), $dec!(1.0001))]
        #[case($dec!(7E+12), $dec!(1.11), $dec!(7000000000001.11))]
        #[case($dec!(1.11), $dec!(7E+12), $dec!(7000000000001.11))]
        #[case($dec!(100), $dec!(1), $dec!(101))]
        #[case($dec!(1), $dec!(100), $dec!(101))]
        #[case($dec!(1E+12), $dec!(1.11), $dec!(1000000000001.11))]
        #[case($dec!(1.11), $dec!(1E+12), $dec!(1000000000001.11))]
        #[case($dec!(1234e6), $dec!(1234e-6), $dec!(1234000000.001234))]
        #[case($dec!(18446744073709551616.0), $dec!(1), $dec!(18446744073709551617.0))]
        #[case($dec!(0), $dec!(77.6), $dec!(77.6))]
        #[case($dec!(80802295e5), $dec!(0), $dec!(8080229500000))]
        #[case($dec!(23.9200), $dec!(0.0101), $dec!(23.9301))]
        #[case($dec!(46.636423395767125), $dec!(123), $dec!(169.636423395767125))]
        #[case($dec!(1.2345), $dec!(123.45), $dec!(124.6845))]
        #[case($dec!(123.43e5), $dec!(1.2345), $dec!(12343001.2345))]
        #[case($dec!(22132e2), $dec!(0.0000), $dec!(2213200.0000))]
        #[case($dec!(14028236093846.346337460743176821145), $dec!(140282366920934633.68211455), $dec!(140296395157028480.028452010743176821145))]
        #[case($dec!(1E+12), $dec!(1.11), $dec!(1000000000001.11))]
        #[case($dec!(1.11), $dec!(1E+12), $dec!(1000000000001.11))]
        #[case($dec!(7E+12), $dec!(1.11), $dec!(7000000000001.11))]
        #[case($dec!(1.11), $dec!(7E+12), $dec!(7000000000001.11))]
        //---------------

        #[case($dec!(0.00), $dec!(0.01), $dec!(0.01))]
        #[case($dec!(0.01), $dec!(0.01), $dec!(0.02))]
        #[case($dec!(0.12), $dec!(0.01), $dec!(0.13))]
        #[case($dec!(0.98), $dec!(0.01), $dec!(0.99))]
        #[case($dec!(0.99), $dec!(0.01), $dec!(1.00))]
        #[case($dec!(1.00), $dec!(0.01), $dec!(1.01))]
        #[case($dec!(1.01), $dec!(0.01), $dec!(1.02))]

        //---------------
        #[case($D::INFINITY, $D::INFINITY, $D::INFINITY)]
        #[case($dec!(1), $D::INFINITY, $D::INFINITY)]
        #[case($dec!(1000), $D::INFINITY, $D::INFINITY)]
        #[case($D::INFINITY, $dec!(1000), $D::INFINITY)]
        fn test_add(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a + b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a += b;
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
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_add_nan(#[case] a: $D, #[case] b: $D) {
            let _ = a + b;
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {

    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0), $dec!(-0), $dec!(-0))]
        #[case($dec!(-0), $dec!(0), $dec!(0))]
        #[case($dec!(0), $dec!(-0), $dec!(0))]
        #[case($dec!(-1), $dec!(0), $dec!(-1))]
        #[case($dec!(5), $dec!(-3), $dec!(2))]
        #[case($dec!(-5), $dec!(-3), $dec!(-8))]
        #[case($dec!(-7), $dec!(2.5), $dec!(-4.5))]
        #[case($dec!(12.34), $dec!(-1.234), $dec!(11.106))]
        #[case($dec!(12.34), $dec!(-12.34), $dec!(0.00))]
        #[case($dec!(23.9200), $dec!(-101), $dec!(-77.0800))]
        #[case($dec!(-316.79), $dec!(0e-6), $dec!(-316.790000))]
        #[case($dec!(316.79), $dec!(-0e-6), $dec!(316.790000))]
        #[case($dec!(1E+12), $dec!(-1), $dec!(999999999999))]
        #[case($dec!(-1), $dec!(1E+12), $dec!(999999999999))]
        #[case($dec!(7E+12), $dec!(-1), $dec!(6999999999999))]
        #[case($dec!(100), $dec!(-1), $dec!(99))]
        #[case($dec!(-1), $dec!(100), $dec!(99))]
        #[case($dec!(1E+12), $dec!(-1), $dec!(999999999999))]
        #[case($dec!(-1), $dec!(1E+12), $dec!(999999999999))]
        #[case($dec!(7E+12), $dec!(-1), $dec!(6999999999999))]
        #[case($dec!(-1), $dec!(7E+12), $dec!(6999999999999))]
        //--------------------
        #[case($dec!(-0.01), $dec!(0.01), $dec!(0.00))]
        #[case($dec!(-0.01), $dec!(-0.01), $dec!(-0.02))]
        #[case($dec!(0.00), $dec!(-0.01), $dec!(-0.01))]
        #[case($dec!(0.01), $dec!(-0.01), $dec!(0.00))]
        #[case($dec!(0.12), $dec!(-0.01), $dec!(0.11))]
        #[case($dec!(0.98), $dec!(-0.01), $dec!(0.97))]
        #[case($dec!(0.99), $dec!(-0.01), $dec!(0.98))]
        #[case($dec!(1.00), $dec!(-0.01), $dec!(0.99))]
        #[case($dec!(1.01), $dec!(-0.01), $dec!(1.00))]
        //--------------------

        #[case($D::NEG_INFINITY, $D::NEG_INFINITY, $D::NEG_INFINITY)]
        #[case($dec!(-1), $D::INFINITY, $D::INFINITY)]
        #[case($dec!(-1000), $D::INFINITY, $D::INFINITY)]
        #[case($D::INFINITY, $dec!(-1000), $D::INFINITY)]
        #[case($D::NEG_INFINITY, $dec!(-1000), $D::NEG_INFINITY)]
        #[case($dec!(-1), $D::NEG_INFINITY, $D::NEG_INFINITY)]
        #[case($dec!(-1000), $D::NEG_INFINITY, $D::NEG_INFINITY)]
        fn test_add_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a + b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a += b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($D::NAN, $dec!(-1))]
        #[case($dec!(-1), $D::NAN)]
        #[case($D::INFINITY, $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, $D::INFINITY)]
        #[case($D::NAN, $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, $D::NAN)]
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_add_nan_signed(#[case] a: $D, #[case] b: $D) {
            let _ = a + b;
        }
    };
}

pub(crate) use test_impl;
