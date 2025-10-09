use crate::bint::{intrinsics::_transmute, Int, UInt};
use core::ptr;

#[allow(unsafe_code)]
#[inline(always)]
pub const unsafe fn transmute<const N: usize, const M: usize>(v: Int<N>) -> Int<M> {
    let bits = v.to_bits();
    let src_digits = bits.digits();

    let digits = if N <= M {
        // Widening conversion: need to sign-extend
        if v.is_negative() {
            // For negative values, initialize with all 1s, then copy N digits
            let mut digits = *(UInt::<M>::MAX.digits());
            // SAFETY: N <= M is guaranteed by the outer if condition. Arrays don't overlap.
            ptr::copy_nonoverlapping(src_digits.as_ptr(), digits.as_mut_ptr(), N);
            digits
        } else {
            // For positive values, initialize with zeros (existing behavior)
            _transmute::<_, _, N>(src_digits)
        }
    } else {
        // Narrowing conversion
        debug_assert!(v.last_digit_index() < M);
        debug_assert!(v.bits() <= Int::<M>::BITS);
        _transmute::<_, _, M>(src_digits)
    };

    Int::from_digits(digits)
}
