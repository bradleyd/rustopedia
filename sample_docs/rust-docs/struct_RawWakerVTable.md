RawWakerVTable in std::task - Rust
std
::
task
Struct
RawWakerVTable
Copy item path
1.36.0
·
Source
pub struct RawWakerVTable {
/* private fields */
}
Expand description
A virtual function pointer table (vtable) that specifies the behavior
of a
RawWaker
.
The pointer passed to all functions inside the vtable is the
data
pointer
from the enclosing
RawWaker
object.
The functions inside this struct are only intended to be called on the
data
pointer of a properly constructed
RawWaker
object from inside the
RawWaker
implementation. Calling one of the contained functions using
any other
data
pointer will cause undefined behavior.
Note that while this type implements
PartialEq
, comparing function pointers, and hence
comparing structs like this that contain function pointers, is unreliable: pointers to the same
function can compare inequal (because functions are duplicated in multiple codegen units), and
pointers to
different
functions can compare equal (since identical functions can be
deduplicated within a codegen unit).
§
Thread safety
If the
RawWaker
will be used to construct a
Waker
then
these functions must all be thread-safe (even though
RawWaker
is
!
Send
+ !
Sync
). This is because
Waker
is
Send
+
Sync
,
and it may be moved to arbitrary threads or invoked by
&
reference. For example,
this means that if the
clone
and
drop
functions manage a reference count,
they must do so atomically.
However, if the
RawWaker
will be used to construct a
LocalWaker
instead, then
these functions don’t need to be thread safe. This means that
!
Send
+ !
Sync
data can be stored in the data pointer, and reference counting does not need any atomic
synchronization. This is because
LocalWaker
is not thread safe itself, so it cannot
be sent across threads.
Implementations
§
Source
§
impl
RawWakerVTable
1.36.0 (const: 1.36.0)
·
Source
pub const fn
new
(
    clone: unsafe
fn
(_:
*const
()
) ->
RawWaker
,
    wake: unsafe
fn
(_:
*const
()
),
    wake_by_ref: unsafe
fn
(_:
*const
()
),
    drop: unsafe
fn
(_:
*const
()
),
) ->
RawWakerVTable
Creates a new
RawWakerVTable
from the provided
clone
,
wake
,
wake_by_ref
, and
drop
functions.
If the
RawWaker
will be used to construct a
Waker
then
these functions must all be thread-safe (even though
RawWaker
is
!
Send
+ !
Sync
). This is because
Waker
is
Send
+
Sync
,
and it may be moved to arbitrary threads or invoked by
&
reference. For example,
this means that if the
clone
and
drop
functions manage a reference count,
they must do so atomically.
However, if the
RawWaker
will be used to construct a
LocalWaker
instead, then
these functions don’t need to be thread safe. This means that
!
Send
+ !
Sync
data can be stored in the data pointer, and reference counting does not need any atomic
synchronization. This is because
LocalWaker
is not thread safe itself, so it cannot
be sent across threads.
§
clone
This function will be called when the
RawWaker
gets cloned, e.g. when
the
Waker
/
LocalWaker
in which the
RawWaker
is stored gets cloned.
The implementation of this function must retain all resources that are
required for this additional instance of a
RawWaker
and associated
task. Calling
wake
on the resulting
RawWaker
should result in a wakeup
of the same task that would have been awoken by the original
RawWaker
.
§
wake
This function will be called when
wake
is called on the
Waker
.
It must wake up the task associated with this
RawWaker
.
The implementation of this function must make sure to release any
resources that are associated with this instance of a
RawWaker
and
associated task.
§
wake_by_ref
This function will be called when
wake_by_ref
is called on the
Waker
.
It must wake up the task associated with this
RawWaker
.
This function is similar to
wake
, but must not consume the provided data
pointer.
§
drop
This function will be called when a
Waker
/
LocalWaker
gets
dropped.
The implementation of this function must make sure to release any
resources that are associated with this instance of a
RawWaker
and
associated task.
Trait Implementations
§
1.36.0
·
Source
§
impl
Clone
for
RawWakerVTable
Source
§
fn
clone
(&self) ->
RawWakerVTable
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
1.36.0
·
Source
§
impl
Debug
for
RawWakerVTable
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
PartialEq
for
RawWakerVTable
Source
§
fn
eq
(&self, other: &
RawWakerVTable
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
1.36.0
·
Source
§
impl
Copy
for
RawWakerVTable
1.36.0
·
Source
§
impl
StructuralPartialEq
for
RawWakerVTable
Auto Trait Implementations
§
§
impl
Freeze
for
RawWakerVTable
§
impl
RefUnwindSafe
for
RawWakerVTable
§
impl
Send
for
RawWakerVTable
§
impl
Sync
for
RawWakerVTable
§
impl
Unpin
for
RawWakerVTable
§
impl
UnwindSafe
for
RawWakerVTable
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