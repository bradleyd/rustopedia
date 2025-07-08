UnwindTerminateReason in std::intrinsics::mir - Rust
std
::
intrinsics
::
mir
Enum
UnwindTerminateReason
Copy item path
Source
pub enum UnwindTerminateReason {
    Abi,
    InCleanup,
}
🔬
This is a nightly-only experimental API. (
custom_mir
)
Expand description
The reason we are terminating the process during unwinding.
Variants
§
§
Abi
🔬
This is a nightly-only experimental API. (
custom_mir
)
Unwinding is just not possible given the ABI of this function.
§
InCleanup
🔬
This is a nightly-only experimental API. (
custom_mir
)
We were already cleaning up for an ongoing unwind, and a
second
,
nested
unwind was
triggered by the drop glue.
Auto Trait Implementations
§
§
impl
Freeze
for
UnwindTerminateReason
§
impl
RefUnwindSafe
for
UnwindTerminateReason
§
impl
Send
for
UnwindTerminateReason
§
impl
Sync
for
UnwindTerminateReason
§
impl
Unpin
for
UnwindTerminateReason
§
impl
UnwindSafe
for
UnwindTerminateReason
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