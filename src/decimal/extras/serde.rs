include!(concat!(env!("OUT_DIR"), "/serde_deserialize_mode.rs"));

/// Determines how to deserialize decimal numbers
///
/// Default deserialize mode is `Strict`
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum DeserializeMode {
    /// Allow only string values such as `"0.1"`, `"0.25"`, etc.
    Strict,
    /// Decimal values such as `0.1` will be stringify to `"0.1"`
    Stringify,
    /// Any values
    Any,
}

impl DeserializeMode {
    pub const fn default() -> Self {
        SERDE_DESERIALIZE_MODE
    }
}
