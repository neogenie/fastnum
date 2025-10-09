use crate::{
    bint::{intrinsics, Int, ParseError, UInt},
    utils::const_generics::{Dimension, Narrow, Widen},
    Cast, TryCast,
};

impl<const N: usize, const M: usize> Cast<Int<N>> for Int<M>
where
    Dimension<N, M>: Widen,
{
    #[inline(always)]
    fn cast(self) -> Int<N> {
        // SAFETY: `N` is always greater or equal than `M`. So we can safely cast to the
        // widest type.
        #[allow(unsafe_code)]
        unsafe {
            self._transmute()
        }
    }
}

impl<const N: usize, const M: usize> TryCast<Int<N>> for Int<M>
where
    Dimension<N, M>: Narrow,
{
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<Int<N>, Self::Error> {
        // For signed integers, narrowing is valid if the upper (M-N) digits are properly sign-extended.
        // That is, they should all be 0x00...00 for positive, or all 0xFF...FF for negative.

        let bits = self.to_bits();
        let digits = bits.digits();

        // Determine the expected value for upper digits based on the sign bit of digit N-1
        // The sign bit is the MSB of digit N-1
        let sign_digit = digits[N - 1];
        // Branchless: use arithmetic right shift to propagate the sign bit
        let expected_digit = ((sign_digit as i64) >> 63) as u64;

        // Check if all upper digits match the expected sign extension
        let mut i = N;
        while i < M {
            if digits[i] != expected_digit {
                return Err(ParseError::PosOverflow);
            }
            i += 1;
        }

        // If we get here, the value fits. Manually create the narrow value by copying the lower N digits.
        // SAFETY: We've verified that the upper (M-N) digits are properly sign-extended,
        // so it's safe to transmute to the narrow type.
        #[allow(unsafe_code)]
        unsafe {
            let narrow_digits = intrinsics::_transmute::<M, N, N>(digits);
            Ok(Int::from_digits(narrow_digits))
        }
    }
}

impl<const N: usize, const M: usize> TryCast<UInt<N>> for Int<M> {
    type Error = ParseError;

    #[inline(always)]
    fn try_cast(self) -> Result<UInt<N>, Self::Error> {
        if self.is_negative() {
            Err(ParseError::Signed)
        } else if self.bits() <= Int::<N>::BITS {
            // SAFETY: UInt<M> is wider (`N` < `M`) but its value fit to UInt<N>. So we can
            // safely cast to the narrow type.
            #[allow(unsafe_code)]
            {
                Ok(unsafe { self._transmute() }.to_bits())
            }
        } else {
            Err(ParseError::PosOverflow)
        }
    }
}
