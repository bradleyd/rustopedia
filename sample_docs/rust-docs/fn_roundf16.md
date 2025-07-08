roundf16 in std::intrinsics - Rust
std
::
intrinsics
Function
roundf16
Copy item path
Source
pub unsafe fn roundf16(x:
f16
) ->
f16
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the nearest integer to an
f16
. Rounds half-way cases away from zero.
The stabilized version of this intrinsic is
f16::round