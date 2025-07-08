copysignf16 in std::intrinsics - Rust
std
::
intrinsics
Function
copysignf16
Copy item path
Source
pub const unsafe fn copysignf16(x:
f16
, y:
f16
) ->
f16
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
f16
values.
The stabilized version of this intrinsic is
f16::copysign