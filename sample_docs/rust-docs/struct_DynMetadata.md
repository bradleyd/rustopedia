DynMetadata in std::ptr - Rust
std
::
ptr
Struct
DynMetadata
Copy item path
Source
pub struct DynMetadata<Dyn>
where
    Dyn: ?
Sized
,
{
/* private fields */
}
🔬
This is a nightly-only experimental API. (
ptr_metadata
#81513
)
Expand description
The metadata for a
Dyn = dyn SomeTrait
trait object type.
It is a pointer to a vtable (virtual call table)
that represents all the necessary information
to manipulate the concrete type stored inside a trait object.
The vtable notably contains:
type size
type alignment
a pointer to the type’s
drop_in_place
impl (may be a no-op for plain-old-data)
pointers to all the methods for the type’s implementation of the trait
Note that the first three are special because they’re necessary to allocate, drop,
and deallocate any trait object.
It is possible to name this struct with a type parameter that is not a
dyn
trait object
(for example
DynMetadata<u64>
) but not to obtain a meaningful value of that struct.
Note that while this type implements
PartialEq
, comparing vtable pointers is unreliable:
pointers to vtables of the same type for the same trait can compare inequal (because vtables are
duplicated in multiple codegen units), and pointers to vtables of
different
types/traits can
compare equal (since identical vtables can be deduplicated within a codegen unit).
Implementations
§
Source
§
impl<Dyn>
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
pub fn
size_of
(self) ->
usize
🔬
This is a nightly-only experimental API. (
ptr_metadata
#81513
)
Returns the size of the type associated with this vtable.
Source
pub fn
align_of
(self) ->
usize
🔬
This is a nightly-only experimental API. (
ptr_metadata
#81513
)
Returns the alignment of the type associated with this vtable.
Source
pub fn
layout
(self) ->
Layout
🔬
This is a nightly-only experimental API. (
ptr_metadata
#81513
)
Returns the size and alignment together as a
Layout
Trait Implementations
§
Source
§
impl<Dyn>
Clone
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
fn
clone
(&self) ->
DynMetadata
<Dyn>
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
impl<Dyn>
Debug
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
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
impl<Dyn>
Hash
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
fn
hash
<H>(&self, hasher:
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
Source
§
impl<Dyn>
Ord
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
fn
cmp
(&self, other: &
DynMetadata
<Dyn>) ->
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
impl<Dyn>
PartialEq
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
fn
eq
(&self, other: &
DynMetadata
<Dyn>) ->
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
impl<Dyn>
PartialOrd
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
fn
partial_cmp
(&self, other: &
DynMetadata
<Dyn>) ->
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
impl<Dyn>
Copy
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
impl<Dyn>
Eq
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
impl<Dyn>
Send
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
impl<Dyn>
Sync
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Source
§
impl<Dyn>
Unpin
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
Auto Trait Implementations
§
§
impl<Dyn>
Freeze
for
DynMetadata
<Dyn>
where
    Dyn: ?
Sized
,
§
impl<Dyn> !
RefUnwindSafe
for
DynMetadata
<Dyn>
§
impl<Dyn> !
UnwindSafe
for
DynMetadata
<Dyn>
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