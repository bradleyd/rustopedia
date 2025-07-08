DIGITS in std::f64 - Rust
std
::
f64
Constant
DIGITS
Copy item path
1.0.0
·
Source
pub const DIGITS:
u32
= f64::DIGITS; // 15u32
👎
Deprecating in a future version: replaced by the
DIGITS
associated constant on
f64
Expand description
Approximate number of significant digits in base 10.
Use
f64::DIGITS
instead.
§
Examples
// deprecated way
let
d = std::f64::DIGITS;
// intended way
let
d = f64::DIGITS;