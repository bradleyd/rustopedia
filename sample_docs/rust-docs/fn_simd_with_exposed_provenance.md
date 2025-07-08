simd_with_exposed_provenance in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_with_exposed_provenance
Copy item path
Source
pub unsafe fn simd_with_exposed_provenance<T, U>(addr: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Creates a vector of pointers from a vector of addresses.
T
must be a vector of
usize
.
U
must be a vector of pointers, with the same length as
T
.