simd_or in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_or
Copy item path
Source
pub unsafe fn simd_or<T>(x: T, y: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
“Ors” vectors elementwise.
T
must be a vector of integers.