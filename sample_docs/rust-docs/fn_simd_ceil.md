simd_ceil in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_ceil
Copy item path
Source
pub unsafe fn simd_ceil<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Rounds up each element to the next highest integer-valued float.
T
must be a vector of floats.