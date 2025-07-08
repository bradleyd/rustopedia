from_boxed_utf8_unchecked in std::str - Rust
std
::
str
Function
from_boxed_utf8_unchecked
Copy item path
1.20.0
·
Source
pub unsafe fn from_boxed_utf8_unchecked(v:
Box
<[
u8
]>) ->
Box
<
str
>
Expand description
Converts a boxed slice of bytes to a boxed string slice without checking
that the string contains valid UTF-8.
§
Examples
let
smile_utf8 = Box::new([
226
,
152
,
186
]);
let
smile =
unsafe
{ std::str::from_boxed_utf8_unchecked(smile_utf8) };
assert_eq!
(
"☺"
,
&*
smile);