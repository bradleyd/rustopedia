from_utf8_unchecked_mut in std::str - Rust
std
::
str
Function
from_utf8_unchecked_mut
Copy item path
1.20.0 (const: 1.83.0)
·
Source
pub const unsafe fn from_utf8_unchecked_mut(v: &mut [
u8
]) -> &mut
str
Expand description
Converts a slice of bytes to a string slice without checking
that the string contains valid UTF-8; mutable version.
See the immutable version,
from_utf8_unchecked()
for more information.
§
Examples
Basic usage:
use
std::str;
let
mut
heart =
vec!
[
240
,
159
,
146
,
150
];
let
heart =
unsafe
{ str::from_utf8_unchecked_mut(
&mut
heart) };
assert_eq!
(
"💖"
, heart);