mut - Rust
Keyword
mut
Copy item path
Source
Expand description
A mutable variable, reference, or pointer.
mut
can be used in several situations. The first is mutable variables,
which can be used anywhere you can bind a value to a variable name. Some
examples:
// A mutable variable in the parameter list of a function.
fn
foo(
mut
x: u8, y: u8) -> u8 {
    x += y;
    x
}
// Modifying a mutable variable.
let
mut
a =
5
;
a =
6
;
assert_eq!
(foo(
3
,
4
),
7
);
assert_eq!
(a,
6
);
The second is mutable references. They can be created from
mut
variables
and must be unique: no other variables can have a mutable reference, nor a
shared reference.
// Taking a mutable reference.
fn
push_two(v:
&mut
Vec<u8>) {
    v.push(
2
);
}
// A mutable reference cannot be taken to a non-mutable variable.
let
mut
v =
vec!
[
0
,
1
];
// Passing a mutable reference.
push_two(
&mut
v);
assert_eq!
(v,
vec!
[
0
,
1
,
2
]);
ⓘ
let
mut
v =
vec!
[
0
,
1
];
let
mut_ref_v =
&mut
v;
let
ref_v =
&
v;
mut_ref_v.push(
2
);
Mutable raw pointers work much like mutable references, with the added
possibility of not pointing to a valid object. The syntax is
*mut Type
.
More information on mutable references and pointers can be found in the
Reference
.