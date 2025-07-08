NAN in std::f64 - Rust
std
::
f64
Constant
NAN
Copy item path
1.0.0
·
Source
pub const NAN:
f64
= f64::NAN; // NaN_f64
👎
Deprecating in a future version: replaced by the
NAN
associated constant on
f64
Expand description
Not a Number (NaN).
Use
f64::NAN
instead.
§
Examples
// deprecated way
let
nan = std::f64::NAN;
// intended way
let
nan = f64::NAN;