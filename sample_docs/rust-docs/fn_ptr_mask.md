ptr_mask in std::intrinsics - Rust
std
::
intrinsics
Function
ptr_mask
Copy item path
Source
pub fn ptr_mask<T>(ptr:
*const T
, mask:
usize
) ->
*const T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Masks out bits of the pointer according to a mask.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
Consider using
pointer::mask
instead.