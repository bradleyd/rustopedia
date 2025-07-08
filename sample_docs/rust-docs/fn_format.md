format in std::fmt - Rust
std
::
fmt
Function
format
Copy item path
1.0.0
·
Source
pub fn format(args:
Arguments
<'_>) ->
String
Expand description
Takes an
Arguments
struct and returns the resulting formatted string.
The
Arguments
instance can be created with the
format_args!
macro.
§
Examples
Basic usage:
use
std::fmt;
let
s = fmt::format(
format_args!
(
"Hello, {}!"
,
"world"
));
assert_eq!
(s,
"Hello, world!"
);
Please note that using
format!
might be preferable.
Example:
let
s =
format!
(
"Hello, {}!"
,
"world"
);
assert_eq!
(s,
"Hello, world!"
);