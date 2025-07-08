line in std - Rust
std
Macro
line
Copy item path
1.38.0
·
Source
macro_rules! line {
    () => { ... };
}
Expand description
Expands to the line number on which it was invoked.
With
column!
and
file!
, these macros provide debugging information for
developers about the location within the source.
The expanded expression has type
u32
and is 1-based, so the first line
in each file evaluates to 1, the second to 2, etc. This is consistent
with error messages by common compilers or popular editors.
The returned line is
not necessarily
the line of the
line!
invocation itself,
but rather the first macro invocation leading up to the invocation
of the
line!
macro.
§
Examples
let
current_line =
line!
();
println!
(
"defined on line: {current_line}"
);