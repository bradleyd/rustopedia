type_id in std::intrinsics - Rust
std
::
intrinsics
Function
type_id
Copy item path
Source
pub const fn type_id<T>() ->
u128
where
    T: 'static + ?
Sized
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Gets an identifier which is globally unique to the specified type. This
function will return the same value for a type regardless of whichever
crate it is invoked in.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized version of this intrinsic is
core::any::TypeId::of
.