simd_extract in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_extract
Copy item path
Source
pub const unsafe fn simd_extract<T, U>(x: T, idx:
u32
) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Extracts an element from a vector.
T
must be a vector with element type
U
.
§
Safety
idx
must be in-bounds of the vector.