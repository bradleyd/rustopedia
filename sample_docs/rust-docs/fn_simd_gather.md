simd_gather in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_gather
Copy item path
Source
pub unsafe fn simd_gather<T, U, V>(val: T, ptr: U, mask: V) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Reads a vector of pointers.
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
, read the pointer.
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