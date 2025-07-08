Range in std::range::legacy - Rust
std
::
range
::
legacy
Struct
Range
Copy item path
Source
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
Expand description
A (half-open) range bounded inclusively below and exclusively above
(
start..end
).
The range
start..end
contains all values with
start <= x < end
.
It is empty if
start >= end
.
§
Examples
The
start..end
syntax is a
Range
:
assert_eq!
((
3
..
5
), std::ops::Range { start:
3
, end:
5
});
assert_eq!
(
3
+
4
+
5
, (
3
..
6
).sum());
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
// This is a `Range`
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
start: Idx
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
The lower bound of the range (inclusive).
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
Range
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
(!(
3
..
5
).contains(
&
2
));
assert!
( (
3
..
5
).contains(
&
3
));
assert!
( (
3
..
5
).contains(
&
4
));
assert!
(!(
3
..
5
).contains(
&
5
));
assert!
(!(
3
..
3
).contains(
&
3
));
assert!
(!(
3
..
2
).contains(
&
3
));
assert!
( (
0.0
..
1.0
).contains(
&
0.5
));
assert!
(!(
0.0
..
1.0
).contains(
&
f32::NAN));
assert!
(!(
0.0
..f32::NAN).contains(
&
0.5
));
assert!
(!(f32::NAN..
1.0
).contains(
&
0.5
));
1.47.0
·
Source
pub fn
is_empty
(&self) ->
bool
Returns
true
if the range contains no items.
§
Examples
assert!
(!(
3
..
5
).is_empty());
assert!
( (
3
..
3
).is_empty());
assert!
( (
3
..
2
).is_empty());
The range is empty if either side is incomparable:
assert!
(!(
3.0
..
5.0
).is_empty());
assert!
( (
3.0
..f32::NAN).is_empty());
assert!
( (f32::NAN..
5.0
).is_empty());
Trait Implementations
§
1.0.0
·
Source
§
impl<Idx>
Clone
for
Range
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
Range
<Idx>
ⓘ
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
Range
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
Default
for
Range
<Idx>
where
    Idx:
Default
,
Source
§
fn
default
() ->
Range
<Idx>
ⓘ
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl<A>
DoubleEndedIterator
for
Range
<A>
where
    A:
Step
,
Source
§
fn
next_back
(&mut self) ->
Option
<A>
Removes and returns an element from the end of the iterator.
Read more
Source
§
fn
nth_back
(&mut self, n:
usize
) ->
Option
<A>
Returns the
n
th element from the end of the iterator.
Read more
Source
§
fn
advance_back_by
(&mut self, n:
usize
) ->
Result
<
()
,
NonZero
<
usize
>>
🔬
This is a nightly-only experimental API. (
iter_advance_by
#77404
)
Advances the iterator from the back by
n
elements.
Read more
1.27.0
·
Source
§
fn
try_rfold
<B, F, R>(&mut self, init: B, f: F) -> R
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> R,
    R:
Try
<Output = B>,
This is the reverse version of
Iterator::try_fold()
: it takes
elements starting from the back of the iterator.
Read more
1.27.0
·
Source
§
fn
rfold
<B, F>(self, init: B, f: F) -> B
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> B,
An iterator method that reduces the iterator’s elements to a single,
final value, starting from the back.
Read more
1.27.0
·
Source
§
fn
rfind
<P>(&mut self, predicate: P) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Searches for an element of an iterator from the back that satisfies a predicate.
Read more
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
i16
>
1.0.0
·
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
i32
>
1.0.0
·
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
i8
>
1.0.0
·
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
isize
>
1.0.0
·
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
u16
>
1.0.0
·
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
u32
>
1.0.0
·
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
u8
>
1.0.0
·
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
usize
>
1.0.0
·
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
Source
§
impl<T>
From
<
Range
<T>> for
Range
<T>
Source
§
fn
from
(value:
Range
<T>) ->
Range
<T>
ⓘ
Converts to this type from the input type.
Source
§
impl<T>
From
<
Range
<T>> for
Range
<T>
Source
§
fn
from
(value:
Range
<T>) ->
Range
<T>
Converts to this type from the input type.
Source
§
impl
GetDisjointMutIndex
for
Range
<
usize
>
Source
§
fn
is_in_bounds
(&self, len:
usize
) ->
bool
🔬
This is a nightly-only experimental API. (
get_disjoint_mut_helpers
)
Returns
true
if
self
is in bounds for
len
slice elements.
Source
§
fn
is_overlapping
(&self, other: &
Range
<
usize
>) ->
bool
🔬
This is a nightly-only experimental API. (
get_disjoint_mut_helpers
)
Returns
true
if
self
overlaps with
other
.
Read more
1.0.0
·
Source
§
impl<Idx>
Hash
for
Range
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
Range
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
Range
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
Range
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
Range
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
Range
<
usize
>> for
ByteStr
Source
§
fn
index_mut
(&mut self, r:
Range
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
Range
<
usize
>> for
ByteString
Source
§
fn
index_mut
(&mut self, r:
Range
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
Range
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
1.0.0
·
Source
§
impl<A>
Iterator
for
Range
<A>
where
    A:
Step
,
Source
§
type
Item
= A
The type of the elements being iterated over.
Source
§
fn
next
(&mut self) ->
Option
<A>
Advances the iterator and returns the next value.
Read more
Source
§
fn
size_hint
(&self) -> (
usize
,
Option
<
usize
>)
Returns the bounds on the remaining length of the iterator.
Read more
Source
§
fn
count
(self) ->
usize
Consumes the iterator, counting the number of iterations and returning it.
Read more
Source
§
fn
nth
(&mut self, n:
usize
) ->
Option
<A>
Returns the
n
th element of the iterator.
Read more
Source
§
fn
last
(self) ->
Option
<A>
Consumes the iterator, returning the last element.
Read more
Source
§
fn
min
(self) ->
Option
<A>
where
    A:
Ord
,
Returns the minimum element of an iterator.
Read more
Source
§
fn
max
(self) ->
Option
<A>
where
    A:
Ord
,
Returns the maximum element of an iterator.
Read more
Source
§
fn
is_sorted
(self) ->
bool
Checks if the elements of this iterator are sorted.
Read more
Source
§
fn
advance_by
(&mut self, n:
usize
) ->
Result
<
()
,
NonZero
<
usize
>>
🔬
This is a nightly-only experimental API. (
iter_advance_by
#77404
)
Advances the iterator by
n
elements.
Read more
Source
§
fn
next_chunk
<const N:
usize
>(
    &mut self,
) ->
Result
<[Self::
Item
;
N
],
IntoIter
<Self::
Item
, N>>
where
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
iter_next_chunk
#98326
)
Advances the iterator and returns an array containing the next
N
values.
Read more
1.28.0
·
Source
§
fn
step_by
(self, step:
usize
) ->
StepBy
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator starting at the same point, but stepping by
the given amount at each iteration.
Read more
1.0.0
·
Source
§
fn
chain
<U>(self, other: U) ->
Chain
<Self, <U as
IntoIterator
>::
IntoIter
>
ⓘ
where
    Self:
Sized
,
    U:
IntoIterator
<Item = Self::
Item
>,
Takes two iterators and creates a new iterator over both in sequence.
Read more
1.0.0
·
Source
§
fn
zip
<U>(self, other: U) ->
Zip
<Self, <U as
IntoIterator
>::
IntoIter
>
ⓘ
where
    Self:
Sized
,
    U:
IntoIterator
,
‘Zips up’ two iterators into a single iterator of pairs.
Read more
Source
§
fn
intersperse
(self, separator: Self::
Item
) ->
Intersperse
<Self>
ⓘ
where
    Self:
Sized
,
    Self::
Item
:
Clone
,
🔬
This is a nightly-only experimental API. (
iter_intersperse
#79524
)
Creates a new iterator which places a copy of
separator
between adjacent
items of the original iterator.
Read more
Source
§
fn
intersperse_with
<G>(self, separator: G) ->
IntersperseWith
<Self, G>
ⓘ
where
    Self:
Sized
,
    G:
FnMut
() -> Self::
Item
,
🔬
This is a nightly-only experimental API. (
iter_intersperse
#79524
)
Creates a new iterator which places an item generated by
separator
between adjacent items of the original iterator.
Read more
1.0.0
·
Source
§
fn
map
<B, F>(self, f: F) ->
Map
<Self, F>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) -> B,
Takes a closure and creates an iterator which calls that closure on each
element.
Read more
1.21.0
·
Source
§
fn
for_each
<F>(self, f: F)
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
),
Calls a closure on each element of an iterator.
Read more
1.0.0
·
Source
§
fn
filter
<P>(self, predicate: P) ->
Filter
<Self, P>
ⓘ
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Creates an iterator which uses a closure to determine if an element
should be yielded.
Read more
1.0.0
·
Source
§
fn
filter_map
<B, F>(self, f: F) ->
FilterMap
<Self, F>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) ->
Option
<B>,
Creates an iterator that both filters and maps.
Read more
1.0.0
·
Source
§
fn
enumerate
(self) ->
Enumerate
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator which gives the current iteration count as well as
the next value.
Read more
1.0.0
·
Source
§
fn
peekable
(self) ->
Peekable
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator which can use the
peek
and
peek_mut
methods
to look at the next element of the iterator without consuming it. See
their documentation for more information.
Read more
1.0.0
·
Source
§
fn
skip_while
<P>(self, predicate: P) ->
SkipWhile
<Self, P>
ⓘ
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Creates an iterator that
skip
s elements based on a predicate.
Read more
1.0.0
·
Source
§
fn
take_while
<P>(self, predicate: P) ->
TakeWhile
<Self, P>
ⓘ
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Creates an iterator that yields elements based on a predicate.
Read more
1.57.0
·
Source
§
fn
map_while
<B, P>(self, predicate: P) ->
MapWhile
<Self, P>
ⓘ
where
    Self:
Sized
,
    P:
FnMut
(Self::
Item
) ->
Option
<B>,
Creates an iterator that both yields elements based on a predicate and maps.
Read more
1.0.0
·
Source
§
fn
skip
(self, n:
usize
) ->
Skip
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator that skips the first
n
elements.
Read more
1.0.0
·
Source
§
fn
take
(self, n:
usize
) ->
Take
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator that yields the first
n
elements, or fewer
if the underlying iterator ends sooner.
Read more
1.0.0
·
Source
§
fn
scan
<St, B, F>(self, initial_state: St, f: F) ->
Scan
<Self, St, F>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(
&mut St
, Self::
Item
) ->
Option
<B>,
An iterator adapter which, like
fold
, holds internal state, but
unlike
fold
, produces a new iterator.
Read more
1.0.0
·
Source
§
fn
flat_map
<U, F>(self, f: F) ->
FlatMap
<Self, U, F>
ⓘ
where
    Self:
Sized
,
    U:
IntoIterator
,
    F:
FnMut
(Self::
Item
) -> U,
Creates an iterator that works like map, but flattens nested structure.
Read more
1.29.0
·
Source
§
fn
flatten
(self) ->
Flatten
<Self>
ⓘ
where
    Self:
Sized
,
    Self::
Item
:
IntoIterator
,
Creates an iterator that flattens nested structure.
Read more
Source
§
fn
map_windows
<F, R, const N:
usize
>(self, f: F) ->
MapWindows
<Self, F, N>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(&[Self::
Item
;
N
]) -> R,
🔬
This is a nightly-only experimental API. (
iter_map_windows
#87155
)
Calls the given function
f
for each contiguous window of size
N
over
self
and returns an iterator over the outputs of
f
. Like
slice::windows()
,
the windows during mapping overlap as well.
Read more
1.0.0
·
Source
§
fn
fuse
(self) ->
Fuse
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator which ends after the first
None
.
Read more
1.0.0
·
Source
§
fn
inspect
<F>(self, f: F) ->
Inspect
<Self, F>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
),
Does something with each element of an iterator, passing the value on.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adapter for this instance of
Iterator
.
Read more
1.0.0
·
Source
§
fn
collect
<B>(self) -> B
where
    B:
FromIterator
<Self::
Item
>,
    Self:
Sized
,
Transforms an iterator into a collection.
Read more
Source
§
fn
try_collect
<B>(
    &mut self,
) -> <<Self::
Item
as
Try
>::
Residual
as
Residual
<B>>::
TryType
where
    Self:
Sized
,
    Self::
Item
:
Try
,
    <Self::
Item
as
Try
>::
Residual
:
Residual
<B>,
    B:
FromIterator
<<Self::
Item
as
Try
>::
Output
>,
🔬
This is a nightly-only experimental API. (
iterator_try_collect
#94047
)
Fallibly transforms an iterator into a collection, short circuiting if
a failure is encountered.
Read more
Source
§
fn
collect_into
<E>(self, collection:
&mut E
) ->
&mut E
where
    E:
Extend
<Self::
Item
>,
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
iter_collect_into
#94780
)
Collects all the items from an iterator into a collection.
Read more
1.0.0
·
Source
§
fn
partition
<B, F>(self, f: F) ->
(B, B)
where
    Self:
Sized
,
    B:
Default
+
Extend
<Self::
Item
>,
    F:
FnMut
(&Self::
Item
) ->
bool
,
Consumes an iterator, creating two collections from it.
Read more
Source
§
fn
partition_in_place
<'a, T, P>(self, predicate: P) ->
usize
where
    T: 'a,
    Self:
Sized
+
DoubleEndedIterator
<Item =
&'a mut T
>,
    P:
FnMut
(
&T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
iter_partition_in_place
#62543
)
Reorders the elements of this iterator
in-place
according to the given predicate,
such that all those that return
true
precede all those that return
false
.
Returns the number of
true
elements found.
Read more
Source
§
fn
is_partitioned
<P>(self, predicate: P) ->
bool
where
    Self:
Sized
,
    P:
FnMut
(Self::
Item
) ->
bool
,
🔬
This is a nightly-only experimental API. (
iter_is_partitioned
#62544
)
Checks if the elements of this iterator are partitioned according to the given predicate,
such that all those that return
true
precede all those that return
false
.
Read more
1.27.0
·
Source
§
fn
try_fold
<B, F, R>(&mut self, init: B, f: F) -> R
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> R,
    R:
Try
<Output = B>,
An iterator method that applies a function as long as it returns
successfully, producing a single, final value.
Read more
1.27.0
·
Source
§
fn
try_for_each
<F, R>(&mut self, f: F) -> R
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) -> R,
    R:
Try
<Output =
()
>,
An iterator method that applies a fallible function to each item in the
iterator, stopping at the first error and returning that error.
Read more
1.0.0
·
Source
§
fn
fold
<B, F>(self, init: B, f: F) -> B
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> B,
Folds every element into an accumulator by applying an operation,
returning the final result.
Read more
1.51.0
·
Source
§
fn
reduce
<F>(self, f: F) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
, Self::
Item
) -> Self::
Item
,
Reduces the elements to a single one, by repeatedly applying a reducing
operation.
Read more
Source
§
fn
try_reduce
<R>(
    &mut self,
    f: impl
FnMut
(Self::
Item
, Self::
Item
) -> R,
) -> <<R as
Try
>::
Residual
as
Residual
<
Option
<<R as
Try
>::
Output
>>>::
TryType
where
    Self:
Sized
,
    R:
Try
<Output = Self::
Item
>,
    <R as
Try
>::
Residual
:
Residual
<
Option
<Self::
Item
>>,
🔬
This is a nightly-only experimental API. (
iterator_try_reduce
#87053
)
Reduces the elements to a single one by repeatedly applying a reducing operation. If the
closure returns a failure, the failure is propagated back to the caller immediately.
Read more
1.0.0
·
Source
§
fn
all
<F>(&mut self, f: F) ->
bool
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) ->
bool
,
Tests if every element of the iterator matches a predicate.
Read more
1.0.0
·
Source
§
fn
any
<F>(&mut self, f: F) ->
bool
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) ->
bool
,
Tests if any element of the iterator matches a predicate.
Read more
1.0.0
·
Source
§
fn
find
<P>(&mut self, predicate: P) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Searches for an element of an iterator that satisfies a predicate.
Read more
1.30.0
·
Source
§
fn
find_map
<B, F>(&mut self, f: F) ->
Option
<B>
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) ->
Option
<B>,
Applies function to the elements of iterator and returns
the first non-none result.
Read more
Source
§
fn
try_find
<R>(
    &mut self,
    f: impl
FnMut
(&Self::
Item
) -> R,
) -> <<R as
Try
>::
Residual
as
Residual
<
Option
<Self::
Item
>>>::
TryType
where
    Self:
Sized
,
    R:
Try
<Output =
bool
>,
    <R as
Try
>::
Residual
:
Residual
<
Option
<Self::
Item
>>,
🔬
This is a nightly-only experimental API. (
try_find
#63178
)
Applies function to the elements of iterator and returns
the first true result or the first error.
Read more
1.0.0
·
Source
§
fn
position
<P>(&mut self, predicate: P) ->
Option
<
usize
>
where
    Self:
Sized
,
    P:
FnMut
(Self::
Item
) ->
bool
,
Searches for an element in an iterator, returning its index.
Read more
1.0.0
·
Source
§
fn
rposition
<P>(&mut self, predicate: P) ->
Option
<
usize
>
where
    P:
FnMut
(Self::
Item
) ->
bool
,
    Self:
Sized
+
ExactSizeIterator
+
DoubleEndedIterator
,
Searches for an element in an iterator from the right, returning its
index.
Read more
1.6.0
·
Source
§
fn
max_by_key
<B, F>(self, f: F) ->
Option
<Self::
Item
>
where
    B:
Ord
,
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
) -> B,
Returns the element that gives the maximum value from the
specified function.
Read more
1.15.0
·
Source
§
fn
max_by
<F>(self, compare: F) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
, &Self::
Item
) ->
Ordering
,
Returns the element that gives the maximum value with respect to the
specified comparison function.
Read more
1.6.0
·
Source
§
fn
min_by_key
<B, F>(self, f: F) ->
Option
<Self::
Item
>
where
    B:
Ord
,
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
) -> B,
Returns the element that gives the minimum value from the
specified function.
Read more
1.15.0
·
Source
§
fn
min_by
<F>(self, compare: F) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
, &Self::
Item
) ->
Ordering
,
Returns the element that gives the minimum value with respect to the
specified comparison function.
Read more
1.0.0
·
Source
§
fn
rev
(self) ->
Rev
<Self>
ⓘ
where
    Self:
Sized
+
DoubleEndedIterator
,
Reverses an iterator’s direction.
Read more
1.0.0
·
Source
§
fn
unzip
<A, B, FromA, FromB>(self) ->
(FromA, FromB)
where
    FromA:
Default
+
Extend
<A>,
    FromB:
Default
+
Extend
<B>,
    Self:
Sized
+
Iterator
<Item =
(A, B)
>,
Converts an iterator of pairs into a pair of containers.
Read more
1.36.0
·
Source
§
fn
copied
<'a, T>(self) ->
Copied
<Self>
ⓘ
where
    T: 'a +
Copy
,
    Self:
Sized
+
Iterator
<Item =
&'a T
>,
Creates an iterator which copies all of its elements.
Read more
1.0.0
·
Source
§
fn
cloned
<'a, T>(self) ->
Cloned
<Self>
ⓘ
where
    T: 'a +
Clone
,
    Self:
Sized
+
Iterator
<Item =
&'a T
>,
Creates an iterator which
clone
s all of its elements.
Read more
1.0.0
·
Source
§
fn
cycle
(self) ->
Cycle
<Self>
ⓘ
where
    Self:
Sized
+
Clone
,
Repeats an iterator endlessly.
Read more
Source
§
fn
array_chunks
<const N:
usize
>(self) ->
ArrayChunks
<Self, N>
ⓘ
where
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
iter_array_chunks
#100450
)
Returns an iterator over
N
elements of the iterator at a time.
Read more
1.11.0
·
Source
§
fn
sum
<S>(self) -> S
where
    Self:
Sized
,
    S:
Sum
<Self::
Item
>,
Sums the elements of an iterator.
Read more
1.11.0
·
Source
§
fn
product
<P>(self) -> P
where
    Self:
Sized
,
    P:
Product
<Self::
Item
>,
Iterates over the entire iterator, multiplying all the elements
Read more
1.5.0
·
Source
§
fn
cmp
<I>(self, other: I) ->
Ordering
where
    I:
IntoIterator
<Item = Self::
Item
>,
    Self::
Item
:
Ord
,
    Self:
Sized
,
Lexicographically
compares the elements of this
Iterator
with those
of another.
Read more
Source
§
fn
cmp_by
<I, F>(self, other: I, cmp: F) ->
Ordering
where
    Self:
Sized
,
    I:
IntoIterator
,
    F:
FnMut
(Self::
Item
, <I as
IntoIterator
>::
Item
) ->
Ordering
,
🔬
This is a nightly-only experimental API. (
iter_order_by
#64295
)
Lexicographically
compares the elements of this
Iterator
with those
of another with respect to the specified comparison function.
Read more
1.5.0
·
Source
§
fn
partial_cmp
<I>(self, other: I) ->
Option
<
Ordering
>
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Lexicographically
compares the
PartialOrd
elements of
this
Iterator
with those of another. The comparison works like short-circuit
evaluation, returning a result without comparing the remaining elements.
As soon as an order can be determined, the evaluation stops and a result is returned.
Read more
Source
§
fn
partial_cmp_by
<I, F>(self, other: I, partial_cmp: F) ->
Option
<
Ordering
>
where
    Self:
Sized
,
    I:
IntoIterator
,
    F:
FnMut
(Self::
Item
, <I as
IntoIterator
>::
Item
) ->
Option
<
Ordering
>,
🔬
This is a nightly-only experimental API. (
iter_order_by
#64295
)
Lexicographically
compares the elements of this
Iterator
with those
of another with respect to the specified comparison function.
Read more
1.5.0
·
Source
§
fn
eq
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialEq
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are equal to those of
another.
Read more
Source
§
fn
eq_by
<I, F>(self, other: I, eq: F) ->
bool
where
    Self:
Sized
,
    I:
IntoIterator
,
    F:
FnMut
(Self::
Item
, <I as
IntoIterator
>::
Item
) ->
bool
,
🔬
This is a nightly-only experimental API. (
iter_order_by
#64295
)
Determines if the elements of this
Iterator
are equal to those of
another with respect to the specified equality function.
Read more
1.5.0
·
Source
§
fn
ne
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialEq
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are not equal to those of
another.
Read more
1.5.0
·
Source
§
fn
lt
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are
lexicographically
less than those of another.
Read more
1.5.0
·
Source
§
fn
le
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are
lexicographically
less or equal to those of another.
Read more
1.5.0
·
Source
§
fn
gt
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are
lexicographically
greater than those of another.
Read more
1.5.0
·
Source
§
fn
ge
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are
lexicographically
greater than or equal to those of another.
Read more
1.82.0
·
Source
§
fn
is_sorted_by
<F>(self, compare: F) ->
bool
where
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
, &Self::
Item
) ->
bool
,
Checks if the elements of this iterator are sorted using the given comparator function.
Read more
1.82.0
·
Source
§
fn
is_sorted_by_key
<F, K>(self, f: F) ->
bool
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) -> K,
    K:
PartialOrd
,
Checks if the elements of this iterator are sorted using the given key extraction
function.
Read more
1.0.0
·
Source
§
impl<Idx>
PartialEq
for
Range
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
Range
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
Range
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
Range
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
Range
<
usize
>
The methods
index
and
index_mut
panic if:
the start of the range is greater than the end of the range or
the end of the range is out of bounds.
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
Range
<
usize
>
Implements substring slicing with syntax
&self[begin .. end]
or
&mut self[begin .. end]
.
Returns a slice of the given string from the byte range
[
begin
,
end
).
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
begin
or
end
does not point to the starting byte offset of
a character (as defined by
is_char_boundary
), if
begin > end
, or if
end > len
.
§
Examples
let
s =
"Löwe 老虎 Léopard"
;
assert_eq!
(
&
s[
0
..
1
],
"L"
);
assert_eq!
(
&
s[
1
..
9
],
"öwe 老"
);
// these will panic:
// byte 2 lies within `ö`:
// &s[2 ..3];

// byte 8 lies within `老`
// &s[1 .. 8];

// byte 100 is outside the string
// &s[3 .. 100];
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
Range
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
Range
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
Range
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
Range
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
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
ⓘ
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
Range
<
usize
> as
SliceIndex
<
str
>>::
Output
ⓘ
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
Eq
for
Range
<Idx>
where
    Idx:
Eq
,
1.26.0
·
Source
§
impl<A>
FusedIterator
for
Range
<A>
where
    A:
Step
,
1.0.0
·
Source
§
impl<Idx>
StructuralPartialEq
for
Range
<Idx>
Source
§
impl<A>
TrustedLen
for
Range
<A>
where
    A:
TrustedStep
,
Auto Trait Implementations
§
§
impl<Idx>
Freeze
for
Range
<Idx>
where
    Idx:
Freeze
,
§
impl<Idx>
RefUnwindSafe
for
Range
<Idx>
where
    Idx:
RefUnwindSafe
,
§
impl<Idx>
Send
for
Range
<Idx>
where
    Idx:
Send
,
§
impl<Idx>
Sync
for
Range
<Idx>
where
    Idx:
Sync
,
§
impl<Idx>
Unpin
for
Range
<Idx>
where
    Idx:
Unpin
,
§
impl<Idx>
UnwindSafe
for
Range
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
impl<I>
IntoIterator
for I
where
    I:
Iterator
,
Source
§
type
Item
= <I as
Iterator
>::
Item
The type of the elements being iterated over.
Source
§
type
IntoIter
= I
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) -> I
Creates an iterator from a value.
Read more
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