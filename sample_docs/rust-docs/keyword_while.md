while - Rust
Keyword
while
Copy item path
Source
Expand description
Loop while a condition is upheld.
A
while
expression is used for predicate loops. The
while
expression runs the conditional
expression before running the loop body, then runs the loop body if the conditional
expression evaluates to
true
, or exits the loop otherwise.
let
mut
counter =
0
;
while
counter <
10
{
println!
(
"{counter}"
);
    counter +=
1
;
}
Like the
for
expression, we can use
break
and
continue
. A
while
expression
cannot break with a value and always evaluates to
()
unlike
loop
.
let
mut
i =
1
;
while
i <
100
{
    i
*
=
2
;
if
i ==
64
{
break
;
// Exit when `i` is 64.
}
}
As
if
expressions have their pattern matching variant in
if let
, so too do
while
expressions with
while let
. The
while let
expression matches the pattern against the
expression, then runs the loop body if pattern matching succeeds, or exits the loop otherwise.
We can use
break
and
continue
in
while let
expressions just like in
while
.
let
mut
counter =
Some
(
0
);
while let
Some
(i) = counter {
if
i ==
10
{
        counter =
None
;
    }
else
{
println!
(
"{i}"
);
        counter =
Some
(i +
1
);
    }
}
For more information on
while
and loops in general, see the
reference
.
See also,
for
,
loop
.