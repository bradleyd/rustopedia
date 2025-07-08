RangeBounds in std::range - Rust
std
::
range
Trait
RangeBounds
Copy item path
Source
pub trait RangeBounds<T>
where
    T: ?
Sized
,
{
    // Required methods
    fn
start_bound
(&self) ->
Bound
<
&T
>;
fn
end_bound
(&self) ->
Bound
<
&T
>;

    // Provided methods
    fn
contains
<U>(&self, item:
&U
) ->
bool
where T:
PartialOrd
<U>,
             U:
PartialOrd
<T> + ?
Sized
{ ... }
fn
is_empty
(&self) ->
bool
where T:
PartialOrd
{ ... }
}
🔬
This is a nightly-only experimental API. (
new_range_api
#125687
)
Expand description
RangeBounds
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
1.28.0
·
Source
fn
start_bound
(&self) ->
Bound
<
&T
>
Start index bound.
Returns the start value as a
Bound
.
§
Examples
use
std::ops::Bound::
*
;
use
std::ops::RangeBounds;
assert_eq!
((..
10
).start_bound(), Unbounded);
assert_eq!
((
3
..
10
).start_bound(), Included(
&
3
));
1.28.0
·
Source
fn
end_bound
(&self) ->
Bound
<
&T
>
End index bound.
Returns the end value as a
Bound
.
§
Examples
use
std::ops::Bound::
*
;
use
std::ops::RangeBounds;
assert_eq!
((
3
..).end_bound(), Unbounded);
assert_eq!
((
3
..
10
).end_bound(), Excluded(
&
10
));
Provided Methods
§
1.35.0
·
Source
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
§
Examples
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
2
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
Source
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
§
Examples
#![feature(range_bounds_is_empty)]
use
std::ops::RangeBounds;
assert!
(!(
3
..).is_empty());
assert!
(!(..
2
).is_empty());
assert!
(!RangeBounds::is_empty(
&
(
3
..
5
)));
assert!
( RangeBounds::is_empty(
&
(
3
..
3
)));
assert!
( RangeBounds::is_empty(
&
(
3
..
2
)));
The range is empty if either side is incomparable:
#![feature(range_bounds_is_empty)]
use
std::ops::RangeBounds;
assert!
(!RangeBounds::is_empty(
&
(
3.0
..
5.0
)));
assert!
( RangeBounds::is_empty(
&
(
3.0
..f32::NAN)));
assert!
( RangeBounds::is_empty(
&
(f32::NAN..
5.0
)));
But never empty is either side is unbounded:
#![feature(range_bounds_is_empty)]
use
std::ops::RangeBounds;
assert!
(!(..
0
).is_empty());
assert!
(!(i32::MAX..).is_empty());
assert!
(!RangeBounds::<u8>::is_empty(
&
(..)));
(Excluded(a), Excluded(b))
is only empty if
a >= b
:
#![feature(range_bounds_is_empty)]
use
std::ops::Bound::
*
;
use
std::ops::RangeBounds;
assert!
(!(Excluded(
1
), Excluded(
3
)).is_empty());
assert!
(!(Excluded(
1
), Excluded(
2
)).is_empty());
assert!
( (Excluded(
1
), Excluded(
1
)).is_empty());
assert!
( (Excluded(
2
), Excluded(
1
)).is_empty());
assert!
( (Excluded(
3
), Excluded(
1
)).is_empty());
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.28.0
·
Source
§
impl<'a, T>
RangeBounds
<T> for (
Bound
<
&'a T
>,
Bound
<
&'a T
>)
where
    T: 'a + ?
Sized
,
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for (
Bound
<T>,
Bound
<T>)
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for std::ops::
Range
<
&T
>
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for std::ops::
Range
<T>
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for std::ops::
RangeFrom
<
&T
>
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for std::ops::
RangeFrom
<T>
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
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for std::ops::
RangeInclusive
<
&T
>
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for std::ops::
RangeInclusive
<T>
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
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for
RangeTo
<T>
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for
RangeToInclusive
<
&T
>
1.28.0
·
Source
§
impl<T>
RangeBounds
<T> for
RangeToInclusive
<T>
Source
§
impl<T>
RangeBounds
<T> for std::range::
Range
<
&T
>
Source
§
impl<T>
RangeBounds
<T> for std::range::
Range
<T>
Source
§
impl<T>
RangeBounds
<T> for std::range::
RangeFrom
<
&T
>
Source
§
impl<T>
RangeBounds
<T> for std::range::
RangeFrom
<T>
Source
§
impl<T>
RangeBounds
<T> for std::range::
RangeInclusive
<
&T
>
Source
§
impl<T>
RangeBounds
<T> for std::range::
RangeInclusive
<T>