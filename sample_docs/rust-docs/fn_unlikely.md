unlikely in std::intrinsics - Rust
std
::
intrinsics
Function
unlikely
Copy item path
Source
pub const fn unlikely(b:
bool
) ->
bool
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Hints to the compiler that branch condition is likely to be false.
Returns the value passed to it.
Any use other than with
if
statements will probably not have an effect.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
This intrinsic does not have a stable counterpart.