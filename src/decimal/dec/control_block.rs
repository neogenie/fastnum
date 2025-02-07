use crate::{
    decimal::{dec::Flags, Context, RoundingMode, Sign, Signal},
    utils::assert_eq_size,
};

/// Control block (CB)
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) struct ControlBlock {
    /// Sign and special values flags.
    flags: Flags,

    /// The exceptional conditions are grouped into signals, which can be
    /// controlled individually. The context contains a flag (which is either 0
    /// or 1) and a trap-enabler (which also is either 0 or 1) for each signal.
    signals: Signal,

    /// Context for decimal operations
    ctx: Context,
}

impl ControlBlock {
    const DEFAULT: Self = Self {
        flags: Flags::default(),
        signals: Signal::empty(),
        ctx: Context::default(),
    };

    const NAN: Self = Self {
        flags: Flags::nan(),
        signals: Signal::empty(),
        ctx: Context::default(),
    };

    const INFINITY: Self = Self {
        flags: Flags::infinity(),
        signals: Signal::empty(),
        ctx: Context::default(),
    };

    const NEG_INFINITY: Self = Self {
        flags: Flags::neg_infinity(),
        signals: Signal::empty(),
        ctx: Context::default(),
    };

    /// Return the default CB instance (no sign, no special flags, no signaling
    /// flags, default context).
    #[inline(always)]
    pub(crate) const fn default() -> Self {
        Self::DEFAULT
    }

    /// Return the default CB instance for `NaN` (no signaling flags, default
    /// context).
    #[inline(always)]
    pub(crate) const fn nan() -> Self {
        Self::NAN
    }

    /// Return the default CB instance for `Inf` (no signaling flags, default
    /// context).
    #[inline(always)]
    pub(crate) const fn infinity() -> Self {
        Self::INFINITY
    }

    /// Return the default CB instance for `-Inf` (no signaling flags, default
    /// context).
    #[inline(always)]
    pub(crate) const fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }

    // ------------------

    #[inline(always)]
    pub(crate) const fn flags(&self) -> Flags {
        self.flags
    }

    #[inline(always)]
    pub(crate) const fn signals(&self) -> Signal {
        self.signals
    }

    #[inline(always)]
    pub(crate) const fn context(&self) -> Context {
        self.ctx
    }

    #[inline(always)]
    pub(crate) const fn is_negative(&self) -> bool {
        self.flags.is_negative()
    }

    #[inline(always)]
    pub(crate) const fn sign(&self) -> Sign {
        self.flags.sign()
    }

    #[inline(always)]
    pub(crate) const fn is_nan(&self) -> bool {
        self.flags.is_nan()
    }

    #[inline(always)]
    pub(crate) const fn is_infinity(&self) -> bool {
        self.flags.is_infinity()
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) const fn is_neg_infinity(&self) -> bool {
        self.flags.is_neg_infinity()
    }

    #[inline(always)]
    pub(crate) const fn is_special(&self) -> bool {
        self.flags.is_special()
    }

    #[inline]
    pub(crate) const fn is_op_div_by_zero(&self) -> bool {
        self.signals.is_raised(Signal::OP_DIV_BY_ZERO)
    }

    #[inline]
    pub(crate) const fn is_op_overflow(&self) -> bool {
        self.signals.is_raised(Signal::OP_OVERFLOW)
    }

    #[inline]
    pub(crate) const fn is_op_underflow(&self) -> bool {
        self.signals.is_raised(Signal::OP_UNDERFLOW)
    }

    #[inline]
    pub(crate) const fn is_op_invalid(&self) -> bool {
        self.signals.is_raised(Signal::OP_INVALID)
    }

    #[inline]
    pub(crate) const fn is_op_subnormal(&self) -> bool {
        self.signals.is_raised(Signal::OP_SUBNORMAL)
    }

    #[inline]
    pub(crate) const fn is_op_inexact(&self) -> bool {
        self.signals.is_raised(Signal::OP_INEXACT)
    }

    #[inline]
    pub(crate) const fn is_op_rounded(&self) -> bool {
        self.signals.is_raised(Signal::OP_ROUNDED)
    }

    #[inline]
    pub(crate) const fn is_op_clamped(&self) -> bool {
        self.signals.is_raised(Signal::OP_CLAMPED)
    }

    #[inline]
    pub(crate) const fn is_op_ok(&self) -> bool {
        self.signals.is_empty()
    }

    // ------------------

    #[inline(always)]
    pub(crate) const fn neg(mut self) -> Self {
        self.flags = self.flags.neg();
        self
    }

    #[inline(always)]
    pub(crate) const fn abs(mut self) -> Self {
        self.flags = self.flags.abs();
        self
    }

    #[inline(always)]
    pub(crate) const fn set_context(mut self, ctx: Context) -> Self {
        self.ctx = ctx;
        self
    }

    #[inline(always)]
    pub(crate) const fn set_rounding_mode(mut self, rm: RoundingMode) -> Self {
        self.ctx = self.ctx.with_rounding_mode(rm);
        self
    }

    #[inline(always)]
    pub(crate) const fn set_flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    #[inline(always)]
    pub(crate) const fn with_flags(mut self, flags: Flags) -> Self {
        self.flags = self.flags.combine(flags);
        self
    }

    #[inline]
    pub(crate) const fn compound(mut self, other: Self) -> Self {
        self.signals = self.signals.combine(other.signals);
        self.ctx = self.ctx.merge(other.ctx);
        self
    }

    #[inline]
    pub(crate) const fn compound_and_raise(mut self, other: Self, signal: Signal) -> Self {
        self.signals = self.signals.combine(other.signals).combine(signal);
        self.ctx = self.ctx.merge(other.ctx);
        self
    }

    #[inline]
    pub(crate) const fn combine_and_set_ctx(mut self, other: Self) -> Self {
        self.signals = self.signals.combine(other.signals);
        self.ctx = other.ctx;
        self.flags = self.flags.combine(other.flags);
        self
    }

    #[inline]
    pub(crate) const fn signaling_nan(mut self) -> Self {
        self.signals = self.signals.set(Signal::OP_INVALID);
        self.flags = Flags::nan();
        self
    }

    #[inline]
    pub(crate) const fn combine(mut self, other: Self) -> Self {
        self.signals = self.signals.combine(other.signals);
        self.ctx = self.ctx.merge(other.ctx);
        self.flags = self.flags.combine(other.flags);
        self
    }

    #[inline]
    pub(crate) const fn combine_mul(mut self, other: Self) -> Self {
        self.signals = self.signals.combine(other.signals);
        self.ctx = self.ctx.merge(other.ctx);
        self.flags = self.flags.mul(other.flags);
        self
    }

    #[inline(always)]
    pub(crate) const fn raise_signal(mut self, signal: Signal) -> Self {
        self.signals = self.signals.combine(signal);
        self
    }

    #[inline(always)]
    pub(crate) const fn quiet_signal(mut self, signal: Signal) -> Self {
        self.signals = self.signals.unset(signal);
        self
    }

    #[inline(always)]
    pub(crate) const fn trap_signals(&self) -> Signal {
        self.ctx.trap_signals(self.signals)
    }
}

assert_eq_size!(ControlBlock, u32);
