simd_shr in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_shr
Copy item path
Source
pub unsafe fn simd_shr<T>(lhs: T, rhs: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Shifts vector right elementwise, with UB on overflow.
T
must be a vector of integers.
Shifts
lhs
right by
rhs
, shifting in sign bits for signed types.
§
Safety
Each element of
rhs
must be less than
<int>::BITS
.