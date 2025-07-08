minnumf16 in std::intrinsics - Rust
std
::
intrinsics
Function
minnumf16
Copy item path
Source
pub const fn minnumf16(x:
f16
, y:
f16
) ->
f16
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the minimum of two
f16
values.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized version of this intrinsic is
f16::min