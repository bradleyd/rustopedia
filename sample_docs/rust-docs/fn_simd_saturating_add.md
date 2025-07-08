simd_saturating_add in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_saturating_add
Copy item path
Source
pub unsafe fn simd_saturating_add<T>(x: T, y: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Adds two simd vectors elementwise, with saturation.
T
must be a vector of integer primitive types.