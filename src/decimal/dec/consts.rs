macro_rules! consts_impl {
    () => {
        impl<const N: usize> Decimal<N> {

            /// **N**ot **A** **N**umber value. More about [`NaN`](crate#special-values).
            pub const NAN: Self = Self::new(UInt::ZERO, 0, ControlBlock::nan(), ExtraPrecision::new());

            /// Infinity (∞). More about [`±Infinity`](crate#special-values).
            pub const INFINITY: Self = Self::new(UInt::MAX, i16::MIN, ControlBlock::infinity(), ExtraPrecision::new());

            /// Negative infinity (−∞). More about [`±Infinity`](crate#special-values).
            pub const NEG_INFINITY: Self = Self::new(UInt::MAX, i16::MIN, ControlBlock::neg_infinity(), ExtraPrecision::new());

            /// The smallest value that can be represented by this decimal type - (2<sup>N</sup> - 1) × 10<sup>32'768</sup>.
            pub const MIN: Self = Self::new(UInt::MAX, i16::MIN, ControlBlock::default().neg(), ExtraPrecision::new());

            /// The maximum value that this type can represent (2<sup>N</sup> - 1) × 10<sup>32'768</sup>.
            pub const MAX: Self = Self::new(UInt::MAX, i16::MIN, ControlBlock::default(), ExtraPrecision::new());

            /// The smallest positive, normalized value that this type can represent.
            pub const MIN_POSITIVE: Self = Self::new(UInt::ONE, i16::MAX, ControlBlock::default(), ExtraPrecision::new());

            /// [Machine epsilon] value.
            ///
            /// This is the difference between `1.0` and the next larger representable number.
            ///
            /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
            pub const EPSILON: Self = Self::new(UInt::ONE, Intrinsics::<N>::MAX_CLENGTH as i16 - 1, ControlBlock::default(), ExtraPrecision::new());

            consts_impl!(CONSTS ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

            /// The value of `0.5` represented by this decimal type.
            pub const HALF: Self = Self::ONE.div(Self::TWO);

            /// Euler's number (e).
            pub const E: Self = math::consts::Consts::<N>::E;

            /// Archimedes’ constant (π).
            pub const PI: Self = math::consts::Consts::<N>::PI;

            /// The full circle constant (τ)
            ///
            /// Equal to 2π.
            pub const TAU: Self = math::consts::Consts::<N>::TAU;

            /// 1/π.
            pub const FRAC_1_PI: Self = math::consts::Consts::<N>::FRAC_1_PI;

            /// 2/π.
            pub const FRAC_2_PI: Self = math::consts::Consts::<N>::FRAC_2_PI;

            /// π/2.
            pub const FRAC_PI_2: Self = math::consts::Consts::<N>::FRAC_PI_2;

            /// π/3.
            pub const FRAC_PI_3: Self = math::consts::Consts::<N>::FRAC_PI_3;

            /// π/4.
            pub const FRAC_PI_4: Self = math::consts::Consts::<N>::FRAC_PI_4;

            /// π/6.
            pub const FRAC_PI_6: Self = math::consts::Consts::<N>::FRAC_PI_6;

            /// π/8.
            pub const FRAC_PI_8: Self = math::consts::Consts::<N>::FRAC_PI_8;

            /// 2/sqrt(π).
            pub const FRAC_2_SQRT_PI: Self = math::consts::Consts::<N>::FRAC_2_SQRT_PI;

            /// ln(2).
            pub const LN_2: Self = math::consts::Consts::<N>::LN_2;

            /// ln(10).
            pub const LN_10: Self = math::consts::Consts::<N>::LN_10;

            /// log<sub>2</sub>(e).
            pub const LOG2_E: Self = math::consts::Consts::<N>::LOG2_E;

            /// log<sub>10</sub>(e).
            pub const LOG10_E: Self = math::consts::Consts::<N>::LOG10_E;

            /// sqrt(2).
            pub const SQRT_2: Self = math::consts::Consts::<N>::SQRT_2;

            /// 1/sqrt(2).
            pub const FRAC_1_SQRT_2: Self = math::consts::Consts::<N>::FRAC_1_SQRT_2;

            /// log<sub>10</sub>(2).
            pub const LOG10_2: Self = math::consts::Consts::<N>::LOG10_2;

            /// log<sub>2</sub>(10).
            pub const LOG2_10: Self = math::consts::Consts::<N>::LOG2_10;
        }
    };
    (CONSTS $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by this decimal type.")]
            pub const $name: Self = Self::new(UInt::$name, 0, ControlBlock::default(), ExtraPrecision::new());
        )*
    }
}

pub(crate) use consts_impl;
