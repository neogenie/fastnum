# Changelog

All user-visible changes to this library will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md).

# [0.7.2] - 2025-09-20

### Fixed

- Incorrect division result for numbers with large amount of decimal digits [#49](https://github.com/neogenie/fastnum/issues/49).
- Fix `no_std` build [#50](https://github.com/neogenie/fastnum/issues/50).

### Changed

- Improve debug formatting of `Decimal` and `UnsignedDecimal`.

### Examples

- Add `no_std` wasm example [#50](https://github.com/neogenie/fastnum/issues/50).

# [0.7.1] - 2025-08-31

### Fixed

- .ln() panic regression (0.2.10 -> 0.7.0) [#46](https://github.com/neogenie/fastnum/issues/46).

### Changed

- Improve log implementation [#38](https://github.com/neogenie/fastnum/issues/38).

### Documentation

- Documentation for `Decimal::with_rounding_mode` and `Decimal::with_ctx`:
    - Clarified that when changing the rounding mode, any existing extra precision (if present) is immediately rounded
      using the new rule.
    - Clarified that applying a new context may trigger traps if the current value already has signaling flags set (
      e.g., `INEXACT`) and the new context enables traps for those signals; this may result in a panic depending on
      build/configuration.
- Minor fixes.

# [0.7.0] - 2025-08-21

### Added

- Big-integer conversion APIs [#44](https://github.com/neogenie/fastnum/issues/44):
    - Parsing from raw bytes:
        - `from_radix_be(buf, radix) -> Option<Self>`
        - `from_radix_le(buf, radix) -> Option<Self>`

    - Encoding to raw bytes:
        - `to_radix_be(radix) -> Vec<u8>`
        - `to_radix_le(radix) -> Vec<u8>`

    - String conversion helpers:
        - `parse_str_radix(s, radix) -> Self` (panicking variant)
        - `parse_bytes(buf, radix) -> Option<Self>`
        - `to_str_radix(radix) -> String`

### Documentation

- Conversion documentation expanded with clearer semantics, base handling, and examples.
- Improved doc comments for:
    - Base-aware parsing behavior and panic conditions.
    - Byte-level import/export helpers.
    - `parse_str(s)` and `from_str(s)` for big integers recognize `0x` and `0b` prefixes (decimal remains default
      without a prefix).

### Internal

- Conversion code reorganized into focused modules:
    - `bint/convert/{from_bytes,to_bytes,from_str,to_str}.rs`
- Macro routing for conversion implementation is simplified and de-duplicated.

# [0.6.2] - 2025-08-18

### Changed

- Minor internal performance improvements.

# [0.6.1] - 2025-08-16

### Changed

- Minor performance improvements.

# [0.6.0] - 2025-08-16

### Breaking changes

- Removed previously deprecated Decimal methods:
    - `from_scale` (use `quantum` instead).
    - `normalized` (use `reduce` instead).
    - `with_scale` (use `rescale` instead).

### Added

- Decimal truncation API ([#39](https://github.com/neogenie/fastnum/issues/39)):
    - `Decimal::trunc()` — truncates to integral with no fractional portion without rounding.
    - `Decimal::trunc_with_scale(scale)` — truncates to the specified scale without rounding.

### Changed

- Internal documentation macro routing for decimal type aliases refined (no public API impact).

### Documentation

- Minor fixes.
- Added a dedicated “Truncate” section with behavior details and examples.

### Internal

- Introduced an internal truncation implementation integrated with scaling and extra-precision handling to ensure true
  truncation semantics (no rounding).

# [0.5.0] - 2025-08-15

### Added

- Type conversion traits for integers and decimals:
    - `Cast` — type-safe, infallible conversion (e.g., widening or lossless transforms).
    - `TryCast` — checked conversion returning an error on overflow, sign mismatch, or incompatible scale.
    - Supported families: unsigned/signed big integers (`U64`, `U128`, `U256`, `U512`, `U1024`, and `I64`, `I128`,
      `I256`, `I512`, `I1024`) [#42](https://github.com/neogenie/fastnum/issues/42) and decimals (`UD64`, `UD128`,
      `UD256`, `UD512`, `UD1024`, and `D64`, `D128`, `D256`,
      `D512`, `D1024`).
    - Common scenarios:
        - Widening cast for integers — via `Cast`.
        - Checked narrowing cast — via `TryCast`, with overflow errors.
        - Conversions between unsigned and signed integers — `TryCast` rejects negative/out-of-range values.
        - Conversions between `UD*` and `D*` — proper sign/scale propagation with checks in `TryCast`.
- Const-generic “dimensions” for conversion direction checks:
    - Internal `Widen`/`Narrow` markers ensure `Cast`/`TryCast` correctness without unstable generic const expressions.

### Deprecated

- Decimal `.transmute()` is now deprecated (since "0.5.0"); removal is planned for a future major release.
  Use `.resize()` or cast via `Cast` and `TryCast` traits instead.

### Documentation

- Minor fixes and clarifications; added usage examples for type conversions.

# [0.4.5] - 2025-08-12

### Documentation

- Minor fixes.

# [0.4.4] - 2025-08-12

### Documentation

- Minor fixes.

# [0.4.3] - 2025-08-12

### Documentation

- Minor fixes.

# [0.4.2] - 2025-08-11

This release is still mostly focused on fix: [#32](https://github.com/neogenie/fastnum/issues/32).

### Documentation

- Minor fixes.

# [0.4.1] - 2025-08-10

### Documentation

- Minor fixes.

# [0.4.0] - 2025-08-10

This release is mostly focused on fix: [#32](https://github.com/neogenie/fastnum/issues/32).
A lot of work was done, but a huge part of the optimization should still be completed.

### Added

- Added core mathematical operations:
    - `widening_mul`: full 256-bit multiplication for 128-bit integers
    - `div_digit`: optimized division by single 64-bit digit
    - `decimal_digits`: efficient decimal digit counting
    - `remaining_decimal_digits`: safe decimal scaling calculation
    - `can_scaled_by_power_of_ten`: overflow prevention for decimal operations

### Fixed

- Fixed postgres padding overflowing [#43](https://github.com/neogenie/fastnum/issues/43).
- Fixed bug in `widening_mul` for u128 where high bits were incorrectly handled.
- Fixed incorrect overflow detection in large number multiplication.
- Fixed edge cases in power-of-ten scaling operations.

### Performance

- Optimized division algorithm for decimal numbers.
- Optimized multiplication algorithm for 128-bit integers.
- Improved overflow checking using precomputed values.
- Enhanced decimal digit counting performance.

### Internal

- Improved code documentation with detailed algorithm descriptions.
- Added comprehensive examples for core numeric operations.
- Enhanced performance-related documentation.

## [0.3.2] – 2025-07-22

### Changed

- Internal performance optimizations.

## [0.3.1] – 2025-07-21

### Changed

This release is mostly focused on fix: [#34](https://github.com/neogenie/fastnum/issues/34).

Converting decimal numbers to floating point IEEE 754 was completely redesigned.
Implementation is based on LLVM’s libc
experience: https://llvm.org/devmtg/2022-11/slides/QuickTalk3-ApproximatingatScale-StringToFloat.pdf
Algorithms:

- First pass: Clinger’s Fast
  Path: [Clinger WD. How to Read Floating Point Numbers Accurately](https://doi.org/10.1145/93548.93557).
- Second Pass: Eisel-Lemire fast_float: [Number Parsing at a Gigabyte per Second](https://arxiv.org/abs/2101.11408)
  and [Noble Mushtak, Daniel Lemire. Fast number parsing without fallback](https://doi.org/10.1002/spe.3198).
- Third Pass: Simple Decimal
  Conversion [Nigel Tao’s description of the Simple Decimal Conversion algorithm](https://nigeltao.github.io/blog/2020/parse-number-f64-simple.html).

### Documentation

- Add decimal -> f64 conversion benchmarks.

## [0.3.0] – 2025-07-15

### Changed

- Bump Rust version to `1.87` (stabilize [
  `#![feature(integer_sign_cast)]`](https://github.com/rust-lang/rust/issues/125882)).
- Replace big integer's implementation from `bnum` type aliasing to wrapped types.
- Some minor performance optimizations.

## [0.2.10] – 2025-06-04

### Fixed

- Fix incorrect D64 to_f64 implementation: [#29](https://github.com/neogenie/fastnum/issues/29).

## [0.2.9] – 2025-06-02

### Fixed

- Fix inaccurate pow: [#28](https://github.com/neogenie/fastnum/issues/28).

## [0.2.8] – 2025-05-17

### Added

- Add `impl From<UnsignedDecimal> for Decimal` convertion trait implementation.

### Fixed

- Fix display rounding error: [#24](https://github.com/neogenie/fastnum/issues/24).

## [0.2.7] – 2025-05-15

### Fixed

- Incorrect cmp implementation: [#22](https://github.com/neogenie/fastnum/issues/22).

## [0.2.6] – 2025-05-13

### Fixed

- The comparison function does not correctly implement a total
  order: [#21](https://github.com/neogenie/fastnum/issues/21).

## [0.2.5] – 2025-05-04

### Added

- Examples for compile time environment variables.

### Fixed

- Failing to converge the Brent iterative method for calculating reciprocals (infinity
  loop): [#20](https://github.com/neogenie/fastnum/issues/20).

## [0.2.4] – 2025-05-03

### Added

- Add public constant functions for convert Decimal and UnsignedDecimal from/to Rust's primitive numeric
  types: [#15](https://github.com/neogenie/fastnum/issues/15).
- Introduce `.transmute()` operation for N-to-M bit unsigned decimal
  conversion: [#18](https://github.com/neogenie/fastnum/issues/18).

## [0.2.3] – 2025-04-08

### Fixed

- Internal conversion to NBase fails for some numbers: [#16](https://github.com/neogenie/fastnum/issues/16).

## [0.2.2] – 2025-02-22

### Changed

- Some float2decimal performance optimizations.

### Fixed

- Minor fixes in float2decimal conversion.
- Minor fixes with extra precision digits.

## [0.2.1] – 2025-02-18

### Fixed

- Incorrect `.ceil()` and `floor()` behavior for negative values.

## [0.2.0] – 2025-02-17

This release primarily focuses on:

- Increasing the accuracy of approximated mathematical calculations due to the use of `7` extra precision digits
  instead of `4`.
- Performance optimizations.

Expect better performance, precision handling, and expanded documentation covering new methods and examples.

- Bump Rust version to `1.83`
    - [Stabilize &mut, *mut, &Cell, and *const Cell in const.](https://github.com/rust-lang/rust/pull/129195)

### Added

- `floor()` and `ceil()` const methods.
- Add basic tests for trivial trigonometric functions for 0–360 degree angles.

### Changed

- Control block is completely refactored for better performance and more compact memory layout.
- Micro-optimizations in rounding.
- Inline optimizations and better precision handling for mathematical rounding.
- Remove custom const float helpers as they're stabilized as const in rust 1.83.

### Fixed

- Incorrect `floor()` and `ceil()` methods in `num_traits` `Float` implementation.

### Documentation

- Minor fixes.

## [0.1.14] – canceled

### Fixed

- Incorrect ceiling rounding: [#9](https://github.com/neogenie/fastnum/issues/9).

### Documentation

- Minor fixes.

## [0.1.13] – 2025-02-07

### Added

- Add `tokio-postgres` feature which enables serialization and deserialization of `fastnum` decimals for [
  `tokio-postgres`](https://docs.rs/tokio-postgres/latest/tokio_postgres/)
  crate [#8](https://github.com/neogenie/fastnum/issues/8).
- Add `tokio-postgres` and `sqlx_postgres` examples.

### Documentation

- Minor fixes.

## [0.1.12] – 2025-02-06

### Fixed

- Incorrect to_f64 impl on Decimal: [#7](https://github.com/neogenie/fastnum/issues/7).

### Documentation

- Minor fixes.

## [0.1.11] – 2025-02-04

### Fixed

- Fixed compilation warnings

## [0.1.10] – 2025-02-04

This release primarily focuses on:

- Implementing a full range of advanced mathematical functions (exponential, roots, power, logarithmic, and
  trigonometric functions) for working with exact precision decimal numbers.
- Increasing the accuracy of approximated mathematical calculations due to the use of extra precision digits and the
  absence of intermediate rounding.
- Fixing and improving conversions between Float and Decimal types.

Expect better performance, precision handling, and expanded documentation covering new methods, constants, and examples.
Highlights include removing `libm` dependency and major refinements for `no-std` environments.

### Added

- Implement exponential `exp` and binary exponential `exp2` functions for decimals.
- Implement logarithmic `ln`, `log10`, `log2`, `log` functions for decimals.
- Implement `sqrt`, `cbrt`, and `nth_root` roots functions for decimals.
- Refactor and extend `pow` to support non-integer exponents.
- Implement base trigonometric functions: `sin`, `cos`, `tan`, `sin_cos` for decimals.
- Implement inverse trigonometric functions: `asin`, `acos`, `atan`, `atan2` for decimals.
- Implement hyperbolic functions: `sinh`, `cosh`, `tanh` for decimals.
- Implement inverse hyperbolic functions: `asinh`, `acosh`, `atanh` for decimals.
- Implement hypotenuse calculation `hypot` function for decimals.
- Introduce fused multiply-add `mul_add` operation without intermediate rounding.
- Introduce `transmute` operation for N-to-M bit decimal conversion.
- Add integer and unsigned integer conversion utilities (`from_int`, `from_uint`).
- Implement `num_traits::float::Float` trait.

### Changed

- Improved arithmetic precision of approximated mathematical calculations due to the use of extra precision digits and
  the absence of intermediate rounding.
- Overhauled mathematical constants. Add `INEXACT` flag and extra precision digits.
- Mark some methods as `inexact` by design.
- Micro-optimizations in rounding.
- Inline optimizations and better precision handling for mathematical rounding.
- Re-implement `recip` method without division.
- Improve test coverage for mathematical operations (e.g., `sqrt`, `ln`, `exp`).
- Remove `libm` dependency for `no-std` environment.

### Fixed

- Corrected edge cases for float (`f32`/`f64`) from/to `Decimal` conversions (
  fix [#5](https://github.com/neogenie/fastnum/issues/5#issue-2813957559)).
- Fix _long_ rounding issues with improved context precision.
- General code quality improvements and bug fixes, including better inexact flag handling.

### Documentation

- Expanded documentation for all major operations with examples.
- Added comprehensive examples and descriptions to `LIB.md`.
- Minor fixes.

## [0.1.9] – 2025-01-01

### Breaking changes

* Replace `decimal::Category` with `core::num::FpCategory`.

### Changed

* Micro-optimizations in rounding.
* Make `.from_parts()` constructor public (Way to directly create decimals with scale without dividing? #3).

### Added

* Implement `TryFrom<Decimal>` for `UnsignedDecimal` and `From<UnsignedDecimal>` for `Decimal` traits.

### Fixed

* Fixed minor issues with ceil/floor rounding.

## [0.1.8] – 2024-12-28

### Fixed

* Fixed a performance issue with parsing and rescaling during arithmetic operations.

### Changed

* Replace `ilog10` method for integers with faster algorithm.
* Replace `x 10` multiplication with a faster algorithm (const static lookup table).
* Some performance improvements.

## [0.1.7] – 2024-12-25

### Fixed

* Division by divisor with non-scalable coefficient.

### Changed

* More strict `#[repr]` for Decimal type.
* Documentation improvements.

### Added

* Implement `num_traits::float::FloatCore` trait.
* Add `MIN_POSITIVE` and `EPSILON` constants.
* Add `.powi()`, `.recip()`, `.to_degrees()`, `.to_radians()` methods.

## [0.1.6] – 2024-12-24

### Changed

* Documentation improvements.

### Added

* Add basic mathematical constants (`PI`, `E`, etc.).
* Implement `num_traits::FloatConst` trait.

## [0.1.5] – 2024-12-22

### Fixed

* cargo test failing two tests on macOS #2

## [0.1.4] – 2024-12-22

The main goals of this release are:

- Stabilize API for more stringent compliance with the recommendations of IEEE 754 and IEEE 854 standards.
- Refactor decimal module and simplify context usage.

### Breaking changes

* Remove [`Context`] from a most methods argument list and put it into decimal control block for better context
  handling.
  Now [`Context`] is a property of any Decimal number instance.

### Added

* Extend test coverage.

### Changed

* Documentation improvements.
* Deprecate and replace some outdated methods to align with the updated structure and functionality.
* Benchmarks were modularized into separate operations, enhancing clarity and maintainability.

## [0.1.3] – 2024-12-16

### Added

* Extend test coverage.

### Fixed

* `sqlx` support for `PostgreSQL`.

### Changed

* Minor documentation fixes.
* Remove `const_str` dependency.

## [0.1.2] – 2024-12-15

### Added

* Added `signals!` macro.
* Extend test coverage.

### Changed

* Deprecated `with_scale!` in favor of `quantum!`.
* Documentation improvements.

## [0.1.1] – 2024-12-11

* Minor fixes.

## [0.1.0] – 2024-12-11

### Breaking changes

* Internal decimal representation for signed and unsigned decimal types.
* Reduce exponent from `64` bit to `16` bit.
* Remove `DecimalResult` and unwrap methods.
* Replace `ArithmeticPolicy`, `OverflowPolicy`, `RoundingPolicy` with `Context`.

### Added

* Add `NaN` and `±Infinity` special values.
* Add `Flags` and `Signal`.
* Add `Context` and `SignalingTraps`.
* Add `with_context!` macro.

### Fixed

* Fix panic shift with overflow.

### Changed

* Stabilize API.
* Re-implement most methods.
* Documentation improvements.

## [0.0.14] – 2024-11-27

* Fix rounding subtraction.
* Extend test coverage for `numtraits` feature.
* Documentation improvements.

## [0.0.13] – 2024-11-24

* Stabilize API

## [0.0.12] – 2024-11-23

## [0.0.11] – 2024-11-23

## [0.0.10] – 2024-11-23

## [0.0.9] – 2024-11-23

* Stabilize API

## [0.0.8] – 2024-11-19

* Bump `utoipa` to 5.0.x version

## [0.0.7] – 2024-11-19

* Stabilize API
* Improve docs

## [0.0.6] – 2024-11-19

* Stabilize API

## [0.0.6]

## [0.0.5]

## [0.0.4]

## [0.0.3]

* Technical releases

## [0.0.1] – 2024-10-21

* Initial commit