Context in std::task - Rust
std
::
task
Struct
Context
Copy item path
1.36.0
·
Source
pub struct Context<'a> {
/* private fields */
}
Expand description
The context of an asynchronous task.
Currently,
Context
only serves to provide access to a
&Waker
which can be used to wake the current task.
Implementations
§
Source
§
impl<'a>
Context
<'a>
1.36.0 (const: 1.82.0)
·
Source
pub const fn
from_waker
(waker: &'a
Waker
) ->
Context
<'a>
Creates a new
Context
from a
&Waker
.
1.36.0 (const: 1.82.0)
·
Source
pub const fn
waker
(&self) -> &'a
Waker
Returns a reference to the
Waker
for the current task.
Source
pub const fn
local_waker
(&self) -> &'a
LocalWaker
🔬
This is a nightly-only experimental API. (
local_waker
#118959
)
Returns a reference to the
LocalWaker
for the current task.
Source
pub const fn
ext
(&mut self) -> &mut (dyn
Any
+ 'static)
🔬
This is a nightly-only experimental API. (
context_ext
#123392
)
Returns a reference to the extension data for the current task.
Trait Implementations
§
1.36.0
·
Source
§
impl
Debug
for
Context
<'_>
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
Context
<'a>
§
impl<'a>
RefUnwindSafe
for
Context
<'a>
§
impl<'a> !
Send
for
Context
<'a>
§
impl<'a> !
Sync
for
Context
<'a>
§
impl<'a>
Unpin
for
Context
<'a>
§
impl<'a>
UnwindSafe
for
Context
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