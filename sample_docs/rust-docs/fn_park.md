park in std::thread - Rust
std
::
thread
Function
park
Copy item path
1.0.0
·
Source
pub fn park()
Expand description
Blocks unless or until the current thread’s token is made available.
A call to
park
does not guarantee that the thread will remain parked
forever, and callers should be prepared for this possibility. However,
it is guaranteed that this function will not panic (it may abort the
process if the implementation encounters some rare errors).
§
park
and
unpark
Every thread is equipped with some basic low-level blocking support, via the
thread::park
function and
thread::Thread::unpark
method.
park
blocks the current thread, which can then be resumed from
another thread by calling the
unpark
method on the blocked thread’s
handle.
Conceptually, each
Thread
handle has an associated token, which is
initially not present:
The
thread::park
function blocks the current thread unless or
until the token is available for its thread handle, at which point it
atomically consumes the token. It may also return
spuriously
, without
consuming the token.
thread::park_timeout
does the same, but allows
specifying a maximum time to block the thread for.
The
unpark
method on a
Thread
atomically makes the token available
if it wasn’t already. Because the token is initially absent,
unpark
followed by
park
will result in the second call returning immediately.
The API is typically used by acquiring a handle to the current thread,
placing that handle in a shared data structure so that other threads can
find it, and then
park
ing in a loop. When some desired condition is met, another
thread calls
unpark
on the handle.
The motivation for this design is twofold:
It avoids the need to allocate mutexes and condvars when building new
synchronization primitives; the threads already provide basic
blocking/signaling.
It can be implemented very efficiently on many platforms.
§
Memory Ordering
Calls to
unpark
synchronize-with
calls to
park
, meaning that memory
operations performed before a call to
unpark
are made visible to the thread that
consumes the token and returns from
park
. Note that all
park
and
unpark
operations for a given thread form a total order and
all
prior
unpark
operations
synchronize-with
park
.
In atomic ordering terms,
unpark
performs a
Release
operation and
park
performs the corresponding
Acquire
operation. Calls to
unpark
for the same
thread form a
release sequence
.
Note that being unblocked does not imply a call was made to
unpark
, because
wakeups can also be spurious. For example, a valid, but inefficient,
implementation could have
park
and
unpark
return immediately without doing anything,
making
all
wakeups spurious.
§
Examples
use
std::thread;
use
std::sync::{Arc, atomic::{Ordering, AtomicBool}};
use
std::time::Duration;
let
flag = Arc::new(AtomicBool::new(
false
));
let
flag2 = Arc::clone(
&
flag);
let
parked_thread = thread::spawn(
move
|| {
// We want to wait until the flag is set. We *could* just spin, but using
    // park/unpark is more efficient.
while
!flag2.load(Ordering::Relaxed) {
println!
(
"Parking thread"
);
        thread::park();
// We *could* get here spuriously, i.e., way before the 10ms below are over!
        // But that is no problem, we are in a loop until the flag is set anyway.
println!
(
"Thread unparked"
);
    }
println!
(
"Flag received"
);
});
// Let some time pass for the thread to be spawned.
thread::sleep(Duration::from_millis(
10
));
// Set the flag, and let the thread wake up.
// There is no race condition here, if `unpark`
// happens first, `park` will return immediately.
// Hence there is no risk of a deadlock.
flag.store(
true
, Ordering::Relaxed);
println!
(
"Unpark the thread"
);
parked_thread.thread().unpark();

parked_thread.join().unwrap();