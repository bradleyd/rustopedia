DLL_SUFFIX in std::env::consts - Rust
std
::
env
::
consts
Constant
DLL_SUFFIX
Copy item path
1.0.0
·
Source
pub const DLL_SUFFIX: &
str
;
Expand description
Specifies the filename suffix, if any, used for shared libraries on this platform.
An example value may be:
".so"
,
".elf"
, or
".dll"
.
The possible values are identical to those of
DLL_EXTENSION
, but with the leading period included.