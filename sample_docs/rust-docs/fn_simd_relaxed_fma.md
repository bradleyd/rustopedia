simd_relaxed_fma in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_relaxed_fma
Copy item path
Source
pub unsafe fn simd_relaxed_fma<T>(x: T, y: T, z: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Computes
(x*y) + z
for each element, non-deterministically executing either
a fused multiply-add or two operations with rounding of the intermediate result.
The operation is fused if the code generator determines that target instruction
set has support for a fused operation, and that the fused operation is more efficient
than the equivalent, separate pair of mul and add instructions. It is unspecified
whether or not a fused operation is selected, and that may depend on optimization
level and context, for example. It may even be the case that some SIMD lanes get fused
and others do not.
T
must be a vector of floats.