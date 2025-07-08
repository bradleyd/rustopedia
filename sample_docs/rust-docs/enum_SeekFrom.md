SeekFrom in std::io - Rust
std
::
io
Enum
SeekFrom
Copy item path
1.0.0
·
Source
pub enum SeekFrom {
    Start(
u64
),
    End(
i64
),
    Current(
i64
),
}
Expand description
Enumeration of possible methods to seek within an I/O object.
It is used by the
Seek
trait.
Variants
§
§
1.0.0
Start(
u64
)
Sets the offset to the provided number of bytes.
§
1.0.0
End(
i64
)
Sets the offset to the size of this object plus the specified number of
bytes.
It is possible to seek beyond the end of an object, but it’s an error to
seek before byte 0.
§
1.0.0
Current(
i64
)
Sets the offset to the current position plus the specified number of
bytes.
It is possible to seek beyond the end of an object, but it’s an error to
seek before byte 0.
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
SeekFrom
Source
§
fn
clone
(&self) ->
SeekFrom
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
1.0.0
·
Source
§
impl
Debug
for
SeekFrom
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
impl
PartialEq
for
SeekFrom
Source
§
fn
eq
(&self, other: &
SeekFrom
) ->
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
1.0.0
·
Source
§
impl
Copy
for
SeekFrom
1.0.0
·
Source
§
impl
Eq
for
SeekFrom
1.0.0
·
Source
§
impl
StructuralPartialEq
for
SeekFrom
Auto Trait Implementations
§
§
impl
Freeze
for
SeekFrom
§
impl
RefUnwindSafe
for
SeekFrom
§
impl
Send
for
SeekFrom
§
impl
Sync
for
SeekFrom
§
impl
Unpin
for
SeekFrom
§
impl
UnwindSafe
for
SeekFrom
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