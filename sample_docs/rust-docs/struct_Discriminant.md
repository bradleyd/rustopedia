Discriminant in std::mem - Rust
std
::
mem
Struct
Discriminant
Copy item path
1.21.0
·
Source
pub struct Discriminant<T>(
/* private fields */
);
Expand description
Opaque type representing the discriminant of an enum.
See the
discriminant
function in this module for more information.
Trait Implementations
§
1.21.0
·
Source
§
impl<T>
Clone
for
Discriminant
<T>
Source
§
fn
clone
(&self) ->
Discriminant
<T>
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
1.21.0
·
Source
§
impl<T>
Debug
for
Discriminant
<T>
Source
§
fn
fmt
(&self, fmt: &mut
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
1.21.0
·
Source
§
impl<T>
Hash
for
Discriminant
<T>
Source
§
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
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
1.21.0
·
Source
§
impl<T>
PartialEq
for
Discriminant
<T>
Source
§
fn
eq
(&self, rhs: &
Discriminant
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
1.21.0
·
Source
§
impl<T>
Copy
for
Discriminant
<T>
1.21.0
·
Source
§
impl<T>
Eq
for
Discriminant
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
Discriminant
<T>
where
    <T as
DiscriminantKind
>::
Discriminant
:
Freeze
,
§
impl<T>
RefUnwindSafe
for
Discriminant
<T>
where
    <T as
DiscriminantKind
>::
Discriminant
:
RefUnwindSafe
,
§
impl<T>
Send
for
Discriminant
<T>
§
impl<T>
Sync
for
Discriminant
<T>
§
impl<T>
Unpin
for
Discriminant
<T>
§
impl<T>
UnwindSafe
for
Discriminant
<T>
where
    <T as
DiscriminantKind
>::
Discriminant
:
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