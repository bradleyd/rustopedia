read_via_copy in std::intrinsics - Rust
std
::
intrinsics
Function
read_via_copy
Copy item path
Source
pub const unsafe fn read_via_copy<T>(ptr:
*const T
) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
This is an implementation detail of
crate::ptr::read
and should
not be used anywhere else.  See its comments for why this exists.
This intrinsic can
only
be called where the pointer is a local without
projections (
read_via_copy(ptr)
, not
read_via_copy(*ptr)
) so that it
trivially obeys runtime-MIR rules about derefs in operands.