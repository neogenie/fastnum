macro_rules! consts_impl {
    () => {
        impl<const N: usize> Decimal<N> {

            /// **N**ot **A** **N**umber value. More about [`NaN`](crate#special-values).
            pub const NAN: Self = Self::new(UInt::ZERO, ControlBlock::NAN);

            /// Infinity (∞). More about [`±Infinity`](crate#special-values).
            pub const INFINITY: Self = Self::new(UInt::MAX, ControlBlock::INFINITY);

            /// Negative infinity (−∞). More about [`±Infinity`](crate#special-values).
            pub const NEG_INFINITY: Self = Self::new(UInt::MAX, ControlBlock::NEG_INFINITY);

            /// The smallest value that can be represented by this decimal type - (2<sup>N</sup> - 1) × 10<sup>32'768</sup>.
            pub const MIN: Self = Self::new(UInt::MAX, ControlBlock::basic(i16::MIN, Sign::Minus));

            /// The maximum value that this type can represent (2<sup>N</sup> - 1) × 10<sup>32'768</sup>.
            pub const MAX: Self = Self::new(UInt::MAX, ControlBlock::basic(i16::MIN, Sign::Plus));

            /// The smallest positive, normalized value that this type can represent.
            pub const MIN_POSITIVE: Self = Self::new(UInt::ONE, ControlBlock::basic(i16::MAX, Sign::Plus));

            /// [Machine epsilon] value.
            ///
            /// This is the difference between `1.0` and the next larger representable number.
            ///
            /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
            pub const EPSILON: Self = Self::new(UInt::ONE, ControlBlock::basic(Intrinsics::<N>::MAX_CLENGTH as i16 - 1, Sign::Plus));

            consts_impl!(CONSTS ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

            /// The value of `0.5` represented by this decimal type.
            pub const HALF: Self = Self::new(UInt::from_digit(5), ControlBlock::basic(1, Sign::Plus));

            /// Euler's number (e).
            pub const E: Self = Consts::<N>::E.round_extra_precision();

            /// Archimedes’ constant (π).
            pub const PI: Self = Consts::<N>::PI.round_extra_precision();

            /// The full circle constant (τ)
            ///
            /// Equal to 2π.
            pub const TAU: Self = Consts::<N>::TAU.round_extra_precision();

            /// 1/π.
            pub const FRAC_1_PI: Self = Consts::<N>::FRAC_1_PI.round_extra_precision();

            /// 2/π.
            pub const FRAC_2_PI: Self = Consts::<N>::FRAC_2_PI.round_extra_precision();

            /// π/2.
            pub const FRAC_PI_2: Self = Consts::<N>::FRAC_PI_2.round_extra_precision();

            /// π/3.
            pub const FRAC_PI_3: Self = Consts::<N>::FRAC_PI_3.round_extra_precision();

            /// π/4.
            pub const FRAC_PI_4: Self = Consts::<N>::FRAC_PI_4.round_extra_precision();

            /// π/6.
            pub const FRAC_PI_6: Self = Consts::<N>::FRAC_PI_6.round_extra_precision();

            /// π/8.
            pub const FRAC_PI_8: Self = Consts::<N>::FRAC_PI_8.round_extra_precision();

            /// 2/sqrt(π).
            pub const FRAC_2_SQRT_PI: Self = Consts::<N>::FRAC_2_SQRT_PI.round_extra_precision();

            /// ln(2).
            pub const LN_2: Self = Consts::<N>::LN_2.round_extra_precision();

            /// ln(10).
            pub const LN_10: Self = Consts::<N>::LN_10.round_extra_precision();

            /// log<sub>2</sub>(e).
            pub const LOG2_E: Self = Consts::<N>::LOG2_E.round_extra_precision();

            /// log<sub>10</sub>(e).
            pub const LOG10_E: Self = Consts::<N>::LOG10_E.round_extra_precision();

            /// sqrt(2).
            pub const SQRT_2: Self = Consts::<N>::SQRT_2.round_extra_precision();

            /// 1/sqrt(2).
            pub const FRAC_1_SQRT_2: Self = Consts::<N>::FRAC_1_SQRT_2.round_extra_precision();

            /// log<sub>10</sub>(2).
            pub const LOG10_2: Self = Consts::<N>::LOG10_2.round_extra_precision();

            /// log<sub>2</sub>(10).
            pub const LOG2_10: Self = Consts::<N>::LOG2_10.round_extra_precision();
        }
    };
    (CONSTS $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by this decimal type.")]
            pub const $name: Self = Self::new(UInt::$name, ControlBlock::basic(0, Sign::Plus));
        )*
    }
}

pub(crate) use consts_impl;
