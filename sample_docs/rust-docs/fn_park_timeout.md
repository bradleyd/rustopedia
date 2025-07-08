park_timeout in std::thread - Rust
std
::
thread
Function
park_timeout
Copy item path
1.4.0
·
Source
pub fn park_timeout(dur:
Duration
)
Expand description
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
dur
long.
See the
park documentation
for more details.
§
Platform-specific behavior
Platforms which do not support nanosecond precision for sleeping will have
dur
rounded up to the nearest granularity of time they can sleep for.
§
Examples
Waiting for the complete expiration of the timeout:
use
std::thread::park_timeout;
use
std::time::{Instant, Duration};
let
timeout = Duration::from_secs(
2
);
let
beginning_park = Instant::now();
let
mut
timeout_remaining = timeout;
loop
{
    park_timeout(timeout_remaining);
let
elapsed = beginning_park.elapsed();
if
elapsed >= timeout {
break
;
    }
println!
(
"restarting park_timeout after {elapsed:?}"
);
    timeout_remaining = timeout - elapsed;
}