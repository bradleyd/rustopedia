BorrowedBuf in std::io - Rust
std
::
io
Struct
BorrowedBuf
Copy item path
Source
pub struct BorrowedBuf<'data> {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Expand description
A borrowed byte buffer which is incrementally filled and initialized.
This type is a sort of “double cursor”. It tracks three regions in the buffer: a region at the beginning of the
buffer that has been logically filled with data, a region that has been initialized at some point but not yet
logically filled, and a region at the end that is fully uninitialized. The filled region is guaranteed to be a
subset of the initialized region.
In summary, the contents of the buffer can be visualized as:
[             capacity              ]
[ filled |         unfilled         ]
[    initialized    | uninitialized ]
A
BorrowedBuf
is created around some existing data (or capacity for data) via a unique reference
(
&mut
). The
BorrowedBuf
can be configured (e.g., using
clear
or
set_init
), but cannot be
directly written. To write into the buffer, use
unfilled
to create a
BorrowedCursor
. The cursor
has write-only access to the unfilled portion of the buffer (you can think of it as a
write-only iterator).
The lifetime
'data
is a bound on the lifetime of the underlying data.
Implementations
§
Source
§
impl<'data>
BorrowedBuf
<'data>
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
Returns the total capacity of the buffer.
Source
pub fn
len
(&self) ->
usize
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns the length of the filled part of the buffer.
Source
pub fn
init_len
(&self) ->
usize
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns the length of the initialized part of the buffer.
Source
pub fn
filled
(&self) -> &[
u8
]
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a shared reference to the filled portion of the buffer.
Source
pub fn
filled_mut
(&mut self) -> &mut [
u8
]
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a mutable reference to the filled portion of the buffer.
Source
pub fn
into_filled
(self) -> &'data [
u8
]
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a shared reference to the filled portion of the buffer with its original lifetime.
Source
pub fn
into_filled_mut
(self) -> &'data mut [
u8
]
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a mutable reference to the filled portion of the buffer with its original lifetime.
Source
pub fn
unfilled
<'this>(&'this mut self) ->
BorrowedCursor
<'this>
ⓘ
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Returns a cursor over the unfilled part of the buffer.
Source
pub fn
clear
(&mut self) -> &mut
BorrowedBuf
<'data>
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Clears the buffer, resetting the filled region to empty.
The number of initialized bytes is not changed, and the contents of the buffer are not modified.
Source
pub unsafe fn
set_init
(&mut self, n:
usize
) -> &mut
BorrowedBuf
<'data>
🔬
This is a nightly-only experimental API. (
core_io_borrowed_buf
#117693
)
Asserts that the first
n
bytes of the buffer are initialized.
BorrowedBuf
assumes that bytes are never de-initialized, so this method does nothing when called with fewer
bytes than are already known to be initialized.
§
Safety
The caller must ensure that the first
n
unfilled bytes of the buffer have already been initialized.
Trait Implementations
§
Source
§
impl
Debug
for
BorrowedBuf
<'_>
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
impl<'data>
From
<&'data mut [
MaybeUninit
<
u8
>]> for
BorrowedBuf
<'data>
Creates a new
BorrowedBuf
from an uninitialized buffer.
Use
set_init
if part of the buffer is known to be already initialized.
Source
§
fn
from
(buf: &'data mut [
MaybeUninit
<
u8
>]) ->
BorrowedBuf
<'data>
Converts to this type from the input type.
Source
§
impl<'data>
From
<&'data mut [
u8
]> for
BorrowedBuf
<'data>
Creates a new
BorrowedBuf
from a fully initialized slice.
Source
§
fn
from
(slice: &'data mut [
u8
]) ->
BorrowedBuf
<'data>
Converts to this type from the input type.
Auto Trait Implementations
§
§
impl<'data>
Freeze
for
BorrowedBuf
<'data>
§
impl<'data>
RefUnwindSafe
for
BorrowedBuf
<'data>
§
impl<'data>
Send
for
BorrowedBuf
<'data>
§
impl<'data>
Sync
for
BorrowedBuf
<'data>
§
impl<'data>
Unpin
for
BorrowedBuf
<'data>
§
impl<'data> !
UnwindSafe
for
BorrowedBuf
<'data>
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