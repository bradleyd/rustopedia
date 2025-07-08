simd_reduce_all in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_reduce_all
Copy item path
Source
pub unsafe fn simd_reduce_all<T>(x: T) ->
bool
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Checks if all mask values are true.
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