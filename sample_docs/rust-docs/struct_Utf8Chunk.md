Utf8Chunk in std::str - Rust
std
::
str
Struct
Utf8Chunk
Copy item path
1.79.0
·
Source
pub struct Utf8Chunk<'a> {
/* private fields */
}
Expand description
An item returned by the
Utf8Chunks
iterator.
A
Utf8Chunk
stores a sequence of
u8
up to the first broken character
when decoding a UTF-8 string.
§
Examples
// An invalid UTF-8 string
let
bytes =
b"foo\xF1\x80bar"
;
// Decode the first `Utf8Chunk`
let
chunk = bytes.utf8_chunks().next().unwrap();
// The first three characters are valid UTF-8
assert_eq!
(
"foo"
, chunk.valid());
// The fourth character is broken
assert_eq!
(
b"\xF1\x80"
, chunk.invalid());
Implementations
§
Source
§
impl<'a>
Utf8Chunk
<'a>
1.79.0
·
Source
pub fn
valid
(&self) -> &'a
str
Returns the next validated UTF-8 substring.
This substring can be empty at the start of the string or between
broken UTF-8 characters.
1.79.0
·
Source
pub fn
invalid
(&self) -> &'a [
u8
]
ⓘ
Returns the invalid sequence that caused a failure.
The returned slice will have a maximum length of 3 and starts after the
substring given by
valid
. Decoding will resume after this sequence.
If empty, this is the last chunk in the string. If non-empty, an
unexpected byte was encountered or the end of the input was reached
unexpectedly.
Lossy decoding would replace this sequence with
U+FFFD REPLACEMENT CHARACTER
.
Trait Implementations
§
1.79.0
·
Source
§
impl<'a>
Clone
for
Utf8Chunk
<'a>
Source
§
fn
clone
(&self) ->
Utf8Chunk
<'a>
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
1.79.0
·
Source
§
impl<'a>
Debug
for
Utf8Chunk
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
1.79.0
·
Source
§
impl<'a>
PartialEq
for
Utf8Chunk
<'a>
Source
§
fn
eq
(&self, other: &
Utf8Chunk
<'a>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.79.0
·
Source
§
impl<'a>
Eq
for
Utf8Chunk
<'a>
1.79.0
·
Source
§
impl<'a>
StructuralPartialEq
for
Utf8Chunk
<'a>
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
Utf8Chunk
<'a>
§
impl<'a>
RefUnwindSafe
for
Utf8Chunk
<'a>
§
impl<'a>
Send
for
Utf8Chunk
<'a>
§
impl<'a>
Sync
for
Utf8Chunk
<'a>
§
impl<'a>
Unpin
for
Utf8Chunk
<'a>
§
impl<'a>
UnwindSafe
for
Utf8Chunk
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
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
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
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
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