Reverse in std::cmp - Rust
std
::
cmp
Struct
Reverse
Copy item path
1.19.0
·
Source
#[repr(transparent)]
pub struct Reverse<T>(pub T);
Expand description
A helper struct for reverse ordering.
This struct is a helper to be used with functions like
Vec::sort_by_key
and
can be used to reverse order a part of a key.
§
Examples
use
std::cmp::Reverse;
let
mut
v =
vec!
[
1
,
2
,
3
,
4
,
5
,
6
];
v.sort_by_key(|
&
num| (num >
3
, Reverse(num)));
assert_eq!
(v,
vec!
[
3
,
2
,
1
,
6
,
5
,
4
]);
Tuple Fields
§
§
0: T
Trait Implementations
§
1.19.0
·
Source
§
impl<T>
Clone
for
Reverse
<T>
where
    T:
Clone
,
Source
§
fn
clone
(&self) ->
Reverse
<T>
Returns a copy of the value.
Read more
Source
§
fn
clone_from
(&mut self, source: &
Reverse
<T>)
Performs copy-assignment from
source
.
Read more
1.19.0
·
Source
§
impl<T>
Debug
for
Reverse
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
1.19.0
·
Source
§
impl<T>
Default
for
Reverse
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
Reverse
<T>
Returns the “default value” for a type.
Read more
1.19.0
·
Source
§
impl<T>
Hash
for
Reverse
<T>
where
    T:
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
1.19.0
·
Source
§
impl<T>
Ord
for
Reverse
<T>
where
    T:
Ord
,
Source
§
fn
cmp
(&self, other: &
Reverse
<T>) ->
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
1.19.0
·
Source
§
impl<T>
PartialEq
for
Reverse
<T>
where
    T:
PartialEq
,
Source
§
fn
eq
(&self, other: &
Reverse
<T>) ->
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
1.19.0
·
Source
§
impl<T>
PartialOrd
for
Reverse
<T>
where
    T:
PartialOrd
,
Source
§
fn
partial_cmp
(&self, other: &
Reverse
<T>) ->
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
Source
§
fn
lt
(&self, other: &
Reverse
<T>) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
Source
§
fn
le
(&self, other: &
Reverse
<T>) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
Source
§
fn
gt
(&self, other: &
Reverse
<T>) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
Source
§
fn
ge
(&self, other: &
Reverse
<T>) ->
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
1.19.0
·
Source
§
impl<T>
Copy
for
Reverse
<T>
where
    T:
Copy
,
1.19.0
·
Source
§
impl<T>
Eq
for
Reverse
<T>
where
    T:
Eq
,
1.19.0
·
Source
§
impl<T>
StructuralPartialEq
for
Reverse
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Reverse
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Reverse
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
Reverse
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
Reverse
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
Reverse
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
Reverse
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