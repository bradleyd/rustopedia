round_ties_even_f32 in std::intrinsics - Rust
std
::
intrinsics
Function
round_ties_even_f32
Copy item path
Source
pub fn round_ties_even_f32(x:
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
. Rounds half-way cases to the number with an even
least significant digit.
The stabilized version of this intrinsic is
f32::round_ties_even