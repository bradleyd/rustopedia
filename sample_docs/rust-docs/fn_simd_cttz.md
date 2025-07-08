simd_cttz in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_cttz
Copy item path
Source
pub unsafe fn simd_cttz<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Counts the trailing zeros of each element.
T
must be a vector of integers.