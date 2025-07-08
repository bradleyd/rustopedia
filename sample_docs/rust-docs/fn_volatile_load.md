volatile_load in std::intrinsics - Rust
std
::
intrinsics
Function
volatile_load
Copy item path
Source
pub unsafe fn volatile_load<T>(src:
*const T
) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs a volatile load from the
src
pointer.
The stabilized version of this intrinsic is
core::ptr::read_volatile
.