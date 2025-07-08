from_utf8_mut in std::str - Rust
std
::
str
Function
from_utf8_mut
Copy item path
1.20.0 (const: 1.87.0)
·
Source
pub const fn from_utf8_mut(v: &mut [
u8
]) ->
Result
<&mut
str
,
Utf8Error
>
Expand description
Converts a mutable slice of bytes to a mutable string slice.
§
Examples
Basic usage:
use
std::str;
// "Hello, Rust!" as a mutable vector
let
mut
hellorust =
vec!
[
72
,
101
,
108
,
108
,
111
,
44
,
32
,
82
,
117
,
115
,
116
,
33
];
// As we know these bytes are valid, we can use `unwrap()`
let
outstr = str::from_utf8_mut(
&mut
hellorust).unwrap();
assert_eq!
(
"Hello, Rust!"
, outstr);
Incorrect bytes:
use
std::str;
// Some invalid bytes in a mutable vector
let
mut
invalid =
vec!
[
128
,
223
];
assert!
(str::from_utf8_mut(
&mut
invalid).is_err());
See the docs for
Utf8Error
for more details on the kinds of
errors that can be returned.