fmaf128 in std::intrinsics - Rust
std
::
intrinsics
Function
fmaf128
Copy item path
Source
pub unsafe fn fmaf128(a:
f128
, b:
f128
, c:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns
a * b + c
for
f128
values.
The stabilized version of this intrinsic is
f128::mul_add