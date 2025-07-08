unchecked_mul in std::intrinsics - Rust
std
::
intrinsics
Function
unchecked_mul
Copy item path
Source
pub const unsafe fn unchecked_mul<T>(x: T, y: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the result of an unchecked multiplication, resulting in
undefined behavior when
x * y > T::MAX
or
x * y < T::MIN
.
The stable counterpart of this intrinsic is
unchecked_mul
on the various
integer types, such as
u16::unchecked_mul
and
i64::unchecked_mul
.