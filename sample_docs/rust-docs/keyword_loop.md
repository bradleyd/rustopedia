loop - Rust
Keyword
loop
Copy item path
Source
Expand description
Loop indefinitely.
loop
is used to define the simplest kind of loop supported in Rust. It runs the code inside
it until the code uses
break
or the program exits.
loop
{
println!
(
"hello world forever!"
);
}
let
mut
i =
1
;
loop
{
println!
(
"i is {i}"
);
if
i >
100
{
break
;
    }
    i
*
=
2
;
}
assert_eq!
(i,
128
);
Unlike the other kinds of loops in Rust (
while
,
while let
, and
for
), loops can be used as
expressions that return values via
break
.
let
mut
i =
1
;
let
something =
loop
{
    i
*
=
2
;
if
i >
100
{
break
i;
    }
};
assert_eq!
(something,
128
);
Every
break
in a loop has to have the same type. When it’s not explicitly giving something,
break;
returns
()
.
For more information on
loop
and loops in general, see the
Reference
.
See also,
for
,
while
.