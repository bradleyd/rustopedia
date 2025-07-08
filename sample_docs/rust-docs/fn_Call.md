Call in std::intrinsics::mir - Rust
std
::
intrinsics
::
mir
Function
Call
Copy item path
Source
pub fn Call(call:
()
, goto:
ReturnToArg
, unwind_action:
UnwindActionArg
)
🔬
This is a nightly-only experimental API. (
custom_mir
)
Expand description
Call a function.
The first argument must be of the form
ret_val = fun(arg1, arg2, ...)
.
The second argument must be of the form
ReturnTo(bb)
, where
bb
is the basic block that
will be jumped to after the function returns.
The third argument describes what happens on unwind. It can be one of:
UnwindContinue
UnwindUnreachable
UnwindTerminate
UnwindCleanup