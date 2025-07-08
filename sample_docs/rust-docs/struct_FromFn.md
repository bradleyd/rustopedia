FromFn in std::fmt - Rust
std
::
fmt
Struct
FromFn
Copy item path
Source
pub struct FromFn<F>(
/* private fields */
)
where
    F:
Fn
(&mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
;
🔬
This is a nightly-only experimental API. (
debug_closure_helpers
#117729
)
Expand description
Implements
fmt::Debug
and
fmt::Display
using a function.
Created with
from_fn
.
Trait Implementations
§
Source
§
impl<F>
Debug
for
FromFn
<F>
where
    F:
Fn
(&mut
Formatter
<'_>) ->
Result
<
()
,
Error
>,
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
impl<F>
Display
for
FromFn
<F>
where
    F:
Fn
(&mut
Formatter
<'_>) ->
Result
<
()
,
Error
>,
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
impl<F>
Freeze
for
FromFn
<F>
where
    F:
Freeze
,
§
impl<F>
RefUnwindSafe
for
FromFn
<F>
where
    F:
RefUnwindSafe
,
§
impl<F>
Send
for
FromFn
<F>
where
    F:
Send
,
§
impl<F>
Sync
for
FromFn
<F>
where
    F:
Sync
,
§
impl<F>
Unpin
for
FromFn
<F>
where
    F:
Unpin
,
§
impl<F>
UnwindSafe
for
FromFn
<F>
where
    F:
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
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
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