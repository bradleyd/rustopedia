type_name in std::intrinsics - Rust
std
::
intrinsics
Function
type_name
Copy item path
Source
pub const fn type_name<T>() -> &'static
str
where
    T: ?
Sized
,
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Gets a static string slice containing the name of a type.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
The stabilized version of this intrinsic is
core::any::type_name
.