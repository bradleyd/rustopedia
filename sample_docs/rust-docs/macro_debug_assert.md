debug_assert in std - Rust
std
Macro
debug_assert
Copy item path
1.0.0
·
Source
macro_rules! debug_assert {
    ($($arg:tt)*) => { ... };
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
Like
assert!
, this macro also has a second version, where a custom panic
message can be provided.
§
Uses
Unlike
assert!
,
debug_assert!
statements are only enabled in non
optimized builds by default. An optimized build will not execute
debug_assert!
statements unless
-C debug-assertions
is passed to the
compiler. This makes
debug_assert!
useful for checks that are too
expensive to be present in a release build but may be helpful during
development. The result of expanding
debug_assert!
is always type checked.
An unchecked assertion allows a program in an inconsistent state to keep
running, which might have unexpected consequences but does not introduce
unsafety as long as this only happens in safe code. The performance cost
of assertions, however, is not measurable in general. Replacing
assert!
with
debug_assert!
is thus only encouraged after thorough profiling, and
more importantly, only in safe code!
§
Examples
// the panic message for these assertions is the stringified value of the
// expression given.
debug_assert!
(
true
);
fn
some_expensive_computation() -> bool {
true
}
// a very simple function
debug_assert!
(some_expensive_computation());
// assert with a custom message
let
x =
true
;
debug_assert!
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
debug_assert!
(a + b ==
30
,
"a = {}, b = {}"
, a, b);