simd_cast_ptr in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_cast_ptr
Copy item path
Source
pub unsafe fn simd_cast_ptr<T, U>(ptr: T) -> U
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Casts a vector of pointers.
T
and
U
must be vectors of pointers with the same number of elements.