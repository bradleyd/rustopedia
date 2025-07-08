break - Rust
Keyword
break
Copy item path
Source
Expand description
Exit early from a loop or labelled block.
When
break
is encountered, execution of the associated loop body is
immediately terminated.
let
mut
last =
0
;
for
x
in
1
..
100
{
if
x >
12
{
break
;
    }
    last = x;
}
assert_eq!
(last,
12
);
println!
(
"{last}"
);
A break expression is normally associated with the innermost loop enclosing the
break
but a label can be used to specify which enclosing loop is affected.
'outer
:
for
i
in
1
..=
5
{
println!
(
"outer iteration (i): {i}"
);
'_inner
:
for
j
in
1
..=
200
{
println!
(
"    inner iteration (j): {j}"
);
if
j >=
3
{
// breaks from inner loop, lets outer loop continue.
break
;
        }
if
i >=
2
{
// breaks from outer loop, and directly to "Bye".
break
'outer
;
        }
    }
}
println!
(
"Bye."
);
When associated with
loop
, a break expression may be used to return a value from that loop.
This is only valid with
loop
and not with any other type of loop.
If no value is specified,
break;
returns
()
.
Every
break
within a loop must return the same type.
let
(
mut
a,
mut
b) = (
1
,
1
);
let
result =
loop
{
if
b >
10
{
break
b;
    }
let
c = a + b;
    a = b;
    b = c;
};
// first number in Fibonacci sequence over 10:
assert_eq!
(result,
13
);
println!
(
"{result}"
);
For more details consult the
Reference on “break expression”
and the
Reference on “break and
loop values”
.