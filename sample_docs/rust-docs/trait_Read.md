Read in std::io - Rust
std
::
io
Trait
Read
Copy item path
1.0.0
·
Source
pub trait Read {
    // Required method
    fn
read
(&mut self, buf: &mut [
u8
]) ->
Result
<
usize
>;

    // Provided methods
    fn
read_vectored
(&mut self, bufs: &mut [
IoSliceMut
<'_>]) ->
Result
<
usize
> { ... }
fn
is_read_vectored
(&self) ->
bool
{ ... }
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
> { ... }
fn
read_to_string
(&mut self, buf: &mut
String
) ->
Result
<
usize
> { ... }
fn
read_exact
(&mut self, buf: &mut [
u8
]) ->
Result
<
()
> { ... }
fn
read_buf
(&mut self, buf:
BorrowedCursor
<'_>) ->
Result
<
()
> { ... }
fn
read_buf_exact
(&mut self, cursor:
BorrowedCursor
<'_>) ->
Result
<
()
> { ... }
fn
by_ref
(&mut self) -> &mut Self
where Self:
Sized
{ ... }
fn
bytes
(self) ->
Bytes
<Self>
ⓘ
where Self:
Sized
{ ... }
fn
chain
<R:
Read
>(self, next: R) ->
Chain
<Self, R>
ⓘ
where Self:
Sized
{ ... }
fn
take
(self, limit:
u64
) ->
Take
<Self>
ⓘ
where Self:
Sized
{ ... }
}
Expand description
The
Read
trait allows for reading bytes from a source.
Implementors of the
Read
trait are called ‘readers’.
Readers are defined by one required method,
read()
. Each call to
read()
will attempt to pull bytes from this source into a provided buffer. A
number of other methods are implemented in terms of
read()
, giving
implementors a number of ways to read bytes while only needing to implement
a single method.
Readers are intended to be composable with one another. Many implementors
throughout
std::io
take and provide types which implement the
Read
trait.
Please note that each call to
read()
may involve a system call, and
therefore, using something that implements
BufRead
, such as
BufReader
, will be more efficient.
Repeated calls to the reader use the same cursor, so for example
calling
read_to_end
twice on a
File
will only return the file’s
contents once. It’s recommended to first call
rewind()
in that case.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
let
mut
buffer = [
0
;
10
];
// read up to 10 bytes
f.read(
&mut
buffer)
?
;
let
mut
buffer = Vec::new();
// read the whole file
f.read_to_end(
&mut
buffer)
?
;
// read into a String, so that you don't need to do the conversion.
let
mut
buffer = String::new();
    f.read_to_string(
&mut
buffer)
?
;
// and more! See the other methods for more details.
Ok
(())
}
Read from
&str
because
&[u8]
implements
Read
:
use
std::io::prelude::
*
;
fn
main() -> io::Result<()> {
let
mut
b =
"This string will be read"
.as_bytes();
let
mut
buffer = [
0
;
10
];
// read up to 10 bytes
b.read(
&mut
buffer)
?
;
// etc... it works exactly as a File does!
Ok
(())
}
Required Methods
§
1.0.0
·
Source
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
This function does not provide any guarantees about whether it blocks
waiting for data, but if an object needs to block for a read and cannot,
it will typically signal this via an
Err
return value.
If the return value of this method is
Ok(n)
, then implementations must
guarantee that
0 <= n <= buf.len()
. A nonzero
n
value indicates
that the buffer
buf
has been filled in with
n
bytes of data from this
source. If
n
is
0
, then it can indicate one of two scenarios:
This reader has reached its “end of file” and will likely no longer
be able to produce bytes. Note that this does not mean that the
reader will
always
no longer be able to produce bytes. As an example,
on Linux, this method will call the
recv
syscall for a
TcpStream
,
where returning zero indicates the connection was shut down correctly. While
for
File
, it is possible to reach the end of file and get zero as result,
but if more data is appended to the file, future calls to
read
will return
more data.
The buffer specified was 0 bytes in length.
It is not an error if the returned value
n
is smaller than the buffer size,
even when the reader is not at the end of the stream yet.
This may happen for example because fewer bytes are actually available right now
(e. g. being close to end-of-file) or because read() was interrupted by a signal.
As this trait is safe to implement, callers in unsafe code cannot rely on
n <= buf.len()
for safety.
Extra care needs to be taken when
unsafe
functions are used to access the read bytes.
Callers have to ensure that no unchecked out-of-bounds accesses are possible even if
n > buf.len()
.
Implementations
of this method can make no assumptions about the contents of
buf
when
this function is called. It is recommended that implementations only write data to
buf
instead of reading its contents.
Correspondingly, however,
callers
of this method in unsafe code must not assume
any guarantees about how the implementation uses
buf
. The trait is safe to implement,
so it is possible that the code that’s supposed to write to the buffer might also read
from it. It is your responsibility to make sure that
buf
is initialized
before calling
read
. Calling
read
with an uninitialized
buf
(of the kind one
obtains via
MaybeUninit<T>
) is not safe, and can lead to undefined behavior.
§
Errors
If this function encounters any form of I/O or other error, an error
variant will be returned. If an error is returned then it must be
guaranteed that no bytes were read.
An error of the
ErrorKind::Interrupted
kind is non-fatal and the read
operation should be retried if there is nothing else to do.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
let
mut
buffer = [
0
;
10
];
// read up to 10 bytes
let
n = f.read(
&mut
buffer[..])
?
;
println!
(
"The bytes: {:?}"
,
&
buffer[..n]);
Ok
(())
}
Provided Methods
§
1.36.0
·
Source
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
Data is copied to fill each buffer in order, with the final buffer
written to possibly being only partially filled. This method must
behave equivalently to a single call to
read
with concatenated
buffers.
The default implementation calls
read
with either the first nonempty
buffer provided, or an empty one if none exists.
Source
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
If a
Read
er does not override the default
read_vectored
implementation, code using it may want to avoid the method all together
and coalesce writes into a single buffer for higher performance.
The default implementation returns
false
.
1.0.0
·
Source
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
All bytes read from this source will be appended to the specified buffer
buf
. This function will continuously call
read()
to append more data to
buf
until
read()
returns either
Ok(0)
or an error of
non-
ErrorKind::Interrupted
kind.
If successful, this function will return the total number of bytes read.
§
Errors
If this function encounters an error of the kind
ErrorKind::Interrupted
then the error is ignored and the operation
will continue.
If any other read error is encountered then this function immediately
returns. Any bytes which have already been read will be appended to
buf
.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
let
mut
buffer = Vec::new();
// read the whole file
f.read_to_end(
&mut
buffer)
?
;
Ok
(())
}
(See also the
std::fs::read
convenience function for reading from a
file.)
§
Implementing
read_to_end
When implementing the
io::Read
trait, it is recommended to allocate
memory using
Vec::try_reserve
. However, this behavior is not guaranteed
by all implementations, and
read_to_end
may not handle out-of-memory
situations gracefully.
fn
read_to_end(
&mut
self
, dest_vec:
&mut
Vec<u8>) -> io::Result<usize> {
let
initial_vec_len = dest_vec.len();
loop
{
let
src_buf =
self
.example_datasource.fill_buf()
?
;
if
src_buf.is_empty() {
break
;
        }
        dest_vec.try_reserve(src_buf.len())
?
;
        dest_vec.extend_from_slice(src_buf);
// Any irreversible side effects should happen after `try_reserve` succeeds,
        // to avoid losing data on allocation error.
let
read = src_buf.len();
self
.example_datasource.consume(read);
    }
Ok
(dest_vec.len() - initial_vec_len)
}
1.0.0
·
Source
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
If successful, this function returns the number of bytes which were read
and appended to
buf
.
§
Errors
If the data in this stream is
not
valid UTF-8 then an error is
returned and
buf
is unchanged.
See
read_to_end
for other error semantics.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
let
mut
buffer = String::new();

    f.read_to_string(
&mut
buffer)
?
;
Ok
(())
}
(See also the
std::fs::read_to_string
convenience function for
reading from a file.)
1.6.0
·
Source
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
This function reads as many bytes as necessary to completely fill the
specified buffer
buf
.
Implementations
of this method can make no assumptions about the contents of
buf
when
this function is called. It is recommended that implementations only write data to
buf
instead of reading its contents. The documentation on
read
has a more detailed
explanation of this subject.
§
Errors
If this function encounters an error of the kind
ErrorKind::Interrupted
then the error is ignored and the operation
will continue.
If this function encounters an “end of file” before completely filling
the buffer, it returns an error of the kind
ErrorKind::UnexpectedEof
.
The contents of
buf
are unspecified in this case.
If any other read error is encountered then this function immediately
returns. The contents of
buf
are unspecified in this case.
If this function returns an error, it is unspecified how many bytes it
has read, but it will never read more than would be necessary to
completely fill the buffer.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
let
mut
buffer = [
0
;
10
];
// read exactly 10 bytes
f.read_exact(
&mut
buffer)
?
;
Ok
(())
}
Source
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
This is equivalent to the
read
method, except that it is passed a
BorrowedCursor
rather than
[u8]
to allow use
with uninitialized buffers. The new data will be appended to any existing contents of
buf
.
The default implementation delegates to
read
.
This method makes it possible to return both data and an error but it is advised against.
Source
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
This is similar to the
read_exact
method, except
that it is passed a
BorrowedCursor
rather than
[u8]
to allow use
with uninitialized buffers.
§
Errors
If this function encounters an error of the kind
ErrorKind::Interrupted
then the error is ignored and the operation will continue.
If this function encounters an “end of file” before completely filling
the buffer, it returns an error of the kind
ErrorKind::UnexpectedEof
.
If any other read error is encountered then this function immediately
returns.
If this function returns an error, all bytes read will be appended to
cursor
.
1.0.0
·
Source
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
The returned adapter also implements
Read
and will simply borrow this
current reader.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::Read;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
let
mut
buffer = Vec::new();
let
mut
other_buffer = Vec::new();

    {
let
reference = f.by_ref();
// read at most 5 bytes
reference.take(
5
).read_to_end(
&mut
buffer)
?
;

    }
// drop our &mut reference so we can use f again

    // original file still usable, read the rest
f.read_to_end(
&mut
other_buffer)
?
;
Ok
(())
}
1.0.0
·
Source
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
The returned type implements
Iterator
where the
Item
is
Result
<
u8
,
io::Error
>
.
The yielded item is
Ok
if a byte was successfully read and
Err
otherwise. EOF is mapped to returning
None
from this iterator.
The default implementation calls
read
for each byte,
which can be very inefficient for data that’s not in memory,
such as
File
. Consider using a
BufReader
in such cases.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::prelude::
*
;
use
std::io::BufReader;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
f = BufReader::new(File::open(
"foo.txt"
)
?
);
for
byte
in
f.bytes() {
println!
(
"{}"
, byte
?
);
    }
Ok
(())
}
1.0.0
·
Source
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
The returned
Read
instance will first read all bytes from this object
until EOF is encountered. Afterwards the output is equivalent to the
output of
next
.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
f1 = File::open(
"foo.txt"
)
?
;
let
f2 = File::open(
"bar.txt"
)
?
;
let
mut
handle = f1.chain(f2);
let
mut
buffer = String::new();
// read the value into a String. We could use any Read method here,
    // this is just one example.
handle.read_to_string(
&mut
buffer)
?
;
Ok
(())
}
1.0.0
·
Source
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
This function returns a new instance of
Read
which will read at most
limit
bytes, after which it will always return EOF (
Ok(0)
). Any
read errors will not count towards the number of bytes read and future
calls to
read()
may succeed.
§
Examples
File
s implement
Read
:
use
std::io;
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> io::Result<()> {
let
f = File::open(
"foo.txt"
)
?
;
let
mut
buffer = [
0
;
5
];
// read at most five bytes
let
mut
handle = f.take(
5
);

    handle.read(
&mut
buffer)
?
;
Ok
(())
}
Implementors
§
1.0.0
·
Source
§
impl
Read
for &
File
1.0.0
·
Source
§
impl
Read
for &
TcpStream
1.87.0
·
Source
§
impl
Read
for &
PipeReader
1.78.0
·
Source
§
impl
Read
for &
Stdin
1.0.0
·
Source
§
impl
Read
for &[
u8
]
Read is implemented for
&[u8]
by copying from the slice.
Note that reading updates the slice to point to the yet unread part.
The slice will be empty when EOF is reached.
1.0.0
·
Source
§
impl
Read
for
File
1.0.0
·
Source
§
impl
Read
for
TcpStream
1.10.0
·
Source
§
impl
Read
for
UnixStream
Available on
Unix
only.
1.0.0
·
Source
§
impl
Read
for
ChildStderr
1.0.0
·
Source
§
impl
Read
for
ChildStdout
1.73.0
·
Source
§
impl
Read
for
Arc
<
File
>
1.0.0
·
Source
§
impl
Read
for
Empty
1.87.0
·
Source
§
impl
Read
for
PipeReader
1.0.0
·
Source
§
impl
Read
for
Repeat
1.0.0
·
Source
§
impl
Read
for
Stdin
1.0.0
·
Source
§
impl
Read
for
StdinLock
<'_>
1.10.0
·
Source
§
impl<'a>
Read
for &'a
UnixStream
Available on
Unix
only.
1.63.0
·
Source
§
impl<A:
Allocator
>
Read
for
VecDeque
<
u8
, A>
Read is implemented for
VecDeque<u8>
by consuming bytes from the front of the
VecDeque
.
1.0.0
·
Source
§
impl<R:
Read
+ ?
Sized
>
Read
for
&mut R
1.0.0
·
Source
§
impl<R:
Read
+ ?
Sized
>
Read
for
Box
<R>
1.0.0
·
Source
§
impl<R: ?
Sized
+
Read
>
Read
for
BufReader
<R>
1.0.0
·
Source
§
impl<T>
Read
for
Cursor
<T>
where
    T:
AsRef
<[
u8
]>,
1.0.0
·
Source
§
impl<T:
Read
>
Read
for
Take
<T>
1.0.0
·
Source
§
impl<T:
Read
, U:
Read
>
Read
for
Chain
<T, U>