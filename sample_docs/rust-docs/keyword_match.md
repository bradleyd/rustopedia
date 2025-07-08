match - Rust
Keyword
match
Copy item path
Source
Expand description
Control flow based on pattern matching.
match
can be used to run code conditionally. Every pattern must
be handled exhaustively either explicitly or by using wildcards like
_
in the
match
. Since
match
is an expression, values can also be
returned.
let
opt = Option::None::<usize>;
let
x =
match
opt {
Some
(int) => int,
None
=>
10
,
};
assert_eq!
(x,
10
);
let
a_number = Option::Some(
10
);
match
a_number {
Some
(x)
if
x <=
5
=>
println!
(
"0 to 5 num = {x}"
),
Some
(x @
6
..=
10
) =>
println!
(
"6 to 10 num = {x}"
),
None
=>
panic!
(),
// all other numbers
_
=>
panic!
(),
}
match
can be used to gain access to the inner members of an enum
and use them directly.
enum
Outer {
    Double(
Option
<u8>,
Option
<String>),
    Single(
Option
<u8>),
    Empty
}
let
get_inner = Outer::Double(
None
,
Some
(String::new()));
match
get_inner {
    Outer::Double(
None
,
Some
(st)) =>
println!
(
"{st}"
),
    Outer::Single(opt) =>
println!
(
"{opt:?}"
),
_
=>
panic!
(),
}
For more information on
match
and matching in general, see the
Reference
.