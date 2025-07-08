simd_div in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_div
Copy item path
Source
pub unsafe fn simd_div<T>(lhs: T, rhs: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Divides
lhs
by
rhs
elementwise.
T
must be a vector of integers or floats.
§
Safety
For integers,
rhs
must not contain any zero elements.
Additionally for signed integers,
<int>::MIN / -1
is undefined behavior.