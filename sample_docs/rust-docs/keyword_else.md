else - Rust
Keyword
else
Copy item path
Source
Expand description
What expression to evaluate when an
if
condition evaluates to
false
.
else
expressions are optional. When no else expressions are supplied it is assumed to evaluate
to the unit type
()
.
The type that the
else
blocks evaluate to must be compatible with the type that the
if
block
evaluates to.
As can be seen below,
else
must be followed by either:
if
,
if let
, or a block
{}
and it
will return the value of that expression.
let
result =
if
true
==
false
{
"oh no"
}
else if
"something"
==
"other thing"
{
"oh dear"
}
else if let
Some
(
200
) =
"blarg"
.parse::<i32>().ok() {
"uh oh"
}
else
{
println!
(
"Sneaky side effect."
);
"phew, nothing's broken"
};
Here’s another example but here we do not try and return an expression:
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
The above is
still
an expression but it will always evaluate to
()
.
There is possibly no limit to the number of
else
blocks that could follow an
if
expression
however if you have several then a
match
expression might be preferable.
Read more about control flow in the
Rust Book
.