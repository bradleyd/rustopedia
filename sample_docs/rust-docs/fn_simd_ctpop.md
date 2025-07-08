simd_ctpop in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_ctpop
Copy item path
Source
pub unsafe fn simd_ctpop<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Counts the number of ones in each element.
T
must be a vector of integers.