MAIN_SEPARATOR in std::path - Rust
std
::
path
Constant
MAIN_SEPARATOR
Copy item path
1.0.0
·
Source
pub const MAIN_SEPARATOR:
char
= crate::sys::path::MAIN_SEP; // '/'
Expand description
The primary separator of path components for the current platform.
For example,
/
on Unix and
\
on Windows.