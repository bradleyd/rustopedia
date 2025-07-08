column in std - Rust
std
Macro
column
Copy item path
1.38.0
·
Source
macro_rules! column {
    () => { ... };
}
Expand description
Expands to the column number at which it was invoked.
With
line!
and
file!
, these macros provide debugging information for
developers about the location within the source.
The expanded expression has type
u32
and is 1-based, so the first column
in each line evaluates to 1, the second to 2, etc. This is consistent
with error messages by common compilers or popular editors.
The returned column is
not necessarily
the line of the
column!
invocation itself,
but rather the first macro invocation leading up to the invocation
of the
column!
macro.
§
Examples
let
current_col =
column!
();
println!
(
"defined on column: {current_col}"
);
column!
counts Unicode code points, not bytes or graphemes. As a result, the first two
invocations return the same value, but the third does not.
let
a = (
"foobar"
,
column!
()).
1
;
let
b = (
"人之初性本善"
,
column!
()).
1
;
let
c = (
"f̅o̅o̅b̅a̅r̅"
,
column!
()).
1
;
// Uses combining overline (U+0305)
assert_eq!
(a, b);
assert_ne!
(b, c);