unchecked_shl in std::intrinsics - Rust
std
::
intrinsics
Function
unchecked_shl
Copy item path
Source
pub const unsafe fn unchecked_shl<T, U>(x: T, y: U) -> T
where
    T:
Copy
,
    U:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs an unchecked left shift, resulting in undefined behavior when
y < 0
or
y >= N
, where N is the width of T in bits.
Safe wrappers for this intrinsic are available on the integer
primitives via the
checked_shl
method. For example,
u32::checked_shl