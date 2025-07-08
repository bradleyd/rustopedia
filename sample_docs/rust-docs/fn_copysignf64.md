copysignf64 in std::intrinsics - Rust
std
::
intrinsics
Function
copysignf64
Copy item path
Source
pub const unsafe fn copysignf64(x:
f64
, y:
f64
) ->
f64
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
f64
values.
The stabilized version of this intrinsic is
f64::copysign