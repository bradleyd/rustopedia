write_via_move in std::intrinsics - Rust
std
::
intrinsics
Function
write_via_move
Copy item path
Source
pub const unsafe fn write_via_move<T>(ptr:
*mut T
, value: T)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
This is an implementation detail of
crate::ptr::write
and should
not be used anywhere else.  See its comments for why this exists.
This intrinsic can
only
be called where the pointer is a local without
projections (
write_via_move(ptr, x)
, not
write_via_move(*ptr, x)
) so
that it trivially obeys runtime-MIR rules about derefs in operands.