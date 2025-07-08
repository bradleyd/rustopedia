simd_reduce_add_unordered in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_reduce_add_unordered
Copy item path
Source
pub unsafe fn simd_reduce_add_unordered<T, U>(x: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Adds elements within a vector in arbitrary order. May also be re-associated with
unordered additions on the inputs/outputs.
T
must be a vector of integers or floats.
U
must be the element type of
T
.