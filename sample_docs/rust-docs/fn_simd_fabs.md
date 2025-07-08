simd_fabs in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_fabs
Copy item path
Source
pub unsafe fn simd_fabs<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns absolute value of a vector, elementwise.
T
must be a vector of floating-point primitive types.