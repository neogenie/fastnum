macro_rules! macro_impl {
    ($UINT: ident, $bits: literal, $name: ident) => {
        #[macro_export]
        #[doc = concat!("A macro to construct ", $bits, "-bit [crate::", stringify!($UINT), "] integer from literals.")]
        ///
        ///
        /// # Examples:
        /// ```
        #[doc = concat!("use fastnum::{", stringify!($name), ", ", stringify!($UINT), "}")]
        ///   use num_traits::{Signed, Zero};
        /// 
        #[doc = concat!("const N: ", stringify!($UINT), " = ", stringify!($name), "!(100);")]
        #[doc = concat!("let x = ", stringify!($name), "!(1);")]
        #[doc = concat!("assert!(", stringify!($name), "!(0).is_zero());")]
        #[doc = concat!("assert_eq!(", stringify!($name), "!(115792089237316195423570985008687907853269984665640564039457584007913129639935), ", stringify!($UINT) ,"::MAX);")]
        /// ```
        macro_rules! $name {
            ($lit:expr) => {{
                const __UINT: $crate::$UINT = $crate::int::parse::$name::parse_str(stringify!($lit));
                __UINT
            }};
        }
    };
}

macro_impl!(U128, 128, u128);
macro_impl!(U256, 256, u256);
macro_impl!(U512, 512, u512);

macro_impl!(I128, 128, i128);
macro_impl!(I256, 256, i256);
macro_impl!(I512, 512, i512);