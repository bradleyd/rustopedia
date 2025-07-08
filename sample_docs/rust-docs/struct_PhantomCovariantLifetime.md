PhantomCovariantLifetime in std::marker - Rust
std
::
marker
Struct
PhantomCovariantLifetime
Copy item path
Source
pub struct PhantomCovariantLifetime<'a>(
/* private fields */
);
🔬
This is a nightly-only experimental API. (
phantom_variance_markers
#135806
)
Expand description
Zero-sized type used to mark a lifetime as covariant.
Covariant lifetimes must live at least as long as declared. See
the reference
for more
information.
§
Layout
For all
'a
, the following are guaranteed:
size_of::<PhantomCovariantLifetime<'a>>() == 0
align_of::<PhantomCovariantLifetime<'a>>() == 1
Implementations
§
Source
§
impl
PhantomCovariantLifetime
<'_>
Source
pub const fn
new
() ->
PhantomCovariantLifetime
<'_>
🔬
This is a nightly-only experimental API. (
phantom_variance_markers
#135806
)
Constructs a new instance of the variance marker.
Trait Implementations
§
Source
§
impl<'a>
Clone
for
PhantomCovariantLifetime
<'a>
Source
§
fn
clone
(&self) ->
PhantomCovariantLifetime
<'a>
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
impl
Debug
for
PhantomCovariantLifetime
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
Source
§
impl<'a>
Default
for
PhantomCovariantLifetime
<'a>
Source
§
fn
default
() ->
PhantomCovariantLifetime
<'a>
Returns the “default value” for a type.
Read more
Source
§
impl<'a>
Hash
for
PhantomCovariantLifetime
<'a>
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
impl<'a>
Ord
for
PhantomCovariantLifetime
<'a>
Source
§
fn
cmp
(&self, other: &
PhantomCovariantLifetime
<'a>) ->
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
impl<'a>
PartialEq
for
PhantomCovariantLifetime
<'a>
Source
§
fn
eq
(&self, other: &
PhantomCovariantLifetime
<'a>) ->
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
impl<'a>
PartialOrd
for
PhantomCovariantLifetime
<'a>
Source
§
fn
partial_cmp
(&self, other: &
PhantomCovariantLifetime
<'a>) ->
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
impl<'a>
Copy
for
PhantomCovariantLifetime
<'a>
Source
§
impl<'a>
Eq
for
PhantomCovariantLifetime
<'a>
Source
§
impl<'a>
StructuralPartialEq
for
PhantomCovariantLifetime
<'a>
Source
§
impl
Variance
for
PhantomCovariantLifetime
<'_>
Auto Trait Implementations
§
§
impl<'a>
Freeze
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
RefUnwindSafe
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
Send
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
Sync
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
Unpin
for
PhantomCovariantLifetime
<'a>
§
impl<'a>
UnwindSafe
for
PhantomCovariantLifetime
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