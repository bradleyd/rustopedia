c_ulonglong in std::os::raw - Rust
std
::
os
::
raw
Type Alias
c_ulonglong
Copy item path
1.1.0
·
Source
pub type c_ulonglong =
c_ulonglong
;
Expand description
Equivalent to C’s
unsigned long long
type.
This type will almost always be
u64
, but may differ on some systems. The C standard technically only requires that this type be an unsigned integer with the size of a
long long
, although in practice, no system would have a
long long
that is not a
u64
, as most systems do not have a standardised
u128
type.