ChildStdout in std::process - Rust
std
::
process
Struct
ChildStdout
Copy item path
1.0.0
·
Source
pub struct ChildStdout {
/* private fields */
}
Expand description
A handle to a child process’s standard output (stdout).
This struct is used in the
stdout
field on
Child
.
When an instance of
ChildStdout
is
dropped
, the
ChildStdout
’s
underlying file handle will be closed.
Trait Implementations
§
1.63.0
·
Source
§
impl
AsFd
for
ChildStdout
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
ChildStdout
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
ChildStdout
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
ChildStdout
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
ChildStdout
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
ChildStdout
> for
OwnedFd
Available on
Unix
only.
Source
§
fn
from
(child_stdout:
ChildStdout
) ->
OwnedFd
Takes ownership of a
ChildStdout
’s file descriptor.
1.63.0
·
Source
§
impl
From
<
ChildStdout
> for
OwnedHandle
Available on
Windows
only.
Source
§
fn
from
(child_stdout:
ChildStdout
) ->
OwnedHandle
Takes ownership of a
ChildStdout
’s file handle.
1.20.0
·
Source
§
impl
From
<
ChildStdout
> for
Stdio
Source
§
fn
from
(child:
ChildStdout
) ->
Stdio
Converts a
ChildStdout
into a
Stdio
.
§
Examples
ChildStdout
will be converted to
Stdio
using
Stdio::from
under the hood.
use
std::process::{Command, Stdio};
let
hello = Command::new(
"echo"
)
    .arg(
"Hello, world!"
)
    .stdout(Stdio::piped())
    .spawn()
    .expect(
"failed echo command"
);
let
reverse = Command::new(
"rev"
)
    .stdin(hello.stdout.unwrap())
// Converted into a Stdio here
.output()
    .expect(
"failed reverse command"
);
assert_eq!
(reverse.stdout,
b"!dlrow ,olleH\n"
);
1.74.0
·
Source
§
impl
From
<
OwnedFd
> for
ChildStdout
Available on
Unix
only.
Creates a
ChildStdout
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
ChildStdout
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
ChildStdout
Available on
Windows
only.
Creates a
ChildStdout
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
ChildStdout
ⓘ
Converts to this type from the input type.
1.4.0
·
Source
§
impl
IntoRawFd
for
ChildStdout
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
ChildStdout
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
1.0.0
·
Source
§
impl
Read
for
ChildStdout
Source
§
fn
read
(&mut self, buf: &mut [
u8
]) ->
Result
<
usize
>
Pull some bytes from this source into the specified buffer, returning
how many bytes were read.
Read more
Source
§
fn
read_buf
(&mut self, buf:
BorrowedCursor
<'_>) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
read_buf
#78485
)
Pull some bytes from this source into the specified buffer.
Read more
Source
§
fn
read_vectored
(&mut self, bufs: &mut [
IoSliceMut
<'_>]) ->
Result
<
usize
>
Like
read
, except that it reads into a slice of buffers.
Read more
Source
§
fn
is_read_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if this
Read
er has an efficient
read_vectored
implementation.
Read more
Source
§
fn
read_to_end
(&mut self, buf: &mut
Vec
<
u8
>) ->
Result
<
usize
>
Reads all bytes until EOF in this source, placing them into
buf
.
Read more
1.0.0
·
Source
§
fn
read_to_string
(&mut self, buf: &mut
String
) ->
Result
<
usize
>
Reads all bytes until EOF in this source, appending them to
buf
.
Read more
1.6.0
·
Source
§
fn
read_exact
(&mut self, buf: &mut [
u8
]) ->
Result
<
()
>
Reads the exact number of bytes required to fill
buf
.
Read more
Source
§
fn
read_buf_exact
(&mut self, cursor:
BorrowedCursor
<'_>) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
read_buf
#78485
)
Reads the exact number of bytes required to fill
cursor
.
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
Creates a “by reference” adaptor for this instance of
Read
.
Read more
1.0.0
·
Source
§
fn
bytes
(self) ->
Bytes
<Self>
ⓘ
where
    Self:
Sized
,
Transforms this
Read
instance to an
Iterator
over its bytes.
Read more
1.0.0
·
Source
§
fn
chain
<R:
Read
>(self, next: R) ->
Chain
<Self, R>
ⓘ
where
    Self:
Sized
,
Creates an adapter which will chain this stream with another.
Read more
1.0.0
·
Source
§
fn
take
(self, limit:
u64
) ->
Take
<Self>
ⓘ
where
    Self:
Sized
,
Creates an adapter which will read at most
limit
bytes from it.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
ChildStdout
§
impl
RefUnwindSafe
for
ChildStdout
§
impl
Send
for
ChildStdout
§
impl
Sync
for
ChildStdout
§
impl
Unpin
for
ChildStdout
§
impl
UnwindSafe
for
ChildStdout
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