simd_ge in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_ge
Copy item path
Source
pub unsafe fn simd_ge<T, U>(x: T, y: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Tests if
x
is greater than or equal to
y
, elementwise.
T
must be a vector of integers or floats.
U
must be a vector of integers with the same number of elements and element size as
T
.
Returns
0
for false and
!0
for true.