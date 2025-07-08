simd_add in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_add
Copy item path
Source
pub unsafe fn simd_add<T>(x: T, y: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Adds two simd vectors elementwise.
T
must be a vector of integers or floats.