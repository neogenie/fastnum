use core::fmt::{Debug, Write};

use diesel::{
    deserialize::{self, FromSql},
    mysql::{Mysql, MysqlType, MysqlValue},
    serialize::{self, IsNull, Output, ToSql},
    sql_types::Numeric,
};

use crate::decimal::{Context, Decimal};

impl<const N: usize> ToSql<Numeric, Mysql> for Decimal<N>
where
    for<'a, 'b> Output<'a, 'b, Mysql>: Write,
    Decimal<N>: Debug,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        write!(out, "{}", *self)
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

impl<const N: usize> FromSql<Numeric, Mysql> for Decimal<N> {
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        let raw = value.as_bytes();

        match value.value_type() {
            MysqlType::UnsignedTiny => {
                let i = raw[0];
                Ok(i.into())
            }
            MysqlType::Tiny => {
                let i = raw[0] as i8;
                Ok(i.into())
            }
            MysqlType::UnsignedShort => {
                let i = u16::from_ne_bytes((&raw[..2]).try_into()?);
                Ok(i.into())
            }
            MysqlType::Short => {
                let i = i16::from_ne_bytes((&raw[..2]).try_into()?);
                Ok(i.into())
            }
            MysqlType::UnsignedLong => {
                let i = u32::from_ne_bytes((&raw[..4]).try_into()?);
                Ok(i.into())
            }
            MysqlType::Long => {
                let i = i32::from_ne_bytes((&raw[..4]).try_into()?);
                Ok(i.into())
            }
            MysqlType::UnsignedLongLong => {
                let i = u64::from_ne_bytes(raw.try_into()?);
                Ok(i.into())
            }
            MysqlType::LongLong => {
                let i = i64::from_ne_bytes(raw.try_into()?);
                Ok(i.into())
            }
            MysqlType::Float => {
                let i = f32::from_ne_bytes(raw.try_into()?);
                Ok(i.into())
            }
            MysqlType::Double => {
                let i = f64::from_ne_bytes(raw.try_into()?);
                Ok(i.into())
            }
            MysqlType::Numeric => {
                let s = core::str::from_utf8(raw)?;
                Decimal::from_str(s, Context::default())
                    .map_err(|_| format!("{s} is not valid decimal number ").into())
            }
            _ => Err(format!("{value:?} is not valid decimal number").into()),
        }
    }
}
