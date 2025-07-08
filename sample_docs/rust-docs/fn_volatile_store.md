volatile_store in std::intrinsics - Rust
std
::
intrinsics
Function
volatile_store
Copy item path
Source
pub unsafe fn volatile_store<T>(dst:
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
The stabilized version of this intrinsic is
core::ptr::write_volatile
.