unchecked_rem in std::intrinsics - Rust
std
::
intrinsics
Function
unchecked_rem
Copy item path
Source
pub const unsafe fn unchecked_rem<T>(x: T, y: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the remainder of an unchecked division, resulting in
undefined behavior when
y == 0
or
x == T::MIN && y == -1
Safe wrappers for this intrinsic are available on the integer
primitives via the
checked_rem
method. For example,
u32::checked_rem