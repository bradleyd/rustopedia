sub_with_overflow in std::intrinsics - Rust
std
::
intrinsics
Function
sub_with_overflow
Copy item path
Source
pub const fn sub_with_overflow<T>(x: T, y: T) -> (T,
bool
)
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs checked integer subtraction
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized versions of this intrinsic are available on the integer
primitives via the
overflowing_sub
method. For example,
u32::overflowing_sub