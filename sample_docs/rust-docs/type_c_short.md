c_short in std::os::raw - Rust
std
::
os
::
raw
Type Alias
c_short
Copy item path
1.1.0
·
Source
pub type c_short =
c_short
;
Expand description
Equivalent to C’s
signed short
(
short
) type.
This type will almost always be
i16
, but may differ on some esoteric systems. The C standard technically only requires that this type be a signed integer with at least 16 bits; some systems may define it as
i32
, for example.