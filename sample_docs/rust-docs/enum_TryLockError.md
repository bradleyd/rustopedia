TryLockError in std::sync::poison - Rust
std
::
sync
::
poison
Enum
TryLockError
Copy item path
Source
pub enum TryLockError<T> {
    Poisoned(
PoisonError
<T>),
    WouldBlock,
}
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
Expand description
An enumeration of possible errors associated with a
TryLockResult
which
can occur while trying to acquire a lock, from the
try_lock
method on a
Mutex
or the
try_read
and
try_write
methods on an
RwLock
.
Variants
§
§
Poisoned(
PoisonError
<T>)
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
The lock could not be acquired because another thread failed while holding
the lock.
§
WouldBlock
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
The lock could not be acquired at this time because the operation would
otherwise block.
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Debug
for
TryLockError
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
TryLockError
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
TryLockError
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
1.0.0
·
Source
§
impl<T>
From
<
PoisonError
<T>> for
TryLockError
<T>
Source
§
fn
from
(err:
PoisonError
<T>) ->
TryLockError
<T>
Converts to this type from the input type.
Auto Trait Implementations
§
§
impl<T>
Freeze
for
TryLockError
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
TryLockError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
TryLockError
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
TryLockError
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
TryLockError
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
TryLockError
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