simd_and in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_and
Copy item path
Source
pub unsafe fn simd_and<T>(x: T, y: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
“Ands” vectors elementwise.
T
must be a vector of integers.