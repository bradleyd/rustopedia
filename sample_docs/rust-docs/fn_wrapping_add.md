wrapping_add in std::intrinsics - Rust
std
::
intrinsics
Function
wrapping_add
Copy item path
Source
pub const fn wrapping_add<T>(a: T, b: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns (a + b) mod 2
N
, where N is the width of T in bits.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized versions of this intrinsic are available on the integer
primitives via the
wrapping_add
method. For example,
u32::wrapping_add