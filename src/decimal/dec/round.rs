use crate::{
    decimal::{dec::ExtraPrecision, Decimal, RoundingMode::*, Signal},
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn round<const N: usize>(mut d: D<N>) -> D<N> {
    if d.extra_precision.has_digits() {
        let digit = d.extra_precision.round_reminder();
        d.extra_precision = ExtraPrecision::new();
        d.cb =
            d.cb.raise_signal(Signal::OP_INEXACT.combine(Signal::OP_ROUNDED));

        if match d.cb.context().rounding_mode() {
            Up => true,
            Down => false,
            Ceiling => !d.cb.is_negative(),
            Floor => d.cb.is_negative(),
            HalfUp => digit >= 5,
            HalfDown => digit > 5,
            HalfEven => {
                if digit > 5 {
                    true
                } else if digit == 5 {
                    let last_digit = d.digits.digits()[0];
                    let last_bit = last_digit & 0x0000_0000_0000_0001_u64;
                    last_bit != 0
                } else {
                    false
                }
            }
        } {
            if d.digits.eq(&UInt::MAX) {
                d.digits = d.digits.strict_div(UInt::TEN);
                d.scale -= 1;
            }

            d.digits = d.digits.strict_add(UInt::ONE);
        }
    }

    d
}
