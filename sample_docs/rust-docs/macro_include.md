include in std - Rust
std
Macro
include
Copy item path
1.38.0
·
Source
macro_rules! include {
    ($file:expr $(,)?) => { ... };
}
Expand description
Parses a file as an expression or an item according to the context.
Warning
: For multi-file Rust projects, the
include!
macro is probably not what you
are looking for. Usually, multi-file Rust projects use
modules
. Multi-file projects and
modules are explained in the Rust-by-Example book
here
and the module system is
explained in the Rust Book
here
.
The included file is placed in the surrounding code
unhygienically
. If
the included file is parsed as an expression and variables or functions share names across
both files, it could result in variables or functions being different from what the
included file expected.
The included file is located relative to the current file (similarly to how modules are
found). The provided path is interpreted in a platform-specific way at compile time. So,
for instance, an invocation with a Windows path containing backslashes
\
would not
compile correctly on Unix.
§
Uses
The
include!
macro is primarily used for two purposes. It is used to include
documentation that is written in a separate file and it is used to include
build artifacts
usually as a result from the
build.rs
script
.
When using the
include
macro to include stretches of documentation, remember that the
included file still needs to be a valid Rust syntax. It is also possible to
use the
include_str
macro as
#![doc = include_str!("...")]
(at the module level) or
#[doc = include_str!("...")]
(at the item level) to include documentation from a plain
text or markdown file.
§
Examples
Assume there are two files in the same directory with the following contents:
File ‘monkeys.in’:
ⓘ
[
'🙈'
,
'🙊'
,
'🙉'
]
    .iter()
    .cycle()
    .take(
6
)
    .collect::<String>()
File ‘main.rs’:
ⓘ
fn
main() {
let
my_string =
include!
(
"monkeys.in"
);
assert_eq!
(
"🙈🙊🙉🙈🙊🙉"
, my_string);
println!
(
"{my_string}"
);
}
Compiling ‘main.rs’ and running the resulting binary will print
“🙈🙊🙉🙈🙊🙉”.