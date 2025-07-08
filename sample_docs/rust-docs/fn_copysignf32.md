copysignf32 in std::intrinsics - Rust
std
::
intrinsics
Function
copysignf32
Copy item path
Source
pub const unsafe fn copysignf32(x:
f32
, y:
f32
) ->
f32
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
f32
values.
The stabilized version of this intrinsic is
f32::copysign