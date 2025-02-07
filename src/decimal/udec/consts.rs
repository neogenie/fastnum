macro_rules! consts_impl {
    () => {
        impl<const N: usize> UnsignedDecimal<N> {

            /// **N**ot **A** **N**umber value. More about [`NaN`](crate#special-values).
            pub const NAN: Self = Self::new(Decimal::NAN);

            /// Infinity (∞). More about [`±Infinity`](crate#special-values).
            pub const INFINITY: Self = Self::new(Decimal::INFINITY);

            /// The smallest value that can be represented by this decimal type (0).
            pub const MIN: Self = Self::ZERO;

            /// The largest value that can be represented by this decimal type (2<sup>N</sup> − 1)×10<sup>32'768</sup>.
            pub const MAX: Self = Self::new(Decimal::MAX);

            /// The smallest positive, normalized value that this type can represent.
            pub const MIN_POSITIVE: Self = Self::new(Decimal::MIN_POSITIVE);

            /// [Machine epsilon] value.
            ///
            /// This is the difference between `1.0` and the next larger representable number.
            ///
            /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
            pub const EPSILON: Self = Self::new(Decimal::EPSILON);

            consts_impl!(CONSTS ZERO 0, ONE 1, TWO 2, THREE 3, FOUR 4, FIVE 5, SIX 6, SEVEN 7, EIGHT 8, NINE 9, TEN 10);

            /// Euler's number (e).
            pub const E: Self = Self::new(Decimal::E);

            /// Archimedes’ constant (π).
            pub const PI: Self = Self::new(Decimal::PI);

            /// The full circle constant (τ)
            ///
            /// Equal to 2π.
            pub const TAU: Self = Self::new(Decimal::TAU);

            /// 1/π.
            pub const FRAC_1_PI: Self = Self::new(Decimal::FRAC_1_PI);

            /// 2/π.
            pub const FRAC_2_PI: Self = Self::new(Decimal::FRAC_2_PI);

            /// π/2.
            pub const FRAC_PI_2: Self = Self::new(Decimal::FRAC_PI_2);

            /// π/3.
            pub const FRAC_PI_3: Self = Self::new(Decimal::FRAC_PI_3);

            /// π/4.
            pub const FRAC_PI_4: Self = Self::new(Decimal::FRAC_PI_4);

            /// π/6.
            pub const FRAC_PI_6: Self = Self::new(Decimal::FRAC_PI_6);

            /// π/8.
            pub const FRAC_PI_8: Self = Self::new(Decimal::FRAC_PI_8);

            /// 2/sqrt(π).
            pub const FRAC_2_SQRT_PI: Self = Self::new(Decimal::FRAC_2_SQRT_PI);

            /// ln(2).
            pub const LN_2: Self = Self::new(Decimal::LN_2);

            /// ln(10).
            pub const LN_10: Self = Self::new(Decimal::LN_10);

            /// log<sub>2</sub>(e).
            pub const LOG2_E: Self = Self::new(Decimal::LOG2_E);

            /// log<sub>10</sub>(e).
            pub const LOG10_E: Self = Self::new(Decimal::LOG10_E);

            /// sqrt(2).
            pub const SQRT_2: Self = Self::new(Decimal::SQRT_2);

            /// 1/sqrt(2).
            pub const FRAC_1_SQRT_2: Self = Self::new(Decimal::FRAC_1_SQRT_2);

            /// log<sub>10</sub>(2).
            pub const LOG10_2: Self = Self::new(Decimal::LOG10_2);

            /// log<sub>2</sub>(10).
            pub const LOG2_10: Self = Self::new(Decimal::LOG2_10);
        }
    };
    (CONSTS $($name: ident $num: literal), *) => {
        $(
            #[doc = concat!("The value of `", $num, "` represented by this decimal type.")]
            pub const $name: Self = Self::new(Decimal::$name);
        )*
    }
}

pub(crate) use consts_impl;
