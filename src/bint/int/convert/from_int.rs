// Helper macro to perform sign-extended conversion from signed integers
// Note: This is an internal implementation detail and should not be used
// directly.
#[doc(hidden)]
#[macro_export]
macro_rules! __sign_extend_from_int_impl {
    ($int:expr, $int_ty:ty) => {{
        // Initialize all digits based on sign (for proper sign extension)
        let mut digits = if $int.is_negative() {
            *($crate::bint::UInt::MAX.digits())
        } else {
            *($crate::bint::UInt::ZERO.digits())
        };

        // Copy the actual bits from the input integer
        let mut i = 0;
        while i << $crate::bint::intrinsics::DIGIT_BIT_SHIFT < <$int_ty>::BITS as usize {
            let d = ($int >> (i << $crate::bint::intrinsics::DIGIT_BIT_SHIFT))
                as $crate::bint::intrinsics::Digit;
            digits[i] = d;
            i += 1;
        }

        digits
    }};
}

macro_rules! from_int_impl {
    ($($name:ident <- $int:ident ($from_uint:ident <- $uint:ident)),*) => {
        $(
            #[inline(always)]
            #[doc = doc::convert::from!($int I 256)]
            pub const fn $name(int: $int) -> Self {
                debug_assert!($int::BITS <= Self::BITS);
                let digits = $crate::__sign_extend_from_int_impl!(int, $int);
                Self::from_digits(digits)
            }
        )*
    };
}

pub(crate) use from_int_impl;

macro_rules! try_from_int_impl {
    ($($from_int:ident <- $int:ident),*) => {
        $(
            #[inline(always)]
            #[doc = doc::convert::from!($int I 256)]
            pub const fn $from_int(int: $int) -> Result<Self, ParseError> {
                if $int::BITS - int.leading_zeros() > Self::BITS {
                     return Err(ParseError::PosOverflow);
                } else {
                    let digits = $crate::__sign_extend_from_int_impl!(int, $int);
                    Ok(Self::from_digits(digits))
                }
            }
        )*
    };
}

pub(crate) use try_from_int_impl;
