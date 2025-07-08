round_ties_even_f128 in std::intrinsics - Rust
std
::
intrinsics
Function
round_ties_even_f128
Copy item path
Source
pub fn round_ties_even_f128(x:
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
. Rounds half-way cases to the number with an even
least significant digit.
The stabilized version of this intrinsic is
f128::round_ties_even