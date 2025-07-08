min_by_key in std::cmp - Rust
std
::
cmp
Function
min_by_key
Copy item path
1.53.0
·
Source
pub fn min_by_key<T, F, K>(v1: T, v2: T, f: F) -> T
where
    F:
FnMut
(
&T
) -> K,
    K:
Ord
,
Expand description
Returns the element that gives the minimum value from the specified function.
Returns the first argument if the comparison determines them to be equal.
§
Examples
use
std::cmp;
let
result = cmp::min_by_key(
2
, -
1
, |x:
&
i32| x.abs());
assert_eq!
(result, -
1
);
let
result = cmp::min_by_key(
2
, -
3
, |x:
&
i32| x.abs());
assert_eq!
(result,
2
);
let
result = cmp::min_by_key(
1
, -
1
, |x:
&
i32| x.abs());
assert_eq!
(result,
1
);