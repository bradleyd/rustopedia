EXE_SUFFIX in std::env::consts - Rust
std
::
env
::
consts
Constant
EXE_SUFFIX
Copy item path
1.0.0
·
Source
pub const EXE_SUFFIX: &
str
;
Expand description
Specifies the filename suffix, if any, used for executable binaries on this platform.
An example value may be:
".exe"
, or
".efi"
.
The possible values are identical to those of
EXE_EXTENSION
, but with the leading period included.