assert_ne in std - Rust
std
Macro
assert_ne
Copy item path
1.0.0
·
Source
macro_rules! assert_ne {
    ($left:expr, $right:expr $(,)?) => { ... };
    ($left:expr, $right:expr, $($arg:tt)+) => { ... };
}
Expand description
Asserts that two expressions are not equal to each other (using
PartialEq
).
Assertions are always checked in both debug and release builds, and cannot
be disabled. See
debug_assert_ne!
for assertions that are disabled in
release builds by default.
On panic, this macro will print the values of the expressions with their
debug representations.
Like
assert!
, this macro has a second form, where a custom
panic message can be provided.
§
Examples
let
a =
3
;
let
b =
2
;
assert_ne!
(a, b);
assert_ne!
(a, b,
"we are testing that the values are not equal"
);