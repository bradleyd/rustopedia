AsyncDropInPlace in std::future - Rust
std
::
future
Struct
AsyncDropInPlace
Copy item path
Source
pub struct AsyncDropInPlace<T>(
/* private fields */
)
where
    T: ?
Sized
;
🔬
This is a nightly-only experimental API. (
async_drop
#126482
)
Expand description
A future returned by the
async_drop_in_place
.
Trait Implementations
§
Source
§
impl<T>
Debug
for
AsyncDropInPlace
<T>
where
    T: ?
Sized
,
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
impl<T>
Future
for
AsyncDropInPlace
<T>
where
    T: ?
Sized
,
Source
§
type
Output
=
()
The type of value produced on completion.
Source
§
fn
poll
(
    self:
Pin
<&mut
AsyncDropInPlace
<T>>,
    cx: &mut
Context
<'_>,
) ->
Poll
<<
AsyncDropInPlace
<T> as
Future
>::
Output
>
Attempts to resolve the future to a final value, registering
the current task for wakeup if the value is not yet available.
Read more
Auto Trait Implementations
§
§
impl<T>
Freeze
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
Freeze
,
    T: ?
Sized
,
§
impl<T>
RefUnwindSafe
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
RefUnwindSafe
,
    T: ?
Sized
,
§
impl<T>
Send
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
Send
,
    T: ?
Sized
,
§
impl<T>
Sync
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
Sync
,
    T: ?
Sized
,
§
impl<T>
Unpin
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
Unpin
,
    T: ?
Sized
,
§
impl<T>
UnwindSafe
for
AsyncDropInPlace
<T>
where
    <T as AsyncDestruct>::AsyncDestructor:
UnwindSafe
,
    T: ?
Sized
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
impl<F>
IntoFuture
for F
where
    F:
Future
,
Source
§
type
Output
= <F as
Future
>::
Output
The output that the future will produce on completion.
Source
§
type
IntoFuture
= F
Which kind of future are we turning this into?
Source
§
fn
into_future
(self) -> <F as
IntoFuture
>::
IntoFuture
Creates a future from a value.
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