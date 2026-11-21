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
        super::test_impl!(COMMON:: 64, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1e39), $dec!(340282366920938463463374607431768211455), $dec!(319435266158123073073250785136463577090))]
        fn test_rem_128(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a % b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());
        }
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 64, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 64, $dec, $D);
    };

    (COMMON:: 64, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 64, $dec, $D);
    };
    (COMMON:: 64, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(1), $dec!(1), $dec!(0))]
        #[case($dec!(2), $dec!(1), $dec!(0))]
        #[case($dec!(1), $dec!(2), $dec!(1))]
        #[case($dec!(2), $dec!(2), $dec!(0))]
        #[case($dec!(0), $dec!(1), $dec!(0))]
        #[case($dec!(0), $dec!(2), $dec!(0))]
        #[case($dec!(1), $dec!(3), $dec!(1))]
        #[case($dec!(2), $dec!(3), $dec!(2))]
        #[case($dec!(3), $dec!(3), $dec!(0))]
        #[case($dec!(3), $dec!(2), $dec!(1))]
        #[case($dec!(100), $dec!(5), $dec!(0))]
        #[case($dec!(2e1), $dec!(1), $dec!(0))]
        #[case($dec!(1), $dec!(5e-1), $dec!(0.0))]
        #[case($dec!(15e-1), $dec!(1), $dec!(0.5))]
        #[case($dec!(1), $dec!(3e-2), $dec!(1e-2))]
        #[case($dec!(10), $dec!(3e-3), $dec!(0.001))]
        #[case($dec!(1234e-2), $dec!(1233e-3), $dec!(10e-3))]
        #[case($dec!(2.4), $dec!(1), $dec!(0.4))]
        #[case($dec!(2.40), $dec!(1), $dec!(0.40))]
        #[case($dec!(2.400), $dec!(1), $dec!(0.400))]
        #[case($dec!(2.4), $dec!(2), $dec!(0.4))]
        #[case($dec!(2.400), $dec!(2), $dec!(0.400))]
        #[case($dec!(2.0), $dec!(2), $dec!(0.0))]
        #[case($dec!(20), $dec!(20), $dec!(0))]
        // Aligning the exponents would overflow a 64-bit coefficient (#69).
        #[case($dec!(1e19), $dec!(7), $dec!(3))]
        #[case($dec!(1e20), $dec!(7), $dec!(2))]
        #[case($dec!(1e30), $dec!(7), $dec!(1))]
        #[case($dec!(1e60), $dec!(7), $dec!(1))]
        #[case($dec!(1e100), $dec!(7), $dec!(4))]
        #[case($dec!(1e200), $dec!(7), $dec!(2))]
        #[case($dec!(1e1000), $dec!(7), $dec!(4))]
        #[case($dec!(20), $dec!(6.283185307179586476), $dec!(1.150444078461240572))]
        #[case($dec!(20), $dec!(6.2831853071795864), $dec!(1.1504440784612408))]
        #[case($dec!(1e25), $dec!(0.7), $dec!(0.2))]
        #[case($dec!(123456789e10), $dec!(9.87654321), $dec!(0.14062500))]
        #[case($dec!(18446744073709551615e5), $dec!(18446744073709551614), $dec!(100000))]
        #[case($dec!(1e40), $dec!(18446744073709551615), $dec!(2098486950404341705))]
        #[case($dec!(1e40), $dec!(1844674407370955161.5), $dec!(253812543033386543.5))]
        #[case($dec!(1e-5), $dec!(3e-30), $dec!(1e-30))]
        #[case($dec!(5e15), $dec!(0.0000000000000003), $dec!(2e-16))]
        fn test_rem(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a % b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a %= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }

        /// `%` is exact whatever the exponents: the same remainder as in a 512-bit decimal,
        /// where aligning these operands' exponents never overflows.
        #[rstest(::trace)]
        fn test_rem_exact_against_wider() {
            const COEFFICIENTS: [&str; 9] = [
                "1",
                "3",
                "7",
                "10",
                "5000",
                "123456789",
                "6283185307179586476",
                "9999999999999999999",
                "18446744073709551615",
            ];

            for a in COEFFICIENTS {
                for b in COEFFICIENTS {
                    for ea in (-60..=60).step_by(10) {
                        for eb in (-60..=60).step_by(10) {
                            let a: $D = format!("{a}e{ea}").parse().unwrap();
                            let b: $D = format!("{b}e{eb}").parse().unwrap();

                            let res = a % b;
                            let expected: $D = (a.resize::<8>() % b.resize::<8>()).resize();

                            assert_eq!(res, expected, "{a} % {b}");
                            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count(), "{a} % {b}");
                            assert!(res.is_op_ok(), "{a} % {b}");
                        }
                    }
                }
            }
        }
    };
    (UNSIGNED:: 64, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 64, $dec, $D);
    };
    (UNSIGNED:: 64, $dec: ident, $D: ident) => {

    };
    (SIGNED:: 64, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 64, $dec, $D);
    };
    (SIGNED:: 64, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(2.4), $dec!(-1), $dec!(0.4))]
        #[case($dec!(-2.4), $dec!(1), $dec!(-0.4))]
        #[case($dec!(-2.4), $dec!(-1), $dec!(-0.4))]
        #[case($dec!(-3), $dec!(2), $dec!(-1))]
        #[case($dec!(3), $dec!(-2), $dec!(1))]
        #[case($dec!(-9.5), $dec!(5.15), $dec!(-4.35))]
        // Aligning the exponents would overflow a 64-bit coefficient (#69).
        #[case($dec!(-1e20), $dec!(7), $dec!(-2))]
        #[case($dec!(1e20), $dec!(-7), $dec!(2))]
        #[case($dec!(-1e20), $dec!(-7), $dec!(-2))]
        #[case($dec!(-20), $dec!(6.283185307179586476), $dec!(-1.150444078461240572))]
        fn test_rem_signed(#[case] a: $D, #[case] b: $D, #[case] expected: $D) {
            let res = a % b;

            assert_eq!(res, expected);
            assert_eq!(res.fractional_digits_count(), expected.fractional_digits_count());
            assert!(res.is_op_ok());

            let mut a = a;

            a %= b;
            assert_eq!(a, expected);
            assert_eq!(a.fractional_digits_count(), expected.fractional_digits_count());
            assert!(a.is_op_ok());
        }
    };
}

pub(crate) use test_impl;
