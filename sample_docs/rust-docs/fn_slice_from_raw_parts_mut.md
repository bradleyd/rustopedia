slice_from_raw_parts_mut in std::ptr - Rust
std
::
ptr
Function
slice_from_raw_parts_mut
Copy item path
1.42.0 (const: 1.83.0)
·
Source
pub const fn slice_from_raw_parts_mut<T>(data:
*mut T
, len:
usize
) ->
*mut
[T]
Expand description
Forms a raw mutable slice from a pointer and a length.
The
len
argument is the number of
elements
, not the number of bytes.
Performs the same functionality as
slice_from_raw_parts
, except that a
raw mutable slice is returned, as opposed to a raw immutable slice.
This function is safe, but actually using the return value is unsafe.
See the documentation of
slice::from_raw_parts_mut
for slice safety requirements.
§
Examples
use
std::ptr;
let
x =
&mut
[
5
,
6
,
7
];
let
raw_pointer = x.as_mut_ptr();
let
slice = ptr::slice_from_raw_parts_mut(raw_pointer,
3
);
unsafe
{
    (
*
slice)[
2
] =
99
;
// assign a value at an index in the slice
};
assert_eq!
(
unsafe
{
&*
slice }[
2
],
99
);
You must ensure that the pointer is valid and not null before dereferencing
the raw slice. A slice reference must never have a null pointer, even if it’s empty.
ⓘ
use
std::ptr;
let
danger:
*mut
[u8] = ptr::slice_from_raw_parts_mut(ptr::null_mut(),
0
);
unsafe
{
    danger.as_mut().expect(
"references must not be null"
);
}