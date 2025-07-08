try_range in std::slice - Rust
std
::
slice
Function
try_range
Copy item path
Source
pub fn try_range<R>(range: R, bounds:
RangeTo
<
usize
>) ->
Option
<
Range
<
usize
>>
where
    R:
RangeBounds
<
usize
>,
🔬
This is a nightly-only experimental API. (
slice_range
#76393
)
Expand description
Performs bounds checking of a range without panicking.
This is a version of
range()
that returns
None
instead of panicking.
§
Examples
#![feature(slice_range)]
use
std::slice;
let
v = [
10
,
40
,
30
];
assert_eq!
(
Some
(
1
..
2
), slice::try_range(
1
..
2
, ..v.len()));
assert_eq!
(
Some
(
0
..
2
), slice::try_range(..
2
, ..v.len()));
assert_eq!
(
Some
(
1
..
3
), slice::try_range(
1
.., ..v.len()));
Returns
None
when
Index::index
would panic:
#![feature(slice_range)]
use
std::slice;
assert_eq!
(
None
, slice::try_range(
2
..
1
, ..
3
));
assert_eq!
(
None
, slice::try_range(
1
..
4
, ..
3
));
assert_eq!
(
None
, slice::try_range(
1
..=usize::MAX, ..
3
));