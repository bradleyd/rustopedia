c_ushort in std::os::raw - Rust
std
::
os
::
raw
Type Alias
c_ushort
Copy item path
1.1.0
·
Source
pub type c_ushort =
c_ushort
;
Expand description
Equivalent to C’s
unsigned short
type.
This type will almost always be
u16
, but may differ on some esoteric systems. The C standard technically only requires that this type be an unsigned integer with the same size as a
short
.