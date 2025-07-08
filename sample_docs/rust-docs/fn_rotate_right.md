rotate_right in std::intrinsics - Rust
std
::
intrinsics
Function
rotate_right
Copy item path
Source
pub const fn rotate_right<T>(x: T, shift:
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
Performs rotate right.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized versions of this intrinsic are available on the integer
primitives via the
rotate_right
method. For example,
u32::rotate_right