use crate::decimal::{dec::ControlBlock, Sign};

/// Control block (CB)
///
/// Flags Memory layout:
///
/// |  Bit  |          Data         |         Bit Mask        |
/// |:-----:|:---------------------:|:-----------------------:|
/// | `...` |      `...`            |          `...`          |
/// | 16    |  Sign bit             | `0x0000_0000_0001_0000` |
/// | 17    |   Nan flag            | `0x0000_0000_0002_0000` |
/// | 18    |    Infinity flag      | `0x0000_0000_0004_0000` |
/// | `...` |      `...`            |         `...`           |
impl ControlBlock {
    // const FLAGS_MASK: u64 = 0x0000_0000_0007_0000;

    const SIGN_MASK: u64 = 0x0000_0000_0001_0000;
    const NAN_MASK: u64 = 0x0000_0000_0002_0000;
    const INFINITY_MASK: u64 = 0x0000_0000_0004_0000;

    const IS_SPECIAL_MASK: u64 = 0x_0000_0000_0006_0000;
    const NEG_INFINITY_MASK: u64 = 0x0000_0000_0005_0000;
    const SIGNALING_NAN_MASK: u64 = Self::NAN_MASK | Self::OP_INVALID_MASK;

    pub const NAN: Self = Self(Self::NAN_MASK | Self::DEFAULT_CONTEXT);
    pub const SIGNALING_NAN: Self = Self(Self::SIGNALING_NAN_MASK | Self::DEFAULT_CONTEXT);
    pub const INFINITY: Self = Self(Self::MIN_SCALE | Self::INFINITY_MASK | Self::DEFAULT_CONTEXT);
    pub const NEG_INFINITY: Self =
        Self(Self::MIN_SCALE | Self::NEG_INFINITY_MASK | Self::DEFAULT_CONTEXT);

    #[inline(always)]
    pub const fn is_negative(&self) -> bool {
        (self.0 & Self::SIGN_MASK) != 0
    }

    #[inline(always)]
    pub const fn get_sign(&self) -> Sign {
        if self.is_negative() {
            Sign::Minus
        } else {
            Sign::Plus
        }
    }

    #[inline(always)]
    pub const fn set_sign(&mut self, sign: Sign) {
        match sign {
            Sign::Plus => {
                self.0 &= !Self::SIGN_MASK;
            }
            Sign::Minus => {
                self.0 |= Self::SIGN_MASK;
            }
        }
    }

    #[inline(always)]
    pub const fn is_nan(&self) -> bool {
        self.0 & Self::NAN_MASK != 0
    }

    #[inline(always)]
    pub const fn is_infinity(&self) -> bool {
        self.0 & Self::INFINITY_MASK != 0
    }

    #[allow(dead_code)]
    #[inline(always)]
    pub const fn is_neg_infinity(&self) -> bool {
        self.0 & Self::NEG_INFINITY_MASK != 0
    }

    #[inline(always)]
    pub const fn is_special(&self) -> bool {
        self.0 & Self::IS_SPECIAL_MASK != 0
    }

    #[inline(always)]
    pub const fn signaling_nan(&mut self) {
        self.0 |= Self::SIGNALING_NAN_MASK;
    }

    #[inline(always)]
    pub const fn abs(&mut self) {
        self.0 &= !Self::SIGN_MASK;
    }

    #[inline(always)]
    pub const fn neg(&mut self) {
        self.0 ^= Self::SIGN_MASK;
    }
}
