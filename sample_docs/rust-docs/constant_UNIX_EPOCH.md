UNIX_EPOCH in std::time - Rust
std
::
time
Constant
UNIX_EPOCH
Copy item path
1.8.0
·
Source
pub const UNIX_EPOCH:
SystemTime
;
Expand description
An anchor in time which can be used to create new
SystemTime
instances or
learn about where in time a
SystemTime
lies.
This constant is defined to be “1970-01-01 00:00:00 UTC” on all systems with
respect to the system clock. Using
duration_since
on an existing
SystemTime
instance can tell how far away from this point in time a
measurement lies, and using
UNIX_EPOCH + duration
can be used to create a
SystemTime
instance to represent another fixed point in time.
duration_since(UNIX_EPOCH).unwrap().as_secs()
returns
the number of non-leap seconds since the start of 1970 UTC.
This is a POSIX
time_t
(as a
u64
),
and is the same time representation as used in many Internet protocols.
§
Examples
use
std::time::{SystemTime, UNIX_EPOCH};
match
SystemTime::now().duration_since(UNIX_EPOCH) {
Ok
(n) =>
println!
(
"1970-01-01 00:00:00 UTC was {} seconds ago!"
, n.as_secs()),
Err
(
_
) =>
panic!
(
"SystemTime before UNIX EPOCH!"
),
}