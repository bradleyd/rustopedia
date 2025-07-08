SyncUnsafeCell in std::cell - Rust
std
::
cell
Struct
SyncUnsafeCell
Copy item path
Source
pub struct SyncUnsafeCell<T>
where
    T: ?
Sized
,
{
/* private fields */
}
🔬
This is a nightly-only experimental API. (
sync_unsafe_cell
#95439
)
Expand description
UnsafeCell
, but
Sync
.
This is just an
UnsafeCell
, except it implements
Sync
if
T
implements
Sync
.
UnsafeCell
doesn’t implement
Sync
, to prevent accidental mis-use.
You can use
SyncUnsafeCell
instead of
UnsafeCell
to allow it to be
shared between threads, if that’s intentional.
Providing proper synchronization is still the task of the user,
making this type just as unsafe to use.
See
UnsafeCell
for details.
Implementations
§
Source
§
impl<T>
SyncUnsafeCell
<T>
Source
pub const fn
new
(value: T) ->
SyncUnsafeCell
<T>
🔬
This is a nightly-only experimental API. (
sync_unsafe_cell
#95439
)
Constructs a new instance of
SyncUnsafeCell
which will wrap the specified value.
Source
pub const fn
into_inner
(self) -> T
🔬
This is a nightly-only experimental API. (
sync_unsafe_cell
#95439
)
Unwraps the value, consuming the cell.
Source
§
impl<T>
SyncUnsafeCell
<T>
where
    T: ?
Sized
,
Source
pub const fn
get
(&self) ->
*mut T
🔬
This is a nightly-only experimental API. (
sync_unsafe_cell
#95439
)
Gets a mutable pointer to the wrapped value.
This can be cast to a pointer of any kind.
Ensure that the access is unique (no active references, mutable or not)
when casting to
&mut T
, and ensure that there are no mutations
or mutable aliases going on when casting to
&T
Source
pub const fn
get_mut
(&mut self) ->
&mut T
🔬
This is a nightly-only experimental API. (
sync_unsafe_cell
#95439
)
Returns a mutable reference to the underlying data.
This call borrows the
SyncUnsafeCell
mutably (at compile-time) which
guarantees that we possess the only reference.
Source
pub const fn
raw_get
(this:
*const
SyncUnsafeCell
<T>) ->
*mut T
🔬
This is a nightly-only experimental API. (
sync_unsafe_cell
#95439
)
Gets a mutable pointer to the wrapped value.
See
UnsafeCell::get
for details.
Trait Implementations
§
Source
§
impl<T>
Debug
for
SyncUnsafeCell
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
Default
for
SyncUnsafeCell
<T>
where
    T:
Default
,
Source
§
fn
default
() ->
SyncUnsafeCell
<T>
Creates an
SyncUnsafeCell
, with the
Default
value for T.
Source
§
impl<T>
From
<T> for
SyncUnsafeCell
<T>
Source
§
fn
from
(t: T) ->
SyncUnsafeCell
<T>
Creates a new
SyncUnsafeCell<T>
containing the given value.
Source
§
impl<T, U>
CoerceUnsized
<
SyncUnsafeCell
<U>> for
SyncUnsafeCell
<T>
where
    T:
CoerceUnsized
<U>,
Source
§
impl<T, U>
DispatchFromDyn
<
SyncUnsafeCell
<U>> for
SyncUnsafeCell
<T>
where
    T:
DispatchFromDyn
<U>,
Source
§
impl<T>
PinCoerceUnsized
for
SyncUnsafeCell
<T>
where
    T: ?
Sized
,
Source
§
impl<T>
PointerLike
for
SyncUnsafeCell
<T>
where
    T:
PointerLike
,
Source
§
impl<T>
Sync
for
SyncUnsafeCell
<T>
where
    T:
Sync
+ ?
Sized
,
Auto Trait Implementations
§
§
impl<T> !
Freeze
for
SyncUnsafeCell
<T>
§
impl<T> !
RefUnwindSafe
for
SyncUnsafeCell
<T>
§
impl<T>
Send
for
SyncUnsafeCell
<T>
where
    T:
Send
+ ?
Sized
,
§
impl<T>
Unpin
for
SyncUnsafeCell
<T>
where
    T:
Unpin
+ ?
Sized
,
§
impl<T>
UnwindSafe
for
SyncUnsafeCell
<T>
where
    T:
UnwindSafe
+ ?
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
<
!
> for T
Source
§
fn
from
(t:
!
) -> T
Converts to this type from the input type.
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