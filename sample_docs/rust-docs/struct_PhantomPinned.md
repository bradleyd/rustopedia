PhantomPinned in std::marker - Rust
std
::
marker
Struct
PhantomPinned
Copy item path
1.33.0
·
Source
pub struct PhantomPinned;
Expand description
A marker type which does not implement
Unpin
.
If a type contains a
PhantomPinned
, it will not implement
Unpin
by default.
Trait Implementations
§
1.33.0
·
Source
§
impl
Clone
for
PhantomPinned
Source
§
fn
clone
(&self) ->
PhantomPinned
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
1.33.0
·
Source
§
impl
Debug
for
PhantomPinned
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
1.33.0
·
Source
§
impl
Default
for
PhantomPinned
Source
§
fn
default
() ->
PhantomPinned
Returns the “default value” for a type.
Read more
1.33.0
·
Source
§
impl
Hash
for
PhantomPinned
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
1.33.0
·
Source
§
impl
Ord
for
PhantomPinned
Source
§
fn
cmp
(&self, other: &
PhantomPinned
) ->
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
1.33.0
·
Source
§
impl
PartialEq
for
PhantomPinned
Source
§
fn
eq
(&self, other: &
PhantomPinned
) ->
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
1.33.0
·
Source
§
impl
PartialOrd
for
PhantomPinned
Source
§
fn
partial_cmp
(&self, other: &
PhantomPinned
) ->
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
1.33.0
·
Source
§
impl
Copy
for
PhantomPinned
1.33.0
·
Source
§
impl
Eq
for
PhantomPinned
1.33.0
·
Source
§
impl
StructuralPartialEq
for
PhantomPinned
1.33.0
·
Source
§
impl !
Unpin
for
PhantomPinned
Auto Trait Implementations
§
§
impl
Freeze
for
PhantomPinned
§
impl
RefUnwindSafe
for
PhantomPinned
§
impl
Send
for
PhantomPinned
§
impl
Sync
for
PhantomPinned
§
impl
UnwindSafe
for
PhantomPinned
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