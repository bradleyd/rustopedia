simd_fmax in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_fmax
Copy item path
Source
pub unsafe fn simd_fmax<T>(x: T, y: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the maximum of two vectors, elementwise.
T
must be a vector of floating-point primitive types.
Follows IEEE-754
maxNum
semantics.