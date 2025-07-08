DLL_EXTENSION in std::env::consts - Rust
std
::
env
::
consts
Constant
DLL_EXTENSION
Copy item path
1.0.0
·
Source
pub const DLL_EXTENSION: &
str
;
Expand description
Specifies the file extension, if any, used for shared libraries on this platform that goes after the dot.
An example value may be:
"so"
,
"elf"
, or
"dll"
.
Full list of possible values
"so"
"dylib"
"dll"
"sgxs"
"a"
"elf"
"wasm"
""
(an empty string)