LocalWaker in std::task - Rust
std
::
task
Struct
LocalWaker
Copy item path
Source
pub struct LocalWaker {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Expand description
A
LocalWaker
is analogous to a
Waker
, but it does not implement
Send
or
Sync
.
This handle encapsulates a
RawWaker
instance, which defines the
executor-specific wakeup behavior.
Local wakers can be requested from a
Context
with the
local_waker
method.
The typical life of a
LocalWaker
is that it is constructed by an executor, wrapped in a
Context
using
ContextBuilder
, then passed to
Future::poll()
. Then, if the future chooses to return
Poll::Pending
, it must also store the waker somehow and call
LocalWaker::wake()
when
the future should be polled again.
Implements
Clone
, but neither
Send
nor
Sync
; therefore, a local waker may
not be moved to other threads. In general, when deciding to use wakers or local wakers,
local wakers are preferable unless the waker needs to be sent across threads. This is because
wakers can incur in additional cost related to memory synchronization.
Note that it is preferable to use
local_waker.clone_from(&new_waker)
instead
of
*local_waker = new_waker.clone()
, as the former will avoid cloning the waker
unnecessarily if the two wakers
wake the same task
.
§
Examples
Usage of a local waker to implement a future analogous to
std::thread::yield_now()
.
#![feature(local_waker)]
use
std::future::{Future, poll_fn};
use
std::task::Poll;
// a future that returns pending once.
fn
yield_now() ->
impl
Future<Output=()> + Unpin {
let
mut
yielded =
false
;
    poll_fn(
move
|cx| {
if
!yielded {
            yielded =
true
;
            cx.local_waker().wake_by_ref();
return
Poll::Pending;
        }
return
Poll::Ready(())
    })
}

yield_now().
await
;
Implementations
§
Source
§
impl
LocalWaker
Source
pub fn
wake
(self)
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Wakes up the task associated with this
LocalWaker
.
As long as the executor keeps running and the task is not finished, it is
guaranteed that each invocation of
wake()
(or
wake_by_ref()
) will be followed by at least one
poll()
of the task to which this
LocalWaker
belongs. This makes
it possible to temporarily yield to other tasks while running potentially
unbounded processing loops.
Note that the above implies that multiple wake-ups may be coalesced into a
single
poll()
invocation by the runtime.
Also note that yielding to competing tasks is not guaranteed: it is the
executor’s choice which task to run and the executor may choose to run the
current task again.
Source
pub fn
wake_by_ref
(&self)
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Wakes up the task associated with this
LocalWaker
without consuming the
LocalWaker
.
This is similar to
wake()
, but may be slightly less efficient in
the case where an owned
Waker
is available. This method should be preferred to
calling
waker.clone().wake()
.
Source
pub fn
will_wake
(&self, other: &
LocalWaker
) ->
bool
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Returns
true
if this
LocalWaker
and another
LocalWaker
would awake the same task.
This function works on a best-effort basis, and may return false even
when the
Waker
s would awaken the same task. However, if this function
returns
true
, it is guaranteed that the
Waker
s will awaken the same task.
This function is primarily used for optimization purposes — for example,
this type’s
clone_from
implementation uses it to
avoid cloning the waker when they would wake the same task anyway.
Source
pub const unsafe fn
new
(
    data:
*const
()
,
    vtable: &'static
RawWakerVTable
,
) ->
LocalWaker
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Creates a new
LocalWaker
from the provided
data
pointer and
vtable
.
The
data
pointer can be used to store arbitrary data as required
by the executor. This could be e.g. a type-erased pointer to an
Arc
that is associated with the task.
The value of this pointer will get passed to all functions that are part
of the
vtable
as the first parameter.
The
vtable
customizes the behavior of a
LocalWaker
. For each
operation on the
LocalWaker
, the associated function in the
vtable
will be called.
§
Safety
The behavior of the returned
Waker
is undefined if the contract defined
in
RawWakerVTable
’s documentation is not upheld.
Source
pub const unsafe fn
from_raw
(waker:
RawWaker
) ->
LocalWaker
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Creates a new
LocalWaker
from
RawWaker
.
The behavior of the returned
LocalWaker
is undefined if the contract defined
in
RawWaker
’s and
RawWakerVTable
’s documentation is not upheld.
Therefore this method is unsafe.
Source
pub const fn
noop
() -> &'static
LocalWaker
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Returns a reference to a
LocalWaker
that does nothing when used.
This is mostly useful for writing tests that need a
Context
to poll
some futures, but are not expecting those futures to wake the waker or
do not need to do anything specific if it happens.
More generally, using
LocalWaker::noop()
to poll a future
means discarding the notification of when the future should be polled again,
So it should only be used when such a notification will not be needed to make progress.
If an owned
LocalWaker
is needed,
clone()
this one.
§
Examples
#![feature(local_waker)]
use
std::future::Future;
use
std::task::{ContextBuilder, LocalWaker, Waker, Poll};
let
mut
cx = ContextBuilder::from_waker(Waker::noop())
    .local_waker(LocalWaker::noop())
    .build();
let
mut
future = Box::pin(
async
{
10
});
assert_eq!
(future.as_mut().poll(
&mut
cx), Poll::Ready(
10
));
Source
pub fn
data
(&self) ->
*const
()
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Gets the
data
pointer used to create this
LocalWaker
.
Source
pub fn
vtable
(&self) -> &'static
RawWakerVTable
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Gets the
vtable
pointer used to create this
LocalWaker
.
Trait Implementations
§
Source
§
impl
AsRef
<
LocalWaker
> for
Waker
Source
§
fn
as_ref
(&self) -> &
LocalWaker
Converts this type into a shared reference of the (usually inferred) input type.
Source
§
impl
Clone
for
LocalWaker
Source
§
fn
clone
(&self) ->
LocalWaker
Returns a copy of the value.
Read more
Source
§
fn
clone_from
(&mut self, source: &
LocalWaker
)
Performs copy-assignment from
source
.
Read more
Source
§
impl
Debug
for
LocalWaker
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
impl
Drop
for
LocalWaker
Source
§
fn
drop
(&mut self)
Executes the destructor for this type.
Read more
Source
§
impl<W>
From
<
Rc
<W>> for
LocalWaker
where
    W:
LocalWake
+ 'static,
Source
§
fn
from
(waker:
Rc
<W>) ->
LocalWaker
Use a
Wake
-able type as a
LocalWaker
.
No heap allocations or atomic operations are used for this conversion.
Source
§
impl !
Send
for
LocalWaker
Source
§
impl !
Sync
for
LocalWaker
Source
§
impl
Unpin
for
LocalWaker
Auto Trait Implementations
§
§
impl
Freeze
for
LocalWaker
§
impl
RefUnwindSafe
for
LocalWaker
§
impl
UnwindSafe
for
LocalWaker
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