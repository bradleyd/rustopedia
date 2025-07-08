minmax_by_key in std::cmp - Rust
std
::
cmp
Function
minmax_by_key
Copy item path
Source
pub fn minmax_by_key<T, F, K>(v1: T, v2: T, f: F) ->
[T; 2]
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
🔬
This is a nightly-only experimental API. (
cmp_minmax
#115939
)
Expand description
Returns minimum and maximum values with respect to the specified key function.
Returns
[v1, v2]
if the comparison determines them to be equal.
§
Examples
#![feature(cmp_minmax)]
use
std::cmp;
assert_eq!
(cmp::minmax_by_key(-
2
,
1
, |x:
&
i32| x.abs()), [
1
, -
2
]);
assert_eq!
(cmp::minmax_by_key(-
2
,
2
, |x:
&
i32| x.abs()), [-
2
,
2
]);
// You can destructure the result using array patterns
let
[min, max] = cmp::minmax_by_key(-
42
,
17
, |x:
&
i32| x.abs());
assert_eq!
(min,
17
);
assert_eq!
(max, -
42
);