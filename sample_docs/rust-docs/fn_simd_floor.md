simd_floor in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_floor
Copy item path
Source
pub unsafe fn simd_floor<T>(x: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Rounds down each element to the next lowest integer-valued float.
T
must be a vector of floats.