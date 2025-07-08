fmaf16 in std::intrinsics - Rust
std
::
intrinsics
Function
fmaf16
Copy item path
Source
pub unsafe fn fmaf16(a:
f16
, b:
f16
, c:
f16
) ->
f16
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns
a * b + c
for
f16
values.
The stabilized version of this intrinsic is
f16::mul_add