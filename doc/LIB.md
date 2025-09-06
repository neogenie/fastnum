# fastnum

Fixed-size signed and unsigned decimal numbers, implemented in pure Rust.
Suitable for financial, crypto and any other fixed-precision calculations.

[IEEE 754]: https://en.wikipedia.org/wiki/IEEE_754

[IEEE 854]: https://en.wikipedia.org/wiki/IEEE_854-1987

[`D64`]: crate::D64

[`UD64`]: crate::UD64

[`D128`]: crate::D128

[`UD128`]: crate::UD128

[`D256`]: crate::D256

[`UD256`]: crate::UD256

[`D512`]: crate::D512

[`UD512`]: crate::UD512

[`U64`]: crate::U64

[`U128`]: crate::U128

[`U128`]: crate::U128

[`U256`]: crate::U256

[`U256`]: crate::U256

[`U512`]: crate::U512

[`U512`]: crate::U512

## Overview

`fastnum` provides signed and unsigned exact precision decimal numbers suitable for financial calculations that
require significant integral and fractional digits with no round-off errors (such as 0.1 + 0.2 ≠ 0.3).

Any `fastnum` decimal type consists of an N-bit big unsigned integer, paired with a 64-bit control block which
contains a 16-bit scaling factor determines the position of the decimal point, sign, special, and signaling flags.
Trailing zeros are preserved and may be exposed when in string form.

Thus, `fastnum` decimal numbers are trivially copyable and don't require any dynamic allocation.
This allows you to get additional performance gains by eliminating not only dynamic allocation, like such, but also will
get rid of one indirect addressing, which improves cache-friendliness and reduces the CPU load.

### Why fastnum?

- **Strictly exact precision**: no round-off errors (such as 0.1 + 0.2 ≠ 0.3).
- **Special values**: `fastnum` support `±0`, `±Infinity` and `NaN` special values with [IEEE 754] semantic.
- **Blazing fast**: `fastnum` numerics are as fast as native types, well almost :).
- **Trivially copyable types**: all `fastnum` numerics are trivially copyable (both integer and decimal, ether signed
  and unsigned) and can be stored on the stack, as they're _fixed size_.
- **No dynamic allocation**: no expensive _sys-call's_, no indirect addressing, cache-friendly.
- **Compile-time integer and decimal parsing**: all the `from_*` methods on `fastnum` integers
  and decimals are `const`, which allows parsing of integers and numerics from string slices and floats at compile time.
  Additionally, the string to be parsed does not have to be a literal: it could, for example, be obtained via [
  `include_str!`](https://doc.rust-lang.org/core/macro.include_str.html), or [
  `env!`](https://doc.rust-lang.org/core/macro.env.html).
- **Const-evaluated in compile time macro-helpers**: any type has
  its [own macro helper](#const-evaluated-in-compile-time-macro-helpers) which can be used for
  definitions of constants or variables whose value is known in advance. This allows you to perform all the necessary
  checks at the compile time.
- **Short dependencies list by default**: `fastnum` depends only upon `bnum` by default.
  All other dependencies are optional.
  Support for crates such as [`rand`](https://docs.rs/rand/latest/rand/) and [
  `serde`](https://docs.rs/serde/latest/serde/) can be enabled with crate [features](#features).
- **`no-std` compatible**: `fastnum` can be used in `no_std` environments.
- **`const` evaluation**: nearly all methods defined on `fastnum` integers and decimals are `const`, which allows
  complex compile-time calculations and checks.
- **Full range of advanced mathematical functions**: exponential, roots, power, logarithmic, and trigonometric functions
  for working with exact precision decimals.
  And yes, they're all `const` too.

## Installation

To install and use `fastnum`, add the following line to your `Cargo.toml` file in the `[dependencies]` section:

```toml
fastnum = "0.7"
```

Or, to enable various `fastnum` features as well, add for example this line instead:

```toml
fastnum = { version = "0.7", features = ["serde"] } # enables the "serde" feature
```

## Example usage

```
use fastnum::{udec256, UD256};

let a = udec256!(0.1);
let b = udec256!(0.2);

assert_eq!(a + b, udec256!(0.3));
```

## Const-evaluated in compile time macro-helpers

Any type has its own macro helper which can be used for definitions of constants or variables whose value is known in
advance. This allows you to perform all the necessary checks at the compile time.

| Decimal type | Integer part | Bits | Signed | Helper macro    |
|--------------|--------------|------|:------:|-----------------|
| [`D64`]      | [`U64`]      | 64   |   ✅    | `dec64!(0.1)`   |
| [`UD64`]     | [`U64`]      | 64   |        | `udec64!(0.1)`  |
| [`D128`]     | [`U128`]     | 128  |   ✅    | `dec128!(0.1)`  |
| [`UD128`]    | [`U128`]     | 128  |        | `udec128!(0.1)` |
| [`D256`]     | [`U256`]     | 256  |   ✅    | `dec256!(0.1)`  |
| [`UD256`]    | [`U256`]     | 256  |        | `udec256!(0.1)` |
| [`D512`]     | [`U512`]     | 512  |   ✅    | `dec512!(0.1)`  |
| [`UD512`]    | [`U512`]     | 512  |        | `udec512!(0.1)` |

### Examples

Basic usage:

```
use fastnum::*;

// This value will be evaluated at compile-time and inlined directly into the relevant context when used. 
const PI: UD256 = udec256!(3.141592653589793115997963468544185161590576171875);
```

Compile-time calculations:

```
use fastnum::*;

// This value will be evaluated at compile-time and inlined directly into the relevant context when used. 
const PI_X_2: UD256 = udec256!(2).mul(udec256!(3.141592653589793115997963468544185161590576171875));
```

Compile-time checks

```compile_fail
use fastnum::*;

// Invalid character.
const E: UD256 = udec256!(A3.5);
```

```compile_fail
use fastnum::*;

// Arithmetic error during calculation.
const E: UD256 = udec256!(1.5).div(udec256!(0));
```

## Representation

### Abstract model

Numbers represent the values which can be manipulated by, or be the results of.
Numbers may be [finite numbers](#finite-numbers) (numbers whose value can be represented exactly),
or they may be [special values](#special-values) (infinities and other values which aren't finite numbers).

#### Finite numbers

The numerical value of a finite number is given by: _(–1)<sup>sign</sup> × coefficient × 10<sup>exponent</sup>_.

- **_sign_** – a value which must be either `0` or `1`, where `1` indicates that the number is negative or is the
  negative zero and `0` indicates that the number is zero or positive.
- **_coefficient_** – an unsigned integer which is zero or positive.
- **_exponent_** – a signed integer which indicates the power of ten by which the coefficient is multiplied.

For example, if the sign had the value `1`, the exponent had the value `–1`, and the coefficient had the value `25`,
then the numerical value of the number is exactly `–2.5`.

Trailing zeros are preserved and may be exposed when in string form. These can be truncated using the normalize or
round functions.

A number with a coefficient of `0` is permitted to have both `+` and `-` sign.
Negative zero is accepted as an operand for all operations (see [IEEE 754]).

The _**quantum**_ of a finite number is given by: _1 × 10<sup>-exp</sup>_.
This is the value of a unit in the least significant position of the _coefficient_ of a finite number.

This abstract definition deliberately allows for multiple representations of values which are numerically equal but are
visually distinct (such as `1` and `1.00`).
However, there is a one-to-one mapping between the abstract representation and the result of the primary conversion to
string using to-scientific-string on that abstract representation.
In other words, if one number has a different abstract representation to another, then the primary string conversion
will also be different.

#### Exponent

The _exponent_ is represented by a two’s complement 16-bit binary integer.

The **_adjusted exponent_** is the value of the decimal number _exponent_ when that number is expressed as though in
scientific notation with one digit (non-zero unless the coefficient is `0`) before any decimal point.
This is given by the value of the _exponent + (C<sub>length</sub> – 1)_, where _C<sub>length</sub>_ is the length of the
coefficient in decimal digits.
When a limit to the exponent _E<sub>limit</sub>_ applies, it must result in a balanced range of positive or negative
numbers, taking into account the magnitude of the coefficient.
To achieve this balanced range, the minimum and maximum values of the adjusted exponent (_E<sub>min</sub>_ and _E<sub>
max</sub>_ respectively) must have magnitudes which differ by no more than one, so _E<sub>min</sub>_ will be _–E<sub>
max</sub> ±1_.
[IEEE 754] further constrains this so that _E<sub>min</sub> = 1 – E<sub>max</sub>_.
Therefore, if the length of the coefficient is _C<sub>length</sub>_ digits, the exponent may take any of the values
_-E<sub>limit</sub> – (C<sub>length</sub> – 1) + 1_ through _E<sub>limit</sub> – (C<sub>length</sub> – 1)_.

#### Special values

[special value]: #special-values

In addition to the finite numbers, numbers must also be able to represent one of three named special values:

##### NaN

[`NaN`]: #nan

Special value called _"Not a Number"_ (`NaN`) to be returned as the result of certain _"invalid"_ operations, such as
`0 / 0`, `∞ × 0`, or `sqrt(−1)`.
In general, `NaN`s will be propagated: most operations involving a `NaN` will result in a `NaN`.
There are two kinds of `NaN`:

- `quiet NaN` – a value representing undefined results (_Not a Number_) which doesn't cause an [`Invalid operation`]
  condition.
- `signaling NaN` – a value representing undefined results (_Not a Number_) which will usually cause an Invalid
  operation condition if used in any operation defined in this specification (see [IEEE 754] §3.2 and §6.2).

Signaling `NaN` could be used by a runtime system to flag uninitialized variables, or extend the decimal numbers
with other special values without slowing down the computations with ordinary values, although such extensions aren't
common.

##### Infinity

[`Infinity`]: #infinity

[`+Infinity`]: #infinity

[`-Infinity`]: #infinity

[`±Infinity`]: #infinity

[`+∞`]: #infinity

[`-∞`]: #infinity

[`∞`]: #infinity

`±Infinity` – a value representing a number whose magnitude is infinitely large (see [IEEE 754] §3.2 and §6.1).

When a number has one of these special values, its coefficient and exponent are undefined.
All special values may have a sign, as for finite numbers.
The sign of an `Infinity` is significant (that is, it is possible to have both _positive_ and _negative_ _infinity_),
and the sign of a `NaN` has no meaning.

#### Signed zero

[Zero]: #signed-zero

[`±0`]: #signed-zero

[`+0`]: #signed-zero

[`-0`]: #signed-zero

Signed zero (`±0`) is zero with an associated sign.
In ordinary arithmetic, the number `0` does not have a sign, so that `−0`, `+0` and `0` are equivalent.
However, in computing, some number representations allow for the existence of two zeros, often denoted by `−0` (negative
zero) and `+0` (positive zero), regarded as equal by the numerical comparison operations but with possible different
behaviors in particular operations.

Real arithmetic with signed zeros can be considered a variant of the extended real number line such that `1/−0 = −∞` and
`1/+0 = +∞`.
Division is undefined only for `±0/±0` and `±∞/±∞`.

Negatively signed zero echoes the mathematical analysis concept of approaching `0` from below as a one-sided limit,
which may be denoted by `x → 0−`, `x → +0`. The notation `−0` may be used informally to denote a negative number that
has been rounded to zero. The concept of negative zero also has some theoretical applications in statistical mechanics
and other disciplines.

More about [Signed Zero](https://en.wikipedia.org/wiki/Signed_zero).

#### Normal numbers, subnormal numbers, and Underflow

[subnormal]: #normal-numbers-subnormal-numbers-and-underflow

In any context where exponents are bounded, most finite numbers are normal.
Non-zero finite numbers whose _adjusted exponents_ are greater than or equal to _E<sub>min</sub>_ are called normal
numbers.
Those non-zero numbers whose _adjusted exponents_ are less than _E<sub>min</sub>_ are called **_subnormal numbers_**.
Like other numbers, subnormal numbers are accepted as operands for all operations and may result from any operation.
If a result is subnormal, before any rounding, then the [`Subnormal`] exception condition is raised.

For a subnormal result, the minimum value of the exponent becomes _E<sub>min</sub> – (precision – 1)_, called _E<sub>
tiny</sub>_, where precision is the working precision, as described below.
The result will be rounded, if necessary, to ensure that the exponent is no smaller than _E<sub>tiny</sub>_.
If, during this rounding, the result becomes inexact, then the [`Underflow`] condition is raised.
A subnormal result doesn't necessarily raise Underflow, therefore, but is always indicated by the Subnormal
condition (even if, after rounding, its value is `0` or ten to the power of _E<sub>min</sub>_).

When a number "underflows" to zero during a calculation, its exponent will be _E<sub>tiny</sub>_.
The maximum value of the exponent is unaffected.

The minimum value of the exponent for subnormal numbers is the same as the minimum value of exponent, which can
arise during operations which don't result in subnormal numbers, which occurs in the case where _C<sub>length</sub> =
precision_.

### Memory layout

Under the hood any N-bit decimal type consists of an N-bit big unsigned integer (_coefficient_), paired with a 64-bit
_control block_ which consists of the following components:

- a signed 16-bit integer scaling factor (negotiated _exponent_),
- decimal number flags (include _sign_ and [special value] flags),
- signaling flags for [Exceptional conditions],
- decimal [Context] includes [rounding mode] and [signals traps],
- [Extra precision] digits.

<style>
.mermaid svg { 
  max-width: 100% !important; 
  height: auto;
  background-color: rgba(255,255,255,50%);
}
</style>
<pre class="mermaid">
    ---
    title: D256 decimal memory layout (bytes)
    config:
      theme: 'forest'
      packet:
        bitsPerRow: 40
        bitWidth: 18
    ---
    packet-beta
    0-31: "coefficient (U256)"
    32-39: "control block"
</pre>
<pre class="mermaid">
    ---
    title: Control block memory layout (bits)
    config:
        theme: 'forest'
        packet:
            bitsPerRow: 64
            bitWidth: 10
    ---
    packet-beta
    0-15: "exp"
    16-18: "flags"
    19-26: "signals"
    27-39: "context"
    40-63: "extra precision"
</pre>

<script type="module">
import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";
var doc_theme = localStorage.getItem("rustdoc-theme");
if (doc_theme === "dark" || doc_theme === "ayu") 
    mermaid.initialize({theme: "dark", rough: true});
</script>

#### Signaling flags and trap-enablers

[Exceptional conditions]: #signaling-flags-and-trap-enablers

The exceptional conditions, which may arise during performing arithmetic operations on decimal numbers, are grouped into
signals, which can be controlled individually.

Signaling flags are very similar to the [processor flag register](https://en.wikipedia.org/wiki/FLAGS_register) and
contain information about arithmetic errors such as operation overflow, division by `0`, or loss of precision
calculations.
Depending on the needs of the application, flags may be ignored, considered as informational, or cause `panic!`.

The signals are:

[Signals]: #signaling-flags-and-trap-enablers

|     **Signal**      | Description                                                                                        |
|:-------------------:|----------------------------------------------------------------------------------------------------|
|      `CLAMPED`      | The exponent of a result has been altered.                                                         |
| `DIVISION_BY_ZERO`  | Non-zero dividend is divided by zero.                                                              |
|      `INEXACT`      | Result is not exact (one or more non-zero coefficient digits were discarded during rounding).      |                                                                                    
| `INVALID_OPERATION` | Invalid operation. Result would be undefined or impossible.                                        |
|     `OVERFLOW`      | Indicates that exponent of the decimal result is too large to fit the target type.                 |
|      `ROUNDED`      | Result has been rounded (that is, some zero or non-zero coefficient digits were discarded).        |
|     `SUBNORMAL`     | Result is _subnormal_ (its adjusted exponent is less than _E<sub>min</sub>_), before any rounding. |
|     `UNDERFLOW`     | Result is both subnormal and inexact.                                                              |

For each of the signals, the corresponding flag in _signals block_ is set to `1` when the signal occurs.
For each of the signals, the corresponding [Context] _trap-enabler_ indicates which action is to be taken when the
signal occurs (see [IEEE 754] §7).
If `0`, a defined result is supplied, and execution continues (for example, an overflow is perhaps converted to a
positive or negative infinity). If `1`, then execution of the operation cause `panic!`.

##### Clamped

[`Clamped`]: #clamped

This occurs and signals clamped if the exponent of a result has been altered in order to fit the constraints of the
underlying `i16` integer type.
This may occur when the exponent of a zero result would be outside the bounds of a representation, or (in the [IEEE 754]
interchange formats) when a large normal number would have an encoded exponent that can't be represented.
In this latter case, the exponent is reduced to fit and the corresponding number of zero digits are appended to the
coefficient (“fold-down”).
The condition always occurs when a subnormal value rounds to zero.

```
use fastnum::{decimal::*, *};

let res = dec256!(1E-10) + dec256!(1E-100);

assert_eq!(res, dec256!(1E-10));
assert!(res.is_op_inexact());
assert!(res.is_op_rounded());
assert!(res.is_op_clamped());

```

##### Division by zero

[`Division by zero`]: #division-by-zero

This occurs and signals division-by-zero if division of a finite number by zero was attempted, and the dividend wasn't
zero.
The result of the operation is `±Infinity`, where sign is the exclusive or of the signs of the operands for divide, or
is `1` for an odd power of `-0`, for power.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = dec256!(1).with_ctx(ctx) / dec256!(0).with_ctx(ctx);

assert!(res.is_infinite());
assert!(res.is_op_div_by_zero());
```

##### Inexact

[`Inexact`]: #inexact

This occurs and signals inexact whenever the result of an operation is not exact (that is, it needed to be rounded, and
any discarded digits were non-zero), or if an overflow or underflow condition occurs.
The result in all cases is unchanged.

For example, `1/3 = 0.333333333333(3)`. The result of division is an infinite decimal fraction that can't
be stored in any of the existing types: performing the operation will cause overflow for any finite count of
decimal digits.
However, the result can be obtained if we make a deal and agree to the loss of precision.
In this case, the result will be the rounded value, accompanied by information that rounding has been performed and
the result may not be exact.

|         | Result            | Rounded (HalfUp) | Rounded (Down) | [`Inexact`] |
|---------|-------------------|------------------|----------------|:-----------:|
| `6 / 3` | 2                 | 2                | 2              |             |
| `1 / 3` | 0.333333333333(3) | 0.333333333333   | 0.333333333333 |     ⁉️      |
| `2 / 3` | 0.666666666666(6) | 0.666666666667   | 0.666666666666 |     ⁉️      |

For most practical purposes this is an acceptable trade-off.
However, we always leave the possibility to be sure that the result is strictly exact and there was no loss of
precision.

```
use fastnum::{decimal::*, *};

let res = dec128!(1) / dec128!(3);

assert_eq!(res, dec128!(0.333333333333333333333333333333333333333));
assert!(res.is_op_inexact());
```

##### Invalid operation

[`Invalid operation`]: #invalid-operation

This occurs and signals invalid-operation if:

- an operand to an operation is [`NaN`].
- an attempt is made to add [`+Infinity`] to [`-Infinity`] during an [addition] or [subtraction] operation.
- an attempt is made to multiply `0` by [`±Infinity`].
- an attempt is made to divide either [`+Infinity`] or [`-Infinity`] by either [`+Infinity`] or [`-Infinity`].
- the divisor for a [remainder] operation is zero.
- the dividend for a [remainder] operation is [`±Infinity`].
- either operand of the [quantize] operation is infinite.
- the operand of any [logarithm function] is less than zero.
- the operand of the [square-root] operation has a negative sign and a non-zero coefficient.
- both operands of the [power] operation are zero, or if the first operand is less than zero and the second
  operand doesn't have an integral value or is infinite.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = D128::INFINITY.with_ctx(ctx) - D128::INFINITY.with_ctx(ctx);

assert!(res.is_nan());
assert!(res.is_op_invalid());
```

##### Overflow

[`Overflow`]: #overflow

This occurs, and signals overflow if the _adjusted exponent_ of a result (from a conversion or from an operation that is
not an attempt to divide by zero), after rounding, would be greater than the largest value that can be handled by the
implementation (the value E<sub>max</sub>).

The result depends on the rounding mode:

- For [round-half-up] and [round-half-even] (and for [round-half-down] and [round-up]), the result of the operation is
  [`±Infinity`], where sign is the sign of the intermediate result.
- For [round-down] the result is the largest finite number that can be represented in the current precision, with the
  sign of the intermediate result.
- For [round-ceiling], the result is the same as for [round-down] if the sign of the intermediate result is `-`, or is
  [`Infinity`] otherwise.
- For [round-floor], the result is the same as for [round-down] if the sign of the intermediate result is `+`, or is
  [`-Infinity`] otherwise.

In all cases, [`Inexact`] and [`Rounded`] will also be raised.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = D128::MAX.with_ctx(ctx) * D128::MAX.with_ctx(ctx);

assert!(res.is_infinite());
assert!(res.is_op_overflow());
assert!(res.is_op_rounded());
assert!(res.is_op_inexact());
```

##### Rounded

[`Rounded`]: #rounded

This occurs and signals rounded whenever the result of an operation is rounded (that is, some zero or non-zero digits
were discarded from the coefficient), or if an overflow or underflow condition occurs.
The result in all cases is unchanged.
The rounded signal may be tested (or trapped) to determine if a given operation (or sequence of operations) caused a
loss of precision.

```
use fastnum::{decimal::*, *};

let res = D128::MAX * dec128!(1.0);

assert_eq!(res, D128::MAX);
assert!(res.is_op_rounded());
```

##### Subnormal

[`Subnormal`]: #subnormal

This occurs and signals subnormal whenever the result of a conversion or operation is subnormal (that is, its adjusted
exponent is less than _E<sub>min</sub>_, before any rounding).
The result in all cases is unchanged. The subnormal signal may be tested (or trapped) to determine if a given or
operation (or sequence of operations) yielded a subnormal result.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = dec128!(1E-30000).with_ctx(ctx) / dec128!(1E2768).with_ctx(ctx);

assert!(res.is_op_subnormal());
```

##### Underflow

[`Underflow`]: #underflow

This occurs and signals underflow if a result is inexact and the adjusted exponent of the result would be smaller (more
negative) than the smallest value that can be handled by the implementation (the value _E<sub>min</sub>_).
That is, the result is both inexact and subnormal.
The result after an underflow will be a subnormal number rounded, if necessary, so that its exponent is not less than
_E<sub>tiny</sub>_.
This may result in `0` with the sign of the intermediate result and an exponent of _E<sub>tiny</sub>_.

In all cases, [`Inexact`], [`Rounded`], and [`Subnormal`] will also be raised.

```
use fastnum::{decimal::*, *};

let ctx = Context::default().with_signal_traps(SignalsTraps::empty());
let res = dec128!(1e-32767).with_ctx(ctx) / D128::MAX.with_ctx(ctx);

assert!(res.is_op_underflow());
assert!(res.is_op_inexact());
assert!(res.is_op_rounded());
assert!(res.is_op_subnormal());

```

## Precision

Precision is an integral number of decimal digits.
It is limited by the maximum decimal digits that N-bit unsigned coefficient can hold.

## Arithmetic

The `fastnum` crate provides support for fast correctly rounded decimal floating-point arithmetic. It offers several
advantages over the native floating-point ([IEEE 754]) types:

### Exact precision

`fastnum` decimals provide an arithmetic that works in the same way as the arithmetic that people learn
at school. Decimal numbers can be represented exactly. In contrast, numbers like `1.1` and `2.2` do not have exact
representations in binary floating point. End users typically would not expect `1.1 + 2.2` to display as
`3.3000000000000003` as it does with binary floating point.

Floats:

```
let n: f64 = 1.1 + 2.2;
assert_eq!(n.to_string(), "3.3000000000000003");
```

Decimals:

```
use fastnum::udec256;

let n = udec256!(1.1) + udec256!(2.2);
assert_eq!(n.to_string(), "3.3");
```

The exactness carries over into arithmetic. In decimal floating point, `0.1 + 0.1 + 0.1 - 0.3` is exactly equal to zero.
In binary floating point, the result is `5.5511151231257827e-017`. While near to zero, the differences prevent reliable
equality testing and differences can accumulate. For this reason, decimal is preferred in accounting applications which
have strict equality invariants.

Floats:

```
let n: f64 = 0.1 + 0.1 + 0.1 - 0.3;
assert_eq!(n.to_string(), "0.00000000000000005551115123125783");
```

Decimals:

```
use fastnum::udec256;

let n = udec256!(0.1) + udec256!(0.1) + udec256!(0.1) - udec256!(0.3);
assert_eq!(n.to_string(), "0.0");
assert!(n.is_zero());
```

### Fixed precision

`fastnum` incorporates a notion of significant places so that `1.30 + 1.20` is `2.50`. The trailing zero is kept
to indicate significance. This is the customary presentation for monetary applications. For multiplication, the
“schoolbook” approach uses all the figures in the multiplicands. For instance, `1.3 * 1.2` gives `1.56` while
`1.30 * 1.20` gives `1.5600`.

Decimals:

```
use fastnum::udec256;

let n = udec256!(1.30) + udec256!(1.20);
assert_eq!(n, udec256!(2.50));

let n = udec256!(1.3) * udec256!(1.2);
assert_eq!(n, udec256!(1.56));

let n = udec256!(1.30) * udec256!(1.20);
assert_eq!(n, udec256!(1.5600));
```

More about decimal
arithmetic: [IBM’s General Decimal Arithmetic Specification](https://speleotrove.com/decimal/decarith.html).

### Arithmetic rules

[general rules]: #arithmetic-rules

The following general rules apply to all arithmetic operations except where stated below.

1. Every operation on finite numbers is carried out as though an exact mathematical result is computed, using integer
   arithmetic on the coefficient (or significant decimal digits) where possible.<br/><br/>

   If the coefficient of the theoretical exact result has no more than precision digits, then (unless there is an
   underflow or overflow) it is used for the result without change. Otherwise (it has more than precision digits) it is
   rounded (shortened) to exactly precision digits, using the current rounding algorithm, and the exponent is increased
   by the number of digits removed.<br/><br/>

   If the value of the adjusted exponent of the result is less than _E<sub>min</sub>_ (that is, the result is zero or
   subnormal), the calculated coefficient and exponent form the result, unless the value of the exponent is less than
   _E<sub>tiny</sub>_, in which case the exponent will be set to _E<sub>tiny</sub>_, the coefficient will be rounded (if
   necessary, and possibly to zero) to match the adjustment of the exponent, and the sign is unchanged.<br/><br/>

   If the result (before rounding) was non-zero and subnormal then the [`Subnormal`] exceptional condition is raised.
   If rounding of a subnormal result leads to an inexact result then the [`Underflow`] exceptional condition is also
   raised.<br/><br/>

   If the value of the adjusted exponent of a non-zero result is larger than _E<sub>max</sub>_, then an exceptional
   condition (overflow) results.
   In this case, the result is as defined under the [`Overflow`] exceptional condition, and may be infinite.
   It will have the same sign as the theoretical result.

2. Arithmetic using the special value [`±Infinity`] follows the usual rules, where [`-Infinity`] is less than every
   finite number and [`+Infinity`] is greater than every finite number.
   Under these rules, an infinite result is always exact.
   Certain uses of infinity raise an [`Invalid operation`] condition.

3. The result of any arithmetic operation that has an operand which is a `NaN` (a quiet `NaN` or a signaling `NaN`)
   is always `NaN`.
   The sign and any diagnostic information is copied from the first operand which is a signaling `NaN`, or if neither is
   signaling then from the first operand which is a `NaN`.
   Whenever a result is a `NaN`, the sign of the result depends only on the copied operand (the following rules don't
   apply).

4. The [`Invalid operation`] condition may be raised when an operand to an operation is invalid (for example, if it
   exceeds the bounds that an implementation can handle, or the operation is a logarithm and the operand is negative).

5. The sign of the multiplication or division result will be `-` only if the operands have different signs.

6. The sign of the addition or subtraction result will be 1 only if the result is less than zero, except for the
   special cases below where the result is a negative `0`.

7. A result which is a negative zero (`-0`) can occur only when:
    - a result is rounded to zero, and the value before rounding had a sign of `-`.
    - the operation is an addition of [`-0`] to [`+0`], or a subtraction of [`+0`] from [`-0`].
    - the operation is an addition of operands with opposite signs (or is a subtraction of operands with the same sign),
      the result has a coefficient of `0`, and the rounding is [round-floor].
    - the operation is a multiplication or division, and the result has a coefficient of `0` and the signs of the
      operands are different.
    - the operation is `power`, the first operand is [`-0`], and the second operand is positive, integral, and
      odd.
    - the operation is `power`, the first operand is [`-Infinity`], and the second operand is negative, integral, and
      odd.
    - the operation is `quantize` or a `round-to-integral`, the first operand is negative, and the magnitude of the
      result is zero.
      In either case the final exponent may be non-zero.
    - the operation is `square-root` and the operand is [`-0`].
    - the operation is one of the operations `max`, `max-magnitude`, `min`, `min-magnitude`, `next-plus`, `next-toward`,
      `reduce`, or is a copy operation.

#### Examples involving special values:

```
use fastnum::{*, decimal::*};

let ctx = Context::default().without_traps();

assert_eq!(D256::INFINITY + dec256!(1), D256::INFINITY);
assert!((D256::NAN.with_ctx(ctx) + dec256!(1).with_ctx(ctx)).is_nan());
assert!((D256::NAN.with_ctx(ctx) + D256::INFINITY.with_ctx(ctx)).is_nan());
assert_eq!(dec256!(1) - D256::INFINITY, D256::NEG_INFINITY);
assert_eq!(dec256!(-1) - D256::INFINITY, D256::NEG_INFINITY);
assert_eq!(dec256!(-0) - dec256!(+0), dec256!(-0));
assert_eq!(dec256!(-1) * dec256!(0), dec256!(-0));
assert_eq!(dec256!(1).with_ctx(ctx) / dec256!(0).with_ctx(ctx), D256::INFINITY);
assert_eq!(dec256!(1).with_ctx(ctx) / dec256!(-0).with_ctx(ctx), D256::NEG_INFINITY);
```

#### Notes:

1. Operands may have more than precision digits and aren't rounded before use.

2. The [Context] defines the rules for unwrapping the result of an arithmetic operation
   containing [Exceptional conditions].

3. If a result is rounded, remains finite, and is not subnormal, its coefficient will have exactly precision
   digits (except after the `quantize` or `round-to-integral` operations, as described below).
   That is, only unrounded or subnormal coefficients can have fewer than precision digits.

4. Trailing zeros aren't removed after operations.
   The reduce operation may be used to remove trailing zeros if desired.

### Arithmetic operations

All arithmetic operations over decimals are exact.

| Operation |   Rust operator   |                    Unsigned                    |                                             Signed                                             |
|-----------|:-----------------:|:----------------------------------------------:|:----------------------------------------------------------------------------------------------:|
| abs       |         ➖         |                       ➖                        | [`abs`](crate::decimal::Decimal::abs), [`unsigned_abs`](crate::decimal::Decimal::unsigned_abs) |
| add       | `a + b`, `a += b` | [`add`](crate::decimal::UnsignedDecimal::add)  |                             [`add`](crate::decimal::Decimal::add)                              |
| subtract  | `a - b`, `a -= b` | [`sub`](crate::decimal::UnsignedDecimal::sub)  |                             [`sub`](crate::decimal::Decimal::sub)                              |
| multiply  | `a * b`, `a *= b` | [`mul`](crate::decimal::UnsignedDecimal::mul)  |                             [`mul`](crate::decimal::Decimal::mul)                              |
| divide    | `a / b`, `a /= b` | [`div`](crate::decimal::UnsignedDecimal::div)  |                             [`div`](crate::decimal::Decimal::div)                              |
| remainder | `a % b`, `a %= b` | [`rem`](crate::decimal::UnsignedDecimal::rem)  |                             [`rem`](crate::decimal::Decimal::rem)                              |
| negation  |       `-a`        | [`neg`](crate::decimal::UnsignedDecimal::neg)  |                             [`neg`](crate::decimal::Decimal::neg)                              |
| powi      |         ➖         | [`pow`](crate::decimal::UnsignedDecimal::powi) |                            [`powi`](crate::decimal::Decimal::powi)                             |

#### Abs

The _absolute value_ or _modulus_ of a decimal number, denoted, is the non-negative value of without regard to its sign.

The [`unsigned_abs`](crate::decimal::Decimal::unsigned_abs) method returns [
`UnsignedDecimal`](crate::decimal::UnsignedDecimal).

##### Examples:

```
use fastnum::*;

assert_eq!(dec256!(1.3).abs(), dec256!(1.3));
assert_eq!(dec256!(-1.3).abs(), dec256!(1.3));
assert_eq!(dec256!(-1.3).unsigned_abs(), udec256!(1.3));
```

#### Addition and subtraction

[addition]: #addition-and-subtraction

[subtraction]: #addition-and-subtraction

[`add(self, other)`](crate::decimal::Decimal::add) | [`sub(self, other)`](crate::decimal::Decimal::sub)

If either operand is a [special value], then the [general rules] apply.
Otherwise, the operands are added (after inverting the sign used for the second operand if the operation is a
subtraction), as follows:

1. The coefficient of the result is computed by adding or subtracting the aligned coefficients of the two operands. The
   aligned coefficients are computed by comparing the exponents of the operands:
    1. If they have the same exponent, the aligned coefficients are the same as the original coefficients.
    2. Otherwise, the aligned coefficient of the number with the larger exponent is its original coefficient multiplied
       by _10<sup>n</sup>_, where _n_ is the absolute difference between the exponents, and the aligned coefficient of
       the other operand is the same as its original coefficient.
    3. If the signs of the operands differ, then the smaller aligned coefficient is subtracted from the larger;
       otherwise they're added.
2. The exponent of the result is the minimum of the exponents of the two operands.
3. The sign of the result is determined as follows:
    1. If the result is non-zero, then the sign of the result is the sign of the operand having the larger absolute
       value.
    2. Otherwise, the sign of a zero result is `0` unless either both operands were negative or the signs of the
       operands were different and the rounding is [round-floor].

##### Examples:

```
use fastnum::{udec256, dec256};

assert_eq!(udec256!(12) + udec256!(7.00), udec256!(19.00));
assert_eq!(udec256!(1E+2) + udec256!(1E+4), udec256!(1.01E+4));
assert_eq!(udec256!(1.3) - udec256!(1.07), udec256!(0.23));
assert_eq!(udec256!(1.3) - udec256!(1.30), udec256!(0.00));
assert_eq!(dec256!(1.3) - dec256!(2.07), dec256!(-0.77));
```

#### Division

[division]: #division

[`div(dividend, divisor)`](crate::decimal::Decimal::div)

If either operand is a [special value], then the [general rules] apply.
Otherwise, if the divisor is zero then the [`Division by zero`] condition is raised and the result is an [`Infinity`]
with a sign which is the exclusive or of the signs of the operands.

Otherwise, a _long division_ is performed, as follows:

1. An integer variable, _adjust_, is initialized to `0`.
2. If the dividend is non-zero, the _coefficient_ of the result is computed as follows (using working copies of the
   operand coefficients, as necessary):
    1. The operand coefficients are adjusted so that the coefficient of the dividend is greater than or equal to the
       coefficient of the divisor and is also less than ten times the coefficient of the divisor, thus:
        1. While the coefficient of the dividend is less than the coefficient of the divisor, it is multiplied by `10`
           and _adjust_ is incremented by `1`.
        2. While the coefficient of the dividend is greater than or equal to ten times the coefficient of the divisor
           the coefficient of the divisor is multiplied by `10` and _adjust_ is decremented by `1`.
    2. The result coefficient is initialized to `0`.
    3. The following steps are then repeated until the division is complete:
        1. While the coefficient of the divisor is smaller than or equal to the coefficient of the dividend the former
           is subtracted from the latter and the coefficient of the result is incremented by `1`.
        2. If the coefficient of the dividend is now `0` and adjust is greater than or equal to `0`, or if the
           coefficient of the result has precision digits, the division is complete.
           Otherwise, the coefficients of the result and the dividend are multiplied by `10` and _adjust_ is incremented
           by `1`.
    4. Any remainder (the final coefficient of the dividend) is recorded and taken into account for rounding.

   Otherwise (the dividend is zero), the coefficient of the result is zero and _adjust_ is unchanged (is `0`).
3. The _exponent_ of the result is computed by subtracting the sum of the original exponent of the divisor and the value
   of _adjust_ at the end of the coefficient calculation from the original exponent of the dividend.
4. The sign of the result is the exclusive or of the signs of the operands.

##### Examples:

```
use fastnum::*;

assert_eq!(dec128!(1) / dec128!(3), dec128!(0.333333333333333333333333333333333333333));
assert_eq!(dec128!(2) / dec128!(3), dec128!(0.66666666666666666666666666666666666667));
assert_eq!(dec128!(5) / dec128!(2), dec128!(2.5));
assert_eq!(dec128!(1) / dec128!(10), dec128!(0.1));
assert_eq!(dec128!(12) / dec128!(12), dec128!(1));
assert_eq!(dec128!(8.00) / dec128!(2), dec128!(4.00));
assert_eq!(dec128!(2.400) / dec128!(2.0), dec128!(1.20));
assert_eq!(dec128!(1000) / dec128!(100), dec128!(10));
assert_eq!(dec128!(1000) / dec128!(1), dec128!(1000));
assert_eq!(dec128!(2.40E+6) / dec128!(2), dec128!(1.20E+6));
```

Note that the results as described above can alternatively be expressed as follows:
The ideal (simplest) exponent for the result of a division is the exponent of the dividend less the exponent of the
divisor.

After the division, if the result is exact, then the coefficient and exponent giving the correct value and with the
exponent closest to the ideal exponent is returned.
If the result is inexact, the coefficient will have exactly precision digits (unless the result is subnormal), and the
exponent will be set appropriately.

#### Multiplication

[multiplication]: #multiplication

[`mul(self, other)`](crate::decimal::Decimal::mul)

If either operand is a [special value], then the [general rules] apply.
Otherwise, the operands are multiplied together (_long multiplication_), resulting in a number which may be as long as
the sum of the lengths of the two operands, as follows:

1. The coefficient of the result, before rounding, is computed by multiplying together the coefficients of the operands.
2. The exponent of the result, before rounding, is the sum of the exponents of the two operands.
3. The sign of the result is the exclusive or of the signs of the operands.

The result is then rounded to precision digits if necessary, counting from the most significant digit of the result.

##### Examples:

```
use fastnum::*;

assert_eq!(dec128!(1.20) * dec128!(3), dec128!(3.60));
assert_eq!(dec128!(7) * dec128!(3), dec128!(21));
assert_eq!(dec128!(0.9) * dec128!(0.8), dec128!(0.72));
assert_eq!(dec128!(0.9) * dec128!(-0), dec128!(-0.0));
assert_eq!(dec128!(654321) * dec128!(654321), dec128!(4.28135971041E+11));
```

#### Fused multiply-add

[fused multiply-add]: #fused-multiply-add

[`mul_add(self, a, b)`](crate::decimal::Decimal::mul_add)

Operation takes three operands.
The first two are multiplied together, using [multiplication],
with sufficient precision and exponent range that the result is exact and unrounded.
No flags are set by the multiplication unless one of the first two operands is a signaling `NaN`, or one is a zero and
the other is an [`Infinity`].
Unless the multiplication failed, the third operand is then added to the result of that multiplication, using add, under
the current context.

In other words, `mul_add(x, y, z)` delivers a result which is `(x × y) + z` with only the one, final, rounding.

##### Examples:

```
use fastnum::*;

assert_eq!(dec128!(1.0).mul_add(dec128!(2), dec128!(0.5)), dec128!(2.5));
```

#### Remainder

[remainder]: #remainder

[`rem(self, other)`](crate::decimal::Decimal::rem) | [`rem(self, other)`](crate::decimal::UnsignedDecimal::rem)

Remainder takes two operands; it returns the remainder from integer division.
If either operand is a [special value], then the [general rules] apply.

Otherwise, the result is the residue of the dividend after the operation of calculating integer division, rounded to
precision digits if necessary.
The sign of the result, if non-zero, is the same as that of the original dividend.

##### Examples:

```
use fastnum::*;

assert_eq!(dec128!(2.1) % dec128!(3), dec128!(2.1));
assert_eq!(dec128!(10) % dec128!(3), dec128!(1));
assert_eq!(dec128!(10) % dec128!(6), dec128!(4));
assert_eq!(dec128!(-10) % dec128!(3), dec128!(-1));
assert_eq!(dec128!(10.2) % dec128!(1), dec128!(0.2));
assert_eq!(dec128!(10) % dec128!(0.3), dec128!(0.1));
assert_eq!(dec128!(3.6) % dec128!(1.3), dec128!(1.0));
```

Notes:

1. The divide-integer and remainder operations are defined so that they may be calculated as a by-product of the
   standard division operation (described above).
   The division process is ended as soon as the integer result is available; the residue of the dividend is the
   remainder.
2. The sign of the result will always be a sign of the dividend.
3. The remainder operation differs from the remainder operation defined in [IEEE 754] (the remainder-near operator), in
   that it gives the same results for numbers, whose values are equal to integers as would the usual remainder operator
   on integers.
   For example, the result of the operation remainder(`10`, `6`) as defined here is `4`, and remainder(`10.0`, `6`)
   would give `4.0` (as would remainder(`10`, `6.0`) or remainder(`10.0`, `6.0`)).
   The [IEEE 754] remainder operation would, however, give the result `-2` because its integer division step chooses the
   closest integer, not the one nearer zero.

#### Power

[power]: #power

[`powi(self, n)`](crate::decimal::Decimal::powi) | [`pow(self, n)`](crate::decimal::Decimal::pow)

If either operand is a [special value], then the [general rules] apply.

The following rules apply:

- If both operands are _zero_, or if the first operand is less than _zero_ and the second operand doesn't have an
  integral value or is _infinite_, an ['Invalid operation'] condition is raised, the result is [`NaN`], and the
  following rules don't apply.
- If the first operand is _infinite_, the result will be exact and will be
    - [`Infinity`] if the second operand is _positive_,
    - `1` if the second operand is a _zero_, and
    - `0` if the second operand is _negative_.
- If the first operand is a _zero_, the result will be exact and will be
    - [`Infinity`] if the second is negative or
    - `0` if the second is positive.
- If the second operand is a _zero_, the result will be `1` and exact.
- In cases not covered above, the result will be inexact unless the second operator has an integral value and the result
  is finite and can be expressed exactly within precision digits.
  In this latter case, if the result is unrounded, then its exponent will be that which would result if
  the operation were calculated by repeated multiplication (if the second operand is negative then the reciprocal of the
  first operand is used, with the absolute value of the second operand determining the multiplications).
- Inexact finite results should be correctly rounded but may be up to 1 ulp (unit in last place) in error.
- The sign of the result will be `-` only if the second operand has an integral value and is odd (and is not _infinite_)
  and also the sign of the first operand is `-`.
  In all other cases, the sign of the result will be `0`.

##### Notes:

1. When the result is inexact, the cost of power at a given precision is likely to be at least twice as expensive as the
   exp function (see notes under that function).
2. An infinite result is always exact, as described in the general rules.
3. [`powi()`](crate::decimal::Decimal::powi) is simpler power operation which only required support for integer powers.
4. It can be argued that the special cases where one operand is zero and the other is
   infinite (such as _power(`0`, [`Infinity`])_ and _power([`Infinity`], `0`)_) should return a [`NaN`],
   whereas the specification above leads to results of `0` and `1` respectively for the two examples.
   If [`NaN`] results are desired instead, then these special cases should be tested for before calling the
   power operation.

##### Examples:

```
use fastnum::*;

assert_eq!(dec256!(2).powi(3), dec256!(8));
assert_eq!(dec256!(-2).powi(3), dec256!(-8));
assert_eq!(dec256!(9).powi(2), dec256!(81));
assert_eq!(dec256!(1).powi(-2), dec256!(1));
assert_eq!(dec256!(10).powi(20), dec256!(1e20));
assert_eq!(dec256!(4).powi(-2), dec256!(0.0625));
assert_eq!(dec256!(2).powi(-3), dec256!(0.125));
assert_eq!(D256::INFINITY.powi(-1), dec256!(0));
assert_eq!(D256::INFINITY.powi(0), dec256!(1));
assert_eq!(D256::INFINITY.powi(1), D256::INFINITY);
assert_eq!(D256::NEG_INFINITY.powi(-1), dec256!(-0));
assert_eq!(D256::NEG_INFINITY.powi(0), dec256!(1));
assert_eq!(D256::NEG_INFINITY.powi(1), D256::NEG_INFINITY);
assert_eq!(D256::NEG_INFINITY.powi(2), D256::INFINITY);
```

#### Square root

[square-root]: #square-root

[`sqrt(self)`](crate::decimal::Decimal::sqrt)

If the operand is a [special value] then the [general rules] apply.
Otherwise, the ideal exponent of the result is defined to be half the exponent of the operand (rounded to an integer,
towards [`-Infinity`], if necessary) and then:

* If the operand is less than zero, an ['Invalid operation'] condition is raised.
* If the operand is greater than zero, the result is the square root of the operand.
* Otherwise (the operand is equal to zero), the result will be the zero with the same sign as the operand and with the
  ideal exponent.

##### Examples:

```
use fastnum::*;

assert_eq!(dec128!(0).sqrt(), dec128!(0));
assert_eq!(dec128!(-0).sqrt(), dec128!(-0));
assert_eq!(dec128!(0.39).sqrt(), dec128!(0.62449979983983982058468931209397944611));
assert_eq!(dec128!(4).sqrt(), dec128!(2));
assert_eq!(dec128!(100).sqrt(), dec128!(10));
assert_eq!(dec128!(1).sqrt(), dec128!(1));
assert_eq!(dec128!(1.0).sqrt(), dec128!(1.0));
assert_eq!(dec128!(1.00).sqrt(), dec128!(1.0));
assert_eq!(dec128!(7).sqrt(), dec128!(2.64575131106459059050161575363926042571));
assert_eq!(dec128!(10).sqrt(), dec128!(3.16227766016837933199889354443271853372));
```

##### Notes:

A negative zero is allowed as an operand as per [IEEE 754] §5.4.1.
Square-root can also be calculated by using the power operation (with a second operand of 0.5).

#### N-th roots

[n-th roots]: #n-th-roots

[`nth_root(self, n)`](crate::decimal::Decimal::nth_root)

If the operand is a [special value], then the [general rules] apply.
Otherwise, the ideal exponent of the result is defined to be half the exponent of the operand (rounded to an integer,
towards [`-Infinity`], if necessary) and then:

* If the operand is less than zero and `n` is even, an ['Invalid operation'] condition is raised.
* If the operand is equal to zero, the result will be the zero with the same sign as the operand and with the ideal
  exponent.
* Otherwise, the result is the N-th root of the operand.

##### Examples:

```
use fastnum::*;

assert_eq!(dec128!(16).nth_root(4), dec128!(2));
```

##### Notes:

N-th root can also be calculated by using the power operation (with a second operand of `1/n`).

#### Exponential function

[exponential function]: #exponential-function

[`exp(self)`](crate::decimal::Decimal::exp)

If the operand is a [special value] then the [general rules] apply.
Otherwise,
the result is [`e`](https://en.wikipedia.org/wiki/E_(mathematical_constant)) raised to the power of the operand,
with the following cases:

* If the operand is [`-Infinity`], the result is `0` and exact.
* If the operand is a zero, the result is `1` and exact.
* If the operand is [`Infinity`], the result is [`Infinity`] and exact. Otherwise, the result is inexact and will be
  rounded using the context rounding mode.
* The coefficient will have exact precision digits (unless the result is subnormal).

##### Examples:

```
use fastnum::*;

assert_eq!(D128::NEG_INFINITY.exp(), dec128!(0));
assert!(D128::INFINITY.exp().is_infinite());
assert_eq!(dec128!(0).exp(), dec128!(1));
assert_eq!(dec128!(1).exp(), D128::E);
assert_eq!(dec128!(-1).exp(), dec128!(0.36787944117144232159552377016146086745));
assert_eq!(D128::LN_2.exp().round(20), dec128!(2));
```

##### Notes:

The standard Taylor series expansion method is used for calculation _e<sup>x</sup>_.

#### Binary exponential function

[binary exponential function]: #binary-exponential-function

[`exp2(self)`](crate::decimal::Decimal::exp2)

If the operand is a [special value] then the [general rules] apply.
Otherwise, the result is `2` raised to the power of the operand, with the following cases:

* If the operand is [`-Infinity`], the result is `0` and exact.
* If the operand is a zero, the result is `1` and exact.
* If the operand is [`Infinity`], the result is [`Infinity`] and exact.
* Otherwise, the result is inexact and will be rounded using the context rounding mode.
* The coefficient will have exactly precision digits (unless the result is subnormal).

##### Examples:

```
use fastnum::*;

assert_eq!(D128::NEG_INFINITY.exp2(), dec128!(0));
assert!(D128::INFINITY.exp2().is_infinite());
assert_eq!(dec128!(0).exp2(), dec128!(1));
assert_eq!(dec128!(1).exp2(), dec128!(2));
assert_eq!(dec128!(2).exp2(), dec128!(4));
```

##### Notes:

The standard Taylor series expansion method is used for calculation _2<sup>x</sup>_.

#### Logarithm function

[logarithm function]: #logarithm-function

|                              Base                              |                      Method                       |                                                 Associated constants                                                 |
|:--------------------------------------------------------------:|:-------------------------------------------------:|:--------------------------------------------------------------------------------------------------------------------:|
| [`e`](https://en.wikipedia.org/wiki/E_(mathematical_constant)) |     [`ln(self)`](crate::decimal::Decimal::ln)     |                 ['ln(2)'](crate::decimal::Decimal::LN_2), ['ln(10)'](crate::decimal::Decimal::LN_10)                 |
|                              `2`                               |   [`log2(self)`](crate::decimal::Decimal::log2)   |  ['log<sub>2</sub>(e)'](crate::decimal::Decimal::LOG2_E), ['log<sub>2</sub>(10)'](crate::decimal::Decimal::LOG2_10)  |
|                              `10`                              |  [`log10(self)`](crate::decimal::Decimal::log10)  | ['log<sub>10</sub>(e)'](crate::decimal::Decimal::LOG10_E), ['log<sub>10</sub>(2)'](crate::decimal::Decimal::LOG10_2) |
|                             _base_                             | [`log(self, base)`](crate::decimal::Decimal::log) |                                                                                                                      |

If the operand is a [special value] then the [general rules] apply.
Otherwise, the operand must be a zero or positive, and the result is the logarithm base _base_ of the operand, with
the following cases:

* If the operand is a zero, the result is [`-Infinity`] and exact.
* If the operand is [`+Infinity`], the result is [`+Infinity`] and exact.
* If the operand equals one, the result is `0` and exact.
* Otherwise, the result is inexact and will be correctly rounded using the [Context] rounding setting.

##### Examples:

```
use fastnum::*;

assert_eq!(dec256!(2).ln(), D256::LN_2);
assert_eq!(dec256!(10).ln(), D256::LN_10);

assert_eq!(dec256!(100).log10(), D256::TWO);
assert_eq!(dec256!(512).log2(), dec256!(9));

```

#### Trigonometric functions

[trigonometric functions]: #trigonometric-functions

##### Base trigonometric functions

[_sin(x)_]: https://en.wikipedia.org/wiki/Sine_and_cosine

[`sin(self)`]: crate::decimal::Decimal::sin

[_cos(x)_]: https://en.wikipedia.org/wiki/Sine_and_cosine

[`cos(self)`]: crate::decimal::Decimal::cos

[_tan(x)_]: https://en.wikipedia.org/wiki/Trigonometric_functions

[`tan(self)`]: crate::decimal::Decimal::tan

|    _ƒ_     |    Method     |    Domain	    | Set of principal values |
|:----------:|:-------------:|:-------------:|:-----------------------:|
| [_sin(x)_] | [`sin(self)`] | _-∞ < x < +∞_ |      _-1 ≤ x ≤ 1_       |
| [_cos(x)_] | [`cos(self)`] | _-∞ < x < +∞_ |      _-1 ≤ x ≤ 1_       |
| [_tan(x)_] | [`tan(self)`] | _-∞ < x < +∞_ |      _-∞ < x < +∞_      |

##### Inverse trigonometric functions

[_asin(x)_]: https://en.wikipedia.org/wiki/Inverse_trigonometric_functions

[`asin(self)`]: crate::decimal::Decimal::asin

[_acos(x)_]: https://en.wikipedia.org/wiki/Inverse_trigonometric_functions

[`acos(self)`]: crate::decimal::Decimal::acos

[_atan(x)_]: https://en.wikipedia.org/wiki/Inverse_trigonometric_functions

[`atan(self)`]: crate::decimal::Decimal::atan

|     _ƒ_     |     Method     |    Domain	    | Set of principal values |
|:-----------:|:--------------:|:-------------:|:-----------------------:|
| [_asin(x)_] | [`asin(self)`] | _-1 ≤ x ≤ 1_  |    _-π/2 ≤ x ≤ π/2_     |
| [_acos(x)_] | [`acos(self)`] | _-1 ≤ x ≤ 1_  |       _0 ≤ x ≤ π_       |
| [_atan(x)_] | [`atan(self)`] | _-∞ < x < +∞_ |    _-π/2 ≤ x ≤ π/2_     |

##### Hyperbolic functions

[_sinh(x)_]: https://en.wikipedia.org/wiki/Hyperbolic_functions

[`sinh(self)`]: crate::decimal::Decimal::sinh

[_cosh(x)_]: https://en.wikipedia.org/wiki/Hyperbolic_functions

[`cosh(self)`]: crate::decimal::Decimal::cosh

[_tanh(x)_]: https://en.wikipedia.org/wiki/Hyperbolic_functions

[`tanh(self)`]: crate::decimal::Decimal::tanh

|     _ƒ_     |     Method     |    Domain	    | Set of principal values |
|:-----------:|:--------------:|:-------------:|:-----------------------:|
| [_sinh(x)_] | [`sinh(self)`] | _-∞ < x < +∞_ |      _-∞ < x < +∞_      |
| [_cosh(x)_] | [`cosh(self)`] | _-∞ < x < +∞_ |      _1 ≤ x < +∞_       |
| [_tanh(x)_] | [`tanh(self)`] | _-∞ < x < +∞_ |      _-1 < x < 1_       |

##### Inverse hyperbolic functions

[_asinh(x)_]: https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions

[`asinh(self)`]: crate::decimal::Decimal::asinh

[_acosh(x)_]: https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions

[`acosh(self)`]: crate::decimal::Decimal::acosh

[_atanh(x)_]: https://en.wikipedia.org/wiki/Inverse_hyperbolic_functions

[`atanh(self)`]: crate::decimal::Decimal::atanh

|     _ƒ_      |     Method      |    Domain	    | Set of principal values |
|:------------:|:---------------:|:-------------:|:-----------------------:|
| [_asinh(x)_] | [`asinh(self)`] | _-∞ < x < +∞_ |      _-∞ < x < +∞_      |
| [_acosh(x)_] | [`acosh(self)`] | _1 ≤ x < +∞_  |      _-∞ < x < +∞_      |
| [_atanh(x)_] | [`atanh(self)`] | _-1 < x < 1_  |      _-∞ < x < +∞_      |

## Compare and ordering

The result of any compare operation is always exact and unrounded.

|       | Rust operator |                     Unsigned                      |                  Signed                   |
|-------|:-------------:|:-------------------------------------------------:|:-----------------------------------------:|
| eq    |     `==`      |    [`eq`](crate::decimal::UnsignedDecimal::eq)    |    [`eq`](crate::decimal::Decimal::eq)    |
| ne    |     `!=`      |    [`ne`](crate::decimal::UnsignedDecimal::ne)    |    [`ne`](crate::decimal::Decimal::ne)    |
| lt    |      `<`      |    [`lt`](crate::decimal::UnsignedDecimal::lt)    |    [`lt`](crate::decimal::Decimal::lt)    |
| le    |     `<=`      |    [`le`](crate::decimal::UnsignedDecimal::le)    |    [`le`](crate::decimal::Decimal::le)    |
| gt    |      `>`      |    [`gt`](crate::decimal::UnsignedDecimal::gt)    |    [`gt`](crate::decimal::Decimal::gt)    |
| ge    |     `>=`      |    [`ge`](crate::decimal::UnsignedDecimal::ge)    |    [`ge`](crate::decimal::Decimal::ge)    |
| cmp   |       ➖       |   [`cmp`](crate::decimal::UnsignedDecimal::cmp)   |   [`cmp`](crate::decimal::Decimal::cmp)   |
| max   |       ➖       |   [`max`](crate::decimal::UnsignedDecimal::max)   |   [`max`](crate::decimal::Decimal::max)   |
| min   |       ➖       |   [`min`](crate::decimal::UnsignedDecimal::min)   |   [`min`](crate::decimal::Decimal::min)   |
| clamp |       ➖       | [`clamp`](crate::decimal::UnsignedDecimal::clamp) | [`clamp`](crate::decimal::Decimal::clamp) |

### Total-ordering predicate

The [IEEE 754] standard provides a _Total-ordering predicate_, which defines a total ordering on canonical members of
the supported arithmetic format.
The predicate agrees with the comparison predicates (see section _§ Comparison predicates_) when one decimal number is
less than the other.

The main differences are:

- [`NaN`] is sortable.
  [`NaN`] is treated as if it had a larger absolute value than [`Infinity`] (or any other decimal numbers).<br/>
  _(-[`NaN`] < [`-Infinity`] < ... < [`+Infinity`] < +[`NaN`])_
- Negative zero is treated as smaller than positive zero.
- If both sides of the comparison refer to the same decimal datum, the one with the lesser exponent is treated as
  having a lesser absolute value.

### Examples

```
use fastnum::*;

assert!(dec256!(0.2) == dec256!(0.2));
assert!(dec256!(0.2) > dec256!(0.1));
assert!(dec256!(0.1) < dec256!(0.3));

assert!(D256::MAX < D256::INFINITY);
assert!(D256::INFINITY < D256::NAN);

assert!(D256::NAN != D256::NAN);
```

## Rust operators overloads

Common numerical operations (such as addition operator `+`, addition assignment operator `+=`, division operator
`/`, [etc...](https://doc.rust-lang.org/book/appendix-02-operators.html)) are overloaded for `fastnum` decimals, so we
can treat them the same way we treat other numbers.

```
use fastnum::*;

let a = udec256!(3.5);
let b = udec256!(2.5);
let c = a + b;

assert_eq!(c, udec256!(6));
```

Unfortunately, the current version of Rust doesn't support const traits, so this example fails to compile:

```compile_fail
use fastnum::*;

const A: UD256 = udec256!(3.5);
const B: UD256 = udec256!(2.5);
const C: UD256 = A + B;
```

In constant calculations and static contexts, until the [`feature`](https://github.com/rust-lang/rust/issues/67792) is
stabilized, the following const methods should be used:

```
use fastnum::*;

const A: UD256 = udec256!(3.5);
const B: UD256 = udec256!(2.5);
const C: UD256 = A.add(B);

assert_eq!(C, udec256!(6));
```

## Decimal context

[Context]: #decimal-context

[signals traps]: #decimal-context

**_Decimal context_** represents the user-selectable parameters and rules which govern the results of arithmetic
operations (for example, the rounding mode when rounding occurs).

The context is defined by the following parameters:

- `rounding_mode`: a named value which indicates the algorithm to be used when rounding is necessary, see more
  about [rounding mode](#rounding-mode);
- `signal_traps`: for each of the signals, the corresponding _trap-enabler_ indicates which action is to be taken when
  the signal occurs (see [IEEE 754] §7).
  See more about [Signals].

### Default context

[Default Context]: #default-context

|     **Signal**      | Trap-enabler |
|:-------------------:|:------------:|
|      `CLAMPED`      |              |
| `DIVISION_BY_ZERO`  |      ⚠️      |
|      `INEXACT`      |              |                                                                                    
| `INVALID_OPERATION` |      ⚠️      |
|     `OVERFLOW`      |      ⚠️      |
|      `ROUNDED`      |              |
|     `SUBNORMAL`     |              |
|     `UNDERFLOW`     |              |

## Reduce

_Normalize_ (or _reduce_) takes one operand and reduces a number to its shortest (_coefficient_) form.
If the final result is finite, it is reduced to its simplest form, with all trailing zeros removed and its sign
preserved.
That is, while the coefficient is non-zero and a multiple of ten the coefficient is divided by ten and the exponent is
incremented by `1`.
Otherwise (the coefficient is zero) the exponent is set to `0`.
In all cases the sign is unchanged.

### Examples

```
use fastnum::{*, decimal::*};

let a = dec256!(-1234500);
assert_eq!(a.digits(), u256!(1234500));
assert_eq!(a.fractional_digits_count(), 0);

let b = a.reduce();
assert_eq!(b.digits(), u256!(12345));
assert_eq!(b.fractional_digits_count(), -2);
```

## Rescale

[rescale]: #rescale

[`rescale(self, new_scale)`](crate::decimal::Decimal::rescale)

If `self` is a [special value] then the [general rules] apply, and an [`Invalid operation`] condition is raised
and the result is [`NaN`].
Otherwise, `rescale` quantize the `self` decimal number specified to the power of ten of the quantum (`new_scale`).

The coefficient:

- may be rounded using the [Context] rounding setting (if the exponent is being increased),
- multiplied by a positive power of ten (if the exponent is being decreased),
- or is unchanged (if the exponent is already equal to the given scale factor).

If the length of the coefficient after the quantize operation overflows max value, then an [`Clamped`] condition is
raised and the result will be saturated.

### Examples

```                                                                                       
use fastnum::{*, decimal::*};     

let ctx = Context::default().without_traps();                                                        
                                                                                          
assert_eq!(dec256!(2.17).rescale(3), dec256!(2.170));                       
assert_eq!(dec256!(2.17).rescale(2), dec256!(2.17));                         
assert_eq!(dec256!(2.17).rescale(1), dec256!(2.2));                           
assert_eq!(dec256!(2.17).rescale(0), dec256!(2));                            
assert_eq!(dec256!(2.17).rescale(-1), dec256!(0));                            
                                                                                          
assert!(D256::NEG_INFINITY.with_ctx(ctx).rescale(2).is_nan());              
assert!(D256::NAN.with_ctx(ctx).rescale(1).is_nan());                                    
```

## Quantize

[quantize]: #quantize

[`quantize(self, other)`](crate::decimal::Decimal::quantize)

Returns a value equal to `self` (rounded), having the exponent of `other`.                
The `quantize` semantics specifies the desired quantum by example.                        
The sign and coefficient of the `other` are ignored.

If either operand is a [special value], then the [general rules] apply,                    
except that:

- if either operand is infinite and the other is finite, an [`Invalid operation`] condition is raised and the result
  is [`NaN`];
- or if both are infinite, then the result is the first operand.

Otherwise (both operands are finite), `quantize` returns the number which is equal in value (except for any rounding)
and sign to the first operand and which has an exponent set to be equal to the exponent of the second operand.

The coefficient:

- may be rounded using the [Context] rounding setting (if the exponent is being increased),
- multiplied by a positive power of ten (if the exponent is being decreased),
- or is unchanged (if the exponent is already equal to the given scale factor).

### Examples

```                                                                                       
use fastnum::{*, decimal::*};    

let ctx = Context::default().without_traps();                                                         
                                                                                          
assert_eq!(dec256!(2.17).quantize(dec256!(0.001)), dec256!(2.170));                       
assert_eq!(dec256!(2.17).quantize(dec256!(0.01)), dec256!(2.17));                         
assert_eq!(dec256!(2.17).quantize(dec256!(0.1)), dec256!(2.2));                           
assert_eq!(dec256!(2.17).quantize(dec256!(1e+0)), dec256!(2));                            
assert_eq!(dec256!(2.17).quantize(dec256!(1e+1)), dec256!(0));                            
                                                                                          
assert_eq!(D256::NEG_INFINITY.quantize(D256::INFINITY), D256::NEG_INFINITY);              
                                                                                          
assert!(dec256!(2).with_ctx(ctx).quantize(D256::INFINITY).is_nan());                                    
                                                                                          
assert_eq!(dec256!(-0.1).quantize(dec256!(1)), dec256!(-0));                              
assert_eq!(dec256!(-0).quantize(dec256!(1e+5)), dec256!(-0E+5));                          
                                                                                          
assert!(dec128!(0.34028).with_ctx(ctx).quantize(dec128!(1e-32765)).is_nan());
assert!(dec128!(-0.34028).with_ctx(ctx).quantize(dec128!(1e-32765)).is_nan());                    
                                                                                          
assert_eq!(dec256!(217).quantize(dec256!(1e-1)), dec256!(217.0));                         
assert_eq!(dec256!(217).quantize(dec256!(1e+0)), dec256!(217));                           
assert_eq!(dec256!(217).quantize(dec256!(1e+1)), dec256!(2.2E+2));                        
assert_eq!(dec256!(217).quantize(dec256!(1e+2)), dec256!(2E+2));                          
```

This operation is very similar to [rescale], which has the same semantics except that the second operand specified the
power of ten of the quantum.                                                                           
In addition, unlike `rescale`, if the length of the coefficient after the `quantize` operation is greater than
precision, then an [`Invalid operation`] condition is raised.                                                  
This guarantees that, unless there is an error condition, the exponent of the `quantize` result is always equal to that
of the second operand.                                                                                  
Also, unlike other operations, `quantize` will never raise [`Underflow`], even if the result is subnormal and inexact.

## Truncate

[truncate]: #truncate

[`trunc(self)`](crate::decimal::Decimal::trunc)
[`trunc_with_scale(self, scale)`](crate::decimal::Decimal::trunc_with_scale)

If `self` is a [special value] then the [general rules] apply, and an [`Invalid operation`] condition is raised
and the result is [`NaN`].
Otherwise, `truncate` quantize the `self` decimal number specified to the power of ten of the quantum (`scale`).

The coefficient:

- **Is not rounded**, in opposition to rescale

### Examples

```
use fastnum::{*, decimal::*};                                                     

assert_eq!(dec256!(2.17).trunc_with_scale(3),dec256!(2.170));
assert_eq!(dec256!(2.17).trunc_with_scale(2), dec256!(2.17));
assert_eq!(dec256!(2.17).trunc_with_scale(1), dec256!(2.1));
assert_eq!(dec256!(2.9).trunc(), dec256!(2));
assert_eq!(dec256!(2.17).trunc_with_scale(-1), dec256!(0));

let ctx = Context::default().without_traps();   

assert!(D256::INFINITY.with_ctx(ctx).trunc_with_scale(2).is_nan());
assert!(D256::NEG_INFINITY.with_ctx(ctx).trunc_with_scale(2).is_nan());
assert!(D256::NAN.with_ctx(ctx).trunc_with_scale(1).is_nan());
```

## Rounding

Rounding is applied when a result coefficient has more significant digits than the value of precision.
In this case the result coefficient is shortened to precision digits and may then be incremented by one (which may
require further shortening), depending on the rounding algorithm selected and the remaining digits of the original
coefficient.
The exponent is adjusted to compensate for any shortening.

When a result is rounded, the coefficient may become longer than the current precision. In this case the least
significant digit of the coefficient (it will be a zero) is removed (reducing the precision by one), and the exponent is
incremented by one. This in turn may give rise to an overflow condition, which determines the result after overflow.

### Rounding mode

[RoundingMode]: #rounding-mode

[rounding mode]: #rounding-mode

[RoundingMode](crate::decimal::RoundingMode) enum determines how to calculate the last digit of the number when
performing rounding:

| Mode       | Description                                                                    | Default |
|------------|--------------------------------------------------------------------------------|:-------:|
| `Up`       | Always round away from zero.                                                   |         |
| `Down`     | Always round towards zero.                                                     |         |
| `Ceiling`  | Round towards +∞.                                                              |         |
| `Floor`    | Round towards -∞.                                                              |         |
| `HalfUp`   | Round to 'nearest neighbor', or up if ending decimal is `5`.                   |    ✅    |
| `HalfDown` | Round to 'nearest neighbor', or down if ending decimal is `5`.                 |         |
| `HalfEven` | Round to 'nearest neighbor', if equidistant, round towards nearest even digit. |         |

#### Up

[round-up]: #up

Round away from zero. If all the discarded digits are zero the result is unchanged. Otherwise, the result coefficient
should be incremented by `1` (rounded up).

* `5.5` → `6.0`
* `2.5` → `3.0`
* `1.6` → `2.0`
* `1.1` → `2.0`
* `-1.1` → `-2.0`
* `-1.6` → `-2.0`
* `-2.5` → `-3.0`
* `-5.5` → `-6.0`

#### Down

[round-down]: #down

Round toward zero, truncate. The discarded digits are ignored; the result is unchanged.

* `5.5` → `5.0`
* `2.5` → `2.0`
* `1.6` → `1.0`
* `1.1` → `1.0`
* `-1.1` → `-1.0`
* `-1.6` → `-1.0`
* `-2.5` → `-2.0`
* `-5.5` → `-5.0`

#### Ceiling

[round-ceiling]: #ceiling

Round toward +∞. If all the discarded digits are zero or if the sign is `1` the result is unchanged. Otherwise, the
result coefficient should be incremented by `1` (rounded up).

* `5.5` → `6.0`
* `2.5` → `3.0`
* `1.6` → `2.0`
* `1.1` → `2.0`
* `-1.1` → `-1.0`
* `-1.6` → `-1.0`
* `-2.5` → `-2.0`
* `-5.5` → `-5.0`

#### Floor

[round-floor]: #floor

Round toward -∞. If all the discarded digits are zero or if the sign is `0` the result is unchanged. Otherwise, the
sign is `1` and the result coefficient should be incremented by `1`.

* `5.5` → `5.0`
* `2.5` → `2.0`
* `1.6` → `1.0`
* `1.1` → `1.0`
* `-1.1` → `-2.0`
* `-1.6` → `-2.0`
* `-2.5` → `-3.0`
* `-5.5` → `-6.0`

#### HalfUp

[round-half-up]: #halfup

If the discarded digits represent greater than or equal to half (`0.5`) of the value of a one in the next left position
then the result coefficient should be incremented by `1` (rounded up). Otherwise, the discarded digits are ignored.

* `5.5` → `6.0`
* `2.5` → `3.0`
* `1.6` → `2.0`
* `1.1` → `1.0`
* `-1.1` → `-1.0`
* `-1.6` → `-2.0`
* `-2.5` → `-3.0`
* `-5.5` → `-6.0`

#### HalfDown

[round-half-down]: #halfdown

If the discarded digits represent greater than half (`0.5`) of the value of a one in the next left position then the
result coefficient should be incremented by `1` (rounded up). Otherwise (the discarded digits are `0.5` or less) the
discarded digits are ignored.

* `5.5` → `5.0`
* `2.5` → `2.0`
* `1.6` → `2.0`
* `1.1` → `1.0`
* `-1.1` → `-1.0`
* `-1.6` → `-2.0`
* `-2.5` → `-2.0`
* `-5.5` → `-5.0`

#### HalfEven

[round-half-even]: #halfeven

If the discarded digits represent greater than half (`0.5`) the value of a one in the next left position then the result
coefficient should be incremented by `1` (rounded up). If they represent less than half, then the result coefficient is
not adjusted (that is, the discarded digits are ignored).

Otherwise (they represent exactly half) the result coefficient is unaltered if its rightmost digit is even, or
incremented by `1` (rounded up) if its rightmost digit is odd (to make an even digit).

* `5.5` → `6.0`
* `2.5` → `2.0`
* `1.6` → `2.0`
* `1.1` → `1.0`
* `-1.1` → `-1.0`
* `-1.6` → `-2.0`
* `-2.5` → `-2.0`
* `-5.5` → `-6.0`

## Extra precision

[Extra precision]: #extra-precision

_Description is coming soon..._

## Base and mathematical constants

[`π`]: https://en.wikipedia.org/wiki/Pi

[`e`]: https://en.wikipedia.org/wiki/E_(mathematical_constant)

[`τ = 2π`]: https://en.wikipedia.org/wiki/Turn_(angle)#Tau_proposals

[Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon

|      Const       |                                    Value                                     |                           Signed                            |                              Unsigned                               |
|:----------------:|:----------------------------------------------------------------------------:|:-----------------------------------------------------------:|:-------------------------------------------------------------------:|
|      `NAN`       |                                   [`NaN`]                                    |            [`NAN`](crate::decimal::Decimal::NAN)            |            [`NAN`](crate::decimal::UnsignedDecimal::NAN)            |
|    `INFINITY`    |                                    [`+∞`]                                    |       [`INFINITY`](crate::decimal::Decimal::INFINITY)       |       [`INFINITY`](crate::decimal::UnsignedDecimal::INFINITY)       |
|  `NEG_INFINITY`  |                                    [`-∞`]                                    |   [`NEG_INFINITY`](crate::decimal::Decimal::NEG_INFINITY)   |                                                                     |
|      `MIN`       | _0_ for unsigned and _-(2<sup>N</sup> - 1) × 10<sup>32'768</sup>_ for signed |            [`MIN`](crate::decimal::Decimal::MIN)            |            [`MIN`](crate::decimal::UnsignedDecimal::MIN)            |
|      `MAX`       |                 _(2<sup>N</sup> - 1) × 10<sup>32'768</sup>_                  |            [`MAX`](crate::decimal::Decimal::MAX)            |            [`MAX`](crate::decimal::UnsignedDecimal::MAX)            |
|  `MIN_POSITIVE`  |                          _1 × 10<sup>-32'768</sup>_                          |   [`MIN_POSITIVE`](crate::decimal::Decimal::MIN_POSITIVE)   |   [`MIN_POSITIVE`](crate::decimal::UnsignedDecimal::MIN_POSITIVE)   |
|    `EPSILON`     |                              [Machine epsilon]                               |        [`EPSILON`](crate::decimal::Decimal::EPSILON)        |        [`EPSILON`](crate::decimal::UnsignedDecimal::EPSILON)        |
|      `ZERO`      |                                     _0_                                      |           [`ZERO`](crate::decimal::Decimal::ZERO)           |           [`ZERO`](crate::decimal::UnsignedDecimal::ZERO)           |
|      `ONE`       |                                     _1_                                      |            [`ONE`](crate::decimal::Decimal::ONE)            |            [`ONE`](crate::decimal::UnsignedDecimal::ONE)            |
|       ...        |                                                                              |                                                             |                                                                     |
|      `TEN`       |                                     _10_                                     |            [`TEN`](crate::decimal::Decimal::TEN)            |            [`TEN`](crate::decimal::UnsignedDecimal::TEN)            |
|       `PI`       |                         Archimedes' constant _[`π`]_                         |             [`PI`](crate::decimal::Decimal::PI)             |             [`PI`](crate::decimal::UnsignedDecimal::PI)             |
|       `E`        |                            Euler's number _[`e`]_                            |              [`E`](crate::decimal::Decimal::E)              |              [`E`](crate::decimal::UnsignedDecimal::E)              |
|      `TAU`       |                    The full circle constant _[`τ = 2π`]_                     |            [`TAU`](crate::decimal::Decimal::TAU)            |            [`TAU`](crate::decimal::UnsignedDecimal::TAU)            |
|   `FRAC_1_PI`    |                                 _1 / [`π`]_                                  |      [`FRAC_1_PI`](crate::decimal::Decimal::FRAC_1_PI)      |      [`FRAC_1_PI`](crate::decimal::UnsignedDecimal::FRAC_1_PI)      |
|   `FRAC_2_PI`    |                                 _2 / [`π`]_                                  |      [`FRAC_2_PI`](crate::decimal::Decimal::FRAC_2_PI)      |      [`FRAC_2_PI`](crate::decimal::UnsignedDecimal::FRAC_2_PI)      |
|   `FRAC_PI_2`    |                                 _[`π`] / 2_                                  |      [`FRAC_PI_2`](crate::decimal::Decimal::FRAC_PI_2)      |      [`FRAC_PI_2`](crate::decimal::UnsignedDecimal::FRAC_PI_2)      |
|   `FRAC_PI_3`    |                                 _[`π`] / 3_                                  |      [`FRAC_PI_3`](crate::decimal::Decimal::FRAC_PI_3)      |      [`FRAC_PI_3`](crate::decimal::UnsignedDecimal::FRAC_PI_3)      |
|   `FRAC_PI_4`    |                                 _[`π`] / 4_                                  |      [`FRAC_PI_4`](crate::decimal::Decimal::FRAC_PI_4)      |      [`FRAC_PI_4`](crate::decimal::UnsignedDecimal::FRAC_PI_4)      |
|   `FRAC_PI_6`    |                                 _[`π`] / 6_                                  |      [`FRAC_PI_6`](crate::decimal::Decimal::FRAC_PI_6)      |      [`FRAC_PI_6`](crate::decimal::UnsignedDecimal::FRAC_PI_6)      |
|   `FRAC_PI_8`    |                                 _[`π`] / 8_                                  |      [`FRAC_PI_8`](crate::decimal::Decimal::FRAC_PI_8)      |      [`FRAC_PI_8`](crate::decimal::UnsignedDecimal::FRAC_PI_8)      |
| `FRAC_2_SQRT_PI` |                              _2 / sqrt([`π`])_                               | [`FRAC_2_SQRT_PI`](crate::decimal::Decimal::FRAC_2_SQRT_PI) | [`FRAC_2_SQRT_PI`](crate::decimal::UnsignedDecimal::FRAC_2_SQRT_PI) |
|      `LN_2`      |                                   _ln(2)_                                    |           [`LN_2`](crate::decimal::Decimal::LN_2)           |           [`LN_2`](crate::decimal::UnsignedDecimal::LN_2)           |
|     `LN_10`      |                                   _ln(10)_                                   |          [`LN_10`](crate::decimal::Decimal::LN_10)          |          [`LN_10`](crate::decimal::UnsignedDecimal::LN_10)          |
|     `LOG2_E`     |                             _log<sub>2</sub>(e)_                             |         [`LOG2_E`](crate::decimal::Decimal::LOG2_E)         |         [`LOG2_E`](crate::decimal::UnsignedDecimal::LOG2_E)         |
|    `LOG10_E`     |                            _log<sub>10</sub>(e)_                             |        [`LOG10_E`](crate::decimal::Decimal::LOG10_E)        |        [`LOG10_E`](crate::decimal::UnsignedDecimal::LOG10_E)        |
|     `SQRT_2`     |                                  _sqrt(2)_                                   |         [`SQRT_2`](crate::decimal::Decimal::SQRT_2)         |         [`SQRT_2`](crate::decimal::UnsignedDecimal::SQRT_2)         |
| `FRAC_1_SQRT_2`  |                                _1 / sqrt(2)_                                 |  [`FRAC_1_SQRT_2`](crate::decimal::Decimal::FRAC_1_SQRT_2)  |  [`FRAC_1_SQRT_2`](crate::decimal::UnsignedDecimal::FRAC_1_SQRT_2)  |
|    `LOG10_2`     |                            _log<sub>10</sub>(2)_                             |        [`LOG10_2`](crate::decimal::Decimal::LOG10_2)        |        [`LOG10_2`](crate::decimal::UnsignedDecimal::LOG10_2)        |
|    `LOG2_10`     |                            _log<sub>2</sub>(10)_                             |        [`LOG2_10`](crate::decimal::Decimal::LOG2_10)        |        [`LOG2_10`](crate::decimal::UnsignedDecimal::LOG2_10)        |

### Examples

```                                                                                       
use fastnum::{*, decimal::RoundingMode::*};     

assert_eq!(D128::PI, dec128!(3.14159265358979323846264338327950288420));                       
assert_eq!(D128::TAU, dec128!(2).with_rounding_mode(Down) * D128::PI);                       
                                  
```

## Formatting

Rust's `fmt::Display` formatting options for `fastnum` decimals:

| Placeholder     | Format                                         | Description                                                                                             |
|-----------------|------------------------------------------------|---------------------------------------------------------------------------------------------------------|
| `{}`            | Default Display                                | Format as "human readable" number.                                                                      |
| `{:.<PREC>}`    | Display with precision                         | Format number with exactly `PREC` digits after the decimal place.                                       |
| `{:e}` / `{:E}` | Exponential format                             | Formats in scientific notation with either `e` or `E` as exponent delimiter. Precision is kept exactly. |
| `{:.<PREC>e}`   | Formats in scientific notation, keeping number | Number is rounded / zero padded until.                                                                  |
| `{:?}`          | Debug                                          | Shows internal representation of decimal.                                                               |
| `{:#?}`         | Alternate Debug (used by `dbg!()`)             | Shows simple int+exponent string representation of decimal.                                             |

### Default display

- "Small" fractional numbers (close to zero) are printed in scientific notation
    - Number is considered "small" by number of leading zeros exceeding a threshold
    - Configurable by the compile-time environment variable:
      [RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD](#compile-time-configuration)
        - Default `5`
    - Example: `1.23e-3` will print as `0.00123` but `1.23e-10` will be `1.23E-10`
- Trailing zeros will be added to "small" integers, avoiding scientific notation
    - May appear to have more precision than they do
    - Example: decimal `1e1` would be rendered as `10`
    - The threshold for "small" is configured by compile-time environment variable:
      [`RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD`](#compile-time-configuration)
        - Default `15`
    - `1e15` => `1000000000000000`
    - Large integers (e.g. `1e50000000`) will print in scientific notation, not
      a 1 followed by fifty million zeros
- All other numbers are printed in standard decimal notation

### Display with precision

Numbers with fractional components will be rounded at precision point, or have zeros padded to precision point. Integers
will have zeros padded to the precision point. To prevent unreasonably sized output, a threshold limits the number of
padded zeros:

- Greater than the default case, since specific precision was requested
- Configurable by the compile-time environment variable:
  [`RUST_BIGDECIMAL_FMT_MAX_INTEGER_PADDING`](#compile-time-configuration)
- Default `1000`

If digits exceed this threshold, they're printed without a decimal-point, suffixed with scale of the decimal.

## Serialization

[Serialization]: #serialization

If you're passing decimal numbers between systems, be sure to use a serialization format
which explicitly supports decimal numbers and doesn't require transformations to
floating-point binary numbers, or there will be information loss.

Text formats like JSON should work ok as long as the receiver will also parse
numbers as decimals so complete precision is kept accurate.
Typically, JSON-parsing implementations don't do this by default, and need special
configuration.

Binary formats like `msgpack` may expect/require representing numbers as 64-bit [IEEE-754]
floating-point, and will likely lose precision by default unless you explicitly format
the decimal as a string, bytes, or some custom structure.

### Serde

`serde` feature allows to serialize and deserialize `fasnum` decimals via the [
`serde`](https://docs.rs/serde/latest/serde/) crate.

- Decimals are always serialized as _as a string_.
- Decimals deserialize behavior defined via [`RUST_FASTNUM_SERDE_DESERIALIZE_MODE`](#compile-time-configuration).
  Possible values are:

| **Mode**    | Description                                                                                                                                                                | Default |
|-------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:-------:|
| `Strict`    | Allow only string values such as `"0.1"`, `"0.25"`, etc. Any other numbers like `0.1` or `1` triggers parsing error.                                                       |    ✅    |
| `Stringify` | Decimal values such as `0.1` will be stringify to `"0.1"` first and then parse as decimal numbers.                                                                         |         |
| `Any`       | Allow any values. Parse performs in-place. Result may be inexact because there is no correct limited binary representation for some floating-point numbers, such as `0.1`. |         |

```toml
[dependencies]
fastnum = { version = "0.2", features = ["serde"] } 
```

Basic usage:

```
# #[cfg(feature="serde")]
# {
use fastnum::{udec256, UD256};
use serde::*;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    name: String,
    value: UD256,
}

let json_src = r#"{ "name": "foo", "value": "1234567e-3" }"#;

let my_struct: MyStruct = serde_json::from_str(&json_src).unwrap();
dbg!(&my_struct);
// MyStruct { name: "foo", value: UD256("1234.567") }

println!("{}", serde_json::to_string(&my_struct).unwrap());
// {"name":"foo","value":"1234.567"}
# }
```

Should panic:

```no_run
# #[cfg(feature="serde")]
# {
use fastnum::{udec256, UD256};
use serde::*;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    name: String,
    value: UD256,
}

let json_src = r#"{ "name": "foo", "value": 1234567e-3 }"#;
let my_struct: MyStruct = serde_json::from_str(&json_src).unwrap();
# }
```

## Features

| Feature           | Default | Description                                                                                                                                                                                         |
|-------------------|:-------:|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `std`             |    ✅    |                                                                                                                                                                                                     |
| `numtraits`       |         | Includes implementations of traits from the [`num_traits`](https://docs.rs/num-traits/latest/num_traits/) crate.                                                                                    |
| `rand`            |         | Allows creation of random `fastnum` decimals via the [`rand`](https://docs.rs/rand/latest/rand/) crate.                                                                                             |
| `zeroize`         |         | Enables the [`Zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html) trait implementation from the [`zeroize`](https://docs.rs/zeroize/latest/zeroize/) crate for `fastnum` decimals. |
| `serde`           |         | Enables serialization and deserialization of `fastnum` decimals via the [`serde`](https://docs.rs/serde/latest/serde/) crate.                                                                       |
| `diesel`          |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) crate.                                                                        |
| `diesel_postgres` |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) PostgreSQL backend.                                                           |
| `diesel_mysql`    |         | Enables serialization and deserialization of `fastnum` decimals for [`diesel`](https://docs.rs/diesel/latest/diesel/) MySQL backend.                                                                |
| `sqlx`            |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) crate.                                                                              |
| `sqlx_postgres`   |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) PostgreSQL backend.                                                                 |
| `sqlx_mysql`      |         | Enables serialization and deserialization of `fastnum` decimals for [`sqlx`](https://docs.rs/sqlx/latest/sqlx/) MySQL backend.                                                                      |
| `tokio-postgres`  |         | Enables serialization and deserialization of `fastnum` decimals for [`tokio-postgres`](https://docs.rs/tokio-postgres/latest/tokio_postgres/) crate.                                                |
| `utoipa`          |         | Enables support of `fastnum` decimals for autogenerated OpenAPI documentation via the [`utoipa`](https://docs.rs/utoipa/latest/utoipa/) crate.                                                      |
| `dev`             |         | This feature opens up some otherwise private API, that can be useful to implement a third party integrations. Do not use this feature for any other use-cases.                                      |

## Compile-time configuration

You can set a few default parameters at _compile-time_ via environment variables:

| Environment Variable                           | Default  |
|------------------------------------------------|----------|
| `RUST_FASTNUM_DEFAULT_ROUNDING_MODE`           | `HalfUp` |
| `RUST_FASTNUM_FMT_EXPONENTIAL_LOWER_THRESHOLD` | `5`      |
| `RUST_FASTNUM_FMT_EXPONENTIAL_UPPER_THRESHOLD` | `15`     |
| `RUST_FASTNUM_FMT_MAX_INTEGER_PADDING`         | `1000`   |
| `RUST_FASTNUM_SERDE_DESERIALIZE_MODE`          | `Strict` |
