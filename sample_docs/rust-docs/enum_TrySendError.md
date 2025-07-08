TrySendError in std::sync::mpsc - Rust
std
::
sync
::
mpsc
Enum
TrySendError
Copy item path
1.0.0
·
Source
pub enum TrySendError<T> {
    Full(T),
    Disconnected(T),
}
Expand description
This enumeration is the list of the possible error outcomes for the
try_send
method.
Variants
§
§
1.0.0
Full(T)
The data could not be sent on the
sync_channel
because it would require that
the callee block to send the data.
If this is a buffered channel, then the buffer is full at this time. If
this is not a buffered channel, then there is no
Receiver
available to
acquire the data.
§
1.0.0
Disconnected(T)
This
sync_channel
’s receiving half has disconnected, so the data could not be
sent. The data is returned back to the callee in this case.
Trait Implementations
§
1.0.0
·
Source
§
impl<T:
Clone
>
Clone
for
TrySendError
<T>
Source
§
fn
clone
(&self) ->
TrySendError
<T>
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
impl<T>
Debug
for
TrySendError
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
impl<T>
Display
for
TrySendError
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
impl<T>
Error
for
TrySendError
<T>
Source
§
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.30.0
·
Source
§
fn
source
(&self) ->
Option
<&(dyn
Error
+ 'static)>
Returns the lower-level source of this error, if any.
Read more
1.0.0
·
Source
§
fn
cause
(&self) ->
Option
<&dyn
Error
>
👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
Source
§
fn
provide
<'a>(&'a self, request: &mut
Request
<'a>)
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides type-based access to context intended for error reports.
Read more
1.24.0
·
Source
§
impl<T>
From
<
SendError
<T>> for
TrySendError
<T>
Source
§
fn
from
(err:
SendError
<T>) ->
TrySendError
<T>
Converts a
SendError<T>
into a
TrySendError<T>
.
This conversion always returns a
TrySendError::Disconnected
containing the data in the
SendError<T>
.
No data is allocated on the heap.
1.0.0
·
Source
§
impl<T:
PartialEq
>
PartialEq
for
TrySendError
<T>
Source
§
fn
eq
(&self, other: &
TrySendError
<T>) ->
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
impl<T:
Copy
>
Copy
for
TrySendError
<T>
1.0.0
·
Source
§
impl<T:
Eq
>
Eq
for
TrySendError
<T>
1.0.0
·
Source
§
impl<T>
StructuralPartialEq
for
TrySendError
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
TrySendError
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
TrySendError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
TrySendError
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
TrySendError
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
TrySendError
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
TrySendError
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
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
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