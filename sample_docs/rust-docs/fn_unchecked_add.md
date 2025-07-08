unchecked_add in std::intrinsics - Rust
std
::
intrinsics
Function
unchecked_add
Copy item path
Source
pub const unsafe fn unchecked_add<T>(x: T, y: T) -> T
where
    T:
Copy
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the result of an unchecked addition, resulting in
undefined behavior when
x + y > T::MAX
or
x + y < T::MIN
.
The stable counterpart of this intrinsic is
unchecked_add
on the various
integer types, such as
u16::unchecked_add
and
i64::unchecked_add
.