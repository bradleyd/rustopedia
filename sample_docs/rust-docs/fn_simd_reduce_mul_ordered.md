simd_reduce_mul_ordered in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_reduce_mul_ordered
Copy item path
Source
pub unsafe fn simd_reduce_mul_ordered<T, U>(x: T, y: U) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Multiplies elements within a vector from left to right.
T
must be a vector of integers or floats.
U
must be the element type of
T
.
Starting with the value
y
, multiply the elements of
x
and accumulate.