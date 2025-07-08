unchecked_sub in std::intrinsics - Rust
std
::
intrinsics
Function
unchecked_sub
Copy item path
Source
pub const unsafe fn unchecked_sub<T>(x: T, y: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the result of an unchecked subtraction, resulting in
undefined behavior when
x - y > T::MAX
or
x - y < T::MIN
.
The stable counterpart of this intrinsic is
unchecked_sub
on the various
integer types, such as
u16::unchecked_sub
and
i64::unchecked_sub
.