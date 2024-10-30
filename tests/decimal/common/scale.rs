macro_rules! test_impl {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($udec!(1), 0)]
        #[case($udec!(0.001), -3)]
        #[case($udec!(1), -0)]
        #[case($udec!(1000), 3)]
        fn test_scale(#[case] d: $UD, #[case] scale: i64) {
            assert_eq!(d, $UD::from_scale(scale));
        }
    };
}

pub(crate) use test_impl;
