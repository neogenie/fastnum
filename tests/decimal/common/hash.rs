macro_rules! test_impl {
    (D, $bits: literal) => {
        paste::paste! { test_impl!(SIGNED: [< dec $bits >], [<D $bits>]); }
    };
    (UD, $bits: literal) => {
        paste::paste! { test_impl!(UNSIGNED: [< udec $bits >], [<UD $bits>]); }
    };
    (UNSIGNED: $dec: ident, $D: ident) => {
        mod $dec {
            use fastnum::{$D, $dec};
            use rstest::*;
            use std::hash::{DefaultHasher, Hash, Hasher};

            pub(crate) fn hash<T>(obj: &T) -> u64
            where
                T: Hash,
            {
                let mut hasher = DefaultHasher::new();
                obj.hash(&mut hasher);
                hasher.finish()
            }

            super::test_impl!(COMMON, $dec, $D);
            super::test_impl!(UNSIGNED, $dec, $D);
        }
    };
    (SIGNED: $dec: ident, $D: ident) => {
        mod $dec {
            use fastnum::{$D, $dec};
            use rstest::*;
            use std::hash::{DefaultHasher, Hash, Hasher};

            pub(crate) fn hash<T>(obj: &T) -> u64
            where
                T: Hash,
            {
                let mut hasher = DefaultHasher::new();
                obj.hash(&mut hasher);
                hasher.finish()
            }

            super::test_impl!(COMMON, $dec, $D);
            super::test_impl!(SIGNED, $dec, $D);
        }
    };
    (COMMON, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(1.1234), $dec!(1.1234000))]
        #[case($dec!(001.1234), $dec!(0001.1234))]
        #[case($dec!(1.1234000000), $dec!(1.1234000))]
        #[case($dec!(1.12340), $dec!(1.1234000000))]
        #[case($dec!(100), $dec!(100.00))]
        #[case($dec!(0.00), $dec!(0))]
        #[case($dec!(0.00), $dec!(0.000))]
        #[case($dec!(100), $dec!(1e2))]
        #[case($dec!(0.01), $dec!(1e-2))]
        fn test_hash_eq(#[case] a: $D, #[case] b: $D) {
            assert_eq!(a, b);
            assert_eq!(hash(&a), hash(&b));
        }

        #[rstest(::trace)]
        #[case($dec!(1.1234), $dec!(1.1234001))]
        #[case($dec!(10000), $dec!(10))]
        #[case($dec!(10), $dec!(10000))]
        #[case($dec!(10.0), $dec!(100))]
        #[case($dec!(1e3), $dec!(1e2))]
        #[case($dec!(0.001), $dec!(0.001e1))]
        fn test_hash_ne(#[case] a: $D, #[case] b: $D) {
            assert_ne!(a, b);
            assert_ne!(hash(&a), hash(&b));
        }
    };
    (UNSIGNED, $dec: ident, $D: ident) => {};
    (SIGNED, $dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0901300e-3), $dec!(-901.3))]
        #[case($dec!(-0.901300e+3), $dec!(-901.3))]
        fn test_hash_eq_signed(#[case] a: $D, #[case] b: $D) {
            assert_eq!(a, b);
            assert_eq!(hash(&a), hash(&b));
        }

        #[rstest(::trace)]
        #[case($dec!(-0901300e-4), $dec!(-901.3))]
        #[case($dec!(-0.901300e+3), $dec!(-901.31))]
        #[case($dec!(-0.00), $dec!(0.000))]
        #[case($dec!(0.00), $dec!(-0.000))]
        fn test_hash_ne_signed(#[case] a: $D, #[case] b: $D) {
            assert_ne!(a, b);
            assert_ne!(hash(&a), hash(&b));
        }
    };
}

pub(crate) use test_impl;
