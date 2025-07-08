simd_bswap in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_bswap
Copy item path
Source
pub unsafe fn simd_bswap<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Swaps bytes of each element.
T
must be a vector of integers.