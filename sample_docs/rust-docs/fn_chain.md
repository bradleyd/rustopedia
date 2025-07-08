chain in std::iter - Rust
std
::
iter
Function
chain
Copy item path
Source
pub fn chain<A, B>(
    a: A,
    b: B,
) ->
Chain
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
<Item = <A as
IntoIterator
>::
Item
>,
🔬
This is a nightly-only experimental API. (
iter_chain
#125964
)
Expand description
Converts the arguments to iterators and links them together, in a chain.
See the documentation of
Iterator::chain
for more.
§
Examples
#![feature(iter_chain)]
use
std::iter::chain;
let
a = [
1
,
2
,
3
];
let
b = [
4
,
5
,
6
];
let
mut
iter = chain(a, b);
assert_eq!
(iter.next(),
Some
(
1
));
assert_eq!
(iter.next(),
Some
(
2
));
assert_eq!
(iter.next(),
Some
(
3
));
assert_eq!
(iter.next(),
Some
(
4
));
assert_eq!
(iter.next(),
Some
(
5
));
assert_eq!
(iter.next(),
Some
(
6
));
assert_eq!
(iter.next(),
None
);