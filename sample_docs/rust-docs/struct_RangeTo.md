RangeTo in std::range - Rust
std
::
range
Struct
RangeTo
Copy item path
Source
pub struct RangeTo<Idx> {
    pub end: Idx,
}
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
Expand description
A range only bounded exclusively above (
..end
).
The
RangeTo
..end
contains all values with
x < end
.
It cannot serve as an
Iterator
because it doesn’t have a starting point.
§
Examples
The
..end
syntax is a
RangeTo
:
assert_eq!
((..
5
), std::ops::RangeTo { end:
5
});
It does not have an
IntoIterator
implementation, so you can’t use it in
a
for
loop directly. This won’t compile:
ⓘ
// error[E0277]: the trait bound `std::ops::RangeTo<{integer}>:
// std::iter::Iterator` is not satisfied
for
i
in
..
5
{
// ...
}
When used as a
slicing index
,
RangeTo
produces a slice of all array
elements before the index indicated by
end
.
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
// This is a `RangeTo`
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
Fields
§
§
end: Idx
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
The upper bound of the range (exclusive).
Implementations
§
Source
§
impl<Idx>
RangeTo
<Idx>
where
    Idx:
PartialOrd
,
1.35.0
·
Source
pub fn
contains
<U>(&self, item:
&U
) ->
bool
where
    Idx:
PartialOrd
<U>,
    U:
PartialOrd
<Idx> + ?
Sized
,
Returns
true
if
item
is contained in the range.
§
Examples
assert!
( (..
5
).contains(
&
-
1_000_000_000
));
assert!
( (..
5
).contains(
&
4
));
assert!
(!(..
5
).contains(
&
5
));
assert!
( (..
1.0
).contains(
&
0.5
));
assert!
(!(..
1.0
).contains(
&
f32::NAN));
assert!
(!(..f32::NAN).contains(
&
0.5
));
Trait Implementations
§
1.0.0
·
Source
§
impl<Idx>
Clone
for
RangeTo
<Idx>
where
    Idx:
Clone
,
Source
§
fn
clone
(&self) ->
RangeTo
<Idx>
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
impl<Idx>
Debug
for
RangeTo
<Idx>
where
    Idx:
Debug
,
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
impl<Idx>
Hash
for
RangeTo
<Idx>
where
    Idx:
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
impl
Index
<
RangeTo
<
usize
>> for
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
(&self, r:
RangeTo
<
usize
>) -> &
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
RangeTo
<
usize
>> for
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
(&self, r:
RangeTo
<
usize
>) -> &
ByteStr
Performs the indexing (
container[index]
) operation.
Read more
Source
§
impl
IndexMut
<
RangeTo
<
usize
>> for
ByteStr
Source
§
fn
index_mut
(&mut self, r:
RangeTo
<
usize
>) -> &mut
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
RangeTo
<
usize
>> for
ByteString
Source
§
fn
index_mut
(&mut self, r:
RangeTo
<
usize
>) -> &mut
ByteStr
Performs the mutable indexing (
container[index]
) operation.
Read more
Source
§
impl<T>
IntoBounds
<T> for
RangeTo
<T>
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
Source
§
impl<T>
OneSidedRange
<T> for
RangeTo
<T>
where
RangeTo
<T>:
RangeBounds
<T>,
Source
§
fn
bound
(self) -> (
OneSidedRangeBound
, T)
🔬
This is a nightly-only experimental API. (
one_sided_range
#69780
)
An internal-only helper function for
split_off
and
split_off_mut
that returns the bound of the one-sided range.
1.0.0
·
Source
§
impl<Idx>
PartialEq
for
RangeTo
<Idx>
where
    Idx:
PartialEq
,
Source
§
fn
eq
(&self, other: &
RangeTo
<Idx>) ->
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
RangeTo
<
&T
>
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
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for
RangeTo
<T>
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
RangeTo
<
usize
>
The methods
index
and
index_mut
panic if the end of the range is out of bounds.
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
RangeTo
<
usize
>
Implements substring slicing with syntax
&self[.. end]
or
&mut self[.. end]
.
Returns a slice of the given string from the byte range [0,
end
).
Equivalent to
&self[0 .. end]
or
&mut self[0 .. end]
.
This operation is
O
(1).
Prior to 1.20.0, these indexing operations were still supported by
direct implementation of
Index
and
IndexMut
.
§
Panics
Panics if
end
does not point to the starting byte offset of a
character (as defined by
is_char_boundary
), or if
end > len
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
(
    self,
    slice: &
str
,
) ->
Option
<&<
RangeTo
<
usize
> as
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
RangeTo
<
usize
> as
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
RangeTo
<
usize
> as
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
RangeTo
<
usize
> as
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
RangeTo
<
usize
> as
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
RangeTo
<
usize
> as
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
impl<Idx>
Copy
for
RangeTo
<Idx>
where
    Idx:
Copy
,
1.0.0
·
Source
§
impl<Idx>
Eq
for
RangeTo
<Idx>
where
    Idx:
Eq
,
1.0.0
·
Source
§
impl<Idx>
StructuralPartialEq
for
RangeTo
<Idx>
Auto Trait Implementations
§
§
impl<Idx>
Freeze
for
RangeTo
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
RefUnwindSafe
for
RangeTo
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
Send
for
RangeTo
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Sync
for
RangeTo
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Unpin
for
RangeTo
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
UnwindSafe
for
RangeTo
<Idx>
where
    Idx:
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