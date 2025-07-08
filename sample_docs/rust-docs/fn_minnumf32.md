minnumf32 in std::intrinsics - Rust
std
::
intrinsics
Function
minnumf32
Copy item path
Source
pub const fn minnumf32(x:
f32
, y:
f32
) ->
f32
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the minimum of two
f32
values.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized version of this intrinsic is
f32::min