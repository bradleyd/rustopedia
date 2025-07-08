minmax in std::cmp - Rust
std
::
cmp
Function
minmax
Copy item path
Source
pub fn minmax<T>(v1: T, v2: T) ->
[T; 2]
where
    T:
Ord
,
🔬
This is a nightly-only experimental API. (
cmp_minmax
#115939
)
Expand description
Compares and sorts two values, returning minimum and maximum.
Returns
[v1, v2]
if the comparison determines them to be equal.
§
Examples
#![feature(cmp_minmax)]
use
std::cmp;
assert_eq!
(cmp::minmax(
1
,
2
), [
1
,
2
]);
assert_eq!
(cmp::minmax(
2
,
1
), [
1
,
2
]);
// You can destructure the result using array patterns
let
[min, max] = cmp::minmax(
42
,
17
);
assert_eq!
(min,
17
);
assert_eq!
(max,
42
);
#![feature(cmp_minmax)]
use
std::cmp::{
self
, Ordering};
#[derive(Eq)]
struct
Equal(
&
'static
str);
impl
PartialEq
for
Equal {
fn
eq(
&
self
, other:
&
Self
) -> bool {
true
}
}
impl
PartialOrd
for
Equal {
fn
partial_cmp(
&
self
, other:
&
Self
) ->
Option
<Ordering> {
Some
(Ordering::Equal) }
}
impl
Ord
for
Equal {
fn
cmp(
&
self
, other:
&
Self
) -> Ordering { Ordering::Equal }
}
assert_eq!
(cmp::minmax(Equal(
"v1"
), Equal(
"v2"
)).map(|v| v.
0
), [
"v1"
,
"v2"
]);