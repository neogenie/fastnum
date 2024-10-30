use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{self, de, ser};

use crate::decimal::unsigned::UnsignedDecimal;
use crate::decimal::{ParseError, TryFromIntError};

impl<UINT> ser::Serialize for UnsignedDecimal<UINT>
where
    UnsignedDecimal<UINT>: Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.collect_str(&self)
    }
}

#[cfg(not(feature = "string-only"))]
impl<'de, UINT> de::Deserialize<'de> for UnsignedDecimal<UINT>
where
    UINT: Default,
    UnsignedDecimal<UINT>: From<u64>,
    UnsignedDecimal<UINT>: From<u128>,
    UnsignedDecimal<UINT>: TryFrom<i64, Error = TryFromIntError>,
    UnsignedDecimal<UINT>: TryFrom<i128, Error = TryFromIntError>,
    UnsignedDecimal<UINT>: TryFrom<f32, Error = ParseError>,
    UnsignedDecimal<UINT>: TryFrom<f64, Error = ParseError>,
    UnsignedDecimal<UINT>: FromStr<Err = ParseError>,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(Visitor::<UINT>::default())
    }
}

#[cfg(feature = "string-only")]
impl<'de, UINT> de::Deserialize<'de> for UnsignedDecimal<UINT>
where
    UINT: Default,
    UnsignedDecimal<UINT>: From<u64>,
    UnsignedDecimal<UINT>: From<u128>,
    UnsignedDecimal<UINT>: TryFrom<i64, Error = TryFromIntError>,
    UnsignedDecimal<UINT>: TryFrom<i128, Error = TryFromIntError>,
    UnsignedDecimal<UINT>: TryFrom<f32, Error = ParseError>,
    UnsignedDecimal<UINT>: TryFrom<f64, Error = ParseError>,
    UnsignedDecimal<UINT>: FromStr<Err = ParseError>,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_str(Visitor::<UINT>::default())
    }
}

#[derive(Default)]
struct Visitor<UINT>(PhantomData<UINT>);

impl<'de, UINT> de::Visitor<'de> for Visitor<UINT>
where
    UnsignedDecimal<UINT>: From<u64>,
    UnsignedDecimal<UINT>: From<u128>,
    UnsignedDecimal<UINT>: TryFrom<i64, Error = TryFromIntError>,
    UnsignedDecimal<UINT>: TryFrom<i128, Error = TryFromIntError>,
    UnsignedDecimal<UINT>: TryFrom<f32, Error = ParseError>,
    UnsignedDecimal<UINT>: TryFrom<f64, Error = ParseError>,
    UnsignedDecimal<UINT>: FromStr<Err = ParseError>,
{
    type Value = UnsignedDecimal<UINT>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a positive number or formatted decimal string")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UnsignedDecimal::<UINT>::from(value))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UnsignedDecimal::<UINT>::from(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UnsignedDecimal::<UINT>::from_str(value).map_err(|err| E::custom(format!("{}", err)))
    }
}
