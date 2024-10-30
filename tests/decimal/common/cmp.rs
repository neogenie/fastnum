macro_rules! test_impl {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($udec!(0), $udec!(1))]
        #[case($udec!(1), $udec!(10))]
        #[case($udec!(100), $udec!(100.1))]
        #[case($udec!(1e2), $udec!(100.1))]
        #[case($udec!(100), $udec!(1.1e2))]
        #[case($udec!(1.2345), $udec!(1.2346))]
        #[case($udec!(25.6), $udec!(25.8))]
        #[case($udec!(1e-9223372036854775807), $udec!(1))]
        #[case($udec!(1e-9223372036854775807), $udec!(1e9223372036854775807))]
        #[case($udec!(1), $udec!(340282366920938463463374607431768211455))]
        #[case($udec!(500), $udec!(51e1))]
        #[case($udec!(44e1), $udec!(500))]
        #[case($udec!(1234000000000), $udec!(12345e9))]
        #[case($udec!(1514932018891593916341142774e-24), $udec!(1514932018891593916341142773.0001))]
        #[case($UD::from_scale(i64::MIN + 1), $UD::from_scale(i64::MAX - 1))]
        fn test_cmp(#[case] a: $UD, #[case] b: $UD) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));
            
            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));
            
            assert_ne!(a, b);
            assert!(a < b);
            assert!(b > a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
        
        #[rstest(::trace)]
        #[case($udec!(0), $udec!(0))]
        #[case($udec!(00), $udec!(0))]
        #[case($udec!(0), $udec!(0.00))]
        #[case($udec!(1), $udec!(1))]
        #[case($udec!(00001), $udec!(1))]
        #[case($udec!(00001), $udec!(1.0000))]
        #[case($udec!(1), $udec!(1.00))]
        #[case($udec!(10), $udec!(10))]
        #[case($udec!(10), $udec!(1e1))]
        #[case($udec!(1.1), $udec!(1.1))]
        #[case($udec!(1.2e-2), $udec!(0.012))]
        #[case($udec!(5000), $udec!(50e2))]
        #[case($udec!(0.000034500), $udec!(345e-7))]
        #[case($udec!(1514932018891593.916341142773), $udec!(1514932018891593916341142773e-12))]
        #[case($UD::from_scale(i64::MAX - 1), $UD::from_scale(i64::MAX - 1))]
        #[case($UD::from_scale(i64::MIN + 1), $UD::from_scale(i64::MIN + 1))]
        fn test_eq(#[case] a: $UD, #[case] b: $UD) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));
            
            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));
            
            assert_eq!(a, b);
            assert_eq!(b, a);
        }
    }
}

macro_rules! test_impl_256 {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($udec!(472697816888807260.1604), $udec!(472697816888807260.16040000000000000000001))]
        #[case($udec!(1), $udec!(1.0000000000000000000000000000000000000000000000000001))]
        #[case($udec!(1000000000000000000000000000000000000000), $udec!(1e41))]
        #[case($udec!(1116386634271380982470843247639640260491505327092723527088459), $udec!(759522625769651746138617259189939751893902453291243506584717e2))]
        fn test_cmp_256(#[case] a: $UD, #[case] b: $UD) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_ne!(a, b);
            assert!(a < b);
            assert!(b > a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
    }
}

macro_rules! test_impl_512 {
    ($udec: ident, $UD: ident) => {
        #[rstest(::trace)]
        #[case($udec!(1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000), $udec!(1.41421356237309504880168872420969807856967187537694807317667974000000000))]
        #[case($udec!(1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000), $udec!(11.41421356237309504880168872420969807856967187537694807317667974000000000))]
        fn test_cmp_512(#[case] a: $UD, #[case] b: $UD) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));

            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));

            assert_ne!(a, b);
            assert!(a < b);
            assert!(b > a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
    }
}

macro_rules! test_impl_signed {
    ($dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-0), $dec!(0))]
        #[case($dec!(-0), $dec!(+0))]
        #[case($dec!(0), $dec!(+0))]
        #[case($dec!(-1), $dec!(1))]
        #[case($dec!(-1), $dec!(0))]
        #[case($dec!(-1), $dec!(-0))]
        #[case($dec!(-1), $dec!(+0))]
        #[case($dec!(-10), $dec!(10))]
        #[case($dec!(-1), $dec!(10))]
        #[case($dec!(-1.1), $dec!(1.01))]
        #[case($dec!(-0.001), $dec!(-0.000000001))]
        #[case($dec!(-0.001), $dec!(+0.000000001))]
        fn test_cmp_signed(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));
            
            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));
            
            assert_ne!(a, b);
            assert!(a < b);
            assert!(b > a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
        
        #[rstest(::trace)]
        #[case($dec!(0), $dec!(0))]
        #[case($dec!(-0), $dec!(-0))]
        #[case($dec!(+0), $dec!(+0))]
        #[case($dec!(+1.1), $dec!(+1.1))]
        #[case($dec!(-1.1), $dec!(-1.1))]
        #[case($dec!(-1), $dec!(-1000e-3))]
        #[case($dec!(-0.000034500), $dec!(-345e-7))]
        fn test_eq_signed(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));
            
            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));
            
            assert_eq!(a, b);
            assert_eq!(b, a);
        }
    }
}

macro_rules! test_impl_signed_256 {
    ($dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-1.0000000000000000000000000000000000000000000000000001), $dec!(-1))]
        fn test_cmp_signed_256(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));
            
            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));
            
            assert_ne!(a, b);
            assert!(a < b);
            assert!(b > a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
    }
}

macro_rules! test_impl_signed_512 {
    ($dec: ident, $D: ident) => {
        #[rstest(::trace)]
        #[case($dec!(-1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000), $dec!(-11.41421356237309504880168872420969807856967187537694807317667974000000000e-2))]
        #[case($dec!(-11.41421356237309504880168872420969807856967187537694807317667974000000000), $dec!(-1.414213562373095048801688724209698078569671875376948073176679730000000000000000000000000000000000000))]
        fn test_cmp_signed_512(#[case] a: $D, #[case] b: $D) {
            #[allow(clippy::eq_op)]
            (assert_eq!(a, a));
            
            #[allow(clippy::eq_op)]
            (assert_eq!(b, b));
            
            assert_ne!(a, b);
            assert!(a < b);
            assert!(b > a);
            assert_eq!(max(a, b), b);
            assert_eq!(min(a, b), a)
        }
    }
}

pub(crate) use test_impl;
pub(crate) use test_impl_256;
pub(crate) use test_impl_512;

pub(crate) use test_impl_signed;
pub(crate) use test_impl_signed_256;
pub(crate) use test_impl_signed_512;

