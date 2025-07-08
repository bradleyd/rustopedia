OneSidedRangeBound in std::ops - Rust
std
::
ops
Enum
OneSidedRangeBound
Copy item path
Source
pub enum OneSidedRangeBound {
    StartInclusive,
    End,
    EndInclusive,
}
🔬
This is a nightly-only experimental API. (
one_sided_range
#69780
)
Expand description
An internal helper for
split_off
functions indicating
which end a
OneSidedRange
is bounded on.
Variants
§
§
StartInclusive
🔬
This is a nightly-only experimental API. (
one_sided_range
#69780
)
The range is bounded inclusively from below and is unbounded above.
§
End
🔬
This is a nightly-only experimental API. (
one_sided_range
#69780
)
The range is bounded exclusively from above and is unbounded below.
§
EndInclusive
🔬
This is a nightly-only experimental API. (
one_sided_range
#69780
)
The range is bounded inclusively from above and is unbounded below.
Auto Trait Implementations
§
§
impl
Freeze
for
OneSidedRangeBound
§
impl
RefUnwindSafe
for
OneSidedRangeBound
§
impl
Send
for
OneSidedRangeBound
§
impl
Sync
for
OneSidedRangeBound
§
impl
Unpin
for
OneSidedRangeBound
§
impl
UnwindSafe
for
OneSidedRangeBound
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