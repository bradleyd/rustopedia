Ready in std::future - Rust
std
::
future
Struct
Ready
Copy item path
1.48.0
·
Source
pub struct Ready<T>(
/* private fields */
);
Expand description
A future that is immediately ready with a value.
This
struct
is created by
ready()
. See its
documentation for more.
Implementations
§
Source
§
impl<T>
Ready
<T>
1.82.0
·
Source
pub fn
into_inner
(self) -> T
Consumes the
Ready
, returning the wrapped value.
§
Panics
Will panic if this
Ready
was already polled to completion.
§
Examples
use
std::future;
let
a = future::ready(
1
);
assert_eq!
(a.into_inner(),
1
);
Trait Implementations
§
1.48.0
·
Source
§
impl<T>
Clone
for
Ready
<T>
where
    T:
Clone
,
Source
§
fn
clone
(&self) ->
Ready
<T>
ⓘ
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
1.48.0
·
Source
§
impl<T>
Debug
for
Ready
<T>
where
    T:
Debug
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
1.48.0
·
Source
§
impl<T>
Future
for
Ready
<T>
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
Ready
<T>>, _cx: &mut
Context
<'_>) ->
Poll
<T>
Attempts to resolve the future to a final value, registering
the current task for wakeup if the value is not yet available.
Read more
1.48.0
·
Source
§
impl<T>
Unpin
for
Ready
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Ready
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Ready
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Ready
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Ready
<T>
where
    T:
Sync
,
§
impl<T>
UnwindSafe
for
Ready
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