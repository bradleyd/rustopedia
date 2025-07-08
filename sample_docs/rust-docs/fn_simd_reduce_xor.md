simd_reduce_xor in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_reduce_xor
Copy item path
Source
pub unsafe fn simd_reduce_xor<T, U>(x: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Logical “exclusive ors” all elements together.
T
must be a vector of integers or floats.
U
must be the element type of
T
.