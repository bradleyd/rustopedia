DoubleEndedIterator in std::iter - Rust
std
::
iter
Trait
DoubleEndedIterator
Copy item path
1.0.0
·
Source
pub trait DoubleEndedIterator:
Iterator
{
    // Required method
    fn
next_back
(&mut self) ->
Option
<Self::
Item
>;

    // Provided methods
    fn
advance_back_by
(&mut self, n:
usize
) ->
Result
<
()
,
NonZero
<
usize
>> { ... }
fn
nth_back
(&mut self, n:
usize
) ->
Option
<Self::
Item
> { ... }
fn
try_rfold
<B, F, R>(&mut self, init: B, f: F) -> R
where Self:
Sized
,
             F:
FnMut
(B, Self::
Item
) -> R,
             R:
Try
<Output = B>
{ ... }
fn
rfold
<B, F>(self, init: B, f: F) -> B
where Self:
Sized
,
             F:
FnMut
(B, Self::
Item
) -> B
{ ... }
fn
rfind
<P>(&mut self, predicate: P) ->
Option
<Self::
Item
>
where Self:
Sized
,
             P:
FnMut
(&Self::
Item
) ->
bool
{ ... }
}
Expand description
An iterator able to yield elements from both ends.
Something that implements
DoubleEndedIterator
has one extra capability
over something that implements
Iterator
: the ability to also take
Item
s from the back, as well as the front.
It is important to note that both back and forth work on the same range,
and do not cross: iteration is over when they meet in the middle.
In a similar fashion to the
Iterator
protocol, once a
DoubleEndedIterator
returns
None
from a
next_back()
, calling it
again may or may not ever return
Some
again.
next()
and
next_back()
are interchangeable for this purpose.
§
Examples
Basic usage:
let
numbers =
vec!
[
1
,
2
,
3
,
4
,
5
,
6
];
let
mut
iter = numbers.iter();
assert_eq!
(
Some
(
&
1
), iter.next());
assert_eq!
(
Some
(
&
6
), iter.next_back());
assert_eq!
(
Some
(
&
5
), iter.next_back());
assert_eq!
(
Some
(
&
2
), iter.next());
assert_eq!
(
Some
(
&
3
), iter.next());
assert_eq!
(
Some
(
&
4
), iter.next());
assert_eq!
(
None
, iter.next());
assert_eq!
(
None
, iter.next_back());
Required Methods
§
1.0.0
·
Source
fn
next_back
(&mut self) ->
Option
<Self::
Item
>
Removes and returns an element from the end of the iterator.
Returns
None
when there are no more elements.
The
trait-level
docs contain more details.
§
Examples
Basic usage:
let
numbers =
vec!
[
1
,
2
,
3
,
4
,
5
,
6
];
let
mut
iter = numbers.iter();
assert_eq!
(
Some
(
&
1
), iter.next());
assert_eq!
(
Some
(
&
6
), iter.next_back());
assert_eq!
(
Some
(
&
5
), iter.next_back());
assert_eq!
(
Some
(
&
2
), iter.next());
assert_eq!
(
Some
(
&
3
), iter.next());
assert_eq!
(
Some
(
&
4
), iter.next());
assert_eq!
(
None
, iter.next());
assert_eq!
(
None
, iter.next_back());
§
Remarks
The elements yielded by
DoubleEndedIterator
’s methods may differ from
the ones yielded by
Iterator
’s methods:
let
vec =
vec!
[(
1
,
'a'
), (
1
,
'b'
), (
1
,
'c'
), (
2
,
'a'
), (
2
,
'b'
)];
let
uniq_by_fst_comp = || {
let
mut
seen = std::collections::HashSet::new();
    vec.iter().copied().filter(
move
|x| seen.insert(x.
0
))
};
assert_eq!
(uniq_by_fst_comp().last(),
Some
((
2
,
'a'
)));
assert_eq!
(uniq_by_fst_comp().next_back(),
Some
((
2
,
'b'
)));
assert_eq!
(
    uniq_by_fst_comp().fold(
vec!
[], |
mut
v, x| {v.push(x); v}),
vec!
[(
1
,
'a'
), (
2
,
'a'
)]
);
assert_eq!
(
    uniq_by_fst_comp().rfold(
vec!
[], |
mut
v, x| {v.push(x); v}),
vec!
[(
2
,
'b'
), (
1
,
'c'
)]
);
Provided Methods
§
Source
fn
advance_back_by
(&mut self, n:
usize
) ->
Result
<
()
,
NonZero
<
usize
>>
🔬
This is a nightly-only experimental API. (
iter_advance_by
#77404
)
Advances the iterator from the back by
n
elements.
advance_back_by
is the reverse version of
advance_by
. This method will
eagerly skip
n
elements starting from the back by calling
next_back
up
to
n
times until
None
is encountered.
advance_back_by(n)
will return
Ok(())
if the iterator successfully advances by
n
elements, or a
Err(NonZero<usize>)
with value
k
if
None
is encountered, where
k
is remaining number of steps that could not be advanced because the iterator ran out.
If
self
is empty and
n
is non-zero, then this returns
Err(n)
.
Otherwise,
k
is always less than
n
.
Calling
advance_back_by(0)
can do meaningful work, for example
Flatten
can advance its
outer iterator until it finds an inner iterator that is not empty, which then often
allows it to return a more accurate
size_hint()
than in its initial state.
§
Examples
Basic usage:
#![feature(iter_advance_by)]
use
std::num::NonZero;
let
a = [
3
,
4
,
5
,
6
];
let
mut
iter = a.iter();
assert_eq!
(iter.advance_back_by(
2
),
Ok
(()));
assert_eq!
(iter.next_back(),
Some
(
&
4
));
assert_eq!
(iter.advance_back_by(
0
),
Ok
(()));
assert_eq!
(iter.advance_back_by(
100
),
Err
(NonZero::new(
99
).unwrap()));
// only `&3` was skipped
1.37.0
·
Source
fn
nth_back
(&mut self, n:
usize
) ->
Option
<Self::
Item
>
Returns the
n
th element from the end of the iterator.
This is essentially the reversed version of
Iterator::nth()
.
Although like most indexing operations, the count starts from zero, so
nth_back(0)
returns the first value from the end,
nth_back(1)
the
second, and so on.
Note that all elements between the end and the returned element will be
consumed, including the returned element. This also means that calling
nth_back(0)
multiple times on the same iterator will return different
elements.
nth_back()
will return
None
if
n
is greater than or equal to the
length of the iterator.
§
Examples
Basic usage:
let
a = [
1
,
2
,
3
];
assert_eq!
(a.iter().nth_back(
2
),
Some
(
&
1
));
Calling
nth_back()
multiple times doesn’t rewind the iterator:
let
a = [
1
,
2
,
3
];
let
mut
iter = a.iter();
assert_eq!
(iter.nth_back(
1
),
Some
(
&
2
));
assert_eq!
(iter.nth_back(
1
),
None
);
Returning
None
if there are less than
n + 1
elements:
let
a = [
1
,
2
,
3
];
assert_eq!
(a.iter().nth_back(
10
),
None
);
1.27.0
·
Source
fn
try_rfold
<B, F, R>(&mut self, init: B, f: F) -> R
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> R,
    R:
Try
<Output = B>,
This is the reverse version of
Iterator::try_fold()
: it takes
elements starting from the back of the iterator.
§
Examples
Basic usage:
let
a = [
"1"
,
"2"
,
"3"
];
let
sum = a.iter()
    .map(|
&
s| s.parse::<i32>())
    .try_rfold(
0
, |acc, x| x.and_then(|y|
Ok
(acc + y)));
assert_eq!
(sum,
Ok
(
6
));
Short-circuiting:
let
a = [
"1"
,
"rust"
,
"3"
];
let
mut
it = a.iter();
let
sum = it
    .by_ref()
    .map(|
&
s| s.parse::<i32>())
    .try_rfold(
0
, |acc, x| x.and_then(|y|
Ok
(acc + y)));
assert!
(sum.is_err());
// Because it short-circuited, the remaining elements are still
// available through the iterator.
assert_eq!
(it.next_back(),
Some
(
&
"1"
));
1.27.0
·
Source
fn
rfold
<B, F>(self, init: B, f: F) -> B
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> B,
An iterator method that reduces the iterator’s elements to a single,
final value, starting from the back.
This is the reverse version of
Iterator::fold()
: it takes elements
starting from the back of the iterator.
rfold()
takes two arguments: an initial value, and a closure with two
arguments: an ‘accumulator’, and an element. The closure returns the value that
the accumulator should have for the next iteration.
The initial value is the value the accumulator will have on the first
call.
After applying this closure to every element of the iterator,
rfold()
returns the accumulator.
This operation is sometimes called ‘reduce’ or ‘inject’.
Folding is useful whenever you have a collection of something, and want
to produce a single value from it.
Note:
rfold()
combines elements in a
right-associative
fashion. For associative
operators like
+
, the order the elements are combined in is not important, but for non-associative
operators like
-
the order will affect the final result.
For a
left-associative
version of
rfold()
, see
Iterator::fold()
.
§
Examples
Basic usage:
let
a = [
1
,
2
,
3
];
// the sum of all of the elements of a
let
sum = a.iter()
           .rfold(
0
, |acc,
&
x| acc + x);
assert_eq!
(sum,
6
);
This example demonstrates the right-associative nature of
rfold()
:
it builds a string, starting with an initial value
and continuing with each element from the back until the front:
let
numbers = [
1
,
2
,
3
,
4
,
5
];
let
zero =
"0"
.to_string();
let
result = numbers.iter().rfold(zero, |acc,
&
x| {
format!
(
"({x} + {acc})"
)
});
assert_eq!
(result,
"(1 + (2 + (3 + (4 + (5 + 0)))))"
);
1.27.0
·
Source
fn
rfind
<P>(&mut self, predicate: P) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Searches for an element of an iterator from the back that satisfies a predicate.
rfind()
takes a closure that returns
true
or
false
. It applies
this closure to each element of the iterator, starting at the end, and if any
of them return
true
, then
rfind()
returns
Some(element)
. If they all return
false
, it returns
None
.
rfind()
is short-circuiting; in other words, it will stop processing
as soon as the closure returns
true
.
Because
rfind()
takes a reference, and many iterators iterate over
references, this leads to a possibly confusing situation where the
argument is a double reference. You can see this effect in the
examples below, with
&&x
.
§
Examples
Basic usage:
let
a = [
1
,
2
,
3
];
assert_eq!
(a.iter().rfind(|&&x| x ==
2
),
Some
(
&
2
));
assert_eq!
(a.iter().rfind(|&&x| x ==
5
),
None
);
Stopping at the first
true
:
let
a = [
1
,
2
,
3
];
let
mut
iter = a.iter();
assert_eq!
(iter.rfind(|&&x| x ==
2
),
Some
(
&
2
));
// we can still use `iter`, as there are more elements.
assert_eq!
(iter.next_back(),
Some
(
&
1
));
Implementors
§
1.0.0
·
Source
§
impl
DoubleEndedIterator
for
EscapeDefault
1.59.0
·
Source
§
impl
DoubleEndedIterator
for
ToLowercase
1.59.0
·
Source
§
impl
DoubleEndedIterator
for
ToUppercase
1.12.0
·
Source
§
impl
DoubleEndedIterator
for
Args
1.12.0
·
Source
§
impl
DoubleEndedIterator
for
ArgsOs
1.0.0
·
Source
§
impl
DoubleEndedIterator
for
Bytes
<'_>
1.6.0
·
Source
§
impl
DoubleEndedIterator
for std::string::
Drain
<'_>
Source
§
impl
DoubleEndedIterator
for
IntoChars
1.0.0
·
Source
§
impl<'a>
DoubleEndedIterator
for
Components
<'a>
1.0.0
·
Source
§
impl<'a>
DoubleEndedIterator
for std::path::
Iter
<'a>
1.60.0
·
Source
§
impl<'a>
DoubleEndedIterator
for
EscapeAscii
<'a>
1.0.0
·
Source
§
impl<'a>
DoubleEndedIterator
for
CharIndices
<'a>
1.0.0
·
Source
§
impl<'a>
DoubleEndedIterator
for
Chars
<'a>
1.0.0
·
Source
§
impl<'a>
DoubleEndedIterator
for
Lines
<'a>
1.0.0
·
Source
§
impl<'a>
DoubleEndedIterator
for
LinesAny
<'a>
1.34.0
·
Source
§
impl<'a>
DoubleEndedIterator
for
SplitAsciiWhitespace
<'a>
1.1.0
·
Source
§
impl<'a>
DoubleEndedIterator
for
SplitWhitespace
<'a>
1.0.0
·
Source
§
impl<'a, A>
DoubleEndedIterator
for std::option::
Iter
<'a, A>
1.0.0
·
Source
§
impl<'a, A>
DoubleEndedIterator
for std::option::
IterMut
<'a, A>
1.0.0
·
Source
§
impl<'a, I>
DoubleEndedIterator
for
&'a mut I
where
    I:
DoubleEndedIterator
+ ?
Sized
,
1.1.0
·
Source
§
impl<'a, I, T>
DoubleEndedIterator
for
Cloned
<I>
where
    T: 'a +
Clone
,
    I:
DoubleEndedIterator
<Item =
&'a T
>,
1.36.0
·
Source
§
impl<'a, I, T>
DoubleEndedIterator
for
Copied
<I>
where
    T: 'a +
Copy
,
    I:
DoubleEndedIterator
<Item =
&'a T
>,
1.0.0
·
Source
§
impl<'a, K, V>
DoubleEndedIterator
for std::collections::btree_map::
Iter
<'a, K, V>
where
    K: 'a,
    V: 'a,
1.0.0
·
Source
§
impl<'a, K, V>
DoubleEndedIterator
for std::collections::btree_map::
IterMut
<'a, K, V>
1.0.0
·
Source
§
impl<'a, K, V>
DoubleEndedIterator
for
Keys
<'a, K, V>
1.17.0
·
Source
§
impl<'a, K, V>
DoubleEndedIterator
for std::collections::btree_map::
Range
<'a, K, V>
1.17.0
·
Source
§
impl<'a, K, V>
DoubleEndedIterator
for
RangeMut
<'a, K, V>
1.0.0
·
Source
§
impl<'a, K, V>
DoubleEndedIterator
for
Values
<'a, K, V>
1.10.0
·
Source
§
impl<'a, K, V>
DoubleEndedIterator
for
ValuesMut
<'a, K, V>
1.5.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for
MatchIndices
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.2.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for
Matches
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.5.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for
RMatchIndices
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.2.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for
RMatches
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.0.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for std::str::
RSplit
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.0.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for
RSplitTerminator
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.0.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for std::str::
Split
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.51.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for std::str::
SplitInclusive
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.0.0
·
Source
§
impl<'a, P>
DoubleEndedIterator
for
SplitTerminator
<'a, P>
where
    P:
Pattern
,
    <P as
Pattern
>::
Searcher
<'a>:
DoubleEndedSearcher
<'a>,
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::collections::binary_heap::
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::collections::btree_set::
Iter
<'a, T>
1.17.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::collections::btree_set::
Range
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::collections::linked_list::
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::collections::linked_list::
IterMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::collections::vec_deque::
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::collections::vec_deque::
IterMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::result::
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::result::
IterMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
Chunks
<'a, T>
1.31.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
ChunksExact
<'a, T>
1.31.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
ChunksExactMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
ChunksMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::slice::
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for std::slice::
IterMut
<'a, T>
1.31.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
RChunks
<'a, T>
1.31.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
RChunksExact
<'a, T>
1.31.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
RChunksExactMut
<'a, T>
1.31.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
RChunksMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
DoubleEndedIterator
for
Windows
<'a, T>
1.77.0
·
Source
§
impl<'a, T, P>
DoubleEndedIterator
for
ChunkBy
<'a, T, P>
where
    T: 'a,
    P:
FnMut
(
&T
,
&T
) ->
bool
,
1.77.0
·
Source
§
impl<'a, T, P>
DoubleEndedIterator
for
ChunkByMut
<'a, T, P>
where
    T: 'a,
    P:
FnMut
(
&T
,
&T
) ->
bool
,
1.27.0
·
Source
§
impl<'a, T, P>
DoubleEndedIterator
for std::slice::
RSplit
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.27.0
·
Source
§
impl<'a, T, P>
DoubleEndedIterator
for
RSplitMut
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.0.0
·
Source
§
impl<'a, T, P>
DoubleEndedIterator
for std::slice::
Split
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.51.0
·
Source
§
impl<'a, T, P>
DoubleEndedIterator
for std::slice::
SplitInclusive
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.51.0
·
Source
§
impl<'a, T, P>
DoubleEndedIterator
for
SplitInclusiveMut
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.0.0
·
Source
§
impl<'a, T, P>
DoubleEndedIterator
for
SplitMut
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
Source
§
impl<'a, T, const N:
usize
>
DoubleEndedIterator
for std::slice::
ArrayChunks
<'a, T, N>
Source
§
impl<'a, T, const N:
usize
>
DoubleEndedIterator
for
ArrayChunksMut
<'a, T, N>
Source
§
impl<'a, T, const N:
usize
>
DoubleEndedIterator
for
ArrayWindows
<'a, T, N>
1.0.0
·
Source
§
impl<A>
DoubleEndedIterator
for std::ops::
Range
<A>
where
    A:
Step
,
1.26.0
·
Source
§
impl<A>
DoubleEndedIterator
for
RangeInclusive
<A>
where
    A:
Step
,
1.0.0
·
Source
§
impl<A>
DoubleEndedIterator
for std::option::
IntoIter
<A>
Source
§
impl<A>
DoubleEndedIterator
for
IterRange
<A>
where
    A:
Step
,
Source
§
impl<A>
DoubleEndedIterator
for
IterRangeInclusive
<A>
where
    A:
Step
,
1.0.0
·
Source
§
impl<A>
DoubleEndedIterator
for
Repeat
<A>
where
    A:
Clone
,
1.82.0
·
Source
§
impl<A>
DoubleEndedIterator
for
RepeatN
<A>
where
    A:
Clone
,
1.0.0
·
Source
§
impl<A, B>
DoubleEndedIterator
for
Chain
<A, B>
where
    A:
DoubleEndedIterator
,
    B:
DoubleEndedIterator
<Item = <A as
Iterator
>::
Item
>,
1.0.0
·
Source
§
impl<A, B>
DoubleEndedIterator
for
Zip
<A, B>
where
    A:
DoubleEndedIterator
+
ExactSizeIterator
,
    B:
DoubleEndedIterator
+
ExactSizeIterator
,
1.43.0
·
Source
§
impl<A, F>
DoubleEndedIterator
for
OnceWith
<F>
where
    F:
FnOnce
() -> A,
1.0.0
·
Source
§
impl<B, I, F>
DoubleEndedIterator
for
FilterMap
<I, F>
where
    I:
DoubleEndedIterator
,
    F:
FnMut
(<I as
Iterator
>::
Item
) ->
Option
<B>,
1.0.0
·
Source
§
impl<B, I, F>
DoubleEndedIterator
for
Map
<I, F>
where
    I:
DoubleEndedIterator
,
    F:
FnMut
(<I as
Iterator
>::
Item
) -> B,
Source
§
impl<I>
DoubleEndedIterator
for
ByRefSized
<'_, I>
where
    I:
DoubleEndedIterator
,
1.0.0
·
Source
§
impl<I>
DoubleEndedIterator
for
Enumerate
<I>
where
    I:
ExactSizeIterator
+
DoubleEndedIterator
,
1.0.0
·
Source
§
impl<I>
DoubleEndedIterator
for
Fuse
<I>
where
    I:
DoubleEndedIterator
,
1.38.0
·
Source
§
impl<I>
DoubleEndedIterator
for
Peekable
<I>
where
    I:
DoubleEndedIterator
,
1.0.0
·
Source
§
impl<I>
DoubleEndedIterator
for
Rev
<I>
where
    I:
DoubleEndedIterator
,
1.9.0
·
Source
§
impl<I>
DoubleEndedIterator
for
Skip
<I>
where
    I:
DoubleEndedIterator
+
ExactSizeIterator
,
1.38.0
·
Source
§
impl<I>
DoubleEndedIterator
for
StepBy
<I>
where
    I:
DoubleEndedIterator
+
ExactSizeIterator
,
1.38.0
·
Source
§
impl<I>
DoubleEndedIterator
for
Take
<I>
where
    I:
DoubleEndedIterator
+
ExactSizeIterator
,
1.0.0
·
Source
§
impl<I, A>
DoubleEndedIterator
for
Box
<I, A>
where
    I:
DoubleEndedIterator
+ ?
Sized
,
    A:
Allocator
,
1.21.0
·
Source
§
impl<I, A>
DoubleEndedIterator
for
Splice
<'_, I, A>
where
    I:
Iterator
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<I, F>
DoubleEndedIterator
for
Inspect
<I, F>
where
    I:
DoubleEndedIterator
,
    F:
FnMut
(&<I as
Iterator
>::
Item
),
1.0.0
·
Source
§
impl<I, P>
DoubleEndedIterator
for
Filter
<I, P>
where
    I:
DoubleEndedIterator
,
    P:
FnMut
(&<I as
Iterator
>::
Item
) ->
bool
,
1.29.0
·
Source
§
impl<I, U>
DoubleEndedIterator
for
Flatten
<I>
where
    I:
DoubleEndedIterator
,
    <I as
Iterator
>::
Item
:
IntoIterator
<IntoIter = U, Item = <U as
Iterator
>::
Item
>,
    U:
DoubleEndedIterator
,
1.0.0
·
Source
§
impl<I, U, F>
DoubleEndedIterator
for
FlatMap
<I, U, F>
where
    I:
DoubleEndedIterator
,
    F:
FnMut
(<I as
Iterator
>::
Item
) -> U,
    U:
IntoIterator
,
    <U as
IntoIterator
>::
IntoIter
:
DoubleEndedIterator
,
Source
§
impl<I, const N:
usize
>
DoubleEndedIterator
for std::iter::
ArrayChunks
<I, N>
where
    I:
DoubleEndedIterator
+
ExactSizeIterator
,
1.0.0
·
Source
§
impl<K, V, A>
DoubleEndedIterator
for std::collections::btree_map::
IntoIter
<K, V, A>
where
    A:
Allocator
+
Clone
,
1.54.0
·
Source
§
impl<K, V, A>
DoubleEndedIterator
for
IntoKeys
<K, V, A>
where
    A:
Allocator
+
Clone
,
1.54.0
·
Source
§
impl<K, V, A>
DoubleEndedIterator
for
IntoValues
<K, V, A>
where
    A:
Allocator
+
Clone
,
1.0.0
·
Source
§
impl<T>
DoubleEndedIterator
for std::result::
IntoIter
<T>
1.2.0
·
Source
§
impl<T>
DoubleEndedIterator
for
Empty
<T>
1.2.0
·
Source
§
impl<T>
DoubleEndedIterator
for
Once
<T>
1.82.0
·
Source
§
impl<T>
DoubleEndedIterator
for
Take
<
Repeat
<T>>
where
    T:
Clone
,
1.6.0
·
Source
§
impl<T, A>
DoubleEndedIterator
for std::collections::binary_heap::
Drain
<'_, T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
DoubleEndedIterator
for std::collections::binary_heap::
IntoIter
<T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
DoubleEndedIterator
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
Allocator
+
Clone
,
1.0.0
·
Source
§
impl<T, A>
DoubleEndedIterator
for std::collections::linked_list::
IntoIter
<T, A>
where
    A:
Allocator
,
1.6.0
·
Source
§
impl<T, A>
DoubleEndedIterator
for std::collections::vec_deque::
Drain
<'_, T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
DoubleEndedIterator
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
Allocator
,
1.6.0
·
Source
§
impl<T, A>
DoubleEndedIterator
for std::vec::
Drain
<'_, T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
DoubleEndedIterator
for std::vec::
IntoIter
<T, A>
where
    A:
Allocator
,
1.40.0
·
Source
§
impl<T, const N:
usize
>
DoubleEndedIterator
for std::array::
IntoIter
<T, N>