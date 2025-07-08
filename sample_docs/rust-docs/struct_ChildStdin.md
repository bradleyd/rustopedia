ChildStdin in std::process - Rust
std
::
process
Struct
ChildStdin
Copy item path
1.0.0
·
Source
pub struct ChildStdin {
/* private fields */
}
Expand description
A handle to a child process’s standard input (stdin).
This struct is used in the
stdin
field on
Child
.
When an instance of
ChildStdin
is
dropped
, the
ChildStdin
’s underlying
file handle will be closed. If the child process was blocked on input prior
to being dropped, it will become unblocked after dropping.
Trait Implementations
§
1.63.0
·
Source
§
impl
AsFd
for
ChildStdin
Available on
Unix
only.
Source
§
fn
as_fd
(&self) ->
BorrowedFd
<'_>
Borrows the file descriptor.
Read more
1.63.0
·
Source
§
impl
AsHandle
for
ChildStdin
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
AsRawFd
for
ChildStdin
Available on
Unix
only.
Source
§
fn
as_raw_fd
(&self) ->
RawFd
Extracts the raw file descriptor.
Read more
1.2.0
·
Source
§
impl
AsRawHandle
for
ChildStdin
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
1.16.0
·
Source
§
impl
Debug
for
ChildStdin
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
ChildStdin
> for
OwnedFd
Available on
Unix
only.
Source
§
fn
from
(child_stdin:
ChildStdin
) ->
OwnedFd
Takes ownership of a
ChildStdin
’s file descriptor.
1.63.0
·
Source
§
impl
From
<
ChildStdin
> for
OwnedHandle
Available on
Windows
only.
Source
§
fn
from
(child_stdin:
ChildStdin
) ->
OwnedHandle
Takes ownership of a
ChildStdin
’s file handle.
1.20.0
·
Source
§
impl
From
<
ChildStdin
> for
Stdio
Source
§
fn
from
(child:
ChildStdin
) ->
Stdio
Converts a
ChildStdin
into a
Stdio
.
§
Examples
ChildStdin
will be converted to
Stdio
using
Stdio::from
under the hood.
use
std::process::{Command, Stdio};
let
reverse = Command::new(
"rev"
)
    .stdin(Stdio::piped())
    .spawn()
    .expect(
"failed reverse command"
);
let
_echo = Command::new(
"echo"
)
    .arg(
"Hello, world!"
)
    .stdout(reverse.stdin.unwrap())
// Converted into a Stdio here
.output()
    .expect(
"failed echo command"
);
// "!dlrow ,olleH" echoed to console
1.74.0
·
Source
§
impl
From
<
OwnedFd
> for
ChildStdin
Available on
Unix
only.
Creates a
ChildStdin
from the provided
OwnedFd
.
The provided file descriptor must point to a pipe
with the
CLOEXEC
flag set.
Source
§
fn
from
(fd:
OwnedFd
) ->
ChildStdin
ⓘ
Converts to this type from the input type.
1.74.0
·
Source
§
impl
From
<
OwnedHandle
> for
ChildStdin
Available on
Windows
only.
Creates a
ChildStdin
from the provided
OwnedHandle
.
The provided handle must be asynchronous, as reading and
writing from and to it is implemented using asynchronous APIs.
Source
§
fn
from
(handle:
OwnedHandle
) ->
ChildStdin
ⓘ
Converts to this type from the input type.
1.4.0
·
Source
§
impl
IntoRawFd
for
ChildStdin
Available on
Unix
only.
Source
§
fn
into_raw_fd
(self) ->
RawFd
Consumes this object, returning the raw underlying file descriptor.
Read more
1.4.0
·
Source
§
impl
IntoRawHandle
for
ChildStdin
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
1.48.0
·
Source
§
impl
Write
for &
ChildStdin
Source
§
fn
write
(&mut self, buf: &[
u8
]) ->
Result
<
usize
>
Writes a buffer into this writer, returning how many bytes were written.
Read more
Source
§
fn
write_vectored
(&mut self, bufs: &[
IoSlice
<'_>]) ->
Result
<
usize
>
Like
write
, except that it writes from a slice of buffers.
Read more
Source
§
fn
is_write_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if this
Write
r has an efficient
write_vectored
implementation.
Read more
Source
§
fn
flush
(&mut self) ->
Result
<
()
>
Flushes this output stream, ensuring that all intermediately buffered
contents reach their destination.
Read more
1.0.0
·
Source
§
fn
write_all
(&mut self, buf: &[
u8
]) ->
Result
<
()
>
Attempts to write an entire buffer into this writer.
Read more
Source
§
fn
write_all_vectored
(&mut self, bufs: &mut [
IoSlice
<'_>]) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
write_all_vectored
#70436
)
Attempts to write multiple buffers into this writer.
Read more
1.0.0
·
Source
§
fn
write_fmt
(&mut self, args:
Arguments
<'_>) ->
Result
<
()
>
Writes a formatted string into this writer, returning any error
encountered.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adapter for this instance of
Write
.
Read more
1.0.0
·
Source
§
impl
Write
for
ChildStdin
Source
§
fn
write
(&mut self, buf: &[
u8
]) ->
Result
<
usize
>
Writes a buffer into this writer, returning how many bytes were written.
Read more
Source
§
fn
write_vectored
(&mut self, bufs: &[
IoSlice
<'_>]) ->
Result
<
usize
>
Like
write
, except that it writes from a slice of buffers.
Read more
Source
§
fn
is_write_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if this
Write
r has an efficient
write_vectored
implementation.
Read more
Source
§
fn
flush
(&mut self) ->
Result
<
()
>
Flushes this output stream, ensuring that all intermediately buffered
contents reach their destination.
Read more
1.0.0
·
Source
§
fn
write_all
(&mut self, buf: &[
u8
]) ->
Result
<
()
>
Attempts to write an entire buffer into this writer.
Read more
Source
§
fn
write_all_vectored
(&mut self, bufs: &mut [
IoSlice
<'_>]) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
write_all_vectored
#70436
)
Attempts to write multiple buffers into this writer.
Read more
1.0.0
·
Source
§
fn
write_fmt
(&mut self, args:
Arguments
<'_>) ->
Result
<
()
>
Writes a formatted string into this writer, returning any error
encountered.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adapter for this instance of
Write
.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
ChildStdin
§
impl
RefUnwindSafe
for
ChildStdin
§
impl
Send
for
ChildStdin
§
impl
Sync
for
ChildStdin
§
impl
Unpin
for
ChildStdin
§
impl
UnwindSafe
for
ChildStdin
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