debug_assert_matches in std::assert_matches - Rust
std
::
assert_matches
Macro
debug_assert_matches
Copy item path
Source
pub macro debug_assert_matches($($arg:tt)*) {
    ...
}
🔬
This is a nightly-only experimental API. (
assert_matches
#82775
)
Expand description
Asserts that an expression matches the provided pattern.
This macro is generally preferable to
debug_assert!(matches!(value, pattern))
, because it can
print the debug representation of the actual value shape that did not meet expectations. In
contrast, using
debug_assert!
will only print that expectations were not met, but not why.
The pattern syntax is exactly the same as found in a match arm and the
matches!
macro. The
optional if guard can be used to add additional checks that must be true for the matched value,
otherwise this macro will panic.
On panic, this macro will print the value of the expression with its debug representation.
Like
assert!
, this macro has a second form, where a custom panic message can be provided.
Unlike
assert_matches!
,
debug_assert_matches!
statements are only enabled in non optimized
builds by default. An optimized build will not execute
debug_assert_matches!
statements unless
-C debug-assertions
is passed to the compiler. This makes
debug_assert_matches!
useful for
checks that are too expensive to be present in a release build but may be helpful during
development. The result of expanding
debug_assert_matches!
is always type checked.
§
Examples
#![feature(assert_matches)]
use
std::assert_matches::debug_assert_matches;
let
a =
Some
(
345
);
let
b =
Some
(
56
);
debug_assert_matches!
(a,
Some
(
_
));
debug_assert_matches!
(b,
Some
(
_
));
debug_assert_matches!
(a,
Some
(
345
));
debug_assert_matches!
(a,
Some
(
345
) |
None
);
// debug_assert_matches!(a, None); // panics
// debug_assert_matches!(b, Some(345)); // panics
// debug_assert_matches!(b, Some(345) | None); // panics
debug_assert_matches!
(a,
Some
(x)
if
x >
100
);
// debug_assert_matches!(a, Some(x) if x < 100); // panics