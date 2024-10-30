use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) fn hash<T>(obj: &T) -> u64
where T: Hash
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

macro_rules! test_impl {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($udec!(1.1234), $udec!(1.1234000))]
        #[case($udec!(001.1234), $udec!(0001.1234))]
        #[case($udec!(1.1234000000), $udec!(1.1234000))]
        #[case($udec!(1.12340), $udec!(1.1234000000))]
        #[case($udec!(100), $udec!(100.00))]
        #[case($udec!(0.00), $udec!(0))]
        #[case($udec!(0.00), $udec!(0.000))]
        #[case($udec!(100), $udec!(1e2))]
        #[case($udec!(0.01), $udec!(1e-2))]
        fn test_hash_eq(#[case] a: $UD, #[case] b: $UD) {
            use crate::decimal::common::hash::hash;
            assert_eq!(a, b);
            assert_eq!(hash(&a), hash(&b));
        }
        
        
        #[rstest(::trace)]
        #[case($udec!(1.1234), $udec!(1.1234001))]
        #[case($udec!(10000), $udec!(10))]
        #[case($udec!(10), $udec!(10000))]
        #[case($udec!(10.0), $udec!(100))]
        #[case($udec!(1e3), $udec!(1e2))]
        #[case($udec!(0.001), $udec!(0.001e1))]
        fn test_hash_ne(#[case] a: $UD, #[case] b: $UD) {
            use crate::decimal::common::hash::hash;
            assert_ne!(a, b);
            assert_ne!(hash(&a), hash(&b));
        }
    };
}

macro_rules! test_impl_signed {
    ($dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0901300e-3), $dec!(-901.3))]
        #[case($dec!(-0.901300e+3), $dec!(-901.3))]
        fn test_hash_eq_signed(#[case] a: $D, #[case] b: $D) {
            use crate::decimal::common::hash::hash;
            assert_eq!(a, b);
            assert_eq!(hash(&a), hash(&b));
        }
        
        #[rstest(::trace)]
        #[case($dec!(-0901300e-4), $dec!(-901.3))]
        #[case($dec!(-0.901300e+3), $dec!(-901.31))]
        #[case($dec!(-0.00), $dec!(0.000))]
        #[case($dec!(0.00), $dec!(-0.000))]
        fn test_hash_ne_signed(#[case] a: $D, #[case] b: $D) {
            use crate::decimal::common::hash::hash;
            assert_ne!(a, b);
            assert_ne!(hash(&a), hash(&b));
        }
    };
}

pub(crate) use test_impl;
pub(crate) use test_impl_signed;