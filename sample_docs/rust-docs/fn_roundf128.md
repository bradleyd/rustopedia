roundf128 in std::intrinsics - Rust
std
::
intrinsics
Function
roundf128
Copy item path
Source
pub unsafe fn roundf128(x:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the nearest integer to an
f128
. Rounds half-way cases away from zero.
The stabilized version of this intrinsic is
f128::round