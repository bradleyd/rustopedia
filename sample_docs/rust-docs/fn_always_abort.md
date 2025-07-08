always_abort in std::panic - Rust
std
::
panic
Function
always_abort
Copy item path
Source
pub fn always_abort()
🔬
This is a nightly-only experimental API. (
panic_always_abort
#84438
)
Expand description
Makes all future panics abort directly without running the panic hook or unwinding.
There is no way to undo this; the effect lasts until the process exits or
execs (or the equivalent).
§
Use after fork
This function is particularly useful for calling after
libc::fork
.  After
fork
, in a
multithreaded program it is (on many platforms) not safe to call the allocator.  It is also
generally highly undesirable for an unwind to unwind past the
fork
, because that results in
the unwind propagating to code that was only ever expecting to run in the parent.
panic::always_abort()
helps avoid both of these.  It directly avoids any further unwinding,
and if there is a panic, the abort will occur without allocating provided that the arguments to
panic can be formatted without allocating.
Examples
#![feature(panic_always_abort)]
use
std::panic;

panic::always_abort();
let _
= panic::catch_unwind(|| {
panic!
(
"inside the catch"
);
});
// We will have aborted already, due to the panic.
unreachable!
();