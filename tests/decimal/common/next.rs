macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: $bits, [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: $bits, [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use fastnum::{decimal::*, *};
            use rstest::*;

            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use fastnum::{decimal::*, *};
            use rstest::*;

            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);
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

    (UNSIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 128, $dec, $D);
    };
    (UNSIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(UNSIGNED:: 64, $dec, $D);
    };
    (SIGNED:: 128, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(340282366920938463463374607431768211455), $dec!(34028236692093846346337460743176821146E+1))]
        #[case($dec!(34028236692093846346337460743176821145.5), $dec!(34028236692093846346337460743176821146))]
        #[case($dec!(1), $dec!(1.00000000000000000000000000000000000001))]
        fn test_next_up_128(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d.next_up(), expected);
            assert_eq!(expected.next_down(), d);
            assert_eq!((-expected).next_up(), -d);
            assert_eq!((-d).next_down(), -expected);
        }
    };
    (SIGNED:: 128, $dec: ident, $D: ident) => {
        super::test_impl!(SIGNED:: 64, $dec, $D);
    };

    (UNSIGNED:: 64, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 64, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1), $dec!(1.0000000000000000001))]
        #[case($dec!(0.9999999999999999999), $dec!(1))]
        #[case($dec!(1844674407370955161.5), $dec!(1844674407370955162))]
        fn test_next_up_down_64(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d.next_up(), expected);
            assert_eq!(expected.next_down(), d);
        }
    };
    (UNSIGNED:: 64, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        fn test_next_up_down() {
            assert_eq!($dec!(1).next_up(), $dec!(1) + $D::EPSILON);
            assert_eq!($dec!(1).next_down(), $dec!(1) - $D::EPSILON);
        }

        #[rstest(::trace)]
        fn test_next_bounds() {
            assert_eq!($D::ZERO.next_up(), $D::MIN_POSITIVE);
            assert!($D::MIN_POSITIVE.next_down().is_zero());
            assert_eq!($D::MAX.next_up(), $D::INFINITY);
            assert_eq!($D::INFINITY.next_down(), $D::MAX);
            assert_eq!($D::INFINITY.next_up(), $D::INFINITY);
        }

        #[rstest(::trace)]
        fn test_next_nan() {
            assert!($D::NAN.next_up().is_nan());
            assert!($D::NAN.next_down().is_nan());
        }

        #[rstest(::trace)]
        #[should_panic(expected = "(fastnum) invalid operation")]
        fn test_next_down_zero() {
            let _ = $D::ZERO.next_down();
        }

        #[rstest(::trace)]
        fn test_next_down_zero_without_traps() {
            let res = $D::ZERO.with_ctx(Context::default().without_traps()).next_down();
            assert!(res.is_nan());
            assert!(res.is_op_invalid());
        }
    };

    (SIGNED:: 64, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 64, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1), $dec!(1.0000000000000000001))]
        #[case($dec!(0.9999999999999999999), $dec!(1))]
        #[case($dec!(1.0999999999999999999), $dec!(1.1))]
        #[case($dec!(9.999999999999999999), $dec!(10))]
        #[case($dec!(18446744073709551615), $dec!(1844674407370955162E+1))]
        #[case($dec!(1844674407370955161.5), $dec!(1844674407370955162))]
        #[case($dec!(1844674407370955161), $dec!(1844674407370955161.1))]
        // At the smallest exponent the coefficient cannot be widened, nor `MAX` taken one place finer.
        #[case($dec!(1844674407370955161E-32767), $dec!(1844674407370955162E-32767))]
        fn test_next_up_64(#[case] d: $D, #[case] expected: $D) {
            assert_eq!(d.next_up(), expected);
            assert_eq!(expected.next_down(), d);
            assert_eq!((-expected).next_up(), -d);
            assert_eq!((-d).next_down(), -expected);
        }
    };
    (SIGNED:: 64, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        fn test_next_epsilon() {
            assert_eq!($dec!(1).next_up(), $dec!(1) + $D::EPSILON);
            assert_eq!($dec!(1).next_down(), $dec!(1) - $D::EPSILON);
            assert_eq!($dec!(1.1).next_down(), $dec!(1.1) - $D::EPSILON);
            assert_eq!($dec!(10).next_down(), $dec!(10) - $D::EPSILON * $dec!(10));
            assert_eq!($dec!(-1).next_up(), $dec!(-1) + $D::EPSILON);
        }

        #[rstest(::trace)]
        fn test_next_zero() {
            assert_eq!($dec!(0).next_up(), $D::MIN_POSITIVE);
            assert_eq!($dec!(-0).next_up(), $D::MIN_POSITIVE);
            assert_eq!($dec!(0).next_down(), -$D::MIN_POSITIVE);
            assert_eq!($dec!(-0).next_down(), -$D::MIN_POSITIVE);
            assert!($dec!(0).next_up().is_sign_positive());
            assert!($dec!(0).next_down().is_sign_negative());
        }

        #[rstest(::trace)]
        fn test_next_towards_signed_zero() {
            let res = $D::MIN_POSITIVE.next_down();
            assert!(res.is_zero());
            assert!(res.is_sign_positive());

            // A negative value climbs to −0, as for the primitive floats.
            let res = (-$D::MIN_POSITIVE).next_up();
            assert!(res.is_zero());
            assert!(res.is_sign_negative());
        }

        #[rstest(::trace)]
        fn test_next_exponent_limits() {
            let ctx = Context::default();
            let two = $D::from_parts(2u8.into(), -32767, Sign::Plus, ctx);

            assert_eq!($D::MIN_POSITIVE.next_up(), two);
            assert_eq!(two.next_down(), $D::MIN_POSITIVE);
        }

        #[rstest(::trace)]
        fn test_next_exponent_limits_part_way() {
            // The exponent, not the coefficient, limits the widening.
            let ctx = Context::default();
            let d = $D::from_parts(1u8.into(), -32760, Sign::Plus, ctx);
            let unit = $D::MIN_POSITIVE;

            assert_eq!(d.next_up(), d + unit);
            assert_eq!(d.next_down(), d - unit);
            assert_eq!(d.next_up().fractional_digits_count(), 32767);
            assert_eq!(d.next_down().fractional_digits_count(), 32767);
        }

        #[rstest(::trace)]
        fn test_next_max_and_min() {
            let below_max = $D::MAX.next_down();
            assert_eq!(below_max.digits(), $D::MAX.digits() - 1u8.into());
            assert_eq!(below_max.fractional_digits_count(), $D::MAX.fractional_digits_count());
            assert_eq!(below_max.next_up(), $D::MAX);
            assert_eq!($D::MIN.next_up(), -below_max);
            assert_eq!((-below_max).next_down(), $D::MIN);
        }

        #[rstest(::trace)]
        fn test_next_infinities() {
            assert_eq!($D::MAX.next_up(), $D::INFINITY);
            assert_eq!($D::INFINITY.next_up(), $D::INFINITY);
            assert_eq!($D::INFINITY.next_down(), $D::MAX);
            assert_eq!($D::MIN.next_down(), $D::NEG_INFINITY);
            assert_eq!($D::NEG_INFINITY.next_down(), $D::NEG_INFINITY);
            assert_eq!($D::NEG_INFINITY.next_up(), $D::MIN);
        }

        #[rstest(::trace)]
        fn test_next_nan() {
            // NaN is returned quietly, as IEEE 754 nextUp and nextDown do for a quiet NaN.
            let res = $D::NAN.next_up();
            assert!(res.is_nan());
            assert!(!res.is_op_invalid());

            let res = $D::NAN.next_down();
            assert!(res.is_nan());
            assert!(!res.is_op_invalid());
        }

        #[rstest(::trace)]
        fn test_next_largest_coefficient_one_place_finer() {
            // `MAX`'s coefficient, one place finer, lies between a value just above a tenth of it and that value minus its ulp.
            let ctx = Context::default();
            let finest = $D::from_parts($D::MAX.digits(), -1, Sign::Plus, ctx);
            let above = finest.trunc() + $D::ONE;

            assert_eq!(above.next_down(), finest);
            assert_eq!(finest.next_up(), above);
            assert_eq!((-above).next_up(), -finest);
            assert_eq!((-finest).next_down(), -above);

            let max = $D::from_parts($D::MAX.digits(), 0, Sign::Plus, ctx);
            let above_max = above * $D::TEN;

            assert_eq!(max.next_up(), above_max);
            assert_eq!(above_max.next_down(), max);
        }

        #[rstest(::trace)]
        #[case($dec!(1))]
        #[case($dec!(-1))]
        #[case($dec!(0))]
        #[case($D::MAX)]
        #[case($D::NEG_INFINITY)]
        fn test_next_keeps_context(#[case] d: $D) {
            // The result keeps the context: an invalid operation on it does not trap.
            let ctx = Context::default().without_traps();
            for res in [d.with_ctx(ctx).next_up(), d.with_ctx(ctx).next_down()] {
                let invalid = if res.is_finite() {
                    res.abs().neg().sqrt()
                } else {
                    res - res
                };
                assert!(invalid.is_nan());
            }
        }

        #[rstest(::trace)]
        fn test_next_is_exact() {
            assert!($dec!(1).next_up().is_op_ok());
            assert!($dec!(1).next_down().is_op_ok());
            assert!($D::MAX.next_up().is_op_ok());
            assert!($D::MIN_POSITIVE.next_down().is_op_ok());
        }

        #[rstest(::trace)]
        fn test_next_leaves_no_decimal_between() {
            let ctx = Context::default();
            let mut state: u64 = 0x9E37_79B9_7F4A_7C15;

            for _ in 0..2000 {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);

                let coefficient = state as i64 >> (state % 60);
                let exponent = ((state >> 32) % 64001) as i32 - 32000;
                let d = $D::from_str(&format!("{coefficient}E{exponent}"), ctx).unwrap();

                let (up, down) = (d.next_up(), d.next_down());
                assert!(down < d && d < up, "{d}: {down} < {d} < {up}");
                assert_eq!(up.next_down(), d, "{d}");
                assert_eq!(down.next_up(), d, "{d}");

                // The midpoint of two neighbours rounds onto one of them.
                if !d.is_zero() {
                    let midpoint: $D = ((d.resize::<16>() + up.resize()) / D1024::TWO).resize();
                    assert!(midpoint == d || midpoint == up, "{d}: {midpoint}");
                }
            }
        }
    };
}

pub(crate) use test_impl;
