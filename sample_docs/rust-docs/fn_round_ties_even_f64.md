round_ties_even_f64 in std::intrinsics - Rust
std
::
intrinsics
Function
round_ties_even_f64
Copy item path
Source
pub fn round_ties_even_f64(x:
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
. Rounds half-way cases to the number with an even
least significant digit.
The stabilized version of this intrinsic is
f64::round_ties_even