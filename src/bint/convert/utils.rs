macro_rules! digits_from_int_impl {
    ($n: expr, $int: ty, $bits: expr $(, $wrap: ident)?) => {{

        let mut digits = $crate::bint::convert::utils::digits_from_int_impl!(@ $n, $($wrap)?);

        let mut i = 0;
        while i << DIGIT_BIT_SHIFT < $bits {
            let d = ($n >> (i << DIGIT_BIT_SHIFT)) as Digit;
            digits[i as usize] = d;
            i += 1;
        }

        digits
    }};

    (@ $n: expr,) => {{
        [0; N]
    }};
    (@ $n: expr, WRAP) => {{
        if $n.is_negative() {
            [Digit::MAX; N]
        } else {
            [0; N]
        }
    }};
}

pub(crate) use digits_from_int_impl;
