#![doc = include_str!("../README.md")]
// #![deny(unsafe_code, missing_docs, clippy::all, clippy::cargo)]

extern crate core;

pub mod decimal;
pub mod int;

mod utils;

/// 128-bit signed integer type.
pub type I128 = int::I128;

/// 256-bit signed integer type.
pub type I256 = int::I256;

/// 512-bit signed integer type.
pub type I512 = int::I512;

/// 1024-bit signed integer type.
pub type I1024 = int::I1024;

/// 2048-bit signed integer type.
pub type I2048 = int::I2048;

/// 4096-bit signed integer type.
pub type I4096 = int::I4096;

/// 8192-bit signed integer type.
pub type I8192 = int::I8192;

/// 128-bit unsigned integer type.
pub type U128 = int::U128;

/// 256-bit unsigned integer type.
pub type U256 = int::U256;

/// 512-bit unsigned integer type.
pub type U512 = int::U512;

/// 1024-bit unsigned integer type.
pub type U1024 = int::U1024;

/// 2048-bit unsigned integer type.
pub type U2048 = int::U2048;

/// 4096-bit unsigned integer type.
pub type U4096 = int::U4096;

/// 8192-bit unsigned integer type.
pub type U8192 = int::U8192;

/// 128-bit unsigned decimal type.
pub type UD128 = decimal::unsigned::UnsignedDecimal<U128>;

/// 256-bit unsigned decimal type.
pub type UD256 = decimal::unsigned::UnsignedDecimal<U256>;

/// 512-bit unsigned decimal type.
pub type UD512 = decimal::unsigned::UnsignedDecimal<U512>;

/// 1024-bit unsigned decimal type.
pub type UD1024 = decimal::unsigned::UnsignedDecimal<U1024>;

/// 2048-bit unsigned decimal type.
pub type UD2048 = decimal::unsigned::UnsignedDecimal<U2048>;

/// 4096-bit unsigned decimal type.
pub type UD4096 = decimal::unsigned::UnsignedDecimal<U4096>;

/// 8192-bit unsigned decimal type.
pub type UD8192 = decimal::unsigned::UnsignedDecimal<U8192>;

/// 128-bit signed decimal type.
pub type D128 = decimal::signed::Decimal<U128>;

/// 256-bit signed decimal type.
pub type D256 = decimal::signed::Decimal<U256>;

/// 512-bit signed decimal type.
pub type D512 = decimal::signed::Decimal<U512>;

/// 1024-bit signed decimal type.
pub type D1024 = decimal::signed::Decimal<U1024>;

/// 2048-bit signed decimal type.
pub type D2048 = decimal::signed::Decimal<U2048>;

/// 4096-bit unsigned decimal type.
pub type D4096 = decimal::signed::Decimal<U4096>;

/// 8192-bit signed decimal type.
pub type D8192 = decimal::signed::Decimal<U8192>;

