use std::fmt;
use std::fmt::Display;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::{self, de, ser};

use crate::decimal::signed::Decimal;
use crate::decimal::{ParseError, TryFromIntError};

impl<UINT> ser::Serialize for Decimal<UINT>
where
    Self: Display,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.collect_str(&self)
    }
}

#[cfg(not(feature = "string-only"))]
impl<'de, UINT> de::Deserialize<'de> for Decimal<UINT>
where
    UINT: Default,
    Self: From<u64>
        + From<u128>
        + TryFrom<i64, Error = TryFromIntError>
        + TryFrom<i128, Error = TryFromIntError>
        + TryFrom<f32, Error = ParseError>
        + TryFrom<f64, Error = ParseError>
        + FromStr<Err = ParseError>,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_any(Visitor::<UINT>::default())
    }
}

#[cfg(feature = "string-only")]
impl<'de, UINT> de::Deserialize<'de> for Decimal<UINT>
where
    UINT: Default,
    Self: From<u64>
        + From<u128>
        + TryFrom<i64, Error = TryFromIntError>
        + TryFrom<i128, Error = TryFromIntError>
        + TryFrom<f32, Error = ParseError>
        + TryFrom<f64, Error = ParseError>
        + FromStr<Err = ParseError>,
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
    Decimal<UINT>: From<u64>,
    Decimal<UINT>: From<u128>,
    Decimal<UINT>: TryFrom<i64, Error = TryFromIntError>,
    Decimal<UINT>: TryFrom<i128, Error = TryFromIntError>,
    Decimal<UINT>: TryFrom<f32, Error = ParseError>,
    Decimal<UINT>: TryFrom<f64, Error = ParseError>,
    Decimal<UINT>: FromStr<Err = ParseError>,
{
    type Value = Decimal<UINT>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a positive number or formatted decimal string")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Decimal::<UINT>::from(value))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Decimal::<UINT>::from(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::try_from(value).map_err(|err| E::custom(format!("{}", err)))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Decimal::<UINT>::from_str(value).map_err(|err| E::custom(format!("{}", err)))
    }
}
