simd_masked_load in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_masked_load
Copy item path
Source
pub unsafe fn simd_masked_load<V, U, T>(mask: V, ptr: U, val: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Reads a vector of pointers.
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
, read the corresponding
pointer offset from
ptr
.
The first element is loaded from
ptr
, the second from
ptr.wrapping_offset(1)
and so on.
Otherwise if the corresponding value in
mask
is
0
, return the corresponding value from
val
.
§
Safety
Unmasked values in
T
must be readable as if by
<ptr>::read
(e.g. aligned to the element
type).
mask
must only contain
0
or
!0
values.