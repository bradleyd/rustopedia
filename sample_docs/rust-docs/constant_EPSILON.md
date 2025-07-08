EPSILON in std::f64 - Rust
std
::
f64
Constant
EPSILON
Copy item path
1.0.0
·
Source
pub const EPSILON:
f64
= f64::EPSILON; // 2.2204460492503131E-16f64
👎
Deprecating in a future version: replaced by the
EPSILON
associated constant on
f64
Expand description
Machine epsilon
value for
f64
.
Use
f64::EPSILON
instead.
This is the difference between
1.0
and the next larger representable number.
§
Examples
// deprecated way
let
e = std::f64::EPSILON;
// intended way
let
e = f64::EPSILON;