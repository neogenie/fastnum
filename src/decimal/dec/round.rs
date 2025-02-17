use crate::{
    decimal::{
        dec::math::{add::add, sub::sub},
        Decimal,
        RoundingMode::*,
    },
    int::UInt,
};

type D<const N: usize> = Decimal<N>;

#[inline]
pub(crate) const fn round<const N: usize>(d: &mut D<N>) {
    if !matches!(d.cb.get_rounding_mode(), No) {
        let digit = d.cb.take_round_reminder();

        if digit != 0
            && match d.cb.get_rounding_mode() {
                No => unreachable!(),
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
            }
        {
            if d.digits.eq(&UInt::MAX) {
                d.digits = d.digits.strict_div(UInt::TEN);
                d.cb.dec_scale(1);
            }

            d.digits = d.digits.strict_add(UInt::ONE);
        }
    }
}

#[inline]
pub(crate) const fn floor<const N: usize>(d: D<N>) -> D<N> {
    if d.cb.is_special() {
        return d.signaling_nan();
    }

    if d.is_integral() {
        d
    } else {
        d.with_rounding_mode(Down).round(0)
    }
}

#[inline]
pub(crate) const fn ceil<const N: usize>(d: D<N>) -> D<N> {
    if d.cb.is_special() {
        return d.signaling_nan();
    }

    if d.is_integral() {
        d
    } else {
        let rounded = d.with_rounding_mode(Down).round(0);
        if d.is_negative() {
            sub(rounded, D::ONE).round_extra_precision().check()
        } else {
            add(rounded, D::ONE).round_extra_precision().check()
        }
    }
}
