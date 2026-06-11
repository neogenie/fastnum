use core::fmt;
use serde::de;

use crate::decimal::{Context, UnsignedDecimal};

type UD<const N: usize> = UnsignedDecimal<N>;

pub struct Visitor<const N: usize>;

impl<'de, const N: usize> de::Visitor<'de> for Visitor<N> {
    type Value = UD<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a positive number or formatted decimal string")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::try_from(value).map_err(|err| E::custom(format!("{err}")))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::try_from(value).map_err(|err| E::custom(format!("{err}")))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(UD::<N>::from(value))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::try_from(value).map_err(|err| E::custom(format!("{err}")))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::try_from(value).map_err(|err| E::custom(format!("{err}")))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::try_from(value).map_err(|err| E::custom(format!("{err}")))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        UD::<N>::from_str(value, Context::default()).map_err(|err| E::custom(format!("{err}")))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut map = map;
        let value = map.next_key::<DecimalKey>()?;
        if value.is_none() {
            return Err(de::Error::invalid_type(de::Unexpected::Map, &self));
        }
        map.next_value()
    }
}

pub struct OptionVisitor<const N: usize>;

impl<'de, const N: usize> de::Visitor<'de> for OptionVisitor<N> {
    type Value = Option<UD<N>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a positive number or formatted decimal string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(Visitor).map(Some)
    }
}

struct DecimalKey;

const SERDE_JSON_NUMBER_TOKEN: &str = "$serde_json::private::Number";

impl<'de> de::Deserialize<'de> for DecimalKey {
    fn deserialize<D>(deserializer: D) -> Result<DecimalKey, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = ();

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid decimal field")
            }

            fn visit_str<E>(self, s: &str) -> Result<(), E>
            where
                E: de::Error,
            {
                if s == SERDE_JSON_NUMBER_TOKEN {
                    Ok(())
                } else {
                    Err(de::Error::custom("expected field with custom name"))
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)?;
        Ok(DecimalKey)
    }
}
