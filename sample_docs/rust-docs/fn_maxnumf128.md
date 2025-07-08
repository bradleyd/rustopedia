maxnumf128 in std::intrinsics - Rust
std
::
intrinsics
Function
maxnumf128
Copy item path
Source
pub const fn maxnumf128(x:
f128
, y:
f128
) ->
f128
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the maximum of two
f128
values.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized version of this intrinsic is
f128::max