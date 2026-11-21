use alloc::string::ToString;
use core::fmt;
use serde::de;

use crate::decimal::Context;

type UD<const N: usize> = crate::decimal::UnsignedDecimal<N>;

pub struct Visitor<const N: usize>;

impl<const N: usize> Visitor<N> {
    pub const fn default() -> Self {
        Self
    }
}

impl<const N: usize> de::Visitor<'_> for Visitor<N> {
    type Value = UD<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a positive number or formatted decimal string")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(&value.to_string(), Context::default()).map_err(|err| E::custom(err))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(&value.to_string(), Context::default()).map_err(|err| E::custom(err))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(&value.to_string(), Context::default()).map_err(|err| E::custom(err))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(&value.to_string(), Context::default()).map_err(|err| E::custom(err))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(&value.to_string(), Context::default()).map_err(|err| E::custom(err))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(&value.to_string(), Context::default()).map_err(|err| E::custom(err))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(value, Context::default()).map_err(|err| E::custom(err))
    }
}
