use core::ops::DerefMut;

use sqlx::{
    decode::Decode,
    encode::{Encode, IsNull},
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueFormat, PgValueRef, Postgres},
    types::Type,
};

use crate::decimal::{
    errors::parse::pretty_error_msg,
    extras::{
        sqlx::pg::{NUMERIC, NUMERIC_ARRAY},
        utils::db::postgres::NBase,
    },
    Decimal, ParseError, UnsignedDecimal,
};

type D<const N: usize> = Decimal<N>;
type UD<const N: usize> = UnsignedDecimal<N>;

impl<const N: usize> Type<Postgres> for UD<N> {
    fn type_info() -> PgTypeInfo {
        NUMERIC
    }
}

impl<const N: usize> PgHasArrayType for UD<N> {
    fn array_type_info() -> PgTypeInfo {
        NUMERIC_ARRAY
    }
}

impl<const N: usize> Encode<'_, Postgres> for UD<N> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let nbase: NBase = self
            .0
            .try_into()
            .map_err(|e| pretty_error_msg(UD::<N>::type_name().as_str(), e))?;
        nbase.encode(buf.deref_mut())?;

        Ok(IsNull::No)
    }
}

impl<const N: usize> Decode<'_, Postgres> for UD<N> {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => {
                let dec: D<N> = NBase::decode(value.as_bytes()?)?
                    .try_into()
                    .map_err(|e| pretty_error_msg(UD::<N>::type_name().as_str(), e))?;

                if dec.is_negative() {
                    return Err(
                        pretty_error_msg(Self::type_name().as_str(), ParseError::Signed).into(),
                    );
                }

                Ok(UD::new(dec))
            }
            PgValueFormat::Text => Ok(value.as_str()?.parse::<UD<N>>()?),
        }
    }
}
