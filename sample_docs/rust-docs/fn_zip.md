zip in std::iter - Rust
std
::
iter
Function
zip
Copy item path
1.59.0
·
Source
pub fn zip<A, B>(
    a: A,
    b: B,
) ->
Zip
<<A as
IntoIterator
>::
IntoIter
, <B as
IntoIterator
>::
IntoIter
>
ⓘ
where
    A:
IntoIterator
,
    B:
IntoIterator
,
Expand description
Converts the arguments to iterators and zips them.
See the documentation of
Iterator::zip
for more.
§
Examples
use
std::iter::zip;
let
xs = [
1
,
2
,
3
];
let
ys = [
4
,
5
,
6
];
let
mut
iter = zip(xs, ys);
assert_eq!
(iter.next().unwrap(), (
1
,
4
));
assert_eq!
(iter.next().unwrap(), (
2
,
5
));
assert_eq!
(iter.next().unwrap(), (
3
,
6
));
assert!
(iter.next().is_none());
// Nested zips are also possible:
let
zs = [
7
,
8
,
9
];
let
mut
iter = zip(zip(xs, ys), zs);
assert_eq!
(iter.next().unwrap(), ((
1
,
4
),
7
));
assert_eq!
(iter.next().unwrap(), ((
2
,
5
),
8
));
assert_eq!
(iter.next().unwrap(), ((
3
,
6
),
9
));
assert!
(iter.next().is_none());