c_float in std::os::raw - Rust
std
::
os
::
raw
Type Alias
c_float
Copy item path
1.1.0
·
Source
pub type c_float =
c_float
;
Expand description
Equivalent to C’s
float
type.
This type will almost always be
f32
, which is guaranteed to be an
IEEE 754 single-precision float
in Rust. That said, the standard technically only guarantees that it be a floating-point number, and it may have less precision than
f32
or not follow the IEEE-754 standard at all.