c_longlong in std::os::raw - Rust
std
::
os
::
raw
Type Alias
c_longlong
Copy item path
1.1.0
·
Source
pub type c_longlong =
c_longlong
;
Expand description
Equivalent to C’s
signed long long
(
long long
) type.
This type will almost always be
i64
, but may differ on some systems. The C standard technically only requires that this type be a signed integer that is at least 64 bits and at least the size of a
long
, although in practice, no system would have a
long long
that is not an
i64
, as most systems do not have a standardised
i128
type.