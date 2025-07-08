include_str in std - Rust
std
Macro
include_str
Copy item path
1.38.0
·
Source
macro_rules! include_str {
    ($file:expr $(,)?) => { ... };
}
Expand description
Includes a UTF-8 encoded file as a string.
The file is located relative to the current file (similarly to how
modules are found). The provided path is interpreted in a platform-specific
way at compile time. So, for instance, an invocation with a Windows path
containing backslashes
\
would not compile correctly on Unix.
This macro will yield an expression of type
&'static str
which is the
contents of the file.
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
my_str =
include_str!
(
"spanish.in"
);
assert_eq!
(my_str,
"adiós\n"
);
print!
(
"{my_str}"
);
}
Compiling ‘main.rs’ and running the resulting binary will print “adiós”.