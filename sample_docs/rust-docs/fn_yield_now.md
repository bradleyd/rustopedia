yield_now in std::thread - Rust
std
::
thread
Function
yield_now
Copy item path
1.0.0
·
Source
pub fn yield_now()
Expand description
Cooperatively gives up a timeslice to the OS scheduler.
This calls the underlying OS scheduler’s yield primitive, signaling
that the calling thread is willing to give up its remaining timeslice
so that the OS may schedule other threads on the CPU.
A drawback of yielding in a loop is that if the OS does not have any
other ready threads to run on the current CPU, the thread will effectively
busy-wait, which wastes CPU time and energy.
Therefore, when waiting for events of interest, a programmer’s first
choice should be to use synchronization devices such as
channel
s,
Condvar
s,
Mutex
es or
join
since these primitives are
implemented in a blocking manner, giving up the CPU until the event
of interest has occurred which avoids repeated yielding.
yield_now
should thus be used only rarely, mostly in situations where
repeated polling is required because there is no other suitable way to
learn when an event of interest has occurred.
§
Examples
use
std::thread;

thread::yield_now();