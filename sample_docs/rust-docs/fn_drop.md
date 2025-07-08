Drop in std::intrinsics::mir - Rust
std
::
intrinsics
::
mir
Function
Drop
Copy item path
Source
pub fn Drop<T>(place: T, goto:
ReturnToArg
, unwind_action:
UnwindActionArg
)
🔬
This is a nightly-only experimental API. (
custom_mir
)
Expand description
Drop the contents of a place.
The first argument must be a place.
The second argument must be of the form
ReturnTo(bb)
, where
bb
is the basic block that
will be jumped to after the destructor returns.
The third argument describes what happens on unwind. It can be one of:
UnwindContinue
UnwindUnreachable
UnwindTerminate
UnwindCleanup