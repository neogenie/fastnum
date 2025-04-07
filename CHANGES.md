# Changelog

All user-visible changes to this library will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

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