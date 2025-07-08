simd_reduce_max in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_reduce_max
Copy item path
Source
pub unsafe fn simd_reduce_max<T, U>(x: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the maximum element of a vector.
T
must be a vector of integers or floats.
U
must be the element type of
T
.
For floating-point values, uses IEEE-754
maxNum
.