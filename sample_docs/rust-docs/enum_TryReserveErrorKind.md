TryReserveErrorKind in std::collections - Rust
std
::
collections
Enum
TryReserveErrorKind
Copy item path
Source
pub enum TryReserveErrorKind {
    CapacityOverflow,
    AllocError {
        layout:
Layout
,
/* private fields */
},
}
🔬
This is a nightly-only experimental API. (
try_reserve_kind
#48043
)
Expand description
Details of the allocation that caused a
TryReserveError
Variants
§
§
CapacityOverflow
🔬
This is a nightly-only experimental API. (
try_reserve_kind
#48043
)
Error due to the computed capacity exceeding the collection’s maximum
(usually
isize::MAX
bytes).
§
AllocError
🔬
This is a nightly-only experimental API. (
try_reserve_kind
#48043
)
The memory allocator returned an error
Fields
§
layout:
Layout
🔬
This is a nightly-only experimental API. (
try_reserve_kind
#48043
)
The layout of allocation request that failed
Trait Implementations
§
Source
§
impl
Clone
for
TryReserveErrorKind
Source
§
fn
clone
(&self) ->
TryReserveErrorKind
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
TryReserveErrorKind
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
impl
From
<
LayoutError
> for
TryReserveErrorKind
Source
§
fn
from
(_:
LayoutError
) ->
TryReserveErrorKind
Always evaluates to
TryReserveErrorKind::CapacityOverflow
.
Source
§
impl
From
<
TryReserveErrorKind
> for
TryReserveError
Source
§
fn
from
(kind:
TryReserveErrorKind
) ->
TryReserveError
Converts to this type from the input type.
Source
§
impl
PartialEq
for
TryReserveErrorKind
Source
§
fn
eq
(&self, other: &
TryReserveErrorKind
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
Source
§
impl
Eq
for
TryReserveErrorKind
Source
§
impl
StructuralPartialEq
for
TryReserveErrorKind
Auto Trait Implementations
§
§
impl
Freeze
for
TryReserveErrorKind
§
impl
RefUnwindSafe
for
TryReserveErrorKind
§
impl
Send
for
TryReserveErrorKind
§
impl
Sync
for
TryReserveErrorKind
§
impl
Unpin
for
TryReserveErrorKind
§
impl
UnwindSafe
for
TryReserveErrorKind
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