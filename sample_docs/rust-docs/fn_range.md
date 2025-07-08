range in std::slice - Rust
std
::
slice
Function
range
Copy item path
Source
pub fn range<R>(range: R, bounds:
RangeTo
<
usize
>) ->
Range
<
usize
>
ⓘ
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
Performs bounds checking of a range.
This method is similar to
Index::index
for slices, but it returns a
Range
equivalent to
range
. You can use this method to turn any range
into
start
and
end
values.
bounds
is the range of the slice to use for bounds checking. It should
be a
RangeTo
range that ends at the length of the slice.
The returned
Range
is safe to pass to
slice::get_unchecked
and
slice::get_unchecked_mut
for slices with the given range.
§
Panics
Panics if
range
would be out of bounds.
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
1
..
2
, slice::range(
1
..
2
, ..v.len()));
assert_eq!
(
0
..
2
, slice::range(..
2
, ..v.len()));
assert_eq!
(
1
..
3
, slice::range(
1
.., ..v.len()));
Panics when
Index::index
would panic:
ⓘ
#![feature(slice_range)]
use
std::slice;
let _
= slice::range(
2
..
1
, ..
3
);
ⓘ
#![feature(slice_range)]
use
std::slice;
let _
= slice::range(
1
..
4
, ..
3
);
ⓘ
#![feature(slice_range)]
use
std::slice;
let _
= slice::range(
1
..=usize::MAX, ..
3
);