debug_assert_eq in std - Rust
std
Macro
debug_assert_eq
Copy item path
1.0.0
·
Source
macro_rules! debug_assert_eq {
    ($($arg:tt)*) => { ... };
}
Expand description
Asserts that two expressions are equal to each other.
On panic, this macro will print the values of the expressions with their
debug representations.
Unlike
assert_eq!
,
debug_assert_eq!
statements are only enabled in non
optimized builds by default. An optimized build will not execute
debug_assert_eq!
statements unless
-C debug-assertions
is passed to the
compiler. This makes
debug_assert_eq!
useful for checks that are too
expensive to be present in a release build but may be helpful during
development. The result of expanding
debug_assert_eq!
is always type checked.
§
Examples
let
a =
3
;
let
b =
1
+
2
;
debug_assert_eq!
(a, b);