c_uint in std::os::raw - Rust
std
::
os
::
raw
Type Alias
c_uint
Copy item path
1.1.0
·
Source
pub type c_uint =
c_uint
;
Expand description
Equivalent to C’s
unsigned int
type.
This type will almost always be
u32
, but may differ on some esoteric systems. The C standard technically only requires that this type be an unsigned integer with the same size as an
int
; some systems define it as a
u16
, for example.