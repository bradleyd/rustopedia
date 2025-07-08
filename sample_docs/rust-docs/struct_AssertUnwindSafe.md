AssertUnwindSafe in std::panic - Rust
std
::
panic
Struct
AssertUnwindSafe
Copy item path
1.9.0
·
Source
pub struct AssertUnwindSafe<T>(pub T);
Expand description
A simple wrapper around a type to assert that it is unwind safe.
When using
catch_unwind
it may be the case that some of the closed over
variables are not unwind safe. For example if
&mut T
is captured the
compiler will generate a warning indicating that it is not unwind safe. It
might not be the case, however, that this is actually a problem due to the
specific usage of
catch_unwind
if unwind safety is specifically taken into
account. This wrapper struct is useful for a quick and lightweight
annotation that a variable is indeed unwind safe.
§
Examples
One way to use
AssertUnwindSafe
is to assert that the entire closure
itself is unwind safe, bypassing all checks for all variables:
use
std::panic::{
self
, AssertUnwindSafe};
let
mut
variable =
4
;
// This code will not compile because the closure captures `&mut variable`
// which is not considered unwind safe by default.

// panic::catch_unwind(|| {
//     variable += 3;
// });

// This, however, will compile due to the `AssertUnwindSafe` wrapper
let
result = panic::catch_unwind(AssertUnwindSafe(|| {
    variable +=
3
;
}));
// ...
Wrapping the entire closure amounts to a blanket assertion that all captured
variables are unwind safe. This has the downside that if new captures are
added in the future, they will also be considered unwind safe. Therefore,
you may prefer to just wrap individual captures, as shown below. This is
more annotation, but it ensures that if a new capture is added which is not
unwind safe, you will get a compilation error at that time, which will
allow you to consider whether that new capture in fact represent a bug or
not.
use
std::panic::{
self
, AssertUnwindSafe};
let
mut
variable =
4
;
let
other_capture =
3
;
let
result = {
let
mut
wrapper = AssertUnwindSafe(
&mut
variable);
    panic::catch_unwind(
move
|| {
**
wrapper += other_capture;
    })
};
// ...
Tuple Fields
§
§
0: T
Trait Implementations
§
Source
§
impl<S>
AsyncIterator
for
AssertUnwindSafe
<S>
where
    S:
AsyncIterator
,
Source
§
type
Item
= <S as
AsyncIterator
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
AssertUnwindSafe
<S>>,
    cx: &mut
Context
<'_>,
) ->
Poll
<
Option
<<S as
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
1.16.0
·
Source
§
impl<T>
Debug
for
AssertUnwindSafe
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
1.62.0
·
Source
§
impl<T>
Default
for
AssertUnwindSafe
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
AssertUnwindSafe
<T>
ⓘ
Returns the “default value” for a type.
Read more
1.9.0
·
Source
§
impl<T>
Deref
for
AssertUnwindSafe
<T>
Source
§
type
Target
= T
The resulting type after dereferencing.
Source
§
fn
deref
(&self) ->
&T
Dereferences the value.
1.9.0
·
Source
§
impl<T>
DerefMut
for
AssertUnwindSafe
<T>
Source
§
fn
deref_mut
(&mut self) ->
&mut T
Mutably dereferences the value.
1.9.0
·
Source
§
impl<R, F>
FnOnce
() for
AssertUnwindSafe
<F>
where
    F:
FnOnce
() -> R,
Source
§
type
Output
= R
The returned type after the call operator is used.
Source
§
extern "rust-call" fn
call_once
(self, _args:
()
) -> R
🔬
This is a nightly-only experimental API. (
fn_traits
#29625
)
Performs the call operation.
1.36.0
·
Source
§
impl<F>
Future
for
AssertUnwindSafe
<F>
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
The type of value produced on completion.
Source
§
fn
poll
(
    self:
Pin
<&mut
AssertUnwindSafe
<F>>,
    cx: &mut
Context
<'_>,
) ->
Poll
<<
AssertUnwindSafe
<F> as
Future
>::
Output
>
Attempts to resolve the future to a final value, registering
the current task for wakeup if the value is not yet available.
Read more
1.9.0
·
Source
§
impl<T>
RefUnwindSafe
for
AssertUnwindSafe
<T>
1.9.0
·
Source
§
impl<T>
UnwindSafe
for
AssertUnwindSafe
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
AssertUnwindSafe
<T>
where
    T:
Freeze
,
§
impl<T>
Send
for
AssertUnwindSafe
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
AssertUnwindSafe
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
AssertUnwindSafe
<T>
where
    T:
Unpin
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
impl<P, T>
Receiver
for P
where
    P:
Deref
<Target = T> + ?
Sized
,
    T: ?
Sized
,
Source
§
type
Target
= T
🔬
This is a nightly-only experimental API. (
arbitrary_self_types
#44874
)
The target type on which the method may be called.
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