RawWaker in std::task - Rust
std
::
task
Struct
RawWaker
Copy item path
1.36.0
·
Source
pub struct RawWaker {
/* private fields */
}
Expand description
A
RawWaker
allows the implementor of a task executor to create a
Waker
or a
LocalWaker
which provides customized wakeup behavior.
It consists of a data pointer and a
virtual function pointer table (vtable)
that customizes the behavior of the
RawWaker
.
RawWaker
s are unsafe to use.
Implementing the
Wake
trait is a safe alternative that requires memory allocation.
Implementations
§
Source
§
impl
RawWaker
1.36.0 (const: 1.36.0)
·
Source
pub const fn
new
(data:
*const
()
, vtable: &'static
RawWakerVTable
) ->
RawWaker
Creates a new
RawWaker
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
Arc<T: Send + Sync>
when used to construct a
Waker
. This restriction is lifted when
constructing a
LocalWaker
, which allows using types that do not implement
Send
+
Sync
like
Rc<T>
.
The
vtable
customizes the behavior of a
Waker
which gets created
from a
RawWaker
. For each operation on the
Waker
, the associated
function in the
vtable
of the underlying
RawWaker
will be called.
Trait Implementations
§
1.36.0
·
Source
§
impl
Debug
for
RawWaker
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
1.51.0
·
Source
§
impl<W>
From
<
Arc
<W>> for
RawWaker
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
RawWaker
Use a
Wake
-able type as a
RawWaker
.
No heap allocations or atomic operations are used for this conversion.
Source
§
impl<W>
From
<
Rc
<W>> for
RawWaker
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
RawWaker
Use a
Wake
-able type as a
RawWaker
.
No heap allocations or atomic operations are used for this conversion.
1.36.0
·
Source
§
impl
PartialEq
for
RawWaker
Source
§
fn
eq
(&self, other: &
RawWaker
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
StructuralPartialEq
for
RawWaker
Auto Trait Implementations
§
§
impl
Freeze
for
RawWaker
§
impl
RefUnwindSafe
for
RawWaker
§
impl !
Send
for
RawWaker
§
impl !
Sync
for
RawWaker
§
impl
Unpin
for
RawWaker
§
impl
UnwindSafe
for
RawWaker
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