pub mod utils;

mod visitors;

use core::fmt::Display;
use serde::{self, de, ser};

use crate::decimal::{extras::serde::DeserializeMode, UnsignedDecimal};

impl<const N: usize> ser::Serialize for UnsignedDecimal<N>
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

impl<'de, const N: usize> de::Deserialize<'de> for UnsignedDecimal<N> {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        const MODE: DeserializeMode = DeserializeMode::default();

        match MODE {
            DeserializeMode::Strict => d.deserialize_str(visitors::strict::Visitor::<N>::default()),
            DeserializeMode::Stringify => {
                d.deserialize_any(visitors::stringify::Visitor::<N>::default())
            }
            DeserializeMode::Any => d.deserialize_any(visitors::any::Visitor::<N>::default()),
        }
    }
}
