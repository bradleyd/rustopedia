Shutdown in std::net - Rust
std
::
net
Enum
Shutdown
Copy item path
1.0.0
·
Source
pub enum Shutdown {
    Read,
    Write,
    Both,
}
Expand description
Possible values which can be passed to the
TcpStream::shutdown
method.
Variants
§
§
1.0.0
Read
The reading portion of the
TcpStream
should be shut down.
All currently blocked and future
reads
will return
Ok
(0)
.
§
1.0.0
Write
The writing portion of the
TcpStream
should be shut down.
All currently blocked and future
writes
will return an error.
§
1.0.0
Both
Both the reading and the writing portions of the
TcpStream
should be shut down.
See
Shutdown::Read
and
Shutdown::Write
for more information.
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
Shutdown
Source
§
fn
clone
(&self) ->
Shutdown
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
Shutdown
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
Shutdown
Source
§
fn
eq
(&self, other: &
Shutdown
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
Shutdown
1.0.0
·
Source
§
impl
Eq
for
Shutdown
1.0.0
·
Source
§
impl
StructuralPartialEq
for
Shutdown
Auto Trait Implementations
§
§
impl
Freeze
for
Shutdown
§
impl
RefUnwindSafe
for
Shutdown
§
impl
Send
for
Shutdown
§
impl
Sync
for
Shutdown
§
impl
Unpin
for
Shutdown
§
impl
UnwindSafe
for
Shutdown
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