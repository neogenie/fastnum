macro_rules! macro_impl {
    ($DEC: ident, $bits: literal, $sign: ident, $name: ident) => {
        #[macro_export]
        #[doc = concat!("A macro to construct ", $bits, "-bit ", stringify!($sign), " [crate::", stringify!($DEC), "] decimal from literals in compile time.")]
        ///
        /// Const-evaluated in compile time macro-helper can be used for definitions of constants or variables whose value is known in compile time.
        ///
        /// # Examples:
        ///
        /// ## Basic usage:
        ///
        /// ```
        #[doc = concat!("use fastnum::{", stringify!($name), ", ", stringify!($DEC), ", decimal::RoundingMode};")]
        ///
        #[doc = concat!("const N: ", stringify!($DEC), " = ", stringify!($name), "!(1.23456789);")]
        /// assert!(!N.is_zero());
        /// 
        #[doc = concat!("let num = ", stringify!($name), "!(0);")]
        /// assert!(num.is_zero());
        /// 
        #[doc = concat!("const A: ", stringify!($DEC), " = ", stringify!($name), "!(5);")]
        #[doc = concat!("const B: ", stringify!($DEC), " = ", stringify!($name), "!(1_000);")]
        #[doc = concat!("const C: ", stringify!($DEC), " = A.div(B, RoundingMode::HalfUp).unwrap();")]
        ///
        #[doc = concat!("assert_eq!(C, ", stringify!($name), "!(0.005));")]
        /// 
        /// ```
        ///
        /// ## Static assertions:
        ///
        /// ```compile_fail
        /// // The below example will fail to compile, as the function will panic at compile time:
        #[doc = concat!("use fastnum::{", stringify!($name), ", ", stringify!($DEC), "}")]
        ///
        /// // Gives a compile error of "error[E0080]: evaluation of constant value failed... the evaluated program panicked at 'attempt to parse decimal from string containing invalid digit'",
        #[doc = concat!("const N: ", stringify!($DEC), " = ", stringify!($name), "!(A1.23456789);")]
        /// ```
        ///
        ///
        /// This protect from
        /// This allows you to perform all the necessary checks such as potentialy overflow or calculation accuracy loss and others at the compile time.
        /// Protect from potential errors in runtime
        /// Avoid 
        /// 
        ///
        macro_rules! $name {
            ($lit: literal) => {{
                const __DECIMAL: $crate::$DEC = $crate::$DEC::parse_str($crate::const_str::replace!($crate::const_str::squish!(stringify!($lit)), " ", ""));
                __DECIMAL
            }};
            (+ $lit: literal) => {{
                const __DECIMAL: $crate::$DEC = $crate::$DEC::parse_str($crate::const_str::concat!("+", $crate::const_str::replace!($crate::const_str::squish!(stringify!($lit)), " ", "")));
                __DECIMAL
            }};
            ($lit: expr) => {{
                const __DECIMAL: $crate::$DEC = $crate::$DEC::parse_str($crate::const_str::replace!($crate::const_str::squish!(stringify!($lit)), " ", ""));
                __DECIMAL
            }};
        }
    };
}

macro_impl!(UD128, 128, unsigned, udec128);
macro_impl!(UD256, 256, unsigned, udec256);
macro_impl!(UD512, 512, unsigned, udec512);
macro_impl!(UD1024, 1024, unsigned, udec1024);
macro_impl!(UD2048, 2048, unsigned, udec2048);
macro_impl!(UD4096, 4096, unsigned, udec4096);
macro_impl!(UD8192, 8192, unsigned, udec8192);

macro_impl!(D128, 128, signed, dec128);
macro_impl!(D256, 256, signed, dec256);
macro_impl!(D512, 512, signed, dec512);
macro_impl!(D1024, 1024, signed, dec1024);
macro_impl!(D2048, 2048, signed, dec2048);
macro_impl!(D4096, 4096, signed, dec4096);
macro_impl!(D8192, 8192, signed, dec8192);

#[allow(unused_macros)]
macro_rules! decimal_err {
    ($t: ty, $e: expr) => {
        crate::decimal::error::pretty_error_msg(stringify!($t), $e)
    };
}

#[allow(unused_imports)]
pub(crate) use decimal_err;
