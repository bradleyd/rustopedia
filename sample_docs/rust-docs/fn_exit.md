exit in std::process - Rust
std
::
process
Function
exit
Copy item path
1.0.0
·
Source
pub fn exit(code:
i32
) ->
!
Expand description
Terminates the current process with the specified exit code.
This function will never return and will immediately terminate the current
process. The exit code is passed through to the underlying OS and will be
available for consumption by another process.
Note that because this function never returns, and that it terminates the
process, no destructors on the current stack or any other thread’s stack
will be run. If a clean shutdown is needed it is recommended to only call
this function at a known point where there are no more destructors left
to run; or, preferably, simply return a type implementing
Termination
(such as
ExitCode
or
Result
) from the
main
function and avoid this
function altogether:
fn
main() ->
Result
<(), MyError> {
// ...
Ok
(())
}
In its current implementation, this function will execute exit handlers registered with
atexit
as well as other platform-specific exit handlers (e.g.
fini
sections of ELF shared objects).
This means that Rust requires that all exit handlers are safe to execute at any time. In
particular, if an exit handler cleans up some state that might be concurrently accessed by other
threads, it is required that the exit handler performs suitable synchronization with those
threads. (The alternative to this requirement would be to not run exit handlers at all, which is
considered undesirable. Note that returning from
main
also calls
exit
, so making
exit
an
unsafe operation is not an option.)
§
Platform-specific behavior
Unix
: On Unix-like platforms, it is unlikely that all 32 bits of
exit
will be visible to a parent process inspecting the exit code. On most
Unix-like platforms, only the eight least-significant bits are considered.
For example, the exit code for this example will be
0
on Linux, but
256
on Windows:
use
std::process;

process::exit(
0x0100
);
§
Safe interop with C code
On Unix, this function is currently implemented using the
exit
C function
exit
. As
of C23, the C standard does not permit multiple threads to call
exit
concurrently. Rust
mitigates this with a lock, but if C code calls
exit
, that can still cause undefined behavior.
Note that returning from
main
is equivalent to calling
exit
.
Therefore, it is undefined behavior to have two concurrent threads perform the following
without synchronization:
One thread calls Rust’s
exit
function or returns from Rust’s
main
function
Another thread calls the C function
exit
or
quick_exit
, or returns from C’s
main
function
Note that if a binary contains multiple copies of the Rust runtime (e.g., when combining
multiple
cdylib
or
staticlib
), they each have their own separate lock, so from the
perspective of code running in one of the Rust runtimes, the “outside” Rust code is basically C
code, and concurrent
exit
again causes undefined behavior.
Individual C implementations might provide more guarantees than the standard and permit concurrent
calls to
exit
; consult the documentation of your C implementation for details.
For some of the on-going discussion to make
exit
thread-safe in C, see:
Rust issue #126600
Austin Group Bugzilla (for POSIX)
GNU C library Bugzilla