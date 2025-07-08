volatile_copy_memory in std::intrinsics - Rust
std
::
intrinsics
Function
volatile_copy_memory
Copy item path
Source
pub unsafe fn volatile_copy_memory<T>(dst:
*mut T
, src:
*const T
, count:
usize
)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Equivalent to the appropriate
llvm.memmove.p0i8.0i8.*
intrinsic, with
a size of
count * size_of::<T>()
and an alignment of
min_align_of::<T>()
The volatile parameter is set to
true
, so it will not be optimized out
unless size is equal to zero.
This intrinsic does not have a stable counterpart.