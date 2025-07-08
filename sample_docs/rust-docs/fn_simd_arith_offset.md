simd_arith_offset in std::intrinsics::simd - Rust
std
::
intrinsics
::
simd
Function
simd_arith_offset
Copy item path
Source
pub unsafe fn simd_arith_offset<T, U>(ptr: T, offset: U) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Calculates the offset from a pointer vector elementwise, potentially
wrapping.
T
must be a vector of pointers.
U
must be a vector of
isize
or
usize
with the same number of elements as
T
.
Operates as if by
<ptr>::wrapping_offset
.