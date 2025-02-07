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
            #[allow(unused_imports)]
            use fastnum::{*, decimal::*};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            #[allow(unused_imports)]
            use fastnum::{*, decimal::*};

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
        #[case($dec!(995052931372975485719.533153137), $dec!(4.523087321), $dec!(4500711297616988541501.836966993116075977))]
        #[case($dec!(8.37664968), $dec!(1.9086963714056968482094712882596748), $dec!(15.988480848752691653730876239769592670324064))]
        fn test_mul_256(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a * b;

            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            assert!(prod.is_op_ok());

            let mut a = a;

            a *= b;
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
        #[case($dec!(-995052931372975485719.533153137), $dec!(4.523087321), $dec!(-4500711297616988541501.836966993116075977))]
        #[case($dec!(995052931372975485719.533153137), $dec!(-4.523087321), $dec!(-4500711297616988541501.836966993116075977))]
        #[case($dec!(-8.37664968), $dec!(-1.9086963714056968482094712882596748), $dec!(15.988480848752691653730876239769592670324064))]
        fn test_mul_256_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a * b;

            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            assert!(prod.is_op_ok());

            let mut a = a;

            a *= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }
    };

    (COMMON:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(1.0), $dec!(340282366920938463463374607431768211455), signals![!ROUND])]
        #[case($dec!(995052931372975485719.533153137), $dec!(4.523087321), $dec!(4500711297616988541501.8369669931160760), signals![!ROUND, !INEXACT])]
        #[case($dec!(8.37664968), $dec!(1.9086963714056968482094712882596748), $dec!(15.9884808487526916537308762397695926703), signals![!ROUND, !INEXACT])]
        #[case($dec!(1e-2), $dec!(1.23e-32763), $dec!(1.23e-32765), signals![!SN])]
        #[case($dec!(1e-2), $dec!(1e-32767), $D::ZERO, signals![!INEXACT, !ROUND, !SN, !UFW])]
        #[case($dec!(1e5), $dec!(1e32765), $dec!(100e32768), signals![!CP, !ROUND])]
        fn test_mul_128(
            #[case] a: $D,
            #[case] b: $D,
            #[case] expected: $D,
            #[case] signals: Signal
        ) {
            let d = a * b;

            assert_eq!(d, expected);
            assert_eq!(d.fractional_digits_count(), expected.fractional_digits_count());
            assert_eq!(d.op_signals(), signals);
        }

        #[rstest(::trace)]
        #[case($dec!(1e100), $dec!(1e32765))]
        #[should_panic(expected = "(fastnum) overflow was occurred while performing arithmetic operation")]
        fn test_mul_overflow_128(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
        }
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(0.00), $dec!(0), $dec!(0.00))]
        #[case($dec!(0), $dec!(0.00), $dec!(0.00))]
        #[case($dec!(0), $dec!(1.0), $dec!(0.0))]
        #[case($dec!(0.0), $dec!(1), $dec!(0.0))]
        #[case($dec!(0.00), $dec!(1.123), $dec!(0.00000))]
        #[case($dec!(1), $dec!(0), $dec!(0))]
        #[case($dec!(1), $dec!(1), $dec!(1))]
        #[case($dec!(2), $dec!(1), $dec!(2))]
        #[case($dec!(12.34), $dec!(1.234), $dec!(15.22756))]
        #[case($dec!(2e1), $dec!(1), $dec!(2e1))]
        #[case($dec!(3), $dec!(0.333333), $dec!(0.999999))]
        #[case($dec!(2389472934723), $dec!(209481029831), $dec!(500549251119075878721813))]
        #[case($dec!(1e-450), $dec!(1e500), $dec!(0.1e51))]
        #[case($dec!(8.37664968), $dec!(0), $dec!(0.00000000))]
        #[case($dec!(8.561), $dec!(10), $dec!(85.610))]
        #[case($dec!(10000), $dec!(638655273892892437), $dec!(6386552738928924370000))]
        #[case($dec!(1e-10), $dec!(9056180052657301), $dec!(905618.0052657301))]
        #[case($dec!(34028236692093846346337460743176821145), $dec!(1.0), $dec!(34028236692093846346337460743176821145.0))]
        #[case($D::MAX, $dec!(0), $dec!(0))]
        #[case($D::MAX, $dec!(1), $D::MAX)]
        #[case($D::INFINITY, $D::INFINITY, $D::INFINITY)]
        #[case($D::INFINITY, $dec!(1000), $D::INFINITY)]
        #[case($dec!(1000), $D::INFINITY, $D::INFINITY)]
        #[case($dec!(1e-2), $dec!(1e-32765), $dec!(1e-32767))]
        fn test_mul(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a * b;

            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            assert!(prod.is_op_ok());

            let mut a = a;

            a *= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($D::MAX, $dec!(2))]
        #[case($D::MAX, $dec!(10))]
        #[should_panic(expected = "(fastnum) overflow was occurred while performing arithmetic operation")]
        fn test_mul_overflow_panic(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
        }

        #[rstest(::trace)]
        #[case($D::MAX, $dec!(0.3))]
        #[case($D::MAX, $dec!(1.1e-32766))]
        fn test_mul_inexact(#[case] a: $D, #[case] b: $D) {
            let res = a * b;

            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }

        #[rstest(::trace)]
        #[case($D::NAN, $dec!(1))]
        #[case($dec!(1), $D::NAN)]
        #[case($D::NAN, $D::NAN)]
        #[case($D::INFINITY, $dec!(0))]
        #[case($dec!(0), $D::INFINITY)]
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_mul_nan(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
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
        #[case($dec!(-0), $dec!(+0), $dec!(-0))]
        #[case($dec!(-0), $dec!(+1), $dec!(-0))]
        #[case($dec!(-1), $dec!(0), $dec!(-0))]
        #[case($dec!(1), $dec!(-1), $dec!(-1))]
        #[case($dec!(-2), $dec!(1), $dec!(-2))]
        #[case($dec!(-2), $dec!(-1), $dec!(2))]
        #[case($dec!(-8.37664968), $dec!(0), $dec!(-0.00000000))]
        #[case($dec!(-9e-1), $dec!(-368408638655273892892437473), $dec!(331567774789746503603193725.7))]
        #[case($dec!(-1.175470587012343730098), $dec!(577575785), $dec!(-678923347.038065234601180476930))]
        #[case($dec!(1e-10), $dec!(-9056180052657301), $dec!(-905618.0052657301))]
        #[case($D::MIN, $dec!(0), $dec!(-0))]
        #[case($D::MIN, $dec!(1), $D::MIN)]
        #[case($D::NEG_INFINITY, $D::INFINITY, $D::NEG_INFINITY)]
        #[case($D::INFINITY, $D::NEG_INFINITY, $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, $D::NEG_INFINITY, $D::INFINITY)]
        #[case($D::INFINITY, $dec!(-1000), $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, $dec!(1000), $D::NEG_INFINITY)]
        #[case($D::NEG_INFINITY, $dec!(-1000), $D::INFINITY)]
        #[case($dec!(-1000), $D::INFINITY, $D::NEG_INFINITY)]
        #[case($dec!(-1000), $D::NEG_INFINITY, $D::INFINITY)]
        #[case($dec!(1000), $D::NEG_INFINITY, $D::NEG_INFINITY)]
        fn test_mul_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let prod = a * b;

            assert_eq!(prod, expected);
            assert_eq!(prod.fractional_digits_count(), expected.fractional_digits_count());
            assert!(prod.is_op_ok());

            let mut a = a;

            a *= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(prod.is_op_ok());
        }

        #[rstest(::trace)]
        #[case($D::MIN, $dec!(2))]
        #[case($D::MIN, $dec!(10))]
        #[case($D::MIN, $D::MAX)]
        #[case($D::MIN, $dec!(1.1e32768))]
        #[should_panic(expected = "(fastnum) overflow was occurred while performing arithmetic operation")]
        fn test_mul_overflow_panic_signed(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
        }

        #[rstest(::trace)]
        #[case($D::MIN, $dec!(0.3))]
        #[case($D::MIN, $dec!(1.1e-32766))]
        fn test_mul_inexact_signed(#[case] a: $D, #[case] b: $D) {
            let res = a * b;
            assert!(res.is_op_inexact());
            assert!(res.is_op_rounded());
        }

        #[rstest(::trace)]
        #[case($D::NAN, $dec!(-1))]
        #[case($dec!(-1), $D::NAN)]
        #[case($D::INFINITY, $dec!(-0))]
        #[case($D::NEG_INFINITY, $dec!(-0))]
        #[case($D::NEG_INFINITY, $dec!(0))]
        #[case($dec!(0), $D::NEG_INFINITY)]
        #[case($dec!(-0), $D::INFINITY)]
        #[case($dec!(-0), $D::NEG_INFINITY)]
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_mul_nan_signed(#[case] a: $D, #[case] b: $D) {
            let _ = a * b;
        }
    };
}

pub(crate) use test_impl;
