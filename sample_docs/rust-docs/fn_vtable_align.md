vtable_align in std::intrinsics - Rust
std
::
intrinsics
Function
vtable_align
Copy item path
Source
pub unsafe fn vtable_align(ptr:
*const
()
) ->
usize
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
The intrinsic will return the alignment stored in that vtable.
§
Safety
ptr
must point to a vtable.