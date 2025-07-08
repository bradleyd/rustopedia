replace in std::mem - Rust
std
::
mem
Function
replace
Copy item path
1.0.0 (const: 1.83.0)
·
Source
pub const fn replace<T>(dest:
&mut T
, src: T) -> T
Expand description
Moves
src
into the referenced
dest
, returning the previous
dest
value.
Neither value is dropped.
If you want to replace the values of two variables, see
swap
.
If you want to replace with a default value, see
take
.
§
Examples
A simple example:
use
std::mem;
let
mut
v: Vec<i32> =
vec!
[
1
,
2
];
let
old_v = mem::replace(
&mut
v,
vec!
[
3
,
4
,
5
]);
assert_eq!
(
vec!
[
1
,
2
], old_v);
assert_eq!
(
vec!
[
3
,
4
,
5
], v);
replace
allows consumption of a struct field by replacing it with another value.
Without
replace
you can run into issues like these:
ⓘ
struct
Buffer<T> { buf: Vec<T> }
impl
<T> Buffer<T> {
fn
replace_index(
&mut
self
, i: usize, v: T) -> T {
// error: cannot move out of dereference of `&mut`-pointer
let
t =
self
.buf[i];
self
.buf[i] = v;
        t
    }
}
Note that
T
does not necessarily implement
Clone
, so we can’t even clone
self.buf[i]
to
avoid the move. But
replace
can be used to disassociate the original value at that index from
self
, allowing it to be returned:
use
std::mem;
impl
<T> Buffer<T> {
fn
replace_index(
&mut
self
, i: usize, v: T) -> T {
        mem::replace(
&mut
self
.buf[i], v)
    }
}
let
mut
buffer = Buffer { buf:
vec!
[
0
,
1
] };
assert_eq!
(buffer.buf[
0
],
0
);
assert_eq!
(buffer.replace_index(
0
,
2
),
0
);
assert_eq!
(buffer.buf[
0
],
2
);