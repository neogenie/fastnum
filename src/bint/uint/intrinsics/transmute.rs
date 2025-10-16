use crate::bint::{intrinsics::_transmute, UInt};

#[allow(unsafe_code)]
#[inline(always)]
pub const unsafe fn transmute<const N: usize, const M: usize>(v: UInt<N>) -> UInt<M> {
    if N <= M {
        UInt::from_digits(_transmute::<N, M, N>(v.digits()))
    } else {
        debug_assert!(v.last_digit_index() < M);
        debug_assert!(v.bits() <= UInt::<M>::BITS);
        UInt::from_digits(_transmute::<N, M, M>(v.digits()))
    }
}
