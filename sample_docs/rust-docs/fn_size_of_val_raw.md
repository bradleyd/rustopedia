size_of_val_raw in std::mem - Rust
std
::
mem
Function
size_of_val_raw
Copy item path
Source
pub const unsafe fn size_of_val_raw<T>(val:
*const T
) ->
usize
where
    T: ?
Sized
,
🔬
This is a nightly-only experimental API. (
layout_for_ptr
#69835
)
Expand description
Returns the size of the pointed-to value in bytes.
This is usually the same as
size_of::<T>()
. However, when
T
has
no
statically-known size, e.g., a slice
[T]
or a
trait object
,
then
size_of_val_raw
can be used to get the dynamically-known size.
§
Safety
This function is only safe to call if the following conditions hold:
If
T
is
Sized
, this function is always safe to call.
If the unsized tail of
T
is:
a
slice
, then the length of the slice tail must be an initialized
integer, and the size of the
entire value
(dynamic tail length + statically sized prefix) must fit in
isize
.
For the special case where the dynamic tail length is 0, this function
is safe to call.
a
trait object
, then the vtable part of the pointer must point
to a valid vtable acquired by an unsizing coercion, and the size
of the
entire value
(dynamic tail length + statically sized prefix)
must fit in
isize
.
an (unstable)
extern type
, then this function is always safe to
call, but may panic or otherwise return the wrong value, as the
extern type’s layout is not known. This is the same behavior as
size_of_val
on a reference to a type with an extern type tail.
otherwise, it is conservatively not allowed to call this function.
§
Examples
#![feature(layout_for_ptr)]
use
std::mem;
assert_eq!
(
4
, size_of_val(
&
5i32
));
let
x: [u8;
13
] = [
0
;
13
];
let
y:
&
[u8] =
&
x;
assert_eq!
(
13
,
unsafe
{ mem::size_of_val_raw(y) });