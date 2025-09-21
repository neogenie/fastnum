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
    Decimal,
};

type D<const N: usize> = Decimal<N>;

impl<const N: usize> Type<Postgres> for D<N> {
    fn type_info() -> PgTypeInfo {
        NUMERIC
    }
}

impl<const N: usize> PgHasArrayType for D<N> {
    fn array_type_info() -> PgTypeInfo {
        NUMERIC_ARRAY
    }
}

impl<const N: usize> Encode<'_, Postgres> for D<N> {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let nbase: NBase = (*self)
            .try_into()
            .map_err(|e| pretty_error_msg(D::<N>::type_name(), e))?;
        nbase.encode(buf.deref_mut())?;

        Ok(IsNull::No)
    }
}

impl<const N: usize> Decode<'_, Postgres> for D<N> {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => Ok(NBase::decode(value.as_bytes()?)?
                .try_into()
                .map_err(|e| pretty_error_msg(D::<N>::type_name(), e))?),
            PgValueFormat::Text => Ok(value.as_str()?.parse::<Decimal<N>>()?),
        }
    }
}
