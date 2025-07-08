simd_reduce_any in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_reduce_any
Copy item path
Source
pub unsafe fn simd_reduce_any<T>(x: T) ->
bool
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Checks if any mask value is true.
T
must be a vector of integer primitive types.
§
Safety
x
must contain only
0
or
!0
.