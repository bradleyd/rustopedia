assert_matches in std::assert_matches - Rust
std
::
assert_matches
Macro
assert_matches
Copy item path
Source
pub macro assert_matches {
    ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?) => { ... },
    ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)?, $($arg:tt)+) => { ... },
}
🔬
This is a nightly-only experimental API. (
assert_matches
#82775
)
Expand description
Asserts that an expression matches the provided pattern.
This macro is generally preferable to
assert!(matches!(value, pattern))
, because it can print
the debug representation of the actual value shape that did not meet expectations. In contrast,
using
assert!
will only print that expectations were not met, but not why.
The pattern syntax is exactly the same as found in a match arm and the
matches!
macro. The
optional if guard can be used to add additional checks that must be true for the matched value,
otherwise this macro will panic.
Assertions are always checked in both debug and release builds, and cannot
be disabled. See
debug_assert_matches!
for assertions that are disabled in
release builds by default.
On panic, this macro will print the value of the expression with its debug representation.
Like
assert!
, this macro has a second form, where a custom panic message can be provided.
§
Examples
#![feature(assert_matches)]
use
std::assert_matches::assert_matches;
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
assert_matches!
(a,
Some
(
_
));
assert_matches!
(b,
Some
(
_
));
assert_matches!
(a,
Some
(
345
));
assert_matches!
(a,
Some
(
345
) |
None
);
// assert_matches!(a, None); // panics
// assert_matches!(b, Some(345)); // panics
// assert_matches!(b, Some(345) | None); // panics
assert_matches!
(a,
Some
(x)
if
x >
100
);
// assert_matches!(a, Some(x) if x < 100); // panics