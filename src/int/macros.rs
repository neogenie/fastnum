macro_rules! macro_impl {
    ($INT: ident, $bits: literal, $sign: ident, $name: ident) => {
        #[macro_export]
        #[doc = concat!("A macro to construct ", $bits, "-bit [`", stringify!($INT), "`](crate::", stringify!($INT), ") ", stringify!($sign), " integer from literals.")]
        ///
        ///
        /// # Examples:
        ///
        /// ```
        #[doc = concat!("use fastnum::{", stringify!($name), ", ", stringify!($INT), "};")]
        ///
        #[doc = concat!("const N: ", stringify!($INT), " = ", stringify!($name), "!(100);")]
        #[doc = concat!("let x = ", stringify!($name), "!(1);")]
        #[doc = concat!("assert!(", stringify!($name), "!(0).is_zero());")]
        /// println!("{x}");
        /// ```
        ///
        macro_rules! $name {
            ($lit:expr) => {{
                const __INT: $crate::$INT = $crate::int::parse::$sign::parse_str(stringify!($lit));
                __INT
            }};
        }
    };
}

macro_impl!(U128, 128, unsigned, u128);
macro_impl!(U256, 256, unsigned, u256);
macro_impl!(U512, 512, unsigned, u512);
macro_impl!(U1024, 1024, unsigned, u1024);
macro_impl!(U2048, 2048, unsigned, u2048);
macro_impl!(U4096, 4096, unsigned, u4096);
macro_impl!(U8192, 8192, unsigned, u8192);

macro_impl!(I128, 128, signed, i128);
macro_impl!(I256, 256, signed, i256);
macro_impl!(I512, 512, signed, i512);
macro_impl!(I1024, 1024, signed, i1024);
macro_impl!(I2048, 2048, signed, i2048);
macro_impl!(I4096, 4096, signed, i4096);
macro_impl!(I8192, 8192, signed, i8192);
