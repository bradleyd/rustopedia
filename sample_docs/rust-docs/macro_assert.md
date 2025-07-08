assert in std - Rust
std
Macro
assert
Copy item path
1.38.0
·
Source
macro_rules! assert {
    ($cond:expr $(,)?) => { ... };
    ($cond:expr, $($arg:tt)+) => { ... };
}
Expand description
Asserts that a boolean expression is
true
at runtime.
This will invoke the
panic!
macro if the provided expression cannot be
evaluated to
true
at runtime.
§
Uses
Assertions are always checked in both debug and release builds, and cannot
be disabled. See
debug_assert!
for assertions that are not enabled in
release builds by default.
Unsafe code may rely on
assert!
to enforce run-time invariants that, if
violated could lead to unsafety.
Other use-cases of
assert!
include testing and enforcing run-time
invariants in safe code (whose violation cannot result in unsafety).
§
Custom Messages
This macro has a second form, where a custom panic message can
be provided with or without arguments for formatting. See
std::fmt
for syntax for this form. Expressions used as format arguments will only
be evaluated if the assertion fails.
§
Examples
// the panic message for these assertions is the stringified value of the
// expression given.
assert!
(
true
);
fn
some_computation() -> bool {
true
}
// a very simple function
assert!
(some_computation());
// assert with a custom message
let
x =
true
;
assert!
(x,
"x wasn't true!"
);
let
a =
3
;
let
b =
27
;
assert!
(a + b ==
30
,
"a = {}, b = {}"
, a, b);