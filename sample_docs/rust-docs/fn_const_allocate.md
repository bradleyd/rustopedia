const_allocate in std::intrinsics - Rust
std
::
intrinsics
Function
const_allocate
Copy item path
Source
pub const unsafe fn const_allocate(_size:
usize
, _align:
usize
) ->
*mut
u8
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Allocates a block of memory at compile time.
At runtime, just returns a null pointer.
§
Safety
The
align
argument must be a power of two.
At compile time, a compile error occurs if this constraint is violated.
At runtime, it is not checked.