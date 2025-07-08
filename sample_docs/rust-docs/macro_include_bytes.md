include_bytes in std - Rust
std
Macro
include_bytes
Copy item path
1.38.0
·
Source
macro_rules! include_bytes {
    ($file:expr $(,)?) => { ... };
}
Expand description
Includes a file as a reference to a byte array.
The file is located relative to the current file (similarly to how
modules are found). The provided path is interpreted in a platform-specific
way at compile time. So, for instance, an invocation with a Windows path
containing backslashes
\
would not compile correctly on Unix.
This macro will yield an expression of type
&'static [u8; N]
which is
the contents of the file.
§
Examples
Assume there are two files in the same directory with the following
contents:
File ‘spanish.in’:
adiós
File ‘main.rs’:
ⓘ
fn
main() {
let
bytes =
include_bytes!
(
"spanish.in"
);
assert_eq!
(bytes,
b"adi\xc3\xb3s\n"
);
print!
(
"{}"
, String::from_utf8_lossy(bytes));
}
Compiling ‘main.rs’ and running the resulting binary will print “adiós”.