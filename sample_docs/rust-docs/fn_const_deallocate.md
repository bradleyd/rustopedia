const_deallocate in std::intrinsics - Rust
std
::
intrinsics
Function
const_deallocate
Copy item path
Source
pub const unsafe fn const_deallocate(_ptr:
*mut
u8
, _size:
usize
, _align:
usize
)
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Deallocates a memory which allocated by
intrinsics::const_allocate
at compile time.
At runtime, does nothing.
§
Safety
The
align
argument must be a power of two.
At compile time, a compile error occurs if this constraint is violated.
At runtime, it is not checked.
If the
ptr
is created in an another const, this intrinsic doesn’t deallocate it.
If the
ptr
is pointing to a local variable, this intrinsic doesn’t deallocate it.