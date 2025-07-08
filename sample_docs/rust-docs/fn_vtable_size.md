vtable_size in std::intrinsics - Rust
std
::
intrinsics
Function
vtable_size
Copy item path
Source
pub unsafe fn vtable_size(ptr:
*const
()
) ->
usize
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
The intrinsic will return the size stored in that vtable.
§
Safety
ptr
must point to a vtable.