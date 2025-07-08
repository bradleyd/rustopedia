simd_round in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_round
Copy item path
Source
pub unsafe fn simd_round<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Rounds each element to the closest integer-valued float.
Ties are resolved by rounding away from 0.
T
must be a vector of floats.