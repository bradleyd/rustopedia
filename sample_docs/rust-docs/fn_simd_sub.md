simd_sub in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_sub
Copy item path
Source
pub unsafe fn simd_sub<T>(lhs: T, rhs: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Subtracts
rhs
from
lhs
elementwise.
T
must be a vector of integers or floats.