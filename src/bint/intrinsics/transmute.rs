use crate::bint::intrinsics::Digits;
use core::ptr;

#[allow(unsafe_code)]
#[inline(always)]
pub const unsafe fn _transmute<const N: usize, const M: usize, const V: usize>(
    digits: &Digits<N>,
) -> Digits<M> {
    let mut out = [0; M];
    // SAFETY: V <= min(N, M) is guaranteed by caller. Source and destination don't overlap.
    ptr::copy_nonoverlapping(digits.as_ptr(), out.as_mut_ptr(), V);
    out
}
