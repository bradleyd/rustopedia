simd_neg in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_neg
Copy item path
Source
pub unsafe fn simd_neg<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Negates a vector elementwise.
T
must be a vector of integers or floats.
Rust panics for
-<int>::Min
due to overflow, but it is not UB with this intrinsic.