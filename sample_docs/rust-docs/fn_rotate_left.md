rotate_left in std::intrinsics - Rust
std
::
intrinsics
Function
rotate_left
Copy item path
Source
pub const fn rotate_left<T>(x: T, shift:
u32
) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs rotate left.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized versions of this intrinsic are available on the integer
primitives via the
rotate_left
method. For example,
u32::rotate_left