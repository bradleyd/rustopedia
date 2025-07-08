caller_location in std::intrinsics - Rust
std
::
intrinsics
Function
caller_location
Copy item path
Source
pub const fn caller_location() -> &'static
Location
<'static>
🔬
This is a nightly-only experimental API. (
core_intrinsics
)
Expand description
Gets a reference to a static
Location
indicating where it was called.
Note that, unlike most intrinsics, this is safe to call;
it does not require an
unsafe
block.
Therefore, implementations must not require the user to uphold
any safety invariants.
Consider using
core::panic::Location::caller
instead.