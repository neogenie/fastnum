use std::fmt::{Debug, Display};
use std::io::prelude::*;
use std::str::FromStr;

use diesel::deserialize::{self, FromSql};
use diesel::mysql::{Mysql, MysqlType, MysqlValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Numeric;

use crate::decimal::unsigned::UnsignedDecimal;

impl<UINT> ToSql<Numeric, Mysql> for UnsignedDecimal<UINT>
where
    UnsignedDecimal<UINT>: Debug + Display,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        write!(out, "{}", *self)
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}

impl<UINT> FromSql<Numeric, Mysql> for UnsignedDecimal<UINT>
where
    Self: From<u8>
        + TryFrom<i8>
        + From<u16>
        + TryFrom<i16>
        + From<u32>
        + TryFrom<i32>
        + From<u64>
        + TryFrom<i64>
        + TryFrom<f32>
        + TryFrom<f64>
        + FromStr,
{
    fn from_sql(value: MysqlValue<'_>) -> deserialize::Result<Self> {
        let raw = value.as_bytes();

        match value.value_type() {
            MysqlType::UnsignedTiny => {
                let i = raw[0];
                Ok(i.into())
            }
            MysqlType::Tiny => {
                let i = raw[0] as i8;
                i.try_into()
                    .map_err(|_| format!("{i} is not valid decimal number").into())
            }
            MysqlType::UnsignedShort => {
                let i = u16::from_ne_bytes((&raw[..2]).try_into()?);
                Ok(i.into())
            }
            MysqlType::Short => {
                let i = i16::from_ne_bytes((&raw[..2]).try_into()?);
                i.try_into()
                    .map_err(|_| format!("{i} is not valid decimal number").into())
            }
            MysqlType::UnsignedLong => {
                let i = u32::from_ne_bytes((&raw[..4]).try_into()?);
                Ok(i.into())
            }
            MysqlType::Long => {
                let i = i32::from_ne_bytes((&raw[..4]).try_into()?);
                i.try_into()
                    .map_err(|_| format!("{i} is not valid decimal number").into())
            }
            MysqlType::UnsignedLongLong => {
                let i = u64::from_ne_bytes(raw.try_into()?);
                Ok(i.into())
            }
            MysqlType::LongLong => {
                let i = i64::from_ne_bytes(raw.try_into()?);
                i.try_into()
                    .map_err(|_| format!("{i} is not valid decimal number").into())
            }
            MysqlType::Float => {
                let i = f32::from_ne_bytes(raw.try_into()?);
                i.try_into()
                    .map_err(|_| format!("{i} is not valid decimal number").into())
            }
            MysqlType::Double => {
                let i = f64::from_ne_bytes(raw.try_into()?);
                i.try_into()
                    .map_err(|_| format!("{i} is not valid decimal number").into())
            }
            MysqlType::Numeric => {
                let s = std::str::from_utf8(raw)?;
                UnsignedDecimal::from_str(s)
                    .map_err(|_| format!("{s} is not valid decimal number ").into())
            }
            _ => Err(format!("{value:?} is not valid decimal number").into()),
        }
    }
}
