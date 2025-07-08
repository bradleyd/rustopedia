concat_bytes in std - Rust
std
Macro
concat_bytes
Copy item path
Source
macro_rules! concat_bytes {
    ($($e:literal),+ $(,)?) => { ... };
}
🔬
This is a nightly-only experimental API. (
concat_bytes
#87555
)
Expand description
Concatenates literals into a byte slice.
This macro takes any number of comma-separated literals, and concatenates them all into
one, yielding an expression of type
&[u8; _]
, which represents all of the literals
concatenated left-to-right. The literals passed can be any combination of:
byte literals (
b'r'
)
byte strings (
b"Rust"
)
arrays of bytes/numbers (
[b'A', 66, b'C']
)
§
Examples
#![feature(concat_bytes)]
let
s:
&
[u8;
6
] =
concat_bytes!
(
b'A'
,
b"BC"
, [
68
,
b'E'
,
70
]);
assert_eq!
(s,
b"ABCDEF"
);