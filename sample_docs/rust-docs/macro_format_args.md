format_args in std - Rust
std
Macro
format_args
Copy item path
1.38.0
·
Source
macro_rules! format_args {
    ($fmt:expr) => { ... };
    ($fmt:expr, $($args:tt)*) => { ... };
}
Expand description
Constructs parameters for the other string-formatting macros.
This macro functions by taking a formatting string literal containing
{}
for each additional argument passed.
format_args!
prepares the
additional parameters to ensure the output can be interpreted as a string
and canonicalizes the arguments into a single type. Any value that implements
the
Display
trait can be passed to
format_args!
, as can any
Debug
implementation be passed to a
{:?}
within the formatting string.
This macro produces a value of type
fmt::Arguments
. This value can be
passed to the macros within
std::fmt
for performing useful redirection.
All other formatting macros (
format!
,
write!
,
println!
, etc) are
proxied through this one.
format_args!
, unlike its derived macros, avoids
heap allocations.
You can use the
fmt::Arguments
value that
format_args!
returns
in
Debug
and
Display
contexts as seen below. The example also shows
that
Debug
and
Display
format to the same thing: the interpolated
format string in
format_args!
.
let
debug =
format!
(
"{:?}"
,
format_args!
(
"{} foo {:?}"
,
1
,
2
));
let
display =
format!
(
"{}"
,
format_args!
(
"{} foo {:?}"
,
1
,
2
));
assert_eq!
(
"1 foo 2"
, display);
assert_eq!
(display, debug);
See
the formatting documentation in
std::fmt
for details of the macro argument syntax, and further information.
§
Examples
use
std::fmt;
let
s = fmt::format(
format_args!
(
"hello {}"
,
"world"
));
assert_eq!
(s,
format!
(
"hello {}"
,
"world"
));
§
Lifetime limitation
Except when no formatting arguments are used,
the produced
fmt::Arguments
value borrows temporary values,
which means it can only be used within the same expression
and cannot be stored for later use.
This is a known limitation, see
#92698
.