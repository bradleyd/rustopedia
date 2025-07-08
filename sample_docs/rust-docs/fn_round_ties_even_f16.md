round_ties_even_f16 in std::intrinsics - Rust
std
::
intrinsics
Function
round_ties_even_f16
Copy item path
Source
pub fn round_ties_even_f16(x:
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
. Rounds half-way cases to the number with an even
least significant digit.
The stabilized version of this intrinsic is
f16::round_ties_even