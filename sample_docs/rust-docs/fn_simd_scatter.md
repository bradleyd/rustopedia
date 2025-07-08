simd_scatter in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_scatter
Copy item path
Source
pub unsafe fn simd_scatter<T, U, V>(val: T, ptr: U, mask: V)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Writes to a vector of pointers.
T
must be a vector.
U
must be a vector of pointers to the element type of
T
, with the same length as
T
.
V
must be a vector of signed integers with the same length as
T
(but any element size).
For each pointer in
ptr
, if the corresponding value in
mask
is
!0
, write the
corresponding value in
val
to the pointer.
Otherwise if the corresponding value in
mask
is
0
, do nothing.
The stores happen in left-to-right order.
(This is relevant in case two of the stores overlap.)
§
Safety
Unmasked values in
T
must be writeable as if by
<ptr>::write
(e.g. aligned to the element
type).
mask
must only contain
0
or
!0
values.