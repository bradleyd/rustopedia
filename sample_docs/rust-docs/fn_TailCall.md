TailCall in std::intrinsics::mir - Rust
std
::
intrinsics
::
mir
Function
TailCall
Copy item path
Source
pub fn TailCall<T>(call: T)
🔬
This is a nightly-only experimental API. (
custom_mir
)
Expand description
Call a function.
The argument must be of the form
fun(arg1, arg2, ...)
.