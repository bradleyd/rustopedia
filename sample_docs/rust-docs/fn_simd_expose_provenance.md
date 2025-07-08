simd_expose_provenance in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_expose_provenance
Copy item path
Source
pub unsafe fn simd_expose_provenance<T, U>(ptr: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Exposes a vector of pointers as a vector of addresses.
T
must be a vector of pointers.
U
must be a vector of
usize
with the same length as
T
.