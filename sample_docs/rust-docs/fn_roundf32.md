roundf32 in std::intrinsics - Rust
std
::
intrinsics
Function
roundf32
Copy item path
Source
pub unsafe fn roundf32(x:
f32
) ->
f32
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the nearest integer to an
f32
. Rounds half-way cases away from zero.
The stabilized version of this intrinsic is
f32::round