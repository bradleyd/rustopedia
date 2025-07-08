spin_loop in std::hint - Rust
std
::
hint
Function
spin_loop
Copy item path
1.49.0
·
Source
pub fn spin_loop()
Expand description
Emits a machine instruction to signal the processor that it is running in
a busy-wait spin-loop (“spin lock”).
Upon receiving the spin-loop signal the processor can optimize its behavior by,
for example, saving power or switching hyper-threads.
This function is different from
thread::yield_now
which directly
yields to the system’s scheduler, whereas
spin_loop
does not interact
with the operating system.
A common use case for
spin_loop
is implementing bounded optimistic
spinning in a CAS loop in synchronization primitives. To avoid problems
like priority inversion, it is strongly recommended that the spin loop is
terminated after a finite amount of iterations and an appropriate blocking
syscall is made.
Note
: On platforms that do not support receiving spin-loop hints this
function does not do anything at all.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
use
std::sync::Arc;
use
std::{hint, thread};
// A shared atomic value that threads will use to coordinate
let
live = Arc::new(AtomicBool::new(
false
));
// In a background thread we'll eventually set the value
let
bg_work = {
let
live = live.clone();
    thread::spawn(
move
|| {
// Do some work, then make the value live
do_some_work();
        live.store(
true
, Ordering::Release);
    })
};
// Back on our current thread, we wait for the value to be set
while
!live.load(Ordering::Acquire) {
// The spin loop is a hint to the CPU that we're waiting, but probably
    // not for very long
hint::spin_loop();
}
// The value is now set
do_some_work();
bg_work.join()
?
;