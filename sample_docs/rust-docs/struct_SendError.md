SendError in std::sync::mpsc - Rust
std
::
sync
::
mpsc
Struct
SendError
Copy item path
1.0.0
·
Source
pub struct SendError<T>(pub T);
Expand description
An error returned from the
Sender::send
or
SyncSender::send
function on
channel
s.
A
send
operation can only fail if the receiving end of a channel is
disconnected, implying that the data could never be received. The error
contains the data being sent as a payload so it can be recovered.
Tuple Fields
§
§
0: T
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
SendError
<T>
Source
§
fn
clone
(&self) ->
SendError
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
SendError
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
SendError
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
SendError
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
Source
§
impl<T>
From
<
SendError
<T>> for
SendTimeoutError
<T>
Source
§
fn
from
(err:
SendError
<T>) ->
SendTimeoutError
<T>
Converts to this type from the input type.
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
SendError
<T>
Source
§
fn
eq
(&self, other: &
SendError
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
SendError
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
SendError
<T>
1.0.0
·
Source
§
impl<T>
StructuralPartialEq
for
SendError
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
SendError
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
SendError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
SendError
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
SendError
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
SendError
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
SendError
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