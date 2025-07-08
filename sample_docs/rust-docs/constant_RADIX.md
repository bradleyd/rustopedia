RADIX in std::f64 - Rust
std
::
f64
Constant
RADIX
Copy item path
1.0.0
·
Source
pub const RADIX:
u32
= f64::RADIX; // 2u32
👎
Deprecating in a future version: replaced by the
RADIX
associated constant on
f64
Expand description
The radix or base of the internal representation of
f64
.
Use
f64::RADIX
instead.
§
Examples
// deprecated way
let
r = std::f64::RADIX;
// intended way
let
r = f64::RADIX;