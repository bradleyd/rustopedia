fmaf32 in std::intrinsics - Rust
std
::
intrinsics
Function
fmaf32
Copy item path
Source
pub unsafe fn fmaf32(a:
f32
, b:
f32
, c:
f32
) ->
f32
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns
a * b + c
for
f32
values.
The stabilized version of this intrinsic is
f32::mul_add