roundf64 in std::intrinsics - Rust
std
::
intrinsics
Function
roundf64
Copy item path
Source
pub unsafe fn roundf64(x:
f64
) ->
f64
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the nearest integer to an
f64
. Rounds half-way cases away from zero.
The stabilized version of this intrinsic is
f64::round