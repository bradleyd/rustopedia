from_utf8 in std::str - Rust
std
::
str
Function
from_utf8
Copy item path
1.0.0 (const: 1.63.0)
·
Source
pub const fn from_utf8(v: &[
u8
]) ->
Result
<&
str
,
Utf8Error
>
Expand description
Converts a slice of bytes to a string slice.
A string slice (
&str
) is made of bytes (
u8
), and a byte slice
(
&[u8]
) is made of bytes, so this function converts between
the two. Not all byte slices are valid string slices, however:
&str
requires
that it is valid UTF-8.
from_utf8()
checks to ensure that the bytes are valid
UTF-8, and then does the conversion.
If you are sure that the byte slice is valid UTF-8, and you don’t want to
incur the overhead of the validity check, there is an unsafe version of
this function,
from_utf8_unchecked
, which has the same
behavior but skips the check.
If you need a
String
instead of a
&str
, consider
String::from_utf8
.
Because you can stack-allocate a
[u8; N]
, and you can take a
&[u8]
of it, this function is one way to have a
stack-allocated string. There is an example of this in the
examples section below.
§
Errors
Returns
Err
if the slice is not UTF-8 with a description as to why the
provided slice is not UTF-8.
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
// We can use the ? (try) operator to check if the bytes are valid
let
sparkle_heart = str::from_utf8(
&
sparkle_heart)
?
;
assert_eq!
(
"💖"
, sparkle_heart);
Incorrect bytes:
use
std::str;
// some invalid bytes, in a vector
let
sparkle_heart =
vec!
[
0
,
159
,
146
,
150
];
assert!
(str::from_utf8(
&
sparkle_heart).is_err());
See the docs for
Utf8Error
for more details on the kinds of
errors that can be returned.
A “stack allocated string”:
use
std::str;
// some bytes, in a stack-allocated array
let
sparkle_heart = [
240
,
159
,
146
,
150
];
// We know these bytes are valid, so just use `unwrap()`.
let
sparkle_heart:
&
str = str::from_utf8(
&
sparkle_heart).unwrap();
assert_eq!
(
"💖"
, sparkle_heart);