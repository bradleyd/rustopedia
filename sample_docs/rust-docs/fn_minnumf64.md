minnumf64 in std::intrinsics - Rust
std
::
intrinsics
Function
minnumf64
Copy item path
Source
pub const fn minnumf64(x:
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
Returns the minimum of two
f64
values.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized version of this intrinsic is
f64::min