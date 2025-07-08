Take in std::io - Rust
std
::
io
Struct
Take
Copy item path
1.0.0
·
Source
pub struct Take<T> {
/* private fields */
}
Expand description
Reader adapter which limits the bytes read from an underlying reader.
This struct is generally created by calling
take
on a reader.
Please see the documentation of
take
for more details.
Implementations
§
Source
§
impl<T>
Take
<T>
1.0.0
·
Source
pub fn
limit
(&self) ->
u64
Returns the number of bytes that can be read before this instance will
return EOF.
§
Note
This instance may reach
EOF
after reading fewer bytes than indicated by
this method if the underlying
Read
instance reaches EOF.
§
Examples
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
// read at most five bytes
let
handle = f.take(
5
);
println!
(
"limit: {}"
, handle.limit());
Ok
(())
}
1.27.0
·
Source
pub fn
set_limit
(&mut self, limit:
u64
)
Sets the number of bytes that can be read before this instance will
return EOF. This is the same as constructing a new
Take
instance, so
the amount of bytes read and the previous limit value don’t matter when
calling this method.
§
Examples
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
// read at most five bytes
let
mut
handle = f.take(
5
);
    handle.set_limit(
10
);
assert_eq!
(handle.limit(),
10
);
Ok
(())
}
1.15.0
·
Source
pub fn
into_inner
(self) -> T
Consumes the
Take
, returning the wrapped reader.
§
Examples
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
file = File::open(
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
let
mut
handle = file.take(
5
);
    handle.read(
&mut
buffer)
?
;
let
file = handle.into_inner();
Ok
(())
}
1.20.0
·
Source
pub fn
get_ref
(&self) ->
&T
Gets a reference to the underlying reader.
§
Examples
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
file = File::open(
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
let
mut
handle = file.take(
5
);
    handle.read(
&mut
buffer)
?
;
let
file = handle.get_ref();
Ok
(())
}
1.20.0
·
Source
pub fn
get_mut
(&mut self) ->
&mut T
Gets a mutable reference to the underlying reader.
Care should be taken to avoid modifying the internal I/O state of the
underlying reader as doing so may corrupt the internal limit of this
Take
.
§
Examples
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
file = File::open(
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
let
mut
handle = file.take(
5
);
    handle.read(
&mut
buffer)
?
;
let
file = handle.get_mut();
Ok
(())
}
Trait Implementations
§
1.0.0
·
Source
§
impl<T:
BufRead
>
BufRead
for
Take
<T>
Source
§
fn
fill_buf
(&mut self) ->
Result
<&[
u8
]>
Returns the contents of the internal buffer, filling it with more data, via
Read
methods, if empty.
Read more
Source
§
fn
consume
(&mut self, amt:
usize
)
Marks the given
amount
of additional bytes from the internal buffer as having been read.
Subsequent calls to
read
only return bytes that have not been marked as read.
Read more
Source
§
fn
has_data_left
(&mut self) ->
Result
<
bool
>
🔬
This is a nightly-only experimental API. (
buf_read_has_data_left
#86423
)
Checks if there is any data left to be
read
.
Read more
1.0.0
·
Source
§
fn
read_until
(&mut self, byte:
u8
, buf: &mut
Vec
<
u8
>) ->
Result
<
usize
>
Reads all bytes into
buf
until the delimiter
byte
or EOF is reached.
Read more
1.83.0
·
Source
§
fn
skip_until
(&mut self, byte:
u8
) ->
Result
<
usize
>
Skips all bytes until the delimiter
byte
or EOF is reached.
Read more
1.0.0
·
Source
§
fn
read_line
(&mut self, buf: &mut
String
) ->
Result
<
usize
>
Reads all bytes until a newline (the
0xA
byte) is reached, and append
them to the provided
String
buffer.
Read more
1.0.0
·
Source
§
fn
split
(self, byte:
u8
) ->
Split
<Self>
ⓘ
where
    Self:
Sized
,
Returns an iterator over the contents of this reader split on the byte
byte
.
Read more
1.0.0
·
Source
§
fn
lines
(self) ->
Lines
<Self>
ⓘ
where
    Self:
Sized
,
Returns an iterator over the lines of this reader.
Read more
1.0.0
·
Source
§
impl<T:
Debug
>
Debug
for
Take
<T>
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
1.36.0
·
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
1.0.0
·
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
impl<T>
Freeze
for
Take
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Take
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Take
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Take
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
Take
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
Take
<T>
where
    T:
UnwindSafe
,
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