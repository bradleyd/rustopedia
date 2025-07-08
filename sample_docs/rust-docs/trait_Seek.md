Seek in std::io - Rust
std
::
io
Trait
Seek
Copy item path
1.0.0
·
Source
pub trait Seek {
    // Required method
    fn
seek
(&mut self, pos:
SeekFrom
) ->
Result
<
u64
>;

    // Provided methods
    fn
rewind
(&mut self) ->
Result
<
()
> { ... }
fn
stream_len
(&mut self) ->
Result
<
u64
> { ... }
fn
stream_position
(&mut self) ->
Result
<
u64
> { ... }
fn
seek_relative
(&mut self, offset:
i64
) ->
Result
<
()
> { ... }
}
Expand description
The
Seek
trait provides a cursor which can be moved within a stream of
bytes.
The stream typically has a fixed size, allowing seeking relative to either
end or the current offset.
§
Examples
File
s implement
Seek
:
use
std::io;
use
std::io::prelude::
*
;
use
std::fs::File;
use
std::io::SeekFrom;
fn
main() -> io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
// move the cursor 42 bytes from the start of the file
f.seek(SeekFrom::Start(
42
))
?
;
Ok
(())
}
Required Methods
§
1.0.0
·
Source
fn
seek
(&mut self, pos:
SeekFrom
) ->
Result
<
u64
>
Seek to an offset, in bytes, in a stream.
A seek beyond the end of a stream is allowed, but behavior is defined
by the implementation.
If the seek operation completed successfully,
this method returns the new position from the start of the stream.
That position can be used later with
SeekFrom::Start
.
§
Errors
Seeking can fail, for example because it might involve flushing a buffer.
Seeking to a negative offset is considered an error.
Provided Methods
§
1.55.0
·
Source
fn
rewind
(&mut self) ->
Result
<
()
>
Rewind to the beginning of a stream.
This is a convenience method, equivalent to
seek(SeekFrom::Start(0))
.
§
Errors
Rewinding can fail, for example because it might involve flushing a buffer.
§
Example
use
std::io::{Read, Seek, Write};
use
std::fs::OpenOptions;
let
mut
f = OpenOptions::new()
    .write(
true
)
    .read(
true
)
    .create(
true
)
    .open(
"foo.txt"
)
?
;
let
hello =
"Hello!\n"
;
write!
(f,
"{hello}"
)
?
;
f.rewind()
?
;
let
mut
buf = String::new();
f.read_to_string(
&mut
buf)
?
;
assert_eq!
(
&
buf, hello);
Source
fn
stream_len
(&mut self) ->
Result
<
u64
>
🔬
This is a nightly-only experimental API. (
seek_stream_len
#59359
)
Returns the length of this stream (in bytes).
This method is implemented using up to three seek operations. If this
method returns successfully, the seek position is unchanged (i.e. the
position before calling this method is the same as afterwards).
However, if this method returns an error, the seek position is
unspecified.
If you need to obtain the length of
many
streams and you don’t care
about the seek position afterwards, you can reduce the number of seek
operations by simply calling
seek(SeekFrom::End(0))
and using its
return value (it is also the stream length).
Note that length of a stream can change over time (for example, when
data is appended to a file). So calling this method multiple times does
not necessarily return the same length each time.
§
Example
#![feature(seek_stream_len)]
use
std::{
    io::{
self
, Seek},
    fs::File,
};
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
len = f.stream_len()
?
;
println!
(
"The file is currently {len} bytes long"
);
Ok
(())
}
1.51.0
·
Source
fn
stream_position
(&mut self) ->
Result
<
u64
>
Returns the current seek position from the start of the stream.
This is equivalent to
self.seek(SeekFrom::Current(0))
.
§
Example
use
std::{
    io::{
self
, BufRead, BufReader, Seek},
    fs::File,
};
fn
main() -> io::Result<()> {
let
mut
f = BufReader::new(File::open(
"foo.txt"
)
?
);
let
before = f.stream_position()
?
;
    f.read_line(
&mut
String::new())
?
;
let
after = f.stream_position()
?
;
println!
(
"The first line was {} bytes long"
, after - before);
Ok
(())
}
1.80.0
·
Source
fn
seek_relative
(&mut self, offset:
i64
) ->
Result
<
()
>
Seeks relative to the current position.
This is equivalent to
self.seek(SeekFrom::Current(offset))
but
doesn’t return the new position which can allow some implementations
such as
BufReader
to perform more efficient seeks.
§
Example
use
std::{
    io::{
self
, Seek},
    fs::File,
};
fn
main() -> io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
    f.seek_relative(
10
)
?
;
assert_eq!
(f.stream_position()
?
,
10
);
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
Seek
for &
File
1.0.0
·
Source
§
impl
Seek
for
File
1.73.0
·
Source
§
impl
Seek
for
Arc
<
File
>
1.51.0
·
Source
§
impl
Seek
for
Empty
1.0.0
·
Source
§
impl<R: ?
Sized
+
Seek
>
Seek
for
BufReader
<R>
1.0.0
·
Source
§
impl<S:
Seek
+ ?
Sized
>
Seek
for
&mut S
1.0.0
·
Source
§
impl<S:
Seek
+ ?
Sized
>
Seek
for
Box
<S>
1.0.0
·
Source
§
impl<T>
Seek
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
impl<W: ?
Sized
+
Write
+
Seek
>
Seek
for
BufWriter
<W>