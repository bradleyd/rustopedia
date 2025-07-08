if - Rust
Keyword
if
Copy item path
Source
Expand description
Evaluate a block if a condition holds.
if
is a familiar construct to most programmers, and is the main way you’ll often do logic in
your code. However, unlike in most languages,
if
blocks can also act as expressions.
if
1
==
2
{
println!
(
"whoops, mathematics broke"
);
}
else
{
println!
(
"everything's fine!"
);
}
let
greeting =
if
rude {
"sup nerd."
}
else
{
"hello, friend!"
};
if let
Ok
(x) =
"123"
.parse::<i32>() {
println!
(
"{} double that and you get {}!"
, greeting, x *
2
);
}
Shown above are the three typical forms an
if
block comes in. First is the usual kind of
thing you’d see in many languages, with an optional
else
block. Second uses
if
as an
expression, which is only possible if all branches return the same type. An
if
expression can
be used everywhere you’d expect. The third kind of
if
block is an
if let
block, which
behaves similarly to using a
match
expression:
if let
Some
(x) =
Some
(
123
) {
// code
}
else
{
// something else
}
match
Some
(
123
) {
Some
(x) => {
// code
},
_
=> {
// something else
},
}
Each kind of
if
expression can be mixed and matched as needed.
if
true
==
false
{
println!
(
"oh no"
);
}
else if
"something"
==
"other thing"
{
println!
(
"oh dear"
);
}
else if let
Some
(
200
) =
"blarg"
.parse::<i32>().ok() {
println!
(
"uh oh"
);
}
else
{
println!
(
"phew, nothing's broken"
);
}
The
if
keyword is used in one other place in Rust, namely as a part of pattern matching
itself, allowing patterns such as
Some(x) if x > 200
to be used.
For more information on
if
expressions, see the
Rust book
or the
Reference
.