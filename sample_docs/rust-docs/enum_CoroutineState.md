CoroutineState in std::ops - Rust
std
::
ops
Enum
CoroutineState
Copy item path
Source
pub enum CoroutineState<Y, R> {
    Yielded(Y),
    Complete(R),
}
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
Expand description
The result of a coroutine resumption.
This enum is returned from the
Coroutine::resume
method and indicates the
possible return values of a coroutine. Currently this corresponds to either
a suspension point (
Yielded
) or a termination point (
Complete
).
Variants
§
§
Yielded(Y)
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The coroutine suspended with a value.
This state indicates that a coroutine has been suspended, and typically
corresponds to a
yield
statement. The value provided in this variant
corresponds to the expression passed to
yield
and allows coroutines to
provide a value each time they yield.
§
Complete(R)
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The coroutine completed with a return value.
This state indicates that a coroutine has finished execution with the
provided value. Once a coroutine has returned
Complete
it is
considered a programmer error to call
resume
again.
Trait Implementations
§
Source
§
impl<Y, R>
Clone
for
CoroutineState
<Y, R>
where
    Y:
Clone
,
    R:
Clone
,
Source
§
fn
clone
(&self) ->
CoroutineState
<Y, R>
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
impl<Y, R>
Debug
for
CoroutineState
<Y, R>
where
    Y:
Debug
,
    R:
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
impl<Y, R>
Hash
for
CoroutineState
<Y, R>
where
    Y:
Hash
,
    R:
Hash
,
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
Source
§
impl<Y, R>
Ord
for
CoroutineState
<Y, R>
where
    Y:
Ord
,
    R:
Ord
,
Source
§
fn
cmp
(&self, other: &
CoroutineState
<Y, R>) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
Source
§
impl<Y, R>
PartialEq
for
CoroutineState
<Y, R>
where
    Y:
PartialEq
,
    R:
PartialEq
,
Source
§
fn
eq
(&self, other: &
CoroutineState
<Y, R>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
Source
§
impl<Y, R>
PartialOrd
for
CoroutineState
<Y, R>
where
    Y:
PartialOrd
,
    R:
PartialOrd
,
Source
§
fn
partial_cmp
(&self, other: &
CoroutineState
<Y, R>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
Source
§
impl<Y, R>
Copy
for
CoroutineState
<Y, R>
where
    Y:
Copy
,
    R:
Copy
,
Source
§
impl<Y, R>
Eq
for
CoroutineState
<Y, R>
where
    Y:
Eq
,
    R:
Eq
,
Source
§
impl<Y, R>
StructuralPartialEq
for
CoroutineState
<Y, R>
Auto Trait Implementations
§
§
impl<Y, R>
Freeze
for
CoroutineState
<Y, R>
where
    Y:
Freeze
,
    R:
Freeze
,
§
impl<Y, R>
RefUnwindSafe
for
CoroutineState
<Y, R>
where
    Y:
RefUnwindSafe
,
    R:
RefUnwindSafe
,
§
impl<Y, R>
Send
for
CoroutineState
<Y, R>
where
    Y:
Send
,
    R:
Send
,
§
impl<Y, R>
Sync
for
CoroutineState
<Y, R>
where
    Y:
Sync
,
    R:
Sync
,
§
impl<Y, R>
Unpin
for
CoroutineState
<Y, R>
where
    Y:
Unpin
,
    R:
Unpin
,
§
impl<Y, R>
UnwindSafe
for
CoroutineState
<Y, R>
where
    Y:
UnwindSafe
,
    R:
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