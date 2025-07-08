copysignf128 in std::intrinsics - Rust
std
::
intrinsics
Function
copysignf128
Copy item path
Source
pub const unsafe fn copysignf128(x:
f128
, y:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Copies the sign from
y
to
x
for
f128
values.
The stabilized version of this intrinsic is
f128::copysign