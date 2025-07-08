bitreverse in std::intrinsics - Rust
std
::
intrinsics
Function
bitreverse
Copy item path
Source
pub const fn bitreverse<T>(x: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Reverses the bits in an integer type
T
.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized versions of this intrinsic are available on the integer
primitives via the
reverse_bits
method. For example,
u32::reverse_bits