simd_masked_store in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_masked_store
Copy item path
Source
pub unsafe fn simd_masked_store<V, U, T>(mask: V, ptr: U, val: T)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Writes to a vector of pointers.
T
must be a vector.
U
must be a pointer to the element type of
T
V
must be a vector of signed integers with the same length as
T
(but any element size).
For each element, if the corresponding value in
mask
is
!0
, write the corresponding
value in
val
to the pointer offset from
ptr
.
The first element is written to
ptr
, the second to
ptr.wrapping_offset(1)
and so on.
Otherwise if the corresponding value in
mask
is
0
, do nothing.
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