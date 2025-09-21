use crate::{
    bint::{
        intrinsics::{
            Digit, ExpType, _decimal_digits_64, _div_rem_64, _downscale_64, min, DIGIT_POWERS_10,
            DIGIT_POWER_10,
        },
        UInt,
    },
    decimal::{
        dec::{
            construct::construct,
            math::{add::add, mul::mul, sub::sub, utils::correct},
            ExtraPrecision,
        },
        signals::Signals,
        Context, Decimal, Sign,
    },
};

type D<const N: usize> = Decimal<N>;
type U<const N: usize> = UInt<N>;

const EXTRA_PRECISION_DIGITS: ExpType = ExtraPrecision::EXTRA_PRECISION_DIGITS;

#[inline(never)]
pub(crate) const fn div<const N: usize>(mut dividend: D<N>, mut divisor: D<N>) -> D<N> {
    if dividend.is_nan() {
        return dividend.compound(&divisor).op_invalid();
    }

    if divisor.is_nan() {
        return divisor.compound(&dividend).op_invalid();
    }

    let sign = dividend.sign().div(divisor.sign());
    let signals = Signals::combine(dividend.cb.get_signals(), divisor.cb.get_signals());
    let ctx = Context::merge(dividend.cb.get_context(), divisor.cb.get_context());

    if dividend.is_infinite() && divisor.is_infinite() {
        return D::SIGNALING_NAN.raise_signals(signals).set_ctx(ctx);
    }

    if divisor.is_zero() {
        D::INFINITY
            .raise_signals(signals)
            .set_ctx(ctx)
            .set_sign(sign)
            .raise_signals(Signals::OP_DIV_BY_ZERO)
            .op_invalid()
    } else if dividend.is_zero() || divisor.is_one() {
        dividend.compound(&divisor).set_sign(sign)
    } else if dividend.is_infinite() {
        D::INFINITY
            .raise_signals(signals)
            .set_ctx(ctx)
            .set_sign(sign)
    } else if divisor.is_infinite() {
        D::ZERO.raise_signals(signals).set_ctx(ctx).set_sign(sign)
    } else {
        let correction = if dividend.cb.has_extra_precision() || divisor.cb.has_extra_precision() {
            Some(div_correction(&mut dividend, &mut divisor))
        } else {
            None
        };

        let exp = dividend.cb.get_exponent() - divisor.cb.get_exponent();

        let (digits, remainder) = dividend.digits.div_rem(divisor.digits);

        let result =
            DivGeneralCaseImpl::new(digits, remainder, divisor.digits, sign, signals, ctx, exp)
                .div();

        if let Some(correction) = correction {
            correct(result, correction)
        } else {
            result
        }
    }
}

struct DivGeneralCaseImpl<const N: usize> {
    digits: U<N>,
    remainder: U<N>,
    divisor: U<N>,

    sign: Sign,
    signals: Signals,
    ctx: Context,

    exp: i32,

    extra_precision: ExtraPrecision,
}

impl<const N: usize> DivGeneralCaseImpl<N> {
    #[inline(always)]
    const fn new(
        digits: U<N>,
        remainder: U<N>,
        divisor: U<N>,
        sign: Sign,
        signals: Signals,
        ctx: Context,
        exp: i32,
    ) -> Self {
        Self {
            digits,
            remainder,
            divisor,
            sign,
            signals,
            ctx,
            exp,
            extra_precision: ExtraPrecision::new(),
        }
    }

    #[inline(always)]
    const fn div(mut self) -> D<N> {
        if !self.remainder.is_zero() {
            self.fractional_part();
        }

        construct(
            self.digits,
            self.exp,
            self.sign,
            self.signals,
            self.ctx,
            self.extra_precision,
        )
    }

    #[inline(always)]
    const fn mul_add(&mut self, mut quotient: u64, mul_power: ExpType, add_power: ExpType) -> u64 {
        let multiplicator = DIGIT_POWERS_10[mul_power as usize];

        // SAFETY: `self.digits` can always be multiplied by 10^mul_power
        debug_assert!(self.digits.can_scaled_by_power_of_ten(mul_power));
        #[allow(unsafe_code)]
        {
            self.digits = unsafe { self.digits.unchecked_mul_digit(multiplicator) };
        }

        self.exp -= mul_power as i32;

        let qd = DIGIT_POWERS_10[add_power as usize];

        let mut extra;
        (quotient, extra) = _div_rem_64(quotient, qd);

        match self.digits.overflowing_add_digit(quotient) {
            (digits, false) => {
                self.digits = digits;
            }
            (_, true) => {
                self.digits = self.digits.div_digit(10);
                (quotient, extra) = _div_rem_64(quotient, 10);

                // SAFETY: `self.digits` is already scaled down by 10 and the quotient is less
                // than 10.
                #[allow(unsafe_code)]
                {
                    self.digits = unsafe { self.digits.unchecked_add_digit(quotient) };
                }

                self.exp += 1;
            }
        }

        extra
    }

    #[inline(always)]
    const fn fractional_part(&mut self) {
        let mut power = 0;
        let mut extra = 0;
        let mut carry = false;

        while !self.remainder.is_zero() || carry {
            let remaining_decimal_digits = self.digits.remaining_decimal_digits();

            // There is "tail" from the previous step
            if carry {
                debug_assert!(power != 0);
                if remaining_decimal_digits != 0 {
                    if remaining_decimal_digits <= power {
                        let p = power - remaining_decimal_digits;
                        extra = self.mul_add(extra, remaining_decimal_digits, p);
                        return self.extra_precision(extra, p);
                    } else
                    // if remaining_decimal_digits > power
                    {
                        extra = self.mul_add(extra, power, 0);
                        carry = false;
                        continue;
                    }
                }

                return self.extra_precision(extra, power);
            }

            // "Quotient power" - desired decimal power of quotient per iteration
            // It's equal to the rest digit count in result digits limited by max power of
            // ten which can fit in 64 bits
            let mut q_power = min(remaining_decimal_digits, DIGIT_POWER_10);

            // "Max power"
            // It's equal to the rest digit count in result digits plus extra precision
            // digits It also limited by max power of ten which can fit in 64
            // bits
            let max_power = min(q_power + EXTRA_PRECISION_DIGITS, DIGIT_POWER_10);

            let mut quotient;
            quotient = self.next_digit(max_power);

            if remaining_decimal_digits == 0 {
                return self.extra_precision(quotient, max_power);
            }

            if quotient == 0 {
                let p = min(max_power, q_power);

                if !self.digits.is_zero() {
                    let multiplicator = DIGIT_POWERS_10[p as usize];

                    // SAFETY: `self.digits` always can be multiplied by 10^p
                    debug_assert!(self.digits.can_scaled_by_power_of_ten(p));
                    #[allow(unsafe_code)]
                    {
                        self.digits = unsafe { self.digits.unchecked_mul_digit(multiplicator) };
                    }
                }

                self.exp -= p as i32;
                continue;
            }

            // Real decimal power of quotient.
            // If it's less than `max_power` then we need to shift the quotient
            power = _decimal_digits_64(quotient);

            if q_power == 0 {
                return self.extra_precision(quotient, power);
            }

            let leading_zeros = max_power - power;

            if self.remainder.is_zero() {
                let upscaled;
                (quotient, upscaled) = _downscale_64(quotient);
                power -= upscaled;
            }

            if power < q_power {
                q_power = power;
            }

            power = power - q_power + leading_zeros;

            extra = self.mul_add(quotient, q_power, power);

            if power != 0 {
                carry = true;
            }
        }
    }

    /// Performs next _(10^max_power * remainder) / divisor_ for a long decimal
    /// division.
    ///
    /// Returns quotient and remainder.
    ///
    /// We are sure that:
    ///  - _remainder < divisor_
    ///  - _quotient < 10^max_power_
    ///  - _10^max_power * remainder < 10^max_power *divisor_
    #[inline(always)]
    const fn next_digit(&mut self, power: ExpType) -> Digit {
        debug_assert!(self.remainder.lt(&self.divisor));
        debug_assert!(power > 0);

        let (quotient, r) = if self.remainder.can_scaled_by_power_of_ten(power) {
            div_rem_next_slim(self.remainder, self.divisor, power)
        } else {
            div_rem_next_wide(self.remainder, self.divisor, power)
        };

        self.remainder = r;

        quotient
    }

    #[inline(always)]
    const fn last_digit(&self, power: ExpType) -> Digit {
        debug_assert!(self.remainder.lt(&self.divisor));
        debug_assert!(power > 0);

        if self.remainder.can_scaled_by_power_of_ten(power) {
            div_next_slim(self.remainder, self.divisor, power)
        } else {
            div_next_wide(self.remainder, self.divisor, power)
        }
    }

    #[inline(always)]
    const fn extra_precision(&mut self, mut quotient: Digit, power: ExpType) {
        let mut ep = ExtraPrecision::from_digits(quotient, power);
        let rest_power = EXTRA_PRECISION_DIGITS.saturating_sub(power);

        if !self.remainder.is_zero() && rest_power != 0 {
            quotient = self.last_digit(rest_power);
            ep.append(quotient, rest_power);
        }

        if !ep.is_zero() || !self.remainder.is_zero() {
            self.signals.raise(Signals::OP_INEXACT);
            self.signals.raise(Signals::OP_ROUNDED);

            self.extra_precision = ep;
        }
    }
}

#[inline(always)]
const fn div_rem_next_slim<const N: usize>(
    mut remainder: U<N>,
    divisor: U<N>,
    power: ExpType,
) -> (Digit, U<N>) {
    debug_assert!(power > 0);

    let multiplicator = DIGIT_POWERS_10[power as usize];

    // SAFETY: `remainder` can always be multiplied by 10^mul_power
    debug_assert!(remainder.can_scaled_by_power_of_ten(power));
    #[allow(unsafe_code)]
    {
        remainder = unsafe { remainder.unchecked_mul_digit(multiplicator) };
    }

    let (q, r) = remainder.div_rem(divisor);
    (q.digits()[0], r)
}

#[inline(always)]
const fn div_rem_next_wide<const N: usize>(
    remainder: U<N>,
    divisor: U<N>,
    power: ExpType,
) -> (Digit, U<N>) {
    debug_assert!(power > 0);

    let multiplicator = UInt::unchecked_power_of_ten(power);
    let (q, r) = remainder.mul_div_rem(multiplicator, divisor);
    (q.digits()[0], r)
}

#[inline(always)]
const fn div_next_slim<const N: usize>(
    mut remainder: U<N>,
    divisor: U<N>,
    power: ExpType,
) -> Digit {
    debug_assert!(power > 0);

    let multiplicator = DIGIT_POWERS_10[power as usize];

    // SAFETY: `remainder` can always be multiplied by 10^mul_power
    debug_assert!(remainder.can_scaled_by_power_of_ten(power));
    #[allow(unsafe_code)]
    {
        remainder = unsafe { remainder.unchecked_mul_digit(multiplicator) };
    }

    let q = remainder.div(divisor);
    q.digits()[0]
}

#[inline(always)]
const fn div_next_wide<const N: usize>(remainder: U<N>, divisor: U<N>, power: ExpType) -> Digit {
    debug_assert!(power > 0);

    let multiplicator = UInt::unchecked_power_of_ten(power);
    let q = remainder.mul_div(multiplicator, divisor);
    q.digits()[0]
}

#[inline]
const fn div_correction<const N: usize>(dividend: &mut D<N>, divisor: &mut D<N>) -> D<N> {
    if !dividend.cb.has_extra_precision() && !divisor.cb.has_extra_precision() {
        return D::ZERO;
    }

    let xi_dividend = dividend.cb.take_extra_precision_decimal();
    let xi_divisor = divisor.cb.take_extra_precision_decimal();

    if xi_dividend.is_zero() && xi_divisor.is_zero() {
        D::ZERO
    } else if xi_divisor.is_zero() {
        let x = div(xi_dividend, *divisor);

        if x.is_op_underflow() {
            D::ZERO
        } else {
            x
        }
    } else {
        let x = if xi_dividend.is_zero() {
            mul(*dividend, xi_divisor).neg().without_extra_digits()
        } else {
            sub(mul(*divisor, xi_dividend), mul(*dividend, xi_divisor)).without_extra_digits()
        };

        if x.is_zero() || x.is_op_underflow() {
            D::ZERO
        } else {
            let y = add(mul(*divisor, *divisor), mul(*divisor, xi_divisor)).without_extra_digits();
            let z = div(x, y);

            if z.is_op_underflow() {
                D::ZERO
            } else {
                z
            }
        }
    }
}
