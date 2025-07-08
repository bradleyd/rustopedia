matches in std - Rust
std
Macro
matches
Copy item path
1.0.0
·
Source
macro_rules! matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => { ... };
}
Expand description
Returns whether the given expression matches the provided pattern.
The pattern syntax is exactly the same as found in a match arm. The optional if guard can be
used to add additional checks that must be true for the matched value, otherwise this macro will
return
false
.
When testing that a value matches a pattern, it’s generally preferable to use
assert_matches!
as it will print the debug representation of the value if the assertion
fails.
§
Examples
let
foo =
'f'
;
assert!
(
matches!
(foo,
'A'
..=
'Z'
|
'a'
..=
'z'
));
let
bar =
Some
(
4
);
assert!
(
matches!
(bar,
Some
(x)
if
x >
2
));