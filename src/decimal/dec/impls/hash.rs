use core::hash::{Hash, Hasher};

use crate::decimal::{Context, Decimal, Sign};

impl<const N: usize> Hash for Decimal<N> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.is_nan() {
            hash_impl(Self::NAN, state);
        } else if self.is_infinite() {
            match self.sign() {
                Sign::Plus => {
                    hash_impl(Self::INFINITY, state);
                }
                Sign::Minus => {
                    hash_impl(Self::NEG_INFINITY, state);
                }
            }
        } else if self.is_zero() {
            hash_impl(Self::ZERO, state);
        } else {
            let mut d = self.reduce().with_ctx(Context::default());
            d.cb.quiet_signals(d.signals());
            hash_impl(d, state);
        }
    }
}

#[inline(always)]
fn hash_impl<const N: usize, H: Hasher>(d: Decimal<N>, state: &mut H) {
    d.digits.hash(state);
    d.cb.hash(state);
}
