minmax_by in std::cmp - Rust
std
::
cmp
Function
minmax_by
Copy item path
Source
pub fn minmax_by<T, F>(v1: T, v2: T, compare: F) ->
[T; 2]
where
    F:
FnOnce
(
&T
,
&T
) ->
Ordering
,
🔬
This is a nightly-only experimental API. (
cmp_minmax
#115939
)
Expand description
Returns minimum and maximum values with respect to the specified comparison function.
Returns
[v1, v2]
if the comparison determines them to be equal.
§
Examples
#![feature(cmp_minmax)]
use
std::cmp;
let
abs_cmp = |x:
&
i32, y:
&
i32| x.abs().cmp(
&
y.abs());
assert_eq!
(cmp::minmax_by(-
2
,
1
, abs_cmp), [
1
, -
2
]);
assert_eq!
(cmp::minmax_by(-
1
,
2
, abs_cmp), [-
1
,
2
]);
assert_eq!
(cmp::minmax_by(-
2
,
2
, abs_cmp), [-
2
,
2
]);
// You can destructure the result using array patterns
let
[min, max] = cmp::minmax_by(-
42
,
17
, abs_cmp);
assert_eq!
(min,
17
);
assert_eq!
(max, -
42
);