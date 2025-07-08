max in std::cmp - Rust
std
::
cmp
Function
max
Copy item path
1.0.0
·
Source
pub fn max<T>(v1: T, v2: T) -> T
where
    T:
Ord
,
Expand description
Compares and returns the maximum of two values.
Returns the second argument if the comparison determines them to be equal.
Internally uses an alias to
Ord::max
.
§
Examples
use
std::cmp;
assert_eq!
(cmp::max(
1
,
2
),
2
);
assert_eq!
(cmp::max(
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
(cmp::max(Equal(
"v1"
), Equal(
"v2"
)).
0
,
"v2"
);