successors in std::iter - Rust
std
::
iter
Function
successors
Copy item path
1.34.0
·
Source
pub fn successors<T, F>(first:
Option
<T>, succ: F) ->
Successors
<T, F>
ⓘ
where
    F:
FnMut
(
&T
) ->
Option
<T>,
Expand description
Creates an iterator which, starting from an initial item,
computes each successive item from the preceding one.
This iterator stores an optional item (
Option<T>
) and a successor closure (
impl FnMut(&T) -> Option<T>
).
Its
next
method returns the stored optional item and
if it is
Some(val)
calls the stored closure on
&val
to compute and store its successor.
The iterator will apply the closure successively to the stored option’s value until the option is
None
.
This also means that once the stored option is
None
it will remain
None
,
as the closure will not be called again, so the created iterator is a
FusedIterator
.
The iterator’s items will be the initial item and all of its successors as calculated by the successor closure.
use
std::iter::successors;
let
powers_of_10 = successors(
Some
(
1_u16
), |n| n.checked_mul(
10
));
assert_eq!
(powers_of_10.collect::<Vec<
_
>>(),
&
[
1
,
10
,
100
,
1_000
,
10_000
]);