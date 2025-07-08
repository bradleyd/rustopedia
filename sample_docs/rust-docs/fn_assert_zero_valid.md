assert_zero_valid in std::intrinsics - Rust
std
::
intrinsics
Function
assert_zero_valid
Copy item path
Source
pub const fn assert_zero_valid<T>()
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
A guard for unsafe functions that cannot ever be executed if
T
does not permit
zero-initialization: This will statically either panic, or do nothing.
This intrinsic does not have a stable counterpart.