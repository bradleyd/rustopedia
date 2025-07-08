assert_inhabited in std::intrinsics - Rust
std
::
intrinsics
Function
assert_inhabited
Copy item path
Source
pub const fn assert_inhabited<T>()
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
A guard for unsafe functions that cannot ever be executed if
T
is uninhabited:
This will statically either panic, or do nothing.
This intrinsic does not have a stable counterpart.