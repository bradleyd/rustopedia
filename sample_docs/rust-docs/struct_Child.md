Child in std::process - Rust
std
::
process
Struct
Child
Copy item path
1.0.0
·
Source
pub struct Child {
    pub stdin:
Option
<
ChildStdin
>,
    pub stdout:
Option
<
ChildStdout
>,
    pub stderr:
Option
<
ChildStderr
>,
/* private fields */
}
Expand description
Representation of a running or exited child process.
This structure is used to represent and manage child processes. A child
process is created via the
Command
struct, which configures the
spawning process and can itself be constructed using a builder-style
interface.
There is no implementation of
Drop
for child processes,
so if you do not ensure the
Child
has exited then it will continue to
run, even after the
Child
handle to the child process has gone out of
scope.
Calling
wait
(or other functions that wrap around it) will make
the parent process wait until the child has actually exited before
continuing.
§
Warning
On some systems, calling
wait
or similar is necessary for the OS to
release resources. A process that terminated but has not been waited on is
still around as a “zombie”. Leaving too many zombies around may exhaust
global resources (for example process IDs).
The standard library does
not
automatically wait on child processes (not
even if the
Child
is dropped), it is up to the application developer to do
so. As a consequence, dropping
Child
handles without waiting on them first
is not recommended in long-running applications.
§
Examples
ⓘ
use
std::process::Command;
let
mut
child = Command::new(
"/bin/cat"
)
    .arg(
"file.txt"
)
    .spawn()
    .expect(
"failed to execute child"
);
let
ecode = child.wait().expect(
"failed to wait on child"
);
assert!
(ecode.success());
Fields
§
§
stdin:
Option
<
ChildStdin
>
The handle for writing to the child’s standard input (stdin), if it
has been captured. You might find it helpful to do
ⓘ
let
stdin = child.stdin.take().expect(
"handle present"
);
to avoid partially moving the
child
and thus blocking yourself from calling
functions on
child
while using
stdin
.
§
stdout:
Option
<
ChildStdout
>
The handle for reading from the child’s standard output (stdout), if it
has been captured. You might find it helpful to do
ⓘ
let
stdout = child.stdout.take().expect(
"handle present"
);
to avoid partially moving the
child
and thus blocking yourself from calling
functions on
child
while using
stdout
.
§
stderr:
Option
<
ChildStderr
>
The handle for reading from the child’s standard error (stderr), if it
has been captured. You might find it helpful to do
ⓘ
let
stderr = child.stderr.take().expect(
"handle present"
);
to avoid partially moving the
child
and thus blocking yourself from calling
functions on
child
while using
stderr
.
Implementations
§
Source
§
impl
Child
1.0.0
·
Source
pub fn
kill
(&mut self) ->
Result
<
()
>
Forces the child process to exit. If the child has already exited,
Ok(())
is returned.
The mapping to
ErrorKind
s is not part of the compatibility contract of the function.
This is equivalent to sending a SIGKILL on Unix platforms.
§
Examples
use
std::process::Command;
let
mut
command = Command::new(
"yes"
);
if let
Ok
(
mut
child) = command.spawn() {
    child.kill().expect(
"command couldn't be killed"
);
}
else
{
println!
(
"yes command didn't start"
);
}
1.3.0
·
Source
pub fn
id
(&self) ->
u32
Returns the OS-assigned process identifier associated with this child.
§
Examples
use
std::process::Command;
let
mut
command = Command::new(
"ls"
);
if let
Ok
(child) = command.spawn() {
println!
(
"Child's ID is {}"
, child.id());
}
else
{
println!
(
"ls command didn't start"
);
}
1.0.0
·
Source
pub fn
wait
(&mut self) ->
Result
<
ExitStatus
>
Waits for the child to exit completely, returning the status that it
exited with. This function will continue to have the same return value
after it has been called at least once.
The stdin handle to the child process, if any, will be closed
before waiting. This helps avoid deadlock: it ensures that the
child does not block waiting for input from the parent, while
the parent waits for the child to exit.
§
Examples
use
std::process::Command;
let
mut
command = Command::new(
"ls"
);
if let
Ok
(
mut
child) = command.spawn() {
    child.wait().expect(
"command wasn't running"
);
println!
(
"Child has finished its execution!"
);
}
else
{
println!
(
"ls command didn't start"
);
}
1.18.0
·
Source
pub fn
try_wait
(&mut self) ->
Result
<
Option
<
ExitStatus
>>
Attempts to collect the exit status of the child if it has already
exited.
This function will not block the calling thread and will only
check to see if the child process has exited or not. If the child has
exited then on Unix the process ID is reaped. This function is
guaranteed to repeatedly return a successful exit status so long as the
child has already exited.
If the child has exited, then
Ok(Some(status))
is returned. If the
exit status is not available at this time then
Ok(None)
is returned.
If an error occurs, then that error is returned.
Note that unlike
wait
, this function will not attempt to drop stdin.
§
Examples
use
std::process::Command;
let
mut
child = Command::new(
"ls"
).spawn()
?
;
match
child.try_wait() {
Ok
(
Some
(status)) =>
println!
(
"exited with: {status}"
),
Ok
(
None
) => {
println!
(
"status not ready yet, let's really wait"
);
let
res = child.wait();
println!
(
"result: {res:?}"
);
    }
Err
(e) =>
println!
(
"error attempting to wait: {e}"
),
}
1.0.0
·
Source
pub fn
wait_with_output
(self) ->
Result
<
Output
>
Simultaneously waits for the child to exit and collect all remaining
output on the stdout/stderr handles, returning an
Output
instance.
The stdin handle to the child process, if any, will be closed
before waiting. This helps avoid deadlock: it ensures that the
child does not block waiting for input from the parent, while
the parent waits for the child to exit.
By default, stdin, stdout and stderr are inherited from the parent.
In order to capture the output into this
Result<Output>
it is
necessary to create new pipes between parent and child. Use
stdout(Stdio::piped())
or
stderr(Stdio::piped())
, respectively.
§
Examples
ⓘ
use
std::process::{Command, Stdio};
let
child = Command::new(
"/bin/cat"
)
    .arg(
"file.txt"
)
    .stdout(Stdio::piped())
    .spawn()
    .expect(
"failed to execute child"
);
let
output = child
    .wait_with_output()
    .expect(
"failed to wait on child"
);
assert!
(output.status.success());
Trait Implementations
§
1.63.0
·
Source
§
impl
AsHandle
for
Child
Available on
Windows
only.
Source
§
fn
as_handle
(&self) ->
BorrowedHandle
<'_>
Borrows the handle.
Read more
1.2.0
·
Source
§
impl
AsRawHandle
for
Child
Available on
Windows
only.
Source
§
fn
as_raw_handle
(&self) ->
RawHandle
Extracts the raw handle.
Read more
Source
§
impl
ChildExt
for
Child
Available on
Windows
only.
Source
§
fn
main_thread_handle
(&self) ->
BorrowedHandle
<'_>
🔬
This is a nightly-only experimental API. (
windows_process_extensions_main_thread_handle
#96723
)
Extracts the main thread raw handle, without taking ownership
1.16.0
·
Source
§
impl
Debug
for
Child
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
1.63.0
·
Source
§
impl
From
<
Child
> for
OwnedHandle
Available on
Windows
only.
Source
§
fn
from
(child:
Child
) ->
OwnedHandle
Takes ownership of a
Child
’s process handle.
1.4.0
·
Source
§
impl
IntoRawHandle
for
Child
Available on
Windows
only.
Source
§
fn
into_raw_handle
(self) ->
RawHandle
Consumes this object, returning the raw underlying handle.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
Child
§
impl
RefUnwindSafe
for
Child
§
impl
Send
for
Child
§
impl
Sync
for
Child
§
impl
Unpin
for
Child
§
impl
UnwindSafe
for
Child
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.