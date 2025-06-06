use bytes::{BufMut, BytesMut};
use core::error::Error;
use tokio_postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};

use crate::decimal::{
    errors::parse::pretty_error_msg, extras::utils::db::postgres::NBase, Decimal,
};

type D<const N: usize> = Decimal<N>;

impl<'a, const N: usize> FromSql<'a> for D<N> {
    fn from_sql(_: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(NBase::decode(raw)?
            .try_into()
            .map_err(|e| pretty_error_msg(D::<N>::type_name().as_str(), e))?)
    }

    accepts!(NUMERIC);
}

impl<const N: usize> ToSql for D<N> {
    fn to_sql(&self, _: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        let nbase: NBase = (*self)
            .try_into()
            .map_err(|e| pretty_error_msg(D::<N>::type_name().as_str(), e))?;
        nbase.encode(&mut out.writer())?;

        Ok(IsNull::No)
    }

    accepts!(NUMERIC);
    to_sql_checked!();
}
