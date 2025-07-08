fmaf64 in std::intrinsics - Rust
std
::
intrinsics
Function
fmaf64
Copy item path
Source
pub unsafe fn fmaf64(a:
f64
, b:
f64
, c:
f64
) ->
f64
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns
a * b + c
for
f64
values.
The stabilized version of this intrinsic is
f64::mul_add