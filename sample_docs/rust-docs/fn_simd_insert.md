simd_insert in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_insert
Copy item path
Source
pub const unsafe fn simd_insert<T, U>(x: T, idx:
u32
, val: U) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Inserts an element into a vector, returning the updated vector.
T
must be a vector with element type
U
.
§
Safety
idx
must be in-bounds of the vector.