use alloc::string::ToString;
use core::fmt;
use serde::de;

use crate::decimal::Context;

type D<const N: usize> = crate::decimal::Decimal<N>;

pub struct Visitor<const N: usize>;

impl<const N: usize> Visitor<N> {
    pub const fn default() -> Self {
        Self
    }
}

impl<const N: usize> de::Visitor<'_> for Visitor<N> {
    type Value = D<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a valid number or formatted decimal string")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(D::<N>::from(value))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        D::<N>::try_from(value).map_err(|err| E::custom(err))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(D::<N>::from(value))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        D::<N>::try_from(value).map_err(|err| E::custom(err))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        D::<N>::from_str(&value.to_string(), Context::default()).map_err(|err| E::custom(err))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        D::<N>::from_str(&value.to_string(), Context::default()).map_err(|err| E::custom(err))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        D::<N>::from_str(value, Context::default()).map_err(|err| E::custom(err))
    }
}
