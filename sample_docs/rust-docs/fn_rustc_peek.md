rustc_peek in std::intrinsics - Rust
std
::
intrinsics
Function
rustc_peek
Copy item path
Source
pub fn rustc_peek<T>(_: T) -> T
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Magic intrinsic that derives its meaning from attributes
attached to the function.
For example, dataflow uses this to inject static assertions so
that
rustc_peek(potentially_uninitialized)
would actually
double-check that dataflow did indeed compute that it is
uninitialized at that point in the control flow.
This intrinsic should not be used outside of the compiler.