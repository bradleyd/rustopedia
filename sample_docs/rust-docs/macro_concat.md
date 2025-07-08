concat in std - Rust
std
Macro
concat
Copy item path
1.38.0
·
Source
macro_rules! concat {
    ($($e:expr),* $(,)?) => { ... };
}
Expand description
Concatenates literals into a static string slice.
This macro takes any number of comma-separated literals, yielding an
expression of type
&'static str
which represents all of the literals
concatenated left-to-right.
Integer and floating point literals are
stringified
in order to be
concatenated.
§
Examples
let
s =
concat!
(
"test"
,
10
,
'b'
,
true
);
assert_eq!
(s,
"test10btrue"
);