Yeet in std::ops - Rust
std
::
ops
Struct
Yeet
Copy item path
Source
pub struct Yeet<T>(pub T);
🔬
This is a nightly-only experimental API. (
try_trait_v2_yeet
#96374
)
Expand description
Implement
FromResidual<Yeet<T>>
on your type to enable
do yeet expr
syntax in functions returning your type.
Tuple Fields
§
§
0: T
🔬
This is a nightly-only experimental API. (
try_trait_v2_yeet
#96374
)
Trait Implementations
§
Source
§
impl<T>
Debug
for
Yeet
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
Source
§
impl<T>
FromResidual
<
Yeet
<
()
>> for
Option
<T>
Source
§
fn
from_residual
(_:
Yeet
<
()
>) ->
Option
<T>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
Source
§
impl<T, E, F>
FromResidual
<
Yeet
<E>> for
Result
<T, F>
where
    F:
From
<E>,
Source
§
fn
from_residual
(_:
Yeet
<E>) ->
Result
<T, F>
🔬
This is a nightly-only experimental API. (
try_trait_v2
#84277
)
Constructs the type from a compatible
Residual
type.
Read more
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Yeet
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Yeet
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Yeet
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Yeet
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
Yeet
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
Yeet
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