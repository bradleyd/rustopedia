min_by in std::cmp - Rust
std
::
cmp
Function
min_by
Copy item path
1.53.0
·
Source
pub fn min_by<T, F>(v1: T, v2: T, compare: F) -> T
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
Expand description
Returns the minimum of two values with respect to the specified comparison function.
Returns the first argument if the comparison determines them to be equal.
§
Examples
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
let
result = cmp::min_by(
2
, -
1
, abs_cmp);
assert_eq!
(result, -
1
);
let
result = cmp::min_by(
2
, -
3
, abs_cmp);
assert_eq!
(result,
2
);
let
result = cmp::min_by(
1
, -
1
, abs_cmp);
assert_eq!
(result,
1
);