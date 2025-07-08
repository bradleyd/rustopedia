Waker in std::task - Rust
std
::
task
Struct
Waker
Copy item path
1.36.0
·
Source
pub struct Waker {
/* private fields */
}
Expand description
A
Waker
is a handle for waking up a task by notifying its executor that it
is ready to be run.
This handle encapsulates a
RawWaker
instance, which defines the
executor-specific wakeup behavior.
The typical life of a
Waker
is that it is constructed by an executor, wrapped in a
Context
, then passed to
Future::poll()
. Then, if the future chooses to return
Poll::Pending
, it must also store the waker somehow and call
Waker::wake()
when
the future should be polled again.
Implements
Clone
,
Send
, and
Sync
; therefore, a waker may be invoked
from any thread, including ones not in any way managed by the executor. For example,
this might be done to wake a future when a blocking function call completes on another
thread.
Note that it is preferable to use
waker.clone_from(&new_waker)
instead
of
*waker = new_waker.clone()
, as the former will avoid cloning the waker
unnecessarily if the two wakers
wake the same task
.
Constructing a
Waker
from a
RawWaker
is unsafe.
Implementing the
Wake
trait is a safe alternative that requires memory allocation.
Implementations
§
Source
§
impl
Waker
1.36.0
·
Source
pub fn
wake
(self)
Wakes up the task associated with this
Waker
.
As long as the executor keeps running and the task is not finished, it is
guaranteed that each invocation of
wake()
(or
wake_by_ref()
) will be followed by at least one
poll()
of the task to which this
Waker
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
1.36.0
·
Source
pub fn
wake_by_ref
(&self)
Wakes up the task associated with this
Waker
without consuming the
Waker
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
1.36.0
·
Source
pub fn
will_wake
(&self, other: &
Waker
) ->
bool
Returns
true
if this
Waker
and another
Waker
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
1.83.0 (const: 1.83.0)
·
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
Waker
Creates a new
Waker
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
It is important to consider that the
data
pointer must point to a
thread safe type such as an
Arc
.
The
vtable
customizes the behavior of a
Waker
. For each operation
on the
Waker
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
(Authors wishing to avoid unsafe code may implement the
Wake
trait instead, at the
cost of a required heap allocation.)
1.36.0 (const: 1.82.0)
·
Source
pub const unsafe fn
from_raw
(waker:
RawWaker
) ->
Waker
Creates a new
Waker
from
RawWaker
.
§
Safety
The behavior of the returned
Waker
is undefined if the contract defined
in
RawWaker
’s and
RawWakerVTable
’s documentation is not upheld.
(Authors wishing to avoid unsafe code may implement the
Wake
trait instead, at the
cost of a required heap allocation.)
1.85.0 (const: 1.85.0)
·
Source
pub const fn
noop
() -> &'static
Waker
Returns a reference to a
Waker
that does nothing when used.
This is mostly useful for writing tests that need a
Context
to poll
some futures, but are not expecting those futures to wake the waker or
do not need to do anything specific if it happens.
More generally, using
Waker::noop()
to poll a future
means discarding the notification of when the future should be polled again.
So it should only be used when such a notification will not be needed to make progress.
If an owned
Waker
is needed,
clone()
this one.
§
Examples
use
std::future::Future;
use
std::task;
let
mut
cx = task::Context::from_waker(task::Waker::noop());
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
cx), task::Poll::Ready(
10
));
1.83.0
·
Source
pub fn
data
(&self) ->
*const
()
Gets the
data
pointer used to create this
Waker
.
1.83.0
·
Source
pub fn
vtable
(&self) -> &'static
RawWakerVTable
Gets the
vtable
pointer used to create this
Waker
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
1.36.0
·
Source
§
impl
Clone
for
Waker
Source
§
fn
clone_from
(&mut self, source: &
Waker
)
Assigns a clone of
source
to
self
, unless
self.will_wake(source)
anyway.
This method is preferred over simply assigning
source.clone()
to
self
,
as it avoids cloning the waker if
self
is already the same waker.
§
Examples
use
std::future::Future;
use
std::pin::Pin;
use
std::sync::{Arc, Mutex};
use
std::task::{Context, Poll, Waker};
struct
Waiter {
    shared: Arc<Mutex<Shared>>,
}
struct
Shared {
    waker: Waker,
// ...
}
impl
Future
for
Waiter {
type
Output = ();
fn
poll(
self
: Pin<
&mut
Self
>, cx:
&mut
Context<
'_
>) -> Poll<()> {
let
mut
shared =
self
.shared.lock().unwrap();
// update the waker
shared.waker.clone_from(cx.waker());
// readiness logic ...
}
}
Source
§
fn
clone
(&self) ->
Waker
Returns a copy of the value.
Read more
1.36.0
·
Source
§
impl
Debug
for
Waker
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
1.36.0
·
Source
§
impl
Drop
for
Waker
Source
§
fn
drop
(&mut self)
Executes the destructor for this type.
Read more
1.51.0
·
Source
§
impl<W>
From
<
Arc
<W>> for
Waker
where
    W:
Wake
+
Send
+
Sync
+ 'static,
Source
§
fn
from
(waker:
Arc
<W>) ->
Waker
Use a
Wake
-able type as a
Waker
.
No heap allocations or atomic operations are used for this conversion.
1.36.0
·
Source
§
impl
Send
for
Waker
1.36.0
·
Source
§
impl
Sync
for
Waker
1.36.0
·
Source
§
impl
Unpin
for
Waker
Auto Trait Implementations
§
§
impl
Freeze
for
Waker
§
impl
RefUnwindSafe
for
Waker
§
impl
UnwindSafe
for
Waker
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