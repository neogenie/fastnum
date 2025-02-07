use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use core::error::Error;

use crate::{decimal::extras::utils::db::postgres::NBase, utils::err_msg};

impl NBase {
    // https://github.com/postgres/postgres/blob/bcd1c3630095e48bc3b1eb0fc8e8c8a7c851eba1/src/backend/utils/adt/numeric.c#L167-L170
    const SIGN_POS: u16 = 0x0000;
    const SIGN_NEG: u16 = 0x4000;
    const SIGN_NAN: u16 = 0xC000;

    pub(crate) fn decode<T: ReadBytesExt>(
        mut buf: T,
    ) -> Result<Self, Box<dyn Error + Sync + Send>> {
        let digit_count = buf.read_u16::<NetworkEndian>()?;
        let weight = buf.read_i16::<NetworkEndian>()?;
        let sign = buf.read_u16::<NetworkEndian>()?;
        let scale = buf.read_u16::<NetworkEndian>()?;

        if sign == Self::SIGN_NAN {
            Ok(NBase::NaN)
        } else {
            let digits = (0..digit_count)
                .map(|_| buf.read_i16::<NetworkEndian>())
                .collect::<Result<Vec<_>, _>>()?;

            match sign {
                Self::SIGN_POS => Ok(NBase::Positive {
                    weight,
                    scale,
                    digits,
                }),
                Self::SIGN_NEG => Ok(NBase::Negative {
                    weight,
                    scale,
                    digits,
                }),
                _ => {
                    Err(err_msg!("sign for numeric field was not one of 0, 0x4000, 0xC000").into())
                }
            }
        }
    }

    pub(crate) fn encode<T: WriteBytesExt>(
        self,
        buf: &mut T,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        let sign = match self {
            Self::Positive { .. } => Self::SIGN_POS,
            Self::Negative { .. } => Self::SIGN_NEG,
            Self::NaN => Self::SIGN_NAN,
        };

        let empty_vec = Vec::new();
        let digits = match self {
            NBase::Positive { ref digits, .. } | NBase::Negative { ref digits, .. } => digits,
            NBase::NaN => &empty_vec,
        };

        let weight = match self {
            NBase::Positive { weight, .. } | NBase::Negative { weight, .. } => weight,
            NBase::NaN => 0,
        };

        let scale = match self {
            NBase::Positive { scale, .. } | NBase::Negative { scale, .. } => scale,
            NBase::NaN => 0,
        };

        let digits_len = u16::try_from(digits.len())
            .map_err(|_| err_msg!("numeric digits count should not overflow u16"))?;

        buf.write_u16::<NetworkEndian>(digits_len)?;
        buf.write_i16::<NetworkEndian>(weight)?;
        buf.write_u16::<NetworkEndian>(sign)?;
        buf.write_u16::<NetworkEndian>(scale)?;

        for digit in digits.iter() {
            buf.write_i16::<NetworkEndian>(*digit)?;
        }

        Ok(())
    }
}
