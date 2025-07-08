repeat_n in std::iter - Rust
std
::
iter
Function
repeat_n
Copy item path
1.82.0
·
Source
pub fn repeat_n<T>(element: T, count:
usize
) ->
RepeatN
<T>
ⓘ
where
    T:
Clone
,
Expand description
Creates a new iterator that repeats a single element a given number of times.
The
repeat_n()
function repeats a single value exactly
n
times.
This is very similar to using
repeat()
with
Iterator::take()
,
but
repeat_n()
can return the original value, rather than always cloning.
§
Examples
Basic usage:
use
std::iter;
// four of the number four:
let
mut
four_fours = iter::repeat_n(
4
,
4
);
assert_eq!
(
Some
(
4
), four_fours.next());
assert_eq!
(
Some
(
4
), four_fours.next());
assert_eq!
(
Some
(
4
), four_fours.next());
assert_eq!
(
Some
(
4
), four_fours.next());
// no more fours
assert_eq!
(
None
, four_fours.next());
For non-
Copy
types,
use
std::iter;
let
v: Vec<i32> = Vec::with_capacity(
123
);
let
mut
it = iter::repeat_n(v,
5
);
for
i
in
0
..
4
{
// It starts by cloning things
let
cloned = it.next().unwrap();
assert_eq!
(cloned.len(),
0
);
assert_eq!
(cloned.capacity(),
0
);
}
// ... but the last item is the original one
let
last = it.next().unwrap();
assert_eq!
(last.len(),
0
);
assert_eq!
(last.capacity(),
123
);
// ... and now we're done
assert_eq!
(
None
, it.next());