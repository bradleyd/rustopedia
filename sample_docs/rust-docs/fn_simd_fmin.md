simd_fmin in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_fmin
Copy item path
Source
pub unsafe fn simd_fmin<T>(x: T, y: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the minimum of two vectors, elementwise.
T
must be a vector of floating-point primitive types.
Follows IEEE-754
minNum
semantics.