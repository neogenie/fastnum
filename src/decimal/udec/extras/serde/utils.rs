//! # Unsigned decimal number serialization and deserialization utils for serde

#[cfg(feature = "serde-arbitrary-precision")]
mod details;

#[cfg(feature = "serde-arbitrary-precision")]
type UD<const N: usize> = crate::decimal::UnsignedDecimal<N>;

/// Serialize/deserialize [crate::decimal::UnsignedDecimal] as arbitrary precision numbers in
/// JSON using the `arbitrary_precision` feature within `serde_json`.
///
/// ```
/// # use serde::Deserialize;
/// # use fastnum::*;
///
/// #[derive(Deserialize)]
/// pub struct Arbitrary {
///     #[serde(with = "decimal::serde::udec::arbitrary_precision")]
///     value: UD128,
/// }
///
/// let data: Arbitrary = serde_json::from_str(r#"{"value": 1.09861228866810969139524523692252570465}"#).unwrap();
/// assert_eq!(udec128!(1.09861228866810969139524523692252570465), data.value);
/// ```
#[cfg(feature = "serde-arbitrary-precision")]
pub mod arbitrary_precision {
    use super::*;

    use serde::{de, Serialize};
    use std::str::FromStr;

    /// Deserializer for `[`#[serde(with = "...")]`](<https://serde.rs/field-attrs.html#with>).
    pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<UD<N>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(details::Visitor)
    }

    /// Serializer for [`#[serde(with = "...")]`](<https://serde.rs/field-attrs.html#with>).
    pub fn serialize<S, const N: usize>(value: &UD<N>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::Number::from_str(&value.to_string())
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

/// Serialize/deserialize optional [crate::decimal::UnsignedDecimal] as arbitrary precision
/// numbers in JSON using the `arbitrary_precision` feature within `serde_json`.
///
/// ```
/// # use serde::Deserialize;
/// # use fastnum::*;
///
/// #[derive(Deserialize)]
/// pub struct Arbitrary {
///     #[serde(with = "decimal::serde::udec::arbitrary_precision_option")]
///     value: Option<UD128>,
/// }
///
/// let data: Arbitrary = serde_json::from_str(r#"{"value": 1.09861228866810969139524523692252570465}"#).unwrap();
/// assert_eq!(Some(udec128!(1.09861228866810969139524523692252570465)), data.value);
///
/// let data: Arbitrary = serde_json::from_str(r#"{"value": null}"#).unwrap();
/// assert_eq!(None, data.value);
/// ```
#[cfg(feature = "serde-arbitrary-precision")]
pub mod arbitrary_precision_option {
    use super::*;

    use serde::{de, Serialize};

    /// Deserializer for [`#[serde(with = "...")]`](<https://serde.rs/field-attrs.html#with>).
    pub fn deserialize<'de, D, const N: usize>(deserializer: D) -> Result<Option<UD<N>>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(details::OptionVisitor)
    }

    /// Serializer for [`#[serde(with = "...")]`](<https://serde.rs/field-attrs.html#with>).
    pub fn serialize<S, const N: usize>(
        value: &Option<UD<N>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match value {
            Some(decimal) => {
                serde_json::Number::from_string_unchecked(decimal.to_string()).serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }
}
