abort in std::process - Rust
std
::
process
Function
abort
Copy item path
1.17.0
·
Source
pub fn abort() ->
!
Expand description
Terminates the process in an abnormal fashion.
The function will never return and will immediately terminate the current
process in a platform specific “abnormal” manner. As a consequence,
no destructors on the current stack or any other thread’s stack
will be run, Rust IO buffers (eg, from
BufWriter
) will not be flushed,
and C stdio buffers will (on most platforms) not be flushed.
This is in contrast to the default behavior of
panic!
which unwinds
the current thread’s stack and calls all destructors.
When
panic="abort"
is set, either as an argument to
rustc
or in a
crate’s Cargo.toml,
panic!
and
abort
are similar. However,
panic!
will still call the
panic hook
while
abort
will not.
If a clean shutdown is needed it is recommended to only call
this function at a known point where there are no more destructors left
to run.
The process’s termination will be similar to that from the C
abort()
function.  On Unix, the process will terminate with signal
SIGABRT
, which
typically means that the shell prints “Aborted”.
§
Examples
use
std::process;
fn
main() {
println!
(
"aborting"
);

    process::abort();
// execution never gets here
}
The
abort
function terminates the process, so the destructor will not
get run on the example below:
use
std::process;
struct
HasDrop;
impl
Drop
for
HasDrop {
fn
drop(
&mut
self
) {
println!
(
"This will never be printed!"
);
    }
}
fn
main() {
let
_x = HasDrop;
    process::abort();
// the destructor implemented for HasDrop will never get run
}