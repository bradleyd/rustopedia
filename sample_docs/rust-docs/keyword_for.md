for - Rust
Keyword
for
Copy item path
Source
Expand description
Iteration with
in
, trait implementation with
impl
, or
higher-ranked trait bounds
(
for<'a>
).
The
for
keyword is used in many syntactic locations:
for
is used in for-in-loops (see below).
for
is used when implementing traits as in
impl Trait for Type
(see
impl
for more info
on that).
for
is also used for
higher-ranked trait bounds
as in
for<'a> &'a T: PartialEq<i32>
.
for-in-loops, or to be more precise, iterator loops, are a simple syntactic sugar over a common
practice within Rust, which is to loop over anything that implements
IntoIterator
until the
iterator returned by
.into_iter()
returns
None
(or the loop body uses
break
).
for
i
in
0
..
5
{
println!
(
"{}"
, i *
2
);
}
for
i
in
std::iter::repeat(
5
) {
println!
(
"turns out {i} never stops being 5"
);
break
;
// would loop forever otherwise
}
'outer
:
for
x
in
5
..
50
{
for
y
in
0
..
10
{
if
x == y {
break
'outer
;
        }
    }
}
As shown in the example above,
for
loops (along with all other loops) can be tagged, using
similar syntax to lifetimes (only visually similar, entirely distinct in practice). Giving the
same tag to
break
breaks the tagged loop, which is useful for inner loops. It is definitely
not a goto.
A
for
loop expands as shown:
for
loop_variable
in
iterator {
    code()
}
{
let
result =
match
IntoIterator::into_iter(iterator) {
mut
iter =>
loop
{
match
iter.next() {
None
=>
break
,
Some
(loop_variable) => { code(); },
            };
        },
    };
    result
}
More details on the functionality shown can be seen at the
IntoIterator
docs.
For more information on for-loops, see the
Rust book
or the
Reference
.
See also,
loop
,
while
.