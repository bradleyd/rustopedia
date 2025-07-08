writeln in std - Rust
std
Macro
writeln
Copy item path
1.0.0
·
Source
macro_rules! writeln {
    ($dst:expr $(,)?) => { ... };
    ($dst:expr, $($arg:tt)*) => { ... };
}
Expand description
Writes formatted data into a buffer, with a newline appended.
On all platforms, the newline is the LINE FEED character (
\n
/
U+000A
) alone
(no additional CARRIAGE RETURN (
\r
/
U+000D
).
For more information, see
write!
. For information on the format string syntax, see
std::fmt
.
§
Examples
use
std::io::{Write,
Result
};
fn
main() ->
Result
<()> {
let
mut
w = Vec::new();
writeln!
(
&mut
w)
?
;
writeln!
(
&mut
w,
"test"
)
?
;
writeln!
(
&mut
w,
"formatted {}"
,
"arguments"
)
?
;
assert_eq!
(
&
w[..],
"\ntest\nformatted arguments\n"
.as_bytes());
Ok
(())
}