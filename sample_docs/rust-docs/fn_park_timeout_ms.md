park_timeout_ms in std::thread - Rust
std
::
thread
Function
park_timeout_ms
Copy item path
1.0.0
·
Source
pub fn park_timeout_ms(ms:
u32
)
👎
Deprecated since 1.6.0: replaced by
std::thread::park_timeout
Expand description
Uses
park_timeout
.
Blocks unless or until the current thread’s token is made available or
the specified duration has been reached (may wake spuriously).
The semantics of this function are equivalent to
park
except
that the thread will be blocked for roughly no longer than
dur
. This
method should not be used for precise timing due to anomalies such as
preemption or platform differences that might not cause the maximum
amount of time waited to be precisely
ms
long.
See the
park documentation
for more detail.