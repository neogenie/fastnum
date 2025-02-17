use crate::decimal::{dec::ControlBlock, signals::Signals};

/// Control block (CB)
///
/// Signals memory layout:
///
/// |  Bit  |          Data         |         Bit Mask        |
/// |:-----:|:---------------------:|:-----------------------:|
/// | `...` |      `...`            |          `...`          |
/// | 19    | OP_CLAMPED            | `0x0000_0000_0008_0000` |
/// | 20    | OP_DIV_BY_ZERO        | `0x0000_0000_0010_0000` |
/// | 21    | OP_INVALID            | `0x0000_0000_0020_0000` |
/// | 22    | OP_INEXACT            | `0x0000_0000_0040_0000` |
/// | 23    | OP_OVERFLOW           | `0x0000_0000_0080_0000` |
/// | 24    | OP_ROUNDED            | `0x0000_0000_0100_0000` |
/// | 25    | OP_SUBNORMAL          | `0x0000_0000_0200_0000` |
/// | 26    | OP_UNDERFLOW          | `0x0000_0000_0400_0000` |
/// | `...` |      `...`            |          `...`          |
impl ControlBlock {
    pub(super) const SIGNALS_MASK: u64 = 0x0000_0000_07F8_0000;
    const SIGNALS_SHIFT: u8 = 19;

    pub(super) const OP_INVALID_MASK: u64 = make_signal_mask(Signals::OP_INVALID_MASK);

    #[inline(always)]
    pub const fn get_signals(&self) -> Signals {
        Signals::new(((self.0 & Self::SIGNALS_MASK) >> Self::SIGNALS_SHIFT) as u8)
    }

    #[inline(always)]
    pub const fn raise_signals(&mut self, signals: Signals) {
        self.0 |= ((signals.mask() as u64) << ControlBlock::SIGNALS_SHIFT) & Self::SIGNALS_MASK;
    }

    #[inline(always)]
    pub const fn quiet_signals(&mut self, signals: Signals) {
        self.0 &= !(((signals.mask() as u64) << ControlBlock::SIGNALS_SHIFT) & Self::SIGNALS_MASK);
    }

    #[inline(always)]
    pub const fn is_signals_raised(&self, signals: Signals) -> bool {
        (((self.0 & Self::SIGNALS_MASK) >> Self::SIGNALS_SHIFT) as u8) & signals.mask() != 0
    }

    #[inline(always)]
    pub const fn trap_signals(&self) -> Signals {
        let raised = self.get_signals();
        let signal_traps = self.get_signal_traps();
        signal_traps.trap(raised)
    }

    #[inline(always)]
    pub const fn is_op_ok(&self) -> bool {
        self.0 & Self::SIGNALS_MASK == 0
    }

    #[inline(always)]
    pub(super) const fn combine_signals(&mut self, other: &Self) {
        self.0 |= other.0 & Self::SIGNALS_MASK;
    }
}

const fn make_signal_mask(signal_mask: u8) -> u64 {
    (signal_mask as u64) << ControlBlock::SIGNALS_SHIFT
}
