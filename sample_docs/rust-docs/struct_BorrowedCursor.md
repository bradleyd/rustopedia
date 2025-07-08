BorrowedCursor in std::io - Rust
std
::
io
Struct
BorrowedCursor
Copy item path
Source
pub struct BorrowedCursor<'a> {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Expand description
A writeable view of the unfilled portion of a
BorrowedBuf
.
The unfilled portion consists of an initialized and an uninitialized part; see
BorrowedBuf
for details.
Data can be written directly to the cursor by using
append
or
indirectly by getting a slice of part or all of the cursor and writing into the slice. In the
indirect case, the caller must call
advance
after writing to inform
the cursor how many bytes have been written.
Once data is written to the cursor, it becomes part of the filled portion of the underlying
BorrowedBuf
and can no longer be accessed or re-written by the cursor. I.e., the cursor tracks
the unfilled part of the underlying
BorrowedBuf
.
The lifetime
'a
is a bound on the lifetime of the underlying buffer (which means it is a bound
on the data in that buffer by transitivity).
Implementations
§
Source
§
impl<'a>
BorrowedCursor
<'a>
Source
pub fn
reborrow
<'this>(&'this mut self) ->
BorrowedCursor
<'this>
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Reborrows this cursor by cloning it with a smaller lifetime.
Since a cursor maintains unique access to its underlying buffer, the borrowed cursor is
not accessible while the new cursor exists.
Source
pub fn
capacity
(&self) ->
usize
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns the available space in the cursor.
Source
pub fn
written
(&self) ->
usize
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns the number of bytes written to this cursor since it was created from a
BorrowedBuf
.
Note that if this cursor is a reborrowed clone of another, then the count returned is the
count written via either cursor, not the count since the cursor was reborrowed.
Source
pub fn
init_ref
(&self) -> &[
u8
]
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a shared reference to the initialized portion of the cursor.
Source
pub fn
init_mut
(&mut self) -> &mut [
u8
]
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a mutable reference to the initialized portion of the cursor.
Source
pub fn
uninit_mut
(&mut self) -> &mut [
MaybeUninit
<
u8
>]
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a mutable reference to the uninitialized part of the cursor.
It is safe to uninitialize any of these bytes.
Source
pub unsafe fn
as_mut
(&mut self) -> &mut [
MaybeUninit
<
u8
>]
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a mutable reference to the whole cursor.
§
Safety
The caller must not uninitialize any bytes in the initialized portion of the cursor.
Source
pub fn
advance
(&mut self, n:
usize
) -> &mut
BorrowedCursor
<'a>
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Advances the cursor by asserting that
n
bytes have been filled.
After advancing, the
n
bytes are no longer accessible via the cursor and can only be
accessed via the underlying buffer. I.e., the buffer’s filled portion grows by
n
elements
and its unfilled portion (and the capacity of this cursor) shrinks by
n
elements.
If less than
n
bytes initialized (by the cursor’s point of view),
set_init
should be
called first.
§
Panics
Panics if there are less than
n
bytes initialized.
Source
pub unsafe fn
advance_unchecked
(&mut self, n:
usize
) -> &mut
BorrowedCursor
<'a>
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Advances the cursor by asserting that
n
bytes have been filled.
After advancing, the
n
bytes are no longer accessible via the cursor and can only be
accessed via the underlying buffer. I.e., the buffer’s filled portion grows by
n
elements
and its unfilled portion (and the capacity of this cursor) shrinks by
n
elements.
§
Safety
The caller must ensure that the first
n
bytes of the cursor have been properly
initialised.
Source
pub fn
ensure_init
(&mut self) -> &mut
BorrowedCursor
<'a>
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Initializes all bytes in the cursor.
Source
pub unsafe fn
set_init
(&mut self, n:
usize
) -> &mut
BorrowedCursor
<'a>
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Asserts that the first
n
unfilled bytes of the cursor are initialized.
BorrowedBuf
assumes that bytes are never de-initialized, so this method does nothing when
called with fewer bytes than are already known to be initialized.
§
Safety
The caller must ensure that the first
n
bytes of the buffer have already been initialized.
Source
pub fn
append
(&mut self, buf: &[
u8
])
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Appends data to the cursor, advancing position within its buffer.
§
Panics
Panics if
self.capacity()
is less than
buf.len()
.
Trait Implementations
§
Source
§
impl<'a>
Debug
for
BorrowedCursor
<'a>
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
Source
§
impl<'a>
Write
for
BorrowedCursor
<'a>
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
impl<'a>
Freeze
for
BorrowedCursor
<'a>
§
impl<'a>
RefUnwindSafe
for
BorrowedCursor
<'a>
§
impl<'a>
Send
for
BorrowedCursor
<'a>
§
impl<'a>
Sync
for
BorrowedCursor
<'a>
§
impl<'a>
Unpin
for
BorrowedCursor
<'a>
§
impl<'a> !
UnwindSafe
for
BorrowedCursor
<'a>
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