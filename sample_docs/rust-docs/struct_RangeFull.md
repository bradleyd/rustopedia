RangeFull in std::range - Rust
std
::
range
Struct
RangeFull
Copy item path
Source
pub struct RangeFull;
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
Expand description
An unbounded range (
..
).
RangeFull
is primarily used as a
slicing index
, its shorthand is
..
.
It cannot serve as an
Iterator
because it doesn’t have a starting point.
§
Examples
The
..
syntax is a
RangeFull
:
assert_eq!
(.., std::ops::RangeFull);
It does not have an
IntoIterator
implementation, so you can’t use it in
a
for
loop directly. This won’t compile:
ⓘ
for
i
in
.. {
// ...
}
Used as a
slicing index
,
RangeFull
produces the full array as a slice.
let
arr = [
0
,
1
,
2
,
3
,
4
];
assert_eq!
(arr[ ..  ], [
0
,
1
,
2
,
3
,
4
]);
// This is the `RangeFull`
assert_eq!
(arr[ ..
3
], [
0
,
1
,
2
]);
assert_eq!
(arr[ ..=
3
], [
0
,
1
,
2
,
3
]);
assert_eq!
(arr[
1
..  ], [
1
,
2
,
3
,
4
]);
assert_eq!
(arr[
1
..
3
], [
1
,
2
]);
assert_eq!
(arr[
1
..=
3
], [
1
,
2
,
3
]);
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
RangeFull
Source
§
fn
clone
(&self) ->
RangeFull
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
1.0.0
·
Source
§
impl
Debug
for
RangeFull
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
1.0.0
·
Source
§
impl
Default
for
RangeFull
Source
§
fn
default
() ->
RangeFull
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl
Hash
for
RangeFull
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
impl
Index
<
RangeFull
> for
ByteStr
Source
§
type
Output
=
ByteStr
The returned type after indexing.
Source
§
fn
index
(&self, _:
RangeFull
) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
Index
<
RangeFull
> for
ByteString
Source
§
type
Output
=
ByteStr
The returned type after indexing.
Source
§
fn
index
(&self, _:
RangeFull
) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
1.7.0
·
Source
§
impl
Index
<
RangeFull
> for
CString
Source
§
type
Output
=
CStr
The returned type after indexing.
Source
§
fn
index
(&self, _index:
RangeFull
) -> &
CStr
Performs the indexing (
container[index]
) operation.
Read more
1.0.0
·
Source
§
impl
Index
<
RangeFull
> for
OsString
Source
§
type
Output
=
OsStr
The returned type after indexing.
Source
§
fn
index
(&self, _index:
RangeFull
) -> &
OsStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
RangeFull
> for
ByteStr
Source
§
fn
index_mut
(&mut self, _:
RangeFull
) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
RangeFull
> for
ByteString
Source
§
fn
index_mut
(&mut self, _:
RangeFull
) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
1.44.0
·
Source
§
impl
IndexMut
<
RangeFull
> for
OsString
Source
§
fn
index_mut
(&mut self, _index:
RangeFull
) -> &mut
OsStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl<T>
IntoBounds
<T> for
RangeFull
Source
§
fn
into_bounds
(self) -> (
Bound
<T>,
Bound
<T>)
🔬
This is a nightly-only experimental API. (
range_into_bounds
#136903
)
Convert this range into the start and end bounds.
Returns
(start_bound, end_bound)
.
Read more
Source
§
fn
intersect
<R>(self, other: R) -> (
Bound
<T>,
Bound
<T>)
where
    Self:
Sized
,
    T:
Ord
,
    R:
IntoBounds
<T>,
🔬
This is a nightly-only experimental API. (
range_into_bounds
#136903
)
Compute the intersection of
self
and
other
.
Read more
1.0.0
·
Source
§
impl
PartialEq
for
RangeFull
Source
§
fn
eq
(&self, other: &
RangeFull
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
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for
RangeFull
where
    T: ?
Sized
,
Source
§
fn
start_bound
(&self) ->
Bound
<
&T
>
Start index bound.
Read more
Source
§
fn
end_bound
(&self) ->
Bound
<
&T
>
End index bound.
Read more
1.35.0
·
Source
§
fn
contains
<U>(&self, item:
&U
) ->
bool
where
    T:
PartialOrd
<U>,
    U:
PartialOrd
<T> + ?
Sized
,
Returns
true
if
item
is contained in the range.
Read more
Source
§
fn
is_empty
(&self) ->
bool
where
    T:
PartialOrd
,
🔬
This is a nightly-only experimental API. (
range_bounds_is_empty
#137300
)
Returns
true
if the range contains no items.
One-sided ranges (
RangeFrom
, etc) always return
false
.
Read more
1.15.0
·
Source
§
impl<T>
SliceIndex
<
[T]
> for
RangeFull
Source
§
type
Output
=
[T]
The output type returned by methods.
Source
§
fn
get
(self, slice: &
[T]
) ->
Option
<&
[T]
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(self, slice: &mut
[T]
) ->
Option
<&mut
[T]
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(self, slice:
*const
[T]
) ->
*const
[T]
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(self, slice:
*mut
[T]
) ->
*mut
[T]
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
[T]
) -> &
[T]
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(self, slice: &mut
[T]
) -> &mut
[T]
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.20.0
·
Source
§
impl
SliceIndex
<
str
> for
RangeFull
Implements substring slicing with syntax
&self[..]
or
&mut self[..]
.
Returns a slice of the whole string, i.e., returns
&self
or
&mut self
. Equivalent to
&self[0 .. len]
or
&mut self[0 .. len]
. Unlike
other indexing operations, this can never panic.
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
Equivalent to
&self[0 .. len]
or
&mut self[0 .. len]
.
Source
§
type
Output
=
str
The output type returned by methods.
Source
§
fn
get
(self, slice: &
str
) ->
Option
<&<
RangeFull
as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, if in
bounds.
Source
§
fn
get_mut
(
    self,
    slice: &mut
str
,
) ->
Option
<&mut <
RangeFull
as
SliceIndex
<
str
>>::
Output
>
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, if in
bounds.
Source
§
unsafe fn
get_unchecked
(
    self,
    slice:
*const
str
,
) ->
*const
<
RangeFull
as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
unsafe fn
get_unchecked_mut
(
    self,
    slice:
*mut
str
,
) ->
*mut
<
RangeFull
as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable pointer to the output at this location, without
performing any bounds checking.
Read more
Source
§
fn
index
(self, slice: &
str
) -> &<
RangeFull
as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a shared reference to the output at this location, panicking
if out of bounds.
Source
§
fn
index_mut
(
    self,
    slice: &mut
str
,
) -> &mut <
RangeFull
as
SliceIndex
<
str
>>::
Output
🔬
This is a nightly-only experimental API. (
slice_index_methods
)
Returns a mutable reference to the output at this location, panicking
if out of bounds.
1.0.0
·
Source
§
impl
Copy
for
RangeFull
1.0.0
·
Source
§
impl
Eq
for
RangeFull
1.0.0
·
Source
§
impl
StructuralPartialEq
for
RangeFull
Auto Trait Implementations
§
§
impl
Freeze
for
RangeFull
§
impl
RefUnwindSafe
for
RangeFull
§
impl
Send
for
RangeFull
§
impl
Sync
for
RangeFull
§
impl
Unpin
for
RangeFull
§
impl
UnwindSafe
for
RangeFull
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