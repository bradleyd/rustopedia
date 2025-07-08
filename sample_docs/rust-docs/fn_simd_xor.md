simd_xor in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_xor
Copy item path
Source
pub unsafe fn simd_xor<T>(x: T, y: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
“Exclusive ors” vectors elementwise.
T
must be a vector of integers.