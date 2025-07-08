take in std::mem - Rust
std
::
mem
Function
take
Copy item path
1.40.0
·
Source
pub fn take<T>(dest:
&mut T
) -> T
where
    T:
Default
,
Expand description
Replaces
dest
with the default value of
T
, returning the previous
dest
value.
If you want to replace the values of two variables, see
swap
.
If you want to replace with a passed value instead of the default value, see
replace
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
old_v = mem::take(
&mut
v);
assert_eq!
(
vec!
[
1
,
2
], old_v);
assert!
(v.is_empty());
take
allows taking ownership of a struct field by replacing it with an “empty” value.
Without
take
you can run into issues like these:
ⓘ
struct
Buffer<T> { buf: Vec<T> }
impl
<T> Buffer<T> {
fn
get_and_reset(
&mut
self
) -> Vec<T> {
// error: cannot move out of dereference of `&mut`-pointer
let
buf =
self
.buf;
self
.buf = Vec::new();
        buf
    }
}
Note that
T
does not necessarily implement
Clone
, so it can’t even clone and reset
self.buf
. But
take
can be used to disassociate the original value of
self.buf
from
self
, allowing it to be returned:
use
std::mem;
impl
<T> Buffer<T> {
fn
get_and_reset(
&mut
self
) -> Vec<T> {
        mem::take(
&mut
self
.buf)
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
(buffer.buf.len(),
2
);
assert_eq!
(buffer.get_and_reset(),
vec!
[
0
,
1
]);
assert_eq!
(buffer.buf.len(),
0
);