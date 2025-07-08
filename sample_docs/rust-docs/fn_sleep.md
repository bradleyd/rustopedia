sleep in std::thread - Rust
std
::
thread
Function
sleep
Copy item path
1.4.0
·
Source
pub fn sleep(dur:
Duration
)
Expand description
Puts the current thread to sleep for at least the specified amount of time.
The thread may sleep longer than the duration specified due to scheduling
specifics or platform-dependent functionality. It will never sleep less.
This function is blocking, and should not be used in
async
functions.
§
Platform-specific behavior
On Unix platforms, the underlying syscall may be interrupted by a
spurious wakeup or signal handler. To ensure the sleep occurs for at least
the specified duration, this function may invoke that system call multiple
times.
Platforms which do not support nanosecond precision for sleeping will
have
dur
rounded up to the nearest granularity of time they can sleep for.
Currently, specifying a zero duration on Unix platforms returns immediately
without invoking the underlying
nanosleep
syscall, whereas on Windows
platforms the underlying
Sleep
syscall is always invoked.
If the intention is to yield the current time-slice you may want to use
yield_now
instead.
§
Examples
use
std::{thread, time};
let
ten_millis = time::Duration::from_millis(
10
);
let
now = time::Instant::now();

thread::sleep(ten_millis);
assert!
(now.elapsed() >= ten_millis);