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
        write!(formatter, "formatted decimal string in strict mode")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(value, Context::default()).map_err(|err| E::custom(err))
    }
}
