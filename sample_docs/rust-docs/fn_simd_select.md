simd_select in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_select
Copy item path
Source
pub unsafe fn simd_select<M, T>(mask: M, if_true: T, if_false: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Selects elements from a mask.
T
must be a vector.
M
must be a signed integer vector with the same length as
T
(but any element size).
For each element, if the corresponding value in
mask
is
!0
, select the element from
if_true
.  If the corresponding value in
mask
is
0
, select the element from
if_false
.
§
Safety
mask
must only contain
0
and
!0
.