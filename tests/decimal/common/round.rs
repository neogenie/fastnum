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
            use fastnum::{$dec, $D, decimal::RoundingMode::{self, *}};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(UNSIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (SIGNED: $bits: tt, $dec: ident, $D: ident) => {
        mod $dec {
            use rstest::*;
            use fastnum::{$dec, $D, decimal::RoundingMode::{self, *}};

            super::test_impl!(COMMON:: $bits, $dec, $D, THIS);
            super::test_impl!(SIGNED:: $bits, $dec, $D, THIS);
        }
    };
    (COMMON:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(0.33333333333333333333333333333333333333333333333333333333333333333333333333333333333333), 0, $dec!(0), $dec!(0))]
        #[case($dec!(44), 99, $dec!(44), $dec!(44))]
        #[case($dec!(1.555), 99, $dec!(1.555), $dec!(1.555))]
        fn test_round_512(#[case] x: $D, #[case] digits: i16, #[case] y: $D, #[case] z: $D) {
            assert_eq!(x.with_rounding_mode(RoundingMode::HalfUp).round(digits), y);
            assert_eq!(x.with_rounding_mode(RoundingMode::Down).round(digits), z);
        }
    };
    (UNSIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(UNSIGNED:: 256, $dec, $D);
    };
    (SIGNED:: 512, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(SIGNED:: 256, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(-44), - 99, $dec!(-0), $dec!(-0))]
        #[case($dec!(-1.555), 99, $dec!(-1.555), $dec!(-1.555))]
        fn test_round_512_signed(#[case] x: $D, #[case] digits: i16, #[case] y: $D, #[case] z: $D) {
            assert_eq!(x.with_rounding_mode(RoundingMode::HalfUp).round(digits), y);
            assert_eq!(x.with_rounding_mode(RoundingMode::Down).round(digits), z);
        }
    };


    (COMMON:: 256, $dec: ident, $D: ident, THIS) => {
        super::test_impl!(COMMON:: 256, $dec, $D);
    };
    (COMMON:: 256, $dec: ident, $D: ident) => {
        super::test_impl!(COMMON:: 128, $dec, $D);

        #[rstest(::trace)]
        #[case($dec!(1.0000000000000000000000000000000000000000001), 10, $dec!(1.0000000000), $dec!(1.0000000000))]
        #[case($dec!(1.0000000000000000000000000000000000000000009), 10, $dec!(1.0000000000), $dec!(1.0000000000))]
        #[case($dec!(0.1165085714285714285714285714285714285714), 0, $dec!(0), $dec!(0))]
        #[case($dec!(0.1165085714285714285714285714285714285714), 2, $dec!(0.12), $dec!(0.11))]
        #[case($dec!(0.1165085714285714285714285714285714285714), 5, $dec!(0.11651), $dec!(0.11650))]
        #[case($dec!(0.1165085714285714285714285714285714285714), 8, $dec!(0.11650857), $dec!(0.11650857))]
        fn test_round_256(#[case] x: $D, #[case] digits: i16, #[case] y: $D, #[case] z: $D) {
            assert_eq!(x.with_rounding_mode(HalfUp).round(digits), y);
            assert_eq!(x.with_rounding_mode(Down).round(digits), z);
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
    };
    (COMMON:: 128, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        fn test_round_smoke() {
            let n = $dec!(129.41675);

            assert_eq!(n.with_rounding_mode(Up).round(2), $dec!(129.42));
            assert_eq!(n.with_rounding_mode(Down).round(-1), $dec!(120));
            assert_eq!(n.with_rounding_mode(HalfUp).round(4), $dec!(129.4168));
            assert_eq!(n.with_rounding_mode(HalfEven).round(4), $dec!(129.4168));
            assert_eq!(n.with_rounding_mode(HalfDown).round(4), $dec!(129.4167));
        }

        #[rstest(::trace)]
        #[case($dec!(1.45), 1, $dec!(1.5), $dec!(1.4))]
        #[case($dec!(1.44444), 1, $dec!(1.4), $dec!(1.4))]
        #[case($dec!(1.444445), 1, $dec!(1.4), $dec!(1.4))]
        #[case($dec!(1.44), 1, $dec!(1.4), $dec!(1.4))]
        #[case($dec!(0.444), 2, $dec!(0.44), $dec!(0.44))]
        #[case($dec!(0.0045), 2, $dec!(0.00), $dec!(0.00))]
        #[case($dec!(1.555), 2, $dec!(1.56), $dec!(1.55))]
        #[case($dec!(4.5), 0, $dec!(5), $dec!(4))]
        #[case($dec!(5.5), 0, $dec!(6), $dec!(5))]
        #[case($dec!(4.05), 1, $dec!(4.1), $dec!(4.0))]
        #[case($dec!(4.050), 1, $dec!(4.1), $dec!(4.0))]
        #[case($dec!(4.15), 1, $dec!(4.2), $dec!(4.1))]
        #[case($dec!(1), - 1, $dec!(0), $dec!(0))]
        #[case($dec!(5), - 1, $dec!(10), $dec!(00))]
        #[case($dec!(7), 0, $dec!(7), $dec!(7))]
        #[case($dec!(7), 1, $dec!(7.0), $dec!(7.0))]
        #[case($dec!(895), -2, $dec!(900), $dec!(800))]
        #[case($dec!(8934), -2, $dec!(8900), $dec!(8900))]
        #[case($dec!(8934), -3, $dec!(9000), $dec!(8000))]
        #[case($dec!(1.0001), 5, $dec!(1.0001), $dec!(1.0001))]
        #[case($dec!(1.0001), 4, $dec!(1.0001), $dec!(1.0001))]
        #[case($dec!(1.0001), 3, $dec!(1.0), $dec!(1.0))]
        #[case($dec!(1.00009), 5, $dec!(1.00009), $dec!(1.00009))]
        #[case($dec!(1.00009), 4, $dec!(1.0001), $dec!(1.0))]
        #[case($dec!(1.00009), 3, $dec!(1.0), $dec!(1.0))]
        #[case($dec!(44), - 1, $dec!(40), $dec!(40))]
        #[case($dec!(1.4499999999), 0, $dec!(1), $dec!(1))]
        #[case($dec!(1.4499999999), 1, $dec!(1.4), $dec!(1.4))]
        #[case($dec!(1.4499999999), 2, $dec!(1.45), $dec!(1.44))]
        #[case($dec!(1.4499999999), 3, $dec!(1.450), $dec!(1.449))]
        #[case($dec!(1.4499999999), 4, $dec!(1.4500), $dec!(1.4499))]
        #[case($dec!(1.4499999999), 10, $dec!(1.4499999999), $dec!(1.4499999999))]
        #[case($dec!(1.4499999999), 15, $dec!(1.449999999900000), $dec!(1.449999999900000))]
        #[case($dec!(1.449999999), 1, $dec!(1.4), $dec!(1.4))]
        #[case($dec!(9999.444455556666), 10, $dec!(9999.4444555567), $dec!(9999.4444555566))]
        #[case($dec!(12345678987654321.123456789), 8, $dec!(12345678987654321.12345679), $dec!(12345678987654321.12345678))]
        #[case($dec!(18.2697343863199184516), 18, $dec!(18.269734386319918452), $dec!(18.269734386319918451))]
        #[case($dec!(0.0100000000000000000000000001), 18, $dec!(0.010000000000000000), $dec!(0.010000000000000000))]
        #[case($dec!(12345678987654321.123456789), 8, $dec!(12345678987654321.12345679), $dec!(12345678987654321.12345678))]
        #[case($dec!(1.5), 0, $dec!(2), $dec!(1))]
        #[case($dec!(1.2), 0, $dec!(1), $dec!(1))]
        #[case($dec!(0.68), 0, $dec!(1), $dec!(0))]
        #[case($dec!(0.5), 0, $dec!(1), $dec!(0))]
        #[case($dec!(0.49), 0, $dec!(0), $dec!(0))]
        #[case($dec!(0.648), 1, $dec!(0.6), $dec!(0.6))]
        #[case($dec!(0.648), 2, $dec!(0.65), $dec!(0.64))]
        fn test_round(#[case] x: $D, #[case] digits: i16, #[case] y: $D, #[case] z: $D) {
            assert_eq!(x.with_rounding_mode(HalfUp).round(digits), y);
            assert_eq!(x.with_rounding_mode(Down).round(digits), z);
        }


        #[rstest(::trace)]
        //---------------------------------------
        #[case($dec!(0.1), Up, $dec!(1))]
        #[case($dec!(0.1), Ceiling, $dec!(1))]
        //------
        #[case($dec!(0.1), Down, $dec!(0))]
        #[case($dec!(0.1), Floor, $dec!(0))]
        #[case($dec!(0.1), HalfUp, $dec!(0))]
        #[case($dec!(0.1), HalfDown, $dec!(0))]
        #[case($dec!(0.1), HalfEven, $dec!(0))]
        //---------------------------------------
        #[case($dec!(0.5), Up, $dec!(1))]
        #[case($dec!(0.5), Ceiling, $dec!(1))]
        #[case($dec!(0.5), HalfUp, $dec!(1))]
        //------
        #[case($dec!(0.5), Down, $dec!(0))]
        #[case($dec!(0.5), Floor, $dec!(0))]
        #[case($dec!(0.5), HalfDown, $dec!(0))]
        #[case($dec!(0.5), HalfEven, $dec!(0))]
        //---------------------------------------
        #[case($dec!(0.7), Up, $dec!(1))]
        #[case($dec!(0.7), Ceiling, $dec!(1))]
        #[case($dec!(0.7), HalfUp, $dec!(1))]
        #[case($dec!(0.7), HalfDown, $dec!(1))]
        #[case($dec!(0.7), HalfEven, $dec!(1))]
        //------
        #[case($dec!(0.7), Down, $dec!(0))]
        #[case($dec!(0.7), Floor, $dec!(0))]
        //---------------------------------------
        #[case($dec!(9.5), Up, $dec!(10))]
        #[case($dec!(9.5), Ceiling, $dec!(10))]
        #[case($dec!(9.5), HalfUp, $dec!(10))]
        #[case($dec!(9.5), HalfEven, $dec!(10))]
        //------
        #[case($dec!(9.5), Down, $dec!(9))]
        #[case($dec!(9.5), Floor, $dec!(9))]
        #[case($dec!(9.5), HalfDown, $dec!(9))]
        //---------------------------------------
        #[case($dec!(8.5), Up, $dec!(9))]
        #[case($dec!(8.5), Ceiling, $dec!(9))]
        #[case($dec!(8.5), HalfUp, $dec!(9))]
        //------
        #[case($dec!(8.5), Down, $dec!(8))]
        #[case($dec!(8.5), Floor, $dec!(8))]
        #[case($dec!(8.5), HalfDown, $dec!(8))]
        #[case($dec!(8.5), HalfEven, $dec!(8))]
        //---------------------------------------
        #[case($dec!(3.0), Up, $dec!(3))]
        #[case($dec!(3.0), Ceiling, $dec!(3))]
        #[case($dec!(3.0), HalfUp, $dec!(3))]
        #[case($dec!(3.0), Down, $dec!(3))]
        #[case($dec!(3.0), Floor, $dec!(3))]
        #[case($dec!(3.0), HalfDown, $dec!(3))]
        #[case($dec!(3.0), HalfEven, $dec!(3))]
        //---------------------------------------
        fn test_round_modes(#[case] d: $D, #[case] mode: RoundingMode, #[case] expected: $D) {
            let d = d.with_rounding_mode(mode).round(0);
            assert_eq!(d, expected);
            assert_eq!(d.fractional_digits_count(), expected.fractional_digits_count());
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
        #[case($dec!(-1.45), 1, $dec!(-1.5), $dec!(-1.4))]
        #[case($dec!(-1.444445), 1, $dec!(-1.4), $dec!(-1.4))]
        #[case($dec!(-1.44), 1, $dec!(-1.4), $dec!(-1.4))]
        #[case($dec!(-0.444), 2, $dec!(-0.44), $dec!(-0.44))]
        #[case($dec!(-0.0045), 2, $dec!(-0.00), $dec!(-0.00))]
        #[case($dec!(-1.555), 2, $dec!(-1.56), $dec!(-1.55))]
        #[case($dec!(-5.5), 0, $dec!(-6), $dec!(-5))]
        #[case($dec!(-1), - 1, $dec!(-00), $dec!(-00))]
        #[case($dec!(-5), - 1, $dec!(-10), $dec!(-00))]
        #[case($dec!(-44), - 1, $dec!(-40), $dec!(-40))]
        #[case($dec!(-1.4499999999), 1, $dec!(-1.4), $dec!(-1.4))]
        #[case($dec!(-1.449999999), 1, $dec!(-1.4), $dec!(-1.4))]
        #[case($dec!(-9999.444455556666), 10, $dec!(-9999.4444555567), $dec!(-9999.4444555566))]
        #[case($dec!(-12345678987654321.123456789), 8, $dec!(-12345678987654321.12345679), $dec!(-12345678987654321.12345678))]
        #[case($dec!(-18.2697343863199184516), 18, $dec!(-18.269734386319918452), $dec!(-18.269734386319918451))]
        #[case($dec!(-0.0100000000000000000000000001), 18, $dec!(-0.010000000000000000), $dec!(-0.010000000000000000))]
        #[case($dec!(-0.648), 1, $dec!(-0.6), $dec!(-0.6))]
        #[case($dec!(-0.648), 2, $dec!(-0.65), $dec!(-0.64))]
        fn test_round_signed(#[case] x: $D, #[case] digits: i16, #[case] y: $D, #[case] z: $D) {
            assert_eq!(x.with_rounding_mode(HalfUp).round(digits), y);
            assert_eq!(x.with_rounding_mode(Down).round(digits), z);
        }

        #[rstest(::trace)]
        #[case($dec!(-0.1), Up, $dec!(-1))]
        #[case($dec!(-0.1), Floor, $dec!(-1))]
        //------
        #[case($dec!(-0.1), Down, $dec!(-0))]
        #[case($dec!(-0.1), Ceiling, $dec!(-0))]
        #[case($dec!(-0.1), HalfUp, $dec!(-0))]
        #[case($dec!(-0.1), HalfDown, $dec!(-0))]
        #[case($dec!(-0.1), HalfEven, $dec!(-0))]
        //---------------------------------------
        #[case($dec!(-0.5), Up, $dec!(-1))]
        #[case($dec!(-0.5), Floor, $dec!(-1))]
        #[case($dec!(-0.5), HalfUp, $dec!(-1))]
        //------
        #[case($dec!(-0.5), Down, $dec!(-0))]
        #[case($dec!(-0.5), Ceiling, $dec!(-0))]
        #[case($dec!(-0.5), HalfDown, $dec!(-0))]
        #[case($dec!(-0.5), HalfEven, $dec!(-0))]
        //---------------------------------------
        #[case($dec!(-0.7), Up, $dec!(-1))]
        #[case($dec!(-0.7), Floor, $dec!(-1))]
        #[case($dec!(-0.7), HalfUp, $dec!(-1))]
        #[case($dec!(-0.7), HalfDown, $dec!(-1))]
        #[case($dec!(-0.7), HalfEven, $dec!(-1))]
        //------
        #[case($dec!(-0.7), Down, $dec!(-0))]
        #[case($dec!(-0.7), Ceiling, $dec!(-0))]
        //---------------------------------------
        #[case($dec!(-6.5), Up, $dec!(-7))]
        #[case($dec!(-6.5), Floor, $dec!(-7))]
        #[case($dec!(-6.5), HalfUp, $dec!(-7))]
        //------
        #[case($dec!(-6.5), Down, $dec!(-6))]
        #[case($dec!(-6.5), Ceiling, $dec!(-6))]
        #[case($dec!(-6.5), HalfDown, $dec!(-6))]
        #[case($dec!(-6.5), HalfEven, $dec!(-6))]
        //---------------------------------------
        #[case($dec!(-2.0), Up, $dec!(-2))]
        #[case($dec!(-2.0), Ceiling, $dec!(-2))]
        #[case($dec!(-2.0), HalfUp, $dec!(-2))]
        #[case($dec!(-2.0), Down, $dec!(-2))]
        #[case($dec!(-2.0), Floor, $dec!(-2))]
        #[case($dec!(-2.0), HalfDown, $dec!(-2))]
        #[case($dec!(-2.0), HalfEven, $dec!(-2))]
        //---------------------------------------
        fn test_round_modes_signed(#[case] d: $D, #[case] mode: RoundingMode, #[case] expected: $D) {
            let d = d.with_rounding_mode(mode).round(0);
            assert_eq!(d, expected);
            assert_eq!(d.fractional_digits_count(), expected.fractional_digits_count());
        }
    };
}

pub(crate) use test_impl;
