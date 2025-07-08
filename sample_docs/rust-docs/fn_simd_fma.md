simd_fma in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_fma
Copy item path
Source
pub unsafe fn simd_fma<T>(x: T, y: T, z: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Computes
(x*y) + z
for each element, but without any intermediate rounding.
T
must be a vector of floats.