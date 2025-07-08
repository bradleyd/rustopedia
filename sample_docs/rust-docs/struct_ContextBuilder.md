ContextBuilder in std::task - Rust
std
::
task
Struct
ContextBuilder
Copy item path
Source
pub struct ContextBuilder<'a> {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Expand description
A Builder used to construct a
Context
instance
with support for
LocalWaker
.
§
Examples
#![feature(local_waker)]
use
std::task::{ContextBuilder, LocalWaker, Waker, Poll};
use
std::future::Future;
let
local_waker = LocalWaker::noop();
let
waker = Waker::noop();
let
mut
cx = ContextBuilder::from_waker(
&
waker)
    .local_waker(
&
local_waker)
    .build();
let
mut
future =
std::pin::pin!
(
async
{
20
});
let
poll = future.as_mut().poll(
&mut
cx);
assert_eq!
(poll, Poll::Ready(
20
));
Implementations
§
Source
§
impl<'a>
ContextBuilder
<'a>
Source
pub const fn
from_waker
(waker: &'a
Waker
) ->
ContextBuilder
<'a>
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Creates a ContextBuilder from a Waker.
Source
pub const fn
from
(cx: &'a mut
Context
<'_>) ->
ContextBuilder
<'a>
🔬
This is a nightly-only experimental API. (
context_ext
#123392
)
Creates a ContextBuilder from an existing Context.
Source
pub const fn
waker
(self, waker: &'a
Waker
) ->
ContextBuilder
<'a>
🔬
This is a nightly-only experimental API. (
context_ext
#123392
)
Sets the value for the waker on
Context
.
Source
pub const fn
local_waker
(
    self,
    local_waker: &'a
LocalWaker
,
) ->
ContextBuilder
<'a>
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Sets the value for the local waker on
Context
.
Source
pub const fn
ext
(self, data: &'a mut (dyn
Any
+ 'static)) ->
ContextBuilder
<'a>
🔬
This is a nightly-only experimental API. (
context_ext
#123392
)
Sets the value for the extension data on
Context
.
Source
pub const fn
build
(self) ->
Context
<'a>
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Builds the
Context
.
Trait Implementations
§
Source
§
impl<'a>
Debug
for
ContextBuilder
<'a>
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
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
ContextBuilder
<'a>
§
impl<'a> !
RefUnwindSafe
for
ContextBuilder
<'a>
§
impl<'a> !
Send
for
ContextBuilder
<'a>
§
impl<'a> !
Sync
for
ContextBuilder
<'a>
§
impl<'a>
Unpin
for
ContextBuilder
<'a>
§
impl<'a> !
UnwindSafe
for
ContextBuilder
<'a>
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