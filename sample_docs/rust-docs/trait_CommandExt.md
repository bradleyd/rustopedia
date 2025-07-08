CommandExt in std::os::unix::process - Rust
std
::
os
::
unix
::
process
Trait
CommandExt
Copy item path
1.0.0
·
Source
pub trait CommandExt: Sealed {
    // Required methods
    fn
uid
(&mut self, id:
u32
) -> &mut
Command
;
fn
gid
(&mut self, id:
u32
) -> &mut
Command
;
fn
groups
(&mut self, groups: &[
u32
]) -> &mut
Command
;
unsafe fn
pre_exec
<F>(&mut self, f: F) -> &mut
Command
where F:
FnMut
() ->
Result
<
()
> +
Send
+
Sync
+ 'static
;
fn
exec
(&mut self) ->
Error
;
fn
arg0
<S>(&mut self, arg: S) -> &mut
Command
where S:
AsRef
<
OsStr
>
;
fn
process_group
(&mut self, pgroup:
i32
) -> &mut
Command
;

    // Provided method
    unsafe fn
before_exec
<F>(&mut self, f: F) -> &mut
Command
where F:
FnMut
() ->
Result
<
()
> +
Send
+
Sync
+ 'static
{ ... }
}
Available on
Unix
only.
Expand description
Unix-specific extensions to the
process::Command
builder.
This trait is sealed: it cannot be implemented outside the standard library.
This is so that future additional methods are not breaking changes.
Required Methods
§
1.0.0
·
Source
fn
uid
(&mut self, id:
u32
) -> &mut
Command
Sets the child process’s user ID. This translates to a
setuid
call in the child process. Failure in the
setuid
call will cause the spawn to fail.
§
Notes
This will also trigger a call to
setgroups(0, NULL)
in the child
process if no groups have been specified.
This removes supplementary groups that might have given the child
unwanted permissions.
1.0.0
·
Source
fn
gid
(&mut self, id:
u32
) -> &mut
Command
Similar to
uid
, but sets the group ID of the child process. This has
the same semantics as the
uid
field.
Source
fn
groups
(&mut self, groups: &[
u32
]) -> &mut
Command
🔬
This is a nightly-only experimental API. (
setgroups
#90747
)
Sets the supplementary group IDs for the calling process. Translates to
a
setgroups
call in the child process.
1.34.0
·
Source
unsafe fn
pre_exec
<F>(&mut self, f: F) -> &mut
Command
where
    F:
FnMut
() ->
Result
<
()
> +
Send
+
Sync
+ 'static,
Schedules a closure to be run just before the
exec
function is
invoked.
The closure is allowed to return an I/O error whose OS error code will
be communicated back to the parent and returned as an error from when
the spawn was requested.
Multiple closures can be registered and they will be called in order of
their registration. If a closure returns
Err
then no further closures
will be called and the spawn operation will immediately return with a
failure.
§
Notes and Safety
This closure will be run in the context of the child process after a
fork
. This primarily means that any modifications made to memory on
behalf of this closure will
not
be visible to the parent process.
This is often a very constrained environment where normal operations
like
malloc
, accessing environment variables through
std::env
or acquiring a mutex are not guaranteed to work (due to
other threads perhaps still running when the
fork
was run).
For further details refer to the
POSIX fork() specification
and the equivalent documentation for any targeted
platform, especially the requirements around
async-signal-safety
.
This also means that all resources such as file descriptors and
memory-mapped regions got duplicated. It is your responsibility to make
sure that the closure does not violate library invariants by making
invalid use of these duplicates.
Panicking in the closure is safe only if all the format arguments for the
panic message can be safely formatted; this is because although
Command
calls
std::panic::always_abort
before calling the pre_exec hook, panic will still try to format the
panic message.
When this closure is run, aspects such as the stdio file descriptors and
working directory have successfully been changed, so output to these
locations might not appear where intended.
1.9.0
·
Source
fn
exec
(&mut self) ->
Error
Performs all the required setup by this
Command
, followed by calling
the
execvp
syscall.
On success this function will not return, and otherwise it will return
an error indicating why the exec (or another part of the setup of the
Command
) failed.
exec
not returning has the same implications as calling
process::exit
– no destructors on the current stack or any other
thread’s stack will be run. Therefore, it is recommended to only call
exec
at a point where it is fine to not run any destructors. Note,
that the
execvp
syscall independently guarantees that all memory is
freed and all file descriptors with the
CLOEXEC
option (set by default
on all file descriptors opened by the standard library) are closed.
This function, unlike
spawn
, will
not
fork
the process to create
a new child. Like spawn, however, the default behavior for the stdio
descriptors will be to inherit them from the current process.
§
Notes
The process may be in a “broken state” if this function returns in
error. For example the working directory, environment variables, signal
handling settings, various user/group information, or aspects of stdio
file descriptors may have changed. If a “transactional spawn” is
required to gracefully handle errors it is recommended to use the
cross-platform
spawn
instead.
1.45.0
·
Source
fn
arg0
<S>(&mut self, arg: S) -> &mut
Command
where
    S:
AsRef
<
OsStr
>,
Set executable argument
Set the first process argument,
argv[0]
, to something other than the
default executable path.
1.64.0
·
Source
fn
process_group
(&mut self, pgroup:
i32
) -> &mut
Command
Sets the process group ID (PGID) of the child process. Equivalent to a
setpgid
call in the child process, but may be more efficient.
Process groups determine which processes receive signals.
§
Examples
Pressing Ctrl-C in a terminal will send SIGINT to all processes in
the current foreground process group. By spawning the
sleep
subprocess in a new process group, it will not receive SIGINT from the
terminal.
The parent process could install a signal handler and manage the
subprocess on its own terms.
A process group ID of 0 will use the process ID as the PGID.
use
std::process::Command;
use
std::os::unix::process::CommandExt;

Command::new(
"sleep"
)
    .arg(
"10"
)
    .process_group(
0
)
    .spawn()
?
.wait()
?
;
Provided Methods
§
1.15.0
·
Source
unsafe fn
before_exec
<F>(&mut self, f: F) -> &mut
Command
where
    F:
FnMut
() ->
Result
<
()
> +
Send
+
Sync
+ 'static,
👎
Deprecated since 1.37.0: should be unsafe, use
pre_exec
instead
Schedules a closure to be run just before the
exec
function is
invoked.
before_exec
used to be a safe method, but it needs to be unsafe since the closure may only
perform operations that are
async-signal-safe
. Hence it got deprecated in favor of the
unsafe
pre_exec
. Meanwhile, Rust gained the ability to make an existing safe method
fully unsafe in a new edition, which is how
before_exec
became
unsafe
. It still also
remains deprecated;
pre_exec
should be used instead.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.0.0
·
Source
§
impl
CommandExt
for
Command