macro_rules! macro_impl {
    ($DEC: ident, $bits: literal, $module: ident, $sign: ident, $name: ident) => {
        #[macro_export]
        #[doc = concat!("A macro to construct ", $bits, "-bit [crate::", stringify!($DEC), "] decimal from literals in compile time.")]
        ///
        ///
        /// # Examples:
        /// ```
        #[doc = concat!("use fastnum::{", stringify!($name), ", ", stringify!($DEC), "}")]
        ///   use num_traits::{Signed, Zero};
        /// 
        #[doc = concat!("const N: ", stringify!($DEC), " = ", stringify!($name), "!(1.23456789);")]
        ///   assert!(NUM.is_positive());
        #[doc = concat!("let num = ", stringify!($name), "!(0);")]
        ///   assert!(num.is_zero());
        #[doc = concat!("let num = ", stringify!($name), "!(-0.1);")]
        ///   assert!(num.is_negative());
        /// ```
        macro_rules! $name {
            ($lit: literal) => {{
                const __DECIMAL: $crate::$DEC = $crate::decimal::$sign::parse::$module::parse_str(const_str::replace!(const_str::squish!(stringify!($lit)), " ", ""));
                __DECIMAL
            }};
            (+ $lit: literal) => {{
                const __DECIMAL: $crate::$DEC = $crate::decimal::$sign::parse::$module::parse_str(const_str::concat!("+", const_str::replace!(const_str::squish!(stringify!($lit)), " ", "")));
                __DECIMAL
            }};
            ($lit: expr) => {{
                const __DECIMAL: $crate::$DEC = $crate::decimal::$sign::parse::$module::parse_str(const_str::replace!(const_str::squish!(stringify!($lit)), " ", ""));
                __DECIMAL
            }};
        }
    };
}

macro_impl!(UD128, 128, d128, unsigned, udec128);
macro_impl!(UD256, 256, d256, unsigned, udec256);
macro_impl!(UD512, 512, d512, unsigned, udec512);

macro_impl!(D128, 128, d128, signed, dec128);
macro_impl!(D256, 256, d256, signed, dec256);
macro_impl!(D512, 512, d512, signed, dec512);

macro_rules! decimal_err {
    ($t: ty, $e: expr) => {
        $crate::decimal::error::pretty_error_msg(stringify!($t), $e)
    };
}

pub(crate) use decimal_err;