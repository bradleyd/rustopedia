simd_saturating_sub in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_saturating_sub
Copy item path
Source
pub unsafe fn simd_saturating_sub<T>(lhs: T, rhs: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Subtracts two simd vectors elementwise, with saturation.
T
must be a vector of integer primitive types.
Subtract
rhs
from
lhs
.