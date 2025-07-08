c_double in std::os::raw - Rust
std
::
os
::
raw
Type Alias
c_double
Copy item path
1.1.0
·
Source
pub type c_double =
c_double
;
Expand description
Equivalent to C’s
double
type.
This type will almost always be
f64
, which is guaranteed to be an
IEEE 754 double-precision float
in Rust. That said, the standard technically only guarantees that it be a floating-point number with at least the precision of a
float
, and it may be
f32
or something entirely different from the IEEE-754 standard.