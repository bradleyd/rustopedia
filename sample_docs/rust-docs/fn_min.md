min in std::cmp - Rust
std
::
cmp
Function
min
Copy item path
1.0.0
·
Source
pub fn min<T>(v1: T, v2: T) -> T
where
    T:
Ord
,
Expand description
Compares and returns the minimum of two values.
Returns the first argument if the comparison determines them to be equal.
Internally uses an alias to
Ord::min
.
§
Examples
use
std::cmp;
assert_eq!
(cmp::min(
1
,
2
),
1
);
assert_eq!
(cmp::min(
2
,
2
),
2
);
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
(cmp::min(Equal(
"v1"
), Equal(
"v2"
)).
0
,
"v1"
);