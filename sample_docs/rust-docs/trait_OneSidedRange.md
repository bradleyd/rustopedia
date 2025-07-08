OneSidedRange in std::range - Rust
std
::
range
Trait
OneSidedRange
Copy item path
Source
pub trait OneSidedRange<T>:
RangeBounds
<T>
where
    T: ?
Sized
,
{
    // Required method
    fn
bound
(self) -> (
OneSidedRangeBound
, T);
}
🔬
This is a nightly-only experimental API. (
one_sided_range
#69780
)
Expand description
OneSidedRange
is implemented for built-in range types that are unbounded
on one side. For example,
a..
,
..b
and
..=c
implement
OneSidedRange
,
but
..
,
d..e
, and
f..=g
do not.
Types that implement
OneSidedRange<T>
must return
Bound::Unbounded
from one of
RangeBounds::start_bound
or
RangeBounds::end_bound
.
Required Methods
§
Source
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
OneSidedRange
<T> for
RangeFrom
<T>
where
RangeFrom
<T>:
RangeBounds
<T>,
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
impl<T>
OneSidedRange
<T> for
RangeToInclusive
<T>
where
RangeToInclusive
<T>:
RangeBounds
<T>,