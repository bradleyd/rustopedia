Write in std::io - Rust
std
::
io
Trait
Write
Copy item path
1.0.0
·
Source
pub trait Write {
    // Required methods
    fn
write
(&mut self, buf: &[
u8
]) ->
Result
<
usize
>;
fn
flush
(&mut self) ->
Result
<
()
>;

    // Provided methods
    fn
write_vectored
(&mut self, bufs: &[
IoSlice
<'_>]) ->
Result
<
usize
> { ... }
fn
is_write_vectored
(&self) ->
bool
{ ... }
fn
write_all
(&mut self, buf: &[
u8
]) ->
Result
<
()
> { ... }
fn
write_all_vectored
(&mut self, bufs: &mut [
IoSlice
<'_>]) ->
Result
<
()
> { ... }
fn
write_fmt
(&mut self, args:
Arguments
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
}
Expand description
A trait for objects which are byte-oriented sinks.
Implementors of the
Write
trait are sometimes called ‘writers’.
Writers are defined by two required methods,
write
and
flush
:
The
write
method will attempt to write some data into the object,
returning how many bytes were successfully written.
The
flush
method is useful for adapters and explicit buffers
themselves for ensuring that all buffered data has been pushed out to the
‘true sink’.
Writers are intended to be composable with one another. Many implementors
throughout
std::io
take and provide types which implement the
Write
trait.
§
Examples
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
data =
b"some bytes"
;
let
mut
pos =
0
;
let
mut
buffer = File::create(
"foo.txt"
)
?
;
while
pos < data.len() {
let
bytes_written = buffer.write(
&
data[pos..])
?
;
        pos += bytes_written;
    }
Ok
(())
}
The trait also provides convenience methods like
write_all
, which calls
write
in a loop until its entire input has been written.
Required Methods
§
1.0.0
·
Source
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
This function will attempt to write the entire contents of
buf
, but
the entire write might not succeed, or the write may also generate an
error. Typically, a call to
write
represents one attempt to write to
any wrapped object.
Calls to
write
are not guaranteed to block waiting for data to be
written, and a write which would otherwise block can be indicated through
an
Err
variant.
If this method consumed
n > 0
bytes of
buf
it must return
Ok(n)
.
If the return value is
Ok(n)
then
n
must satisfy
n <= buf.len()
.
A return value of
Ok(0)
typically means that the underlying object is
no longer able to accept bytes and will likely not be able to in the
future as well, or that the buffer provided is empty.
§
Errors
Each call to
write
may generate an I/O error indicating that the
operation could not be completed. If an error is returned then no bytes
in the buffer were written to this writer.
It is
not
considered an error if the entire buffer could not be
written to this writer.
An error of the
ErrorKind::Interrupted
kind is non-fatal and the
write operation should be retried if there is nothing else to do.
§
Examples
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
mut
buffer = File::create(
"foo.txt"
)
?
;
// Writes some prefix of the byte string, not necessarily all of it.
buffer.write(
b"some bytes"
)
?
;
Ok
(())
}
1.0.0
·
Source
fn
flush
(&mut self) ->
Result
<
()
>
Flushes this output stream, ensuring that all intermediately buffered
contents reach their destination.
§
Errors
It is considered an error if not all bytes could be written due to
I/O errors or EOF being reached.
§
Examples
use
std::io::prelude::
*
;
use
std::io::BufWriter;
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
mut
buffer = BufWriter::new(File::create(
"foo.txt"
)
?
);

    buffer.write_all(
b"some bytes"
)
?
;
    buffer.flush()
?
;
Ok
(())
}
Provided Methods
§
1.36.0
·
Source
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
Data is copied from each buffer in order, with the final buffer
read from possibly being only partially consumed. This method must
behave as a call to
write
with the buffers concatenated would.
The default implementation calls
write
with either the first nonempty
buffer provided, or an empty one if none exists.
§
Examples
use
std::io::IoSlice;
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
data1 = [
1
;
8
];
let
data2 = [
15
;
8
];
let
io_slice1 = IoSlice::new(
&
data1);
let
io_slice2 = IoSlice::new(
&
data2);
let
mut
buffer = File::create(
"foo.txt"
)
?
;
// Writes some prefix of the byte string, not necessarily all of it.
buffer.write_vectored(
&
[io_slice1, io_slice2])
?
;
Ok
(())
}
Source
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
If a
Write
r does not override the default
write_vectored
implementation, code using it may want to avoid the method all together
and coalesce writes into a single buffer for higher performance.
The default implementation returns
false
.
1.0.0
·
Source
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
This method will continuously call
write
until there is no more data
to be written or an error of non-
ErrorKind::Interrupted
kind is
returned. This method will not return until the entire buffer has been
successfully written or such an error occurs. The first error that is
not of
ErrorKind::Interrupted
kind generated from this method will be
returned.
If the buffer contains no data, this will never call
write
.
§
Errors
This function will return the first error of
non-
ErrorKind::Interrupted
kind that
write
returns.
§
Examples
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
mut
buffer = File::create(
"foo.txt"
)
?
;

    buffer.write_all(
b"some bytes"
)
?
;
Ok
(())
}
Source
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
This method will continuously call
write_vectored
until there is no
more data to be written or an error of non-
ErrorKind::Interrupted
kind is returned. This method will not return until all buffers have
been successfully written or such an error occurs. The first error that
is not of
ErrorKind::Interrupted
kind generated from this method
will be returned.
If the buffer contains no data, this will never call
write_vectored
.
§
Notes
Unlike
write_vectored
, this takes a
mutable
reference to
a slice of
IoSlice
s, not an immutable one. That’s because we need to
modify the slice to keep track of the bytes already written.
Once this function returns, the contents of
bufs
are unspecified, as
this depends on how many calls to
write_vectored
were necessary. It is
best to understand this function as taking ownership of
bufs
and to
not use
bufs
afterwards. The underlying buffers, to which the
IoSlice
s point (but not the
IoSlice
s themselves), are unchanged and
can be reused.
§
Examples
#![feature(write_all_vectored)]
use
std::io::{Write, IoSlice};
let
mut
writer = Vec::new();
let
bufs =
&mut
[
    IoSlice::new(
&
[
1
]),
    IoSlice::new(
&
[
2
,
3
]),
    IoSlice::new(
&
[
4
,
5
,
6
]),
];

writer.write_all_vectored(bufs)
?
;
// Note: the contents of `bufs` is now undefined, see the Notes section.
assert_eq!
(writer,
&
[
1
,
2
,
3
,
4
,
5
,
6
]);
1.0.0
·
Source
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
This method is primarily used to interface with the
format_args!()
macro, and it is rare that this should
explicitly be called. The
write!()
macro should be favored to
invoke this method instead.
This function internally uses the
write_all
method on
this trait and hence will continuously write data so long as no errors
are received. This also means that partial writes are not indicated in
this signature.
§
Errors
This function will return any I/O error reported while formatting.
§
Examples
use
std::io::prelude::
*
;
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
mut
buffer = File::create(
"foo.txt"
)
?
;
// this call
write!
(buffer,
"{:.*}"
,
2
,
1.234567
)
?
;
// turns into this:
buffer.write_fmt(
format_args!
(
"{:.*}"
,
2
,
1.234567
))
?
;
Ok
(())
}
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
Creates a “by reference” adapter for this instance of
Write
.
The returned adapter also implements
Write
and will simply borrow this
current writer.
§
Examples
use
std::io::Write;
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
mut
buffer = File::create(
"foo.txt"
)
?
;
let
reference = buffer.by_ref();
// we can use reference just like our original buffer
reference.write_all(
b"some bytes"
)
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
Write
for &
File
1.0.0
·
Source
§
impl
Write
for &
TcpStream
1.48.0
·
Source
§
impl
Write
for &
ChildStdin
1.73.0
·
Source
§
impl
Write
for &
Empty
1.87.0
·
Source
§
impl
Write
for &
PipeWriter
1.48.0
·
Source
§
impl
Write
for &
Sink
1.48.0
·
Source
§
impl
Write
for &
Stderr
1.48.0
·
Source
§
impl
Write
for &
Stdout
1.0.0
·
Source
§
impl
Write
for &mut [
u8
]
Write is implemented for
&mut [u8]
by copying into the slice, overwriting
its data.
Note that writing updates the slice to point to the yet unwritten part.
The slice will be empty when it has been completely overwritten.
If the number of bytes to be written exceeds the size of the slice, write operations will
return short writes: ultimately,
Ok(0)
; in this situation,
write_all
returns an error of
kind
ErrorKind::WriteZero
.
1.0.0
·
Source
§
impl
Write
for
File
1.0.0
·
Source
§
impl
Write
for
TcpStream
1.10.0
·
Source
§
impl
Write
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
Write
for
ChildStdin
1.73.0
·
Source
§
impl
Write
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
Write
for
Cursor
<&mut [
u8
]>
1.73.0
·
Source
§
impl
Write
for
Empty
1.87.0
·
Source
§
impl
Write
for
PipeWriter
1.0.0
·
Source
§
impl
Write
for
Sink
1.0.0
·
Source
§
impl
Write
for
Stderr
1.0.0
·
Source
§
impl
Write
for
StderrLock
<'_>
1.0.0
·
Source
§
impl
Write
for
Stdout
1.0.0
·
Source
§
impl
Write
for
StdoutLock
<'_>
1.10.0
·
Source
§
impl<'a>
Write
for &'a
UnixStream
Available on
Unix
only.
Source
§
impl<'a>
Write
for
BorrowedCursor
<'a>
1.25.0
·
Source
§
impl<A>
Write
for
Cursor
<&mut
Vec
<
u8
, A>>
where
    A:
Allocator
,
1.5.0
·
Source
§
impl<A>
Write
for
Cursor
<
Box
<[
u8
], A>>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<A>
Write
for
Cursor
<
Vec
<
u8
, A>>
where
    A:
Allocator
,
1.63.0
·
Source
§
impl<A:
Allocator
>
Write
for
VecDeque
<
u8
, A>
Write is implemented for
VecDeque<u8>
by appending to the
VecDeque
, growing it as needed.
1.0.0
·
Source
§
impl<A:
Allocator
>
Write
for
Vec
<
u8
, A>
Write is implemented for
Vec<u8>
by appending to the vector.
The vector will grow as needed.
1.0.0
·
Source
§
impl<W:
Write
+ ?
Sized
>
Write
for
&mut W
1.0.0
·
Source
§
impl<W:
Write
+ ?
Sized
>
Write
for
Box
<W>
1.0.0
·
Source
§
impl<W: ?
Sized
+
Write
>
Write
for
BufWriter
<W>
1.0.0
·
Source
§
impl<W: ?
Sized
+
Write
>
Write
for
LineWriter
<W>
1.61.0
·
Source
§
impl<const N:
usize
>
Write
for
Cursor
<[
u8
;
N
]>