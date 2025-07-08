c_char in std::os::raw - Rust
std
::
os
::
raw
Type Alias
c_char
Copy item path
1.1.0
·
Source
pub type c_char =
c_char
;
Expand description
Equivalent to C’s
char
type.
C’s
char
type
is completely unlike
Rust’s
char
type
; while Rust’s type represents a unicode scalar value, C’s
char
type is just an ordinary integer. On modern architectures this type will always be either
i8
or
u8
, as they use byte-addresses memory with 8-bit bytes.
C chars are most commonly used to make C strings. Unlike Rust, where the length of a string is included alongside the string, C strings mark the end of a string with the character
'\0'
. See
CStr
for more information.