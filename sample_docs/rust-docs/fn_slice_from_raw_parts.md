slice_from_raw_parts in std::ptr - Rust
std
::
ptr
Function
slice_from_raw_parts
Copy item path
1.42.0 (const: 1.64.0)
·
Source
pub const fn slice_from_raw_parts<T>(data:
*const T
, len:
usize
) ->
*const
[T]
Expand description
Forms a raw slice from a pointer and a length.
The
len
argument is the number of
elements
, not the number of bytes.
This function is safe, but actually using the return value is unsafe.
See the documentation of
slice::from_raw_parts
for slice safety requirements.
§
Examples
use
std::ptr;
// create a slice pointer when starting out with a pointer to the first element
let
x = [
5
,
6
,
7
];
let
raw_pointer = x.as_ptr();
let
slice = ptr::slice_from_raw_parts(raw_pointer,
3
);
assert_eq!
(
unsafe
{
&*
slice }[
2
],
7
);
You must ensure that the pointer is valid and not null before dereferencing
the raw slice. A slice reference must never have a null pointer, even if it’s empty.
ⓘ
use
std::ptr;
let
danger:
*const
[u8] = ptr::slice_from_raw_parts(ptr::null(),
0
);
unsafe
{
    danger.as_ref().expect(
"references must not be null"
);
}