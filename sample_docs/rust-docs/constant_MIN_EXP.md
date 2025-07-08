MIN_EXP in std::f64 - Rust
std
::
f64
Constant
MIN_EXP
Copy item path
1.0.0
·
Source
pub const MIN_EXP:
i32
= f64::MIN_EXP; // -1_021i32
👎
Deprecating in a future version: replaced by the
MIN_EXP
associated constant on
f64
Expand description
One greater than the minimum possible normal power of 2 exponent.
Use
f64::MIN_EXP
instead.
§
Examples
// deprecated way
let
min = std::f64::MIN_EXP;
// intended way
let
min = f64::MIN_EXP;