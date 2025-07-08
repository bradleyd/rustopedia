TrustedLen in std::iter - Rust
std
::
iter
Trait
TrustedLen
Copy item path
Source
pub unsafe trait TrustedLen:
Iterator
{ }
🔬
This is a nightly-only experimental API. (
trusted_len
#37572
)
Expand description
An iterator that reports an accurate length using size_hint.
The iterator reports a size hint where it is either exact
(lower bound is equal to upper bound), or the upper bound is
None
.
The upper bound must only be
None
if the actual iterator length is
larger than
usize::MAX
. In that case, the lower bound must be
usize::MAX
, resulting in an
Iterator::size_hint()
of
(usize::MAX, None)
.
The iterator must produce exactly the number of elements it reported
or diverge before reaching the end.
§
When
shouldn’t
an adapter be
TrustedLen
?
If an adapter makes an iterator
shorter
by a given amount, then it’s
usually incorrect for that adapter to implement
TrustedLen
.  The inner
iterator might return more than
usize::MAX
items, but there’s no way to
know what
k
elements less than that will be, since the
size_hint
from
the inner iterator has already saturated and lost that information.
This is why
Skip<I>
isn’t
TrustedLen
, even when
I
implements
TrustedLen
.
§
Safety
This trait must only be implemented when the contract is upheld. Consumers
of this trait must inspect
Iterator::size_hint()
’s upper bound.
Implementors
§
Source
§
impl
TrustedLen
for
ToLowercase
Source
§
impl
TrustedLen
for
ToUppercase
Source
§
impl
TrustedLen
for
Bytes
<'_>
Source
§
impl<'a, I, T>
TrustedLen
for
Cloned
<I>
where
    T: 'a +
Clone
,
    I:
TrustedLen
<Item =
&'a T
>,
1.36.0
·
Source
§
impl<'a, I, T>
TrustedLen
for
Copied
<I>
where
    T: 'a +
Copy
,
    I:
TrustedLen
<Item =
&'a T
>,
Source
§
impl<A>
TrustedLen
for
Range
<A>
where
    A:
TrustedStep
,
Source
§
impl<A>
TrustedLen
for
RangeFrom
<A>
where
    A:
TrustedStep
,
Source
§
impl<A>
TrustedLen
for
RangeInclusive
<A>
where
    A:
TrustedStep
,
Source
§
impl<A>
TrustedLen
for std::option::
IntoIter
<A>
Source
§
impl<A>
TrustedLen
for std::option::
Iter
<'_, A>
Source
§
impl<A>
TrustedLen
for std::option::
IterMut
<'_, A>
Source
§
impl<A>
TrustedLen
for
IterRange
<A>
where
    A:
TrustedStep
,
Source
§
impl<A>
TrustedLen
for
IterRangeFrom
<A>
where
    A:
TrustedStep
,
Source
§
impl<A>
TrustedLen
for
IterRangeInclusive
<A>
where
    A:
TrustedStep
,
Source
§
impl<A>
TrustedLen
for std::result::
IntoIter
<A>
Source
§
impl<A>
TrustedLen
for std::result::
Iter
<'_, A>
Source
§
impl<A>
TrustedLen
for std::result::
IterMut
<'_, A>
Source
§
impl<A>
TrustedLen
for
Repeat
<A>
where
    A:
Clone
,
Source
§
impl<A>
TrustedLen
for
RepeatN
<A>
where
    A:
Clone
,
Source
§
impl<A, B>
TrustedLen
for
Chain
<A, B>
where
    A:
TrustedLen
,
    B:
TrustedLen
<Item = <A as
Iterator
>::
Item
>,
Source
§
impl<A, B>
TrustedLen
for
Zip
<A, B>
where
    A:
TrustedLen
,
    B:
TrustedLen
,
1.43.0
·
Source
§
impl<A, F>
TrustedLen
for
OnceWith
<F>
where
    F:
FnOnce
() -> A,
Source
§
impl<A, F>
TrustedLen
for
RepeatWith
<F>
where
    F:
FnMut
() -> A,
Source
§
impl<B, I, F>
TrustedLen
for
Map
<I, F>
where
    I:
TrustedLen
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
TrustedLen
for
&mut I
where
    I:
TrustedLen
+ ?
Sized
,
Source
§
impl<I>
TrustedLen
for
Enumerate
<I>
where
    I:
TrustedLen
,
Source
§
impl<I>
TrustedLen
for
Flatten
<I>
where
    I:
Iterator
,
    <I as
Iterator
>::
Item
:
IntoIterator
,
    FlattenCompat<I, <<I as
Iterator
>::
Item
as
IntoIterator
>::
IntoIter
>:
TrustedLen
,
Source
§
impl<I>
TrustedLen
for
Fuse
<I>
where
    I:
TrustedLen
,
Source
§
impl<I>
TrustedLen
for
Peekable
<I>
where
    I:
TrustedLen
,
Source
§
impl<I>
TrustedLen
for
Rev
<I>
where
    I:
TrustedLen
+
DoubleEndedIterator
,
Source
§
impl<I>
TrustedLen
for
Skip
<I>
where
    I:
Iterator
+ TrustedRandomAccess,
Source
§
impl<I>
TrustedLen
for
StepBy
<I>
where
    I:
Iterator
+ TrustedRandomAccess,
Source
§
impl<I>
TrustedLen
for
Take
<I>
where
    I:
TrustedLen
,
Source
§
impl<I, U, F>
TrustedLen
for
FlatMap
<I, U, F>
where
    I:
Iterator
,
    U:
IntoIterator
,
    F:
FnMut
(<I as
Iterator
>::
Item
) -> U,
    FlattenCompat<
Map
<I, F>, <U as
IntoIterator
>::
IntoIter
>:
TrustedLen
,
Source
§
impl<T>
TrustedLen
for std::collections::vec_deque::
Iter
<'_, T>
Source
§
impl<T>
TrustedLen
for std::collections::vec_deque::
IterMut
<'_, T>
Source
§
impl<T>
TrustedLen
for
Chunks
<'_, T>
Source
§
impl<T>
TrustedLen
for
ChunksExact
<'_, T>
Source
§
impl<T>
TrustedLen
for
ChunksExactMut
<'_, T>
Source
§
impl<T>
TrustedLen
for
ChunksMut
<'_, T>
Source
§
impl<T>
TrustedLen
for std::slice::
Iter
<'_, T>
Source
§
impl<T>
TrustedLen
for std::slice::
IterMut
<'_, T>
Source
§
impl<T>
TrustedLen
for
RChunks
<'_, T>
Source
§
impl<T>
TrustedLen
for
RChunksExact
<'_, T>
Source
§
impl<T>
TrustedLen
for
RChunksExactMut
<'_, T>
Source
§
impl<T>
TrustedLen
for
RChunksMut
<'_, T>
Source
§
impl<T>
TrustedLen
for
Windows
<'_, T>
Source
§
impl<T>
TrustedLen
for
Empty
<T>
Source
§
impl<T>
TrustedLen
for
Once
<T>
Source
§
impl<T, A>
TrustedLen
for
DrainSorted
<'_, T, A>
where
    T:
Ord
,
    A:
Allocator
,
Source
§
impl<T, A>
TrustedLen
for
IntoIterSorted
<T, A>
where
    T:
Ord
,
    A:
Allocator
,
Source
§
impl<T, A>
TrustedLen
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
Allocator
,
Source
§
impl<T, A>
TrustedLen
for
Drain
<'_, T, A>
where
    A:
Allocator
,
Source
§
impl<T, A>
TrustedLen
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
TrustedLen
for std::array::
IntoIter
<T, N>
Source
§
impl<T, const N:
usize
>
TrustedLen
for
ArrayChunks
<'_, T, N>
Source
§
impl<T, const N:
usize
>
TrustedLen
for
ArrayChunksMut
<'_, T, N>