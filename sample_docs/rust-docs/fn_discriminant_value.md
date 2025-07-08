discriminant_value in std::intrinsics - Rust
std
::
intrinsics
Function
discriminant_value
Copy item path
Source
pub const fn discriminant_value<T>(
    v:
&T
,
) -> <T as
DiscriminantKind
>::
Discriminant
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Returns the value of the discriminant for the variant in ‘v’;
if
T
has no discriminant, returns
0
.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized version of this intrinsic is
core::mem::discriminant
.