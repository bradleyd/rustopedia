is_separator in std::path - Rust
std
::
path
Function
is_separator
Copy item path
1.0.0
·
Source
pub fn is_separator(c:
char
) ->
bool
Expand description
Determines whether the character is one of the permitted path
separators for the current platform.
§
Examples
use
std::path;
assert!
(path::is_separator(
'/'
));
// '/' works for both Unix and Windows
assert!
(!path::is_separator(
'❤'
));