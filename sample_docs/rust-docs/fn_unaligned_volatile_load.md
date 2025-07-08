unaligned_volatile_load in std::intrinsics - Rust
std
::
intrinsics
Function
unaligned_volatile_load
Copy item path
Source
pub unsafe fn unaligned_volatile_load<T>(src:
*const T
) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs a volatile load from the
src
pointer
The pointer is not required to be aligned.
This intrinsic does not have a stable counterpart.