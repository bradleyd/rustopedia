simd_mul in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_mul
Copy item path
Source
pub unsafe fn simd_mul<T>(x: T, y: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Multiplies two simd vectors elementwise.
T
must be a vector of integers or floats.