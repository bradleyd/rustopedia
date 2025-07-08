abort_unwind in std::panic - Rust
std
::
panic
Function
abort_unwind
Copy item path
Source
pub fn abort_unwind<F, R>(f: F) -> R
where
    F:
FnOnce
() -> R,
🔬
This is a nightly-only experimental API. (
abort_unwind
#130338
)
Expand description
Invokes a closure, aborting if the closure unwinds.
When compiled with aborting panics, this function is effectively a no-op.
With unwinding panics, an unwind results in another call into the panic
hook followed by a process abort.
§
Notes
Instead of using this function, code should attempt to support unwinding.
Implementing
Drop
allows you to restore invariants uniformly in both
return and unwind paths.
If an unwind can lead to logical issues but not soundness issues, you
should allow the unwind. Opting out of
UnwindSafe
indicates to your
consumers that they need to consider correctness in the face of unwinds.
If an unwind would be unsound, then this function should be used in order
to prevent unwinds. However, note that
extern "C" fn
will automatically
convert unwinds to aborts, so using this function isn’t necessary for FFI.