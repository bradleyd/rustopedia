saturating_sub in std::intrinsics - Rust
std
::
intrinsics
Function
saturating_sub
Copy item path
Source
pub const fn saturating_sub<T>(a: T, b: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Computes
a - b
, saturating at numeric bounds.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized versions of this intrinsic are available on the integer
primitives via the
saturating_sub
method. For example,
u32::saturating_sub