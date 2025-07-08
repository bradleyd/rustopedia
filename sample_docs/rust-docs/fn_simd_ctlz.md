simd_ctlz in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_ctlz
Copy item path
Source
pub unsafe fn simd_ctlz<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Counts the leading zeros of each element.
T
must be a vector of integers.