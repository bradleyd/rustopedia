simd_trunc in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_trunc
Copy item path
Source
pub unsafe fn simd_trunc<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the integer part of each element as an integer-valued float.
In other words, non-integer values are truncated towards zero.
T
must be a vector of floats.