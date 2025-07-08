PidFd in std::os::linux::process - Rust
std
::
os
::
linux
::
process
Struct
PidFd
Copy item path
Source
pub struct PidFd {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
linux_pidfd
#82971
)
Available on
Linux
only.
Expand description
This type represents a file descriptor that refers to a process.
A
PidFd
can be obtained by setting the corresponding option on
Command
with
create_pidfd
. Subsequently, the created pidfd can be retrieved
from the
Child
by calling
pidfd
or
into_pidfd
.
Example:
#![feature(linux_pidfd)]
use
std::os::linux::process::{CommandExt, ChildExt};
use
std::process::Command;
let
mut
child = Command::new(
"echo"
)
    .create_pidfd(
true
)
    .spawn()
    .expect(
"Failed to spawn child"
);
let
pidfd = child
    .into_pidfd()
    .expect(
"Failed to retrieve pidfd"
);
// The file descriptor will be closed when `pidfd` is dropped.
Refer to the man page of
pidfd_open(2)
for further details.
Implementations
§
Source
§
impl
PidFd
Source
pub fn
kill
(&self) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
linux_pidfd
#82971
)
Forces the child process to exit.
Unlike
Child::kill
it is possible to attempt to kill
reaped children since PidFd does not suffer from pid recycling
races. But doing so will return an Error.
Source
pub fn
wait
(&self) ->
Result
<
ExitStatus
>
🔬
This is a nightly-only experimental API. (
linux_pidfd
#82971
)
Waits for the child to exit completely, returning the status that it exited with.
Unlike
Child::wait
it does not ensure that the stdin handle is closed.
Additionally it will not return an
ExitStatus
if the child
has already been reaped. Instead an error will be returned.
Source
pub fn
try_wait
(&self) ->
Result
<
Option
<
ExitStatus
>>
🔬
This is a nightly-only experimental API. (
linux_pidfd
#82971
)
Attempts to collect the exit status of the child if it has already exited.
Unlike
Child::try_wait
this method will return an Error
if the child has already been reaped.
Trait Implementations
§
Source
§
impl
AsFd
for
PidFd
Source
§
fn
as_fd
(&self) ->
BorrowedFd
<'_>
Borrows the file descriptor.
Read more
Source
§
impl
AsRawFd
for
PidFd
Source
§
fn
as_raw_fd
(&self) ->
RawFd
Extracts the raw file descriptor.
Read more
Source
§
impl
Debug
for
PidFd
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
Source
§
impl
From
<
OwnedFd
> for
PidFd
Source
§
fn
from
(fd:
OwnedFd
) -> Self
Converts to this type from the input type.
Source
§
impl
From
<
PidFd
> for
OwnedFd
Source
§
fn
from
(pid_fd:
PidFd
) -> Self
Converts to this type from the input type.
Source
§
impl
FromRawFd
for
PidFd
Source
§
unsafe fn
from_raw_fd
(fd:
RawFd
) -> Self
Constructs a new instance of
Self
from the given raw file
descriptor.
Read more
Source
§
impl
IntoRawFd
for
PidFd
Source
§
fn
into_raw_fd
(self) ->
RawFd
Consumes this object, returning the raw underlying file descriptor.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
PidFd
§
impl
RefUnwindSafe
for
PidFd
§
impl
Send
for
PidFd
§
impl
Sync
for
PidFd
§
impl
Unpin
for
PidFd
§
impl
UnwindSafe
for
PidFd
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