PollFn in std::future - Rust
std
::
future
Struct
PollFn
Copy item path
1.64.0
·
Source
pub struct PollFn<F> {
/* private fields */
}
Expand description
A Future that wraps a function returning
Poll
.
This
struct
is created by
poll_fn()
. See its
documentation for more.
Trait Implementations
§
1.64.0
·
Source
§
impl<F>
Debug
for
PollFn
<F>
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
1.64.0
·
Source
§
impl<T, F>
Future
for
PollFn
<F>
where
    F:
FnMut
(&mut
Context
<'_>) ->
Poll
<T>,
Source
§
type
Output
= T
The type of value produced on completion.
Source
§
fn
poll
(self:
Pin
<&mut
PollFn
<F>>, cx: &mut
Context
<'_>) ->
Poll
<T>
Attempts to resolve the future to a final value, registering
the current task for wakeup if the value is not yet available.
Read more
1.64.0
·
Source
§
impl<F>
Unpin
for
PollFn
<F>
where
    F:
Unpin
,
Auto Trait Implementations
§
§
impl<F>
Freeze
for
PollFn
<F>
where
    F:
Freeze
,
§
impl<F>
RefUnwindSafe
for
PollFn
<F>
where
    F:
RefUnwindSafe
,
§
impl<F>
Send
for
PollFn
<F>
where
    F:
Send
,
§
impl<F>
Sync
for
PollFn
<F>
where
    F:
Sync
,
§
impl<F>
UnwindSafe
for
PollFn
<F>
where
    F:
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