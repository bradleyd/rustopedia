unchecked_div in std::intrinsics - Rust
std
::
intrinsics
Function
unchecked_div
Copy item path
Source
pub const unsafe fn unchecked_div<T>(x: T, y: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Performs an unchecked division, resulting in undefined behavior
where
y == 0
or
x == T::MIN && y == -1
Safe wrappers for this intrinsic are available on the integer
primitives via the
checked_div
method. For example,
u32::checked_div