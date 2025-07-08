from_utf8_unchecked in std::str - Rust
std
::
str
Function
from_utf8_unchecked
Copy item path
1.0.0 (const: 1.55.0)
·
Source
pub const unsafe fn from_utf8_unchecked(v: &[
u8
]) -> &
str
Expand description
Converts a slice of bytes to a string slice without checking
that the string contains valid UTF-8.
See the safe version,
from_utf8
, for more information.
§
Safety
The bytes passed in must be valid UTF-8.
§
Examples
Basic usage:
use
std::str;
// some bytes, in a vector
let
sparkle_heart =
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
sparkle_heart =
unsafe
{
    str::from_utf8_unchecked(
&
sparkle_heart)
};
assert_eq!
(
"💖"
, sparkle_heart);