sleep_ms in std::thread - Rust
std
::
thread
Function
sleep_ms
Copy item path
1.0.0
·
Source
pub fn sleep_ms(ms:
u32
)
👎
Deprecated since 1.6.0: replaced by
std::thread::sleep
Expand description
Uses
sleep
.
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
§
Examples
use
std::thread;
// Let's sleep for 2 seconds:
thread::sleep_ms(
2000
);