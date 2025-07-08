simd_shl in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_shl
Copy item path
Source
pub unsafe fn simd_shl<T>(lhs: T, rhs: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Shifts vector left elementwise, with UB on overflow.
Shifts
lhs
left by
rhs
, shifting in sign bits for signed types.
T
must be a vector of integers.
§
Safety
Each element of
rhs
must be less than
<int>::BITS
.