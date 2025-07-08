cold_path in std::intrinsics - Rust
std
::
intrinsics
Function
cold_path
Copy item path
Source
pub const fn cold_path()
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Hints to the compiler that current code path is cold.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
This intrinsic does not have a stable counterpart.