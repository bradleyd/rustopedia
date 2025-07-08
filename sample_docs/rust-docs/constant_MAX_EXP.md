MAX_EXP in std::f64 - Rust
std
::
f64
Constant
MAX_EXP
Copy item path
1.0.0
·
Source
pub const MAX_EXP:
i32
= f64::MAX_EXP; // 1_024i32
👎
Deprecating in a future version: replaced by the
MAX_EXP
associated constant on
f64
Expand description
Maximum possible power of 2 exponent.
Use
f64::MAX_EXP
instead.
§
Examples
// deprecated way
let
max = std::f64::MAX_EXP;
// intended way
let
max = f64::MAX_EXP;