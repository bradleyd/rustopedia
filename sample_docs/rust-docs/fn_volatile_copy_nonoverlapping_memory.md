volatile_copy_nonoverlapping_memory in std::intrinsics - Rust
std
::
intrinsics
Function
volatile_copy_nonoverlapping_memory
Copy item path
Source
pub unsafe fn volatile_copy_nonoverlapping_memory<T>(
    dst:
*mut T
,
    src:
*const T
,
    count:
usize
,
)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Equivalent to the appropriate
llvm.memcpy.p0i8.0i8.*
intrinsic, with
a size of
count
*
size_of::<T>()
and an alignment of
min_align_of::<T>()
This intrinsic does not have a stable counterpart.
§
Safety
The safety requirements are consistent with
copy_nonoverlapping
while the read and write behaviors are volatile,
which means it will not be optimized out unless
_count
or
size_of::<T>()
is equal to zero.