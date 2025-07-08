FromIter in std::async_iter - Rust
std
::
async_iter
Struct
FromIter
Copy item path
Source
pub struct FromIter<I> {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
async_iter_from_iter
#81798
)
Expand description
An async iterator that was created from iterator.
This async iterator is created by the
from_iter
function.
See it documentation for more.
Trait Implementations
§
Source
§
impl<I>
AsyncIterator
for
FromIter
<I>
where
    I:
Iterator
,
Source
§
type
Item
= <I as
Iterator
>::
Item
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of items yielded by the async iterator.
Source
§
fn
poll_next
(
    self:
Pin
<&mut
FromIter
<I>>,
    _cx: &mut
Context
<'_>,
) ->
Poll
<
Option
<<
FromIter
<I> as
AsyncIterator
>::
Item
>>
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Attempts to pull out the next value of this async iterator, registering the
current task for wakeup if the value is not yet available, and returning
None
if the async iterator is exhausted.
Read more
Source
§
fn
size_hint
(&self) -> (
usize
,
Option
<
usize
>)
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Returns the bounds on the remaining length of the async iterator.
Read more
Source
§
impl<I>
Clone
for
FromIter
<I>
where
    I:
Clone
,
Source
§
fn
clone
(&self) ->
FromIter
<I>
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
Source
§
impl<I>
Debug
for
FromIter
<I>
where
    I:
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
Source
§
impl<I>
Unpin
for
FromIter
<I>
Auto Trait Implementations
§
§
impl<I>
Freeze
for
FromIter
<I>
where
    I:
Freeze
,
§
impl<I>
RefUnwindSafe
for
FromIter
<I>
where
    I:
RefUnwindSafe
,
§
impl<I>
Send
for
FromIter
<I>
where
    I:
Send
,
§
impl<I>
Sync
for
FromIter
<I>
where
    I:
Sync
,
§
impl<I>
UnwindSafe
for
FromIter
<I>
where
    I:
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
impl<I>
IntoAsyncIterator
for I
where
    I:
AsyncIterator
,
Source
§
type
Item
= <I as
AsyncIterator
>::
Item
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of the item yielded by the iterator
Source
§
type
IntoAsyncIter
= I
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of the resulting iterator
Source
§
fn
into_async_iter
(self) -> <I as
IntoAsyncIterator
>::
IntoAsyncIter
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Converts
self
into an async iterator
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