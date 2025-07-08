assert_mem_uninitialized_valid in std::intrinsics - Rust
std
::
intrinsics
Function
assert_mem_uninitialized_valid
Copy item path
Source
pub const fn assert_mem_uninitialized_valid<T>()
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
A guard for
std::mem::uninitialized
. This will statically either panic, or do nothing.
This intrinsic does not have a stable counterpart.