simd_bitreverse in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_bitreverse
Copy item path
Source
pub unsafe fn simd_bitreverse<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Reverses bits of each element.
T
must be a vector of integers.