MANTISSA_DIGITS in std::f64 - Rust
std
::
f64
Constant
MANTISSA_DIGITS
Copy item path
1.0.0
·
Source
pub const MANTISSA_DIGITS:
u32
= f64::MANTISSA_DIGITS; // 53u32
👎
Deprecating in a future version: replaced by the
MANTISSA_DIGITS
associated constant on
f64
Expand description
Number of significant digits in base 2.
Use
f64::MANTISSA_DIGITS
instead.
§
Examples
// deprecated way
let
d = std::f64::MANTISSA_DIGITS;
// intended way
let
d = f64::MANTISSA_DIGITS;