//! # Decimal number serialization and deserialization utils for serde

#[cfg(feature = "serde-arbitrary-precision")]
mod details;

#[cfg(feature = "serde-arbitrary-precision")]
type D<const N: usize> = crate::decimal::Decimal<N>;

/// Serialize/deserialize [Decimal] as arbitrary precision numbers in JSON using
/// the `arbitrary_precision` feature within `serde_json`.
///
/// ```
/// # use serde::Deserialize;
/// # use fastnum::*;
///
/// #[derive(Deserialize)]
/// pub struct Arbitrary {
///     #[serde(with = "decimal::serde::dec::arbitrary_precision")]
///     value: D128,
/// }
///
/// let data: Arbitrary = serde_json::from_str(r#"{"value": -1.09861228866810969139524523692252570465}"#).unwrap();
/// assert_eq!(dec128!(-1.09861228866810969139524523692252570465), data.value);
/// ```
#[cfg(feature = "serde-arbitrary-precision")]
pub mod arbitrary_precision {

    use super::*;

    use serde::{de, Serialize};
    use std::str::FromStr;

    /// Deserializer for [`#[serde(with = "...")]`](https://serde.rs/field-attrs.html#with).
    pub fn deserialize<'de, Ds, const N: usize>(deserializer: Ds) -> Result<D<N>, Ds::Error>
    where
        Ds: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(details::Visitor)
    }

    /// Serializer for [`#[serde(with = "...")]`](https://serde.rs/field-attrs.html#with).
    pub fn serialize<S, const N: usize>(value: &D<N>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::Number::from_str(&value.to_string())
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

/// Serialize/deserialize optional [Decimal] as arbitrary precision numbers in
/// JSON using the `arbitrary_precision` feature within `serde_json`.
///
/// ```
/// # use serde::Deserialize;
/// # use fastnum::*;
///
/// #[derive(Deserialize)]
/// pub struct Arbitrary {
///     #[serde(with = "decimal::serde::dec::arbitrary_precision_option")]
///     value: Option<D128>,
/// }
///
/// let data: Arbitrary = serde_json::from_str(r#"{"value": -1.09861228866810969139524523692252570465}"#).unwrap();
/// assert_eq!(Some(dec128!(-1.09861228866810969139524523692252570465)), data.value);
///
/// let data: Arbitrary = serde_json::from_str(r#"{"value": null}"#).unwrap();
/// assert_eq!(None, data.value);
/// ```
#[cfg(feature = "serde-arbitrary-precision")]
pub mod arbitrary_precision_option {
    use super::*;

    use serde::{de, Serialize};

    /// Deserializer for [`#[serde(with = "...")]`](https://serde.rs/field-attrs.html#with).
    pub fn deserialize<'de, Ds, const N: usize>(deserializer: Ds) -> Result<Option<D<N>>, Ds::Error>
    where
        Ds: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(details::OptionVisitor)
    }

    /// Serializer for [`#[serde(with = "...")]`](https://serde.rs/field-attrs.html#with).
    pub fn serialize<S, const N: usize>(
        value: &Option<D<N>>,
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
