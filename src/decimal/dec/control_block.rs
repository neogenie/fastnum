mod context;
mod extra_precision;
mod flags;
mod signals;

use core::cmp::Ordering;

use crate::{
    decimal::{dec::ExtraPrecision, signals::Signals, Context, Sign},
    utils::assert_eq_size,
};

/// Control block (CB)
///
/// Memory layout:
///
/// |  Bit  |          Data         |         Bit Mask        |
/// |:-----:|:---------------------:|:-----------------------:|
/// | 1-15  |    Scale              | `0x0000_0000_0000_FFFF` |
/// | 16    |  Sign bit             | `0x0000_0000_0001_0000` |
/// | 17    |   Nan flag            | `0x0000_0000_0002_0000` |
/// | 18    |    Infinity flag      | `0x0000_0000_0004_0000` |
/// | 19    | OP_CLAMPED            | `0x0000_0000_0008_0000` |
/// | 20    | OP_DIV_BY_ZERO        | `0x0000_0000_0010_0000` |
/// | 21    | OP_INVALID            | `0x0000_0000_0020_0000` |
/// | 22    | OP_INEXACT            | `0x0000_0000_0040_0000` |
/// | 23    | OP_OVERFLOW           | `0x0000_0000_0080_0000` |
/// | 24    | OP_ROUNDED            | `0x0000_0000_0100_0000` |
/// | 25    | OP_SUBNORMAL          | `0x0000_0000_0200_0000` |
/// | 26    | OP_UNDERFLOW          | `0x0000_0000_0400_0000` |
/// | 27    | T OP_CLAMPED          | `0x0000_0000_0800_0000` |
/// | 28    | T OP_DIV_BY_ZERO      | `0x0000_0000_1000_0000` |
/// | 29    | T OP_INVALID          | `0x0000_0000_2000_0000` |
/// | 30    | T OP_INEXACT          | `0x0000_0000_4000_0000` |
/// | 31    | T OP_OVERFLOW         | `0x0000_0000_8000_0000` |
/// | 32    | T OP_ROUNDED          | `0x0000_0001_0000_0000` |
/// | 33    | T OP_SUBNORMAL        | `0x0000_0002_0000_0000` |
/// | 34    | T OP_UNDERFLOW        | `0x0000_0004_0000_0000` |
/// | 35    |      Reserved         | `0x0000_0008_0000_0000` |
/// | 36    |      Reserved         | `0x0000_0010_0000_0000` |
/// | 37-39 | Rounding mode (3 bit) | `0x0000_00E0_0000_0000` |
/// | 40-63 | Extra digits (24 bit) | `0xFFFF_FF00_0000_0000` |
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[repr(transparent)]
pub(crate) struct ControlBlock(u64);

impl ControlBlock {
    const MIN_SCALE: u64 = 0x0000_0000_0000_8000;
    // const MAX_SCALE: u64 = 0x0000_0000_0000_7FFF;

    const SCALE_MASK: u64 = 0x0000_0000_0000_FFFF;

    #[allow(dead_code)]
    #[inline(always)]
    pub const fn default() -> Self {
        Self::new(
            0,
            Sign::Plus,
            Signals::empty(),
            Context::default(),
            ExtraPrecision::new(),
        )
    }

    #[inline(always)]
    pub const fn basic(scale: i16, sign: Sign) -> Self {
        Self::new(
            scale,
            sign,
            Signals::empty(),
            Context::default(),
            ExtraPrecision::new(),
        )
    }

    #[inline(always)]
    pub const fn new(
        scale: i16,
        sign: Sign,
        signals: Signals,
        ctx: Context,
        extra_precision: ExtraPrecision,
    ) -> Self {
        let mut this = Self((scale as u64) & Self::SCALE_MASK);

        // TODO: one operation
        this.set_sign(sign);
        this.raise_signals(signals);
        this.set_context(ctx);
        this.set_extra_precision(extra_precision);

        this
    }

    #[inline(always)]
    pub const fn get_scale(&self) -> i16 {
        (self.0 & Self::SCALE_MASK) as i16
    }

    #[inline(always)]
    pub const fn set_scale(&mut self, scale: i16) {
        self.0 = (self.0 & !Self::SCALE_MASK) | (scale as u64) & Self::SCALE_MASK;
    }

    #[inline(always)]
    pub const fn inc_scale(&mut self, inc: i16) {
        let scale = self.get_scale() + inc;
        self.set_scale(scale);
    }

    #[inline(always)]
    pub const fn dec_scale(&mut self, dec: i16) {
        let scale = self.get_scale() - dec;
        self.set_scale(scale);
    }

    #[inline(always)]
    pub(crate) const fn scale_cmp(&self, other: &Self) -> Ordering {
        let self_scale = self.get_scale();
        let other_scale = other.get_scale();

        // TODO: 3-way comparison
        if self_scale == other_scale {
            Ordering::Equal
        } else if self_scale > other_scale {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }

    #[inline(always)]
    pub const fn get_exponent(&self) -> i32 {
        (self.get_scale() as i32).overflowing_neg().0
    }

    #[inline(always)]
    pub const fn compound(&mut self, other: &Self) {
        self.combine_signals(other);
        self.combine_ctx(other);
    }
}

// {
//     /// Sign and special values flags.
//     flags: Flags,
//
//     /// The exceptional conditions are grouped into signals, which can be
//     /// controlled individually. The context contains a flag (which is either
// 0     /// or 1) and a trap-enabler (which also is either 0 or 1) for each
// signal.     signals: Signal,
//
//     /// Context for decimal operations
//     ctx: Context,
// }

// impl ControlBlock {
//     const DEFAULT: Self = Self {
//         flags: Flags::default(),
//         signals: Signal::empty(),
//         ctx: Context::default(),
//     };
//
//     const NAN: Self = Self {
//         flags: Flags::nan(),
//         signals: Signal::empty(),
//         ctx: Context::default(),
//     };
//
//     const INFINITY: Self = Self {
//         flags: Flags::infinity(),
//         signals: Signal::empty(),
//         ctx: Context::default(),
//     };
//
//     const NEG_INFINITY: Self = Self {
//         flags: Flags::neg_infinity(),
//         signals: Signal::empty(),
//         ctx: Context::default(),
//     };
//
//     /// Return the default CB instance (no sign, no special flags, no
// signaling     /// flags, default context).
//     #[inline(always)]
//     pub(crate) const fn default() -> Self {
//         Self::DEFAULT
//     }
//
//     /// Return the default CB instance for `NaN` (no signaling flags, default
//     /// context).
//     #[inline(always)]
//     pub(crate) const fn nan() -> Self {
//         Self::NAN
//     }
//
//     /// Return the default CB instance for `Inf` (no signaling flags, default
//     /// context).
//     #[inline(always)]
//     pub(crate) const fn infinity() -> Self {
//         Self::INFINITY
//     }
//
//     /// Return the default CB instance for `-Inf` (no signaling flags,
// default     /// context).
//     #[inline(always)]
//     pub(crate) const fn neg_infinity() -> Self {
//         Self::NEG_INFINITY
//     }
//
//     // ------------------
//
//     #[inline(always)]
//     pub(crate) const fn flags(&self) -> Flags {
//         self.flags
//     }
//
//     #[inline(always)]
//     pub(crate) const fn signals(&self) -> Signal {
//         self.signals
//     }
//
//     #[inline(always)]
//     pub(crate) const fn context(&self) -> Context {
//         self.ctx
//     }
//
//
//     #[inline(always)]
//     pub(crate) const fn sign(&self) -> Sign {
//         self.flags.sign()
//     }
//

//     #[inline]
//     pub(crate) const fn is_op_div_by_zero(&self) -> bool {
//         self.signals.is_raised(Signal::OP_DIV_BY_ZERO)
//     }
//
//     #[inline]
//     pub(crate) const fn is_op_overflow(&self) -> bool {
//         self.signals.is_raised(Signal::OP_OVERFLOW)
//     }
//
//     #[inline]
//     pub(crate) const fn is_op_underflow(&self) -> bool {
//         self.signals.is_raised(Signal::OP_UNDERFLOW)
//     }
//
//     #[inline]
//     pub(crate) const fn is_op_invalid(&self) -> bool {
//         self.signals.is_raised(Signal::OP_INVALID)
//     }
//
//     #[inline]
//     pub(crate) const fn is_op_subnormal(&self) -> bool {
//         self.signals.is_raised(Signal::OP_SUBNORMAL)
//     }
//
//     #[inline]
//     pub(crate) const fn is_op_inexact(&self) -> bool {
//         self.signals.is_raised(Signal::OP_INEXACT)
//     }
//
//     #[inline]
//     pub(crate) const fn is_op_rounded(&self) -> bool {
//         self.signals.is_raised(Signal::OP_ROUNDED)
//     }
//
//     #[inline]
//     pub(crate) const fn is_op_clamped(&self) -> bool {
//         self.signals.is_raised(Signal::OP_CLAMPED)
//     }
//
//     #[inline]
//     pub(crate) const fn is_op_ok(&self) -> bool {
//         self.signals.is_empty()
//     }
//
//     // ------------------
//
//     #[inline(always)]
//     pub(crate) const fn neg(mut self) -> Self {
//         self.flags = self.flags.neg();
//         self
//     }
//
//     #[inline(always)]
//     pub(crate) const fn abs(mut self) -> Self {
//         self.flags = self.flags.abs();
//         self
//     }
//
//     #[inline(always)]
//     pub(crate) const fn set_context(mut self, ctx: Context) -> Self {
//         self.ctx = ctx;
//         self
//     }
//
//     #[inline(always)]
//     pub(crate) const fn set_rounding_mode(mut self, rm: RoundingMode) -> Self
// {         self.ctx = self.ctx.with_rounding_mode(rm);
//         self
//     }
//
//     #[inline(always)]
//     pub(crate) const fn set_flags(mut self, flags: Flags) -> Self {
//         self.flags = flags;
//         self
//     }
//
//     #[inline(always)]
//     pub(crate) const fn with_flags(mut self, flags: Flags) -> Self {
//         self.flags = self.flags.combine(flags);
//         self
//     }
//
//     #[inline]
//     pub(crate) const fn compound(mut self, other: Self) -> Self {
//         self.signals = self.signals.combine(other.signals);
//         self.ctx = self.ctx.merge(other.ctx);
//         self
//     }
//
//     #[inline]
//     pub(crate) const fn compound_and_raise(mut self, other: Self, signal:
// Signal) -> Self {         self.signals =
// self.signals.combine(other.signals).combine(signal);         self.ctx =
// self.ctx.merge(other.ctx);         self
//     }
//
//     #[inline]
//     pub(crate) const fn combine_and_set_ctx(mut self, other: Self) -> Self {
//         self.signals = self.signals.combine(other.signals);
//         self.ctx = other.ctx;
//         self.flags = self.flags.combine(other.flags);
//         self
//     }
//

//     #[inline]
//     pub(crate) const fn combine(mut self, other: Self) -> Self {
//         self.signals = self.signals.combine(other.signals);
//         self.ctx = self.ctx.merge(other.ctx);
//         self.flags = self.flags.combine(other.flags);
//         self
//     }
//
//     #[inline]
//     pub(crate) const fn combine_mul(mut self, other: Self) -> Self {
//         self.signals = self.signals.combine(other.signals);
//         self.ctx = self.ctx.merge(other.ctx);
//         self.flags = self.flags.mul(other.flags);
//         self
//     }
//
//     #[inline(always)]
//     pub(crate) const fn raise_signal(mut self, signal: Signal) -> Self {
//         self.signals = self.signals.combine(signal);
//         self
//     }
//
//     #[inline(always)]
//     pub(crate) const fn quiet_signal(mut self, signal: Signal) -> Self {
//         self.signals = self.signals.unset(signal);
//         self
//     }
//
//     #[inline(always)]
//     pub(crate) const fn trap_signals(&self) -> Signal {
//         self.ctx.trap_signals(self.signals)
//     }
// }

assert_eq_size!(ControlBlock, u64);
