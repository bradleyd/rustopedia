unaligned_volatile_store in std::intrinsics - Rust
std
::
intrinsics
Function
unaligned_volatile_store
Copy item path
Source
pub unsafe fn unaligned_volatile_store<T>(dst:
*mut T
, val: T)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs a volatile store to the
dst
pointer.
The pointer is not required to be aligned.
This intrinsic does not have a stable counterpart.