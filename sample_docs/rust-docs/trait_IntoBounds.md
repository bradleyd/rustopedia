IntoBounds in std::range - Rust
std
::
range
Trait
IntoBounds
Copy item path
Source
pub trait IntoBounds<T>:
RangeBounds
<T> {
    // Required method
    fn
into_bounds
(self) -> (
Bound
<T>,
Bound
<T>);

    // Provided method
    fn
intersect
<R>(self, other: R) -> (
Bound
<T>,
Bound
<T>)
where Self:
Sized
,
             T:
Ord
,
             R:
IntoBounds
<T>
{ ... }
}
🔬
This is a nightly-only experimental API. (
range_into_bounds
#136903
)
Expand description
Used to convert a range into start and end bounds, consuming the
range by value.
IntoBounds
is implemented by Rust’s built-in range types, produced
by range syntax like
..
,
a..
,
..b
,
..=c
,
d..e
, or
f..=g
.
Required Methods
§
Source
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
§
Examples
#![feature(range_into_bounds)]
use
std::ops::Bound::
*
;
use
std::ops::IntoBounds;
assert_eq!
((
0
..
5
).into_bounds(), (Included(
0
), Excluded(
5
)));
assert_eq!
((..=
7
).into_bounds(), (Unbounded, Included(
7
)));
Provided Methods
§
Source
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
§
Examples
#![feature(range_into_bounds)]
use
std::ops::Bound::
*
;
use
std::ops::IntoBounds;
assert_eq!
((
3
..).intersect(..
5
), (Included(
3
), Excluded(
5
)));
assert_eq!
((-
12
..
387
).intersect(
0
..
256
), (Included(
0
), Excluded(
256
)));
assert_eq!
((
1
..
5
).intersect(..), (Included(
1
), Excluded(
5
)));
assert_eq!
((
1
..=
9
).intersect(
0
..
10
), (Included(
1
), Included(
9
)));
assert_eq!
((
7
..=
13
).intersect(
8
..
13
), (Included(
8
), Excluded(
13
)));
Combine with
is_empty
to determine if two ranges overlap.
#![feature(range_into_bounds)]
#![feature(range_bounds_is_empty)]
use
std::ops::{RangeBounds, IntoBounds};
assert!
(!(
3
..).intersect(..
5
).is_empty());
assert!
(!(-
12
..
387
).intersect(
0
..
256
).is_empty());
assert!
((
1
..
5
).intersect(
6
..).is_empty());
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
Source
§
impl<T>
IntoBounds
<T> for (
Bound
<T>,
Bound
<T>)
Source
§
impl<T>
IntoBounds
<T> for std::ops::
Range
<T>
Source
§
impl<T>
IntoBounds
<T> for std::ops::
RangeFrom
<T>
Source
§
impl<T>
IntoBounds
<T> for
RangeFull
Source
§
impl<T>
IntoBounds
<T> for std::ops::
RangeInclusive
<T>
Source
§
impl<T>
IntoBounds
<T> for
RangeTo
<T>
Source
§
impl<T>
IntoBounds
<T> for
RangeToInclusive
<T>
Source
§
impl<T>
IntoBounds
<T> for std::range::
Range
<T>
Source
§
impl<T>
IntoBounds
<T> for std::range::
RangeFrom
<T>
Source
§
impl<T>
IntoBounds
<T> for std::range::
RangeInclusive
<T>