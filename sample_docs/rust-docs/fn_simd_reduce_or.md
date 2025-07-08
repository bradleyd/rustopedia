simd_reduce_or in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_reduce_or
Copy item path
Source
pub unsafe fn simd_reduce_or<T, U>(x: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Logical “ors” all elements together.
T
must be a vector of integers or floats.
U
must be the element type of
T
.