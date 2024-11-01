macro_rules! test_impl {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($udec!(0), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![] })]
        #[case($udec!(1), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![1] })]
        #[case($udec!(10), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![10] })]
        #[case($udec!(100), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![100] })]
        #[case($udec!(10000), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1] })]
        #[case($udec!(10001), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1, 1] })]
        #[case($udec!(1234), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![1234] })]
        #[case($udec!(12345), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1, 2345] })]
        #[case($udec!(12345678), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1234, 5678] })]
        #[case($udec!(100000000), PgNumeric::Positive { weight: 2, scale: 0, digits: vec![1] })]
        #[case($udec!(1.0), PgNumeric::Positive { weight: 0, scale: 1, digits: vec![1] })]
        #[case($udec!(1.1), PgNumeric::Positive { weight: 0, scale: 1, digits: vec![1, 1000] })]
        #[case($udec!(1.10), PgNumeric::Positive { weight: 0, scale: 2, digits: vec![1, 1000] })]
        #[case($udec!(100000000.0001), PgNumeric::Positive { weight: 2, scale: 4, digits: vec![1, 0, 0, 1] })]
        #[case($udec!(0.1), PgNumeric::Positive { weight: -1, scale: 1, digits: vec![1000] })]
        #[case($udec!(0.01), PgNumeric::Positive { weight: -1, scale: 2, digits: vec![100] })]
        #[case($udec!(0.012), PgNumeric::Positive { weight: -1, scale: 3, digits: vec![120] })]
        #[case($udec!(1.2345), PgNumeric::Positive { weight: 0, scale: 4, digits: vec![1, 2345] })]
        #[case($udec!(0.12345), PgNumeric::Positive { weight: -1, scale: 5, digits: vec![1234, 5000] })]
        #[case($udec!(0.01234), PgNumeric::Positive { weight: -1, scale: 5, digits: vec![0123, 4000] })]
        #[case($udec!(12345.67890), PgNumeric::Positive { weight: 1, scale: 5, digits: vec![1, 2345, 6789] })]
        #[case($udec!(0.00001234), PgNumeric::Positive { weight: -2, scale: 8, digits: vec![1234] })]
        #[case($udec!(123.456), PgNumeric::Positive { weight: 0, scale: 3, digits: vec![123, 4560] })]
        #[case($udec!(50e2), PgNumeric::Positive { weight: 0, scale: 0, digits: vec![5000] })]
        #[case($udec!(1e4), PgNumeric::Positive { weight: 1, scale: 0, digits: vec![1] })]
        #[case($udec!(0.1000000000000000), PgNumeric::Positive { weight: -1, scale: 16, digits: vec![1000] })]
        #[case($udec!(0.00315937), PgNumeric::Positive { weight: -1, scale: 8, digits: vec![31, 5937] })]
        #[case($udec!(0.003159370000000000), PgNumeric::Positive { weight: -1, scale: 18, digits: vec![31, 5937] })]
        fn test_convert(#[case] d: $UD, #[case] expected: PgNumeric) {
            let numeric: PgNumeric = d.try_into().unwrap();
            assert_eq!(numeric, expected);

            let decimal: $UD = expected.try_into().unwrap();
            assert_eq!(decimal, d);
        }
        
        #[rstest(::trace)]
        fn test_nan() {
            let e = $UD::try_from(PgNumeric::NaN).unwrap_err();
            assert_eq!(e.to_string(), String::from(concat!("(fastnum) ", stringify!($UD), " does not support NaN values")));
        }
    };
}

macro_rules! test_impl_unsigned {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case(PgNumeric::Negative { weight: 0, scale: 0, digits: vec![1234] })]
        fn test_unsigned(#[case] numeric: PgNumeric) {
            let e = $UD::try_from(numeric).unwrap_err();
            assert_eq!(e.to_string(), String::from(concat!("(fastnum) ", stringify!($UD), " does not support negative values")));
        }
    };
}

macro_rules! test_impl_signed {
    ($dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-1234), PgNumeric::Negative { weight: 0, scale: 0, digits: vec![1234] })]
        #[case($dec!(-12345678), PgNumeric::Negative { weight: 1, scale: 0, digits: vec![1234, 5678] })]
        #[case($dec!(-123.456), PgNumeric::Negative { weight: 0, scale: 3, digits: vec![123, 4560] })]
        #[case($dec!(-56.78), PgNumeric::Negative { weight: 0, scale: 2, digits: vec![56, 7800] })]
        fn test_convert_signed(#[case] d: $D, #[case] expected: PgNumeric) {
            let numeric: PgNumeric = d.try_into().unwrap();
            assert_eq!(numeric, expected);

            let decimal: $D = expected.try_into().unwrap();
            assert_eq!(decimal, d);
        }
    };
}

pub(crate) use test_impl;
pub(crate) use test_impl_signed;
pub(crate) use test_impl_unsigned;
