repeat_with in std::iter - Rust
std
::
iter
Function
repeat_with
Copy item path
1.28.0
·
Source
pub fn repeat_with<A, F>(repeater: F) ->
RepeatWith
<F>
ⓘ
where
    F:
FnMut
() -> A,
Expand description
Creates a new iterator that repeats elements of type
A
endlessly by
applying the provided closure, the repeater,
F: FnMut() -> A
.
The
repeat_with()
function calls the repeater over and over again.
Infinite iterators like
repeat_with()
are often used with adapters like
Iterator::take()
, in order to make them finite.
If the element type of the iterator you need implements
Clone
, and
it is OK to keep the source element in memory, you should instead use
the
repeat()
function.
An iterator produced by
repeat_with()
is not a
DoubleEndedIterator
.
If you need
repeat_with()
to return a
DoubleEndedIterator
,
please open a GitHub issue explaining your use case.
§
Examples
Basic usage:
use
std::iter;
// let's assume we have some value of a type that is not `Clone`
// or which we don't want to have in memory just yet because it is expensive:
#[derive(PartialEq, Debug)]
struct
Expensive;
// a particular value forever:
let
mut
things = iter::repeat_with(|| Expensive);
assert_eq!
(
Some
(Expensive), things.next());
assert_eq!
(
Some
(Expensive), things.next());
assert_eq!
(
Some
(Expensive), things.next());
assert_eq!
(
Some
(Expensive), things.next());
assert_eq!
(
Some
(Expensive), things.next());
Using mutation and going finite:
use
std::iter;
// From the zeroth to the third power of two:
let
mut
curr =
1
;
let
mut
pow2 = iter::repeat_with(|| {
let
tmp = curr; curr
*
=
2
; tmp })
                    .take(
4
);
assert_eq!
(
Some
(
1
), pow2.next());
assert_eq!
(
Some
(
2
), pow2.next());
assert_eq!
(
Some
(
4
), pow2.next());
assert_eq!
(
Some
(
8
), pow2.next());
// ... and now we're done
assert_eq!
(
None
, pow2.next());