FusedIterator in std::iter - Rust
std
::
iter
Trait
FusedIterator
Copy item path
1.26.0
·
Source
pub trait FusedIterator:
Iterator
{ }
Expand description
An iterator that always continues to yield
None
when exhausted.
Calling next on a fused iterator that has returned
None
once is guaranteed
to return
None
again. This trait should be implemented by all iterators
that behave this way because it allows optimizing
Iterator::fuse()
.
Note: In general, you should not use
FusedIterator
in generic bounds if
you need a fused iterator. Instead, you should just call
Iterator::fuse()
on the iterator. If the iterator is already fused, the additional
Fuse
wrapper will be a no-op with no performance penalty.
Implementors
§
Source
§
impl
FusedIterator
for core::ffi::c_str::
Bytes
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for std::ascii::
EscapeDefault
1.26.0
·
Source
§
impl
FusedIterator
for std::char::
EscapeDebug
1.26.0
·
Source
§
impl
FusedIterator
for std::char::
EscapeDefault
1.26.0
·
Source
§
impl
FusedIterator
for std::char::
EscapeUnicode
1.26.0
·
Source
§
impl
FusedIterator
for
ToLowercase
1.26.0
·
Source
§
impl
FusedIterator
for
ToUppercase
1.64.0
·
Source
§
impl
FusedIterator
for
Incoming
<'_>
Source
§
impl
FusedIterator
for
IntoIncoming
1.62.0
·
Source
§
impl
FusedIterator
for
EncodeWide
<'_>
1.28.0
·
Source
§
impl
FusedIterator
for
Ancestors
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for
Components
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for std::path::
Iter
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for std::str::
Bytes
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for
CharIndices
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for
Chars
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for
EncodeUtf16
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for
Lines
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for
LinesAny
<'_>
1.34.0
·
Source
§
impl
FusedIterator
for
SplitAsciiWhitespace
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for
SplitWhitespace
<'_>
1.79.0
·
Source
§
impl
FusedIterator
for
Utf8Chunks
<'_>
1.26.0
·
Source
§
impl
FusedIterator
for std::string::
Drain
<'_>
Source
§
impl
FusedIterator
for
IntoChars
Source
§
impl<'a>
FusedIterator
for
Source
<'a>
1.60.0
·
Source
§
impl<'a>
FusedIterator
for
EscapeAscii
<'a>
1.34.0
·
Source
§
impl<'a>
FusedIterator
for std::str::
EscapeDebug
<'a>
1.34.0
·
Source
§
impl<'a>
FusedIterator
for std::str::
EscapeDefault
<'a>
1.34.0
·
Source
§
impl<'a>
FusedIterator
for std::str::
EscapeUnicode
<'a>
1.26.0
·
Source
§
impl<'a, I, T>
FusedIterator
for
Cloned
<I>
where
    T: 'a +
Clone
,
    I:
FusedIterator
<Item =
&'a T
>,
1.36.0
·
Source
§
impl<'a, I, T>
FusedIterator
for
Copied
<I>
where
    T: 'a +
Copy
,
    I:
FusedIterator
<Item =
&'a T
>,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
for
MatchIndices
<'a, P>
where
    P:
Pattern
,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
for
Matches
<'a, P>
where
    P:
Pattern
,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
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
ReverseSearcher
<'a>,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
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
ReverseSearcher
<'a>,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
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
ReverseSearcher
<'a>,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
for std::str::
RSplitN
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
ReverseSearcher
<'a>,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
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
ReverseSearcher
<'a>,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
for std::str::
Split
<'a, P>
where
    P:
Pattern
,
1.51.0
·
Source
§
impl<'a, P>
FusedIterator
for std::str::
SplitInclusive
<'a, P>
where
    P:
Pattern
,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
for std::str::
SplitN
<'a, P>
where
    P:
Pattern
,
1.26.0
·
Source
§
impl<'a, P>
FusedIterator
for
SplitTerminator
<'a, P>
where
    P:
Pattern
,
1.77.0
·
Source
§
impl<'a, T, P>
FusedIterator
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
FusedIterator
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
1.26.0
·
Source
§
impl<'a, T, P>
FusedIterator
for std::slice::
RSplitN
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.26.0
·
Source
§
impl<'a, T, P>
FusedIterator
for
RSplitNMut
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.26.0
·
Source
§
impl<'a, T, P>
FusedIterator
for std::slice::
SplitN
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.26.0
·
Source
§
impl<'a, T, P>
FusedIterator
for
SplitNMut
<'a, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.26.0
·
Source
§
impl<A>
FusedIterator
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
FusedIterator
for
RangeFrom
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
FusedIterator
for
RangeInclusive
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
FusedIterator
for std::option::
IntoIter
<A>
1.26.0
·
Source
§
impl<A>
FusedIterator
for std::option::
Iter
<'_, A>
1.26.0
·
Source
§
impl<A>
FusedIterator
for std::option::
IterMut
<'_, A>
Source
§
impl<A>
FusedIterator
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
FusedIterator
for
IterRangeFrom
<A>
where
    A:
Step
,
Source
§
impl<A>
FusedIterator
for
IterRangeInclusive
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
FusedIterator
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
FusedIterator
for
RepeatN
<A>
where
    A:
Clone
,
1.26.0
·
Source
§
impl<A, B>
FusedIterator
for
Chain
<A, B>
where
    A:
FusedIterator
,
    B:
FusedIterator
<Item = <A as
Iterator
>::
Item
>,
1.26.0
·
Source
§
impl<A, B>
FusedIterator
for
Zip
<A, B>
where
    A:
FusedIterator
,
    B:
FusedIterator
,
1.43.0
·
Source
§
impl<A, F>
FusedIterator
for
OnceWith
<F>
where
    F:
FnOnce
() -> A,
1.28.0
·
Source
§
impl<A, F>
FusedIterator
for
RepeatWith
<F>
where
    F:
FnMut
() -> A,
1.26.0
·
Source
§
impl<B, I, F>
FusedIterator
for
FilterMap
<I, F>
where
    I:
FusedIterator
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
1.26.0
·
Source
§
impl<B, I, F>
FusedIterator
for
Map
<I, F>
where
    I:
FusedIterator
,
    F:
FnMut
(<I as
Iterator
>::
Item
) -> B,
1.26.0
·
Source
§
impl<I>
FusedIterator
for
&mut I
where
    I:
FusedIterator
+ ?
Sized
,
1.75.0
·
Source
§
impl<I>
FusedIterator
for
DecodeUtf16
<I>
where
    I:
Iterator
<Item =
u16
> +
FusedIterator
,
1.26.0
·
Source
§
impl<I>
FusedIterator
for
Cycle
<I>
where
    I:
Clone
+
Iterator
,
1.26.0
·
Source
§
impl<I>
FusedIterator
for
Enumerate
<I>
where
    I:
FusedIterator
,
1.26.0
·
Source
§
impl<I>
FusedIterator
for
Fuse
<I>
where
    I:
Iterator
,
Source
§
impl<I>
FusedIterator
for
Intersperse
<I>
where
    I:
FusedIterator
,
    <I as
Iterator
>::
Item
:
Clone
,
1.26.0
·
Source
§
impl<I>
FusedIterator
for
Peekable
<I>
where
    I:
FusedIterator
,
1.26.0
·
Source
§
impl<I>
FusedIterator
for
Rev
<I>
where
    I:
FusedIterator
+
DoubleEndedIterator
,
1.26.0
·
Source
§
impl<I>
FusedIterator
for
Skip
<I>
where
    I:
FusedIterator
,
1.26.0
·
Source
§
impl<I>
FusedIterator
for
Take
<I>
where
    I:
FusedIterator
,
1.26.0
·
Source
§
impl<I, A>
FusedIterator
for
Box
<I, A>
where
    I:
FusedIterator
+ ?
Sized
,
    A:
Allocator
,
1.26.0
·
Source
§
impl<I, F>
FusedIterator
for
Inspect
<I, F>
where
    I:
FusedIterator
,
    F:
FnMut
(&<I as
Iterator
>::
Item
),
Source
§
impl<I, F, R, const N:
usize
>
FusedIterator
for
MapWindows
<I, F, N>
where
    I:
Iterator
,
    F:
FnMut
(&[<I as
Iterator
>::
Item
;
N
]) -> R,
Source
§
impl<I, G>
FusedIterator
for
IntersperseWith
<I, G>
where
    I:
FusedIterator
,
    G:
FnMut
() -> <I as
Iterator
>::
Item
,
1.26.0
·
Source
§
impl<I, P>
FusedIterator
for
Filter
<I, P>
where
    I:
FusedIterator
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
1.26.0
·
Source
§
impl<I, P>
FusedIterator
for
SkipWhile
<I, P>
where
    I:
FusedIterator
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
1.26.0
·
Source
§
impl<I, P>
FusedIterator
for
TakeWhile
<I, P>
where
    I:
FusedIterator
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
FusedIterator
for
Flatten
<I>
where
    I:
FusedIterator
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
Iterator
,
1.26.0
·
Source
§
impl<I, U, F>
FusedIterator
for
FlatMap
<I, U, F>
where
    I:
FusedIterator
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
Source
§
impl<I, const N:
usize
>
FusedIterator
for std::iter::
ArrayChunks
<I, N>
where
    I:
FusedIterator
,
1.26.0
·
Source
§
impl<K>
FusedIterator
for std::collections::hash_set::
Drain
<'_, K>
1.26.0
·
Source
§
impl<K>
FusedIterator
for std::collections::hash_set::
IntoIter
<K>
1.26.0
·
Source
§
impl<K>
FusedIterator
for std::collections::hash_set::
Iter
<'_, K>
Source
§
impl<K, F>
FusedIterator
for std::collections::hash_set::
ExtractIf
<'_, K, F>
where
    F:
FnMut
(
&K
) ->
bool
,
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::btree_map::
Iter
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::btree_map::
IterMut
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::btree_map::
Keys
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::btree_map::
Range
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for
RangeMut
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::btree_map::
Values
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::btree_map::
ValuesMut
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
Drain
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
IntoIter
<K, V>
1.54.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
IntoKeys
<K, V>
1.54.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
IntoValues
<K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
Iter
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
IterMut
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
Keys
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
Values
<'_, K, V>
1.26.0
·
Source
§
impl<K, V>
FusedIterator
for std::collections::hash_map::
ValuesMut
<'_, K, V>
1.26.0
·
Source
§
impl<K, V, A>
FusedIterator
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
FusedIterator
for std::collections::btree_map::
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
FusedIterator
for std::collections::btree_map::
IntoValues
<K, V, A>
where
    A:
Allocator
+
Clone
,
Source
§
impl<K, V, F>
FusedIterator
for std::collections::btree_map::
ExtractIf
<'_, K, V, F>
where
    F:
FnMut
(
&K
,
&mut V
) ->
bool
,
Source
§
impl<K, V, F>
FusedIterator
for std::collections::hash_map::
ExtractIf
<'_, K, V, F>
where
    F:
FnMut
(
&K
,
&mut V
) ->
bool
,
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::binary_heap::
Iter
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::btree_set::
Iter
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::btree_set::
Range
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::btree_set::
SymmetricDifference
<'_, T>
where
    T:
Ord
,
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::btree_set::
Union
<'_, T>
where
    T:
Ord
,
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::linked_list::
Iter
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::linked_list::
IterMut
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::vec_deque::
Iter
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::collections::vec_deque::
IterMut
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::result::
IntoIter
<T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::result::
Iter
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::result::
IterMut
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for
Chunks
<'_, T>
1.31.0
·
Source
§
impl<T>
FusedIterator
for
ChunksExact
<'_, T>
1.31.0
·
Source
§
impl<T>
FusedIterator
for
ChunksExactMut
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for
ChunksMut
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::slice::
Iter
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for std::slice::
IterMut
<'_, T>
1.31.0
·
Source
§
impl<T>
FusedIterator
for
RChunks
<'_, T>
1.31.0
·
Source
§
impl<T>
FusedIterator
for
RChunksExact
<'_, T>
1.31.0
·
Source
§
impl<T>
FusedIterator
for
RChunksExactMut
<'_, T>
1.31.0
·
Source
§
impl<T>
FusedIterator
for
RChunksMut
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for
Windows
<'_, T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for
Empty
<T>
1.26.0
·
Source
§
impl<T>
FusedIterator
for
Once
<T>
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::collections::binary_heap::
Drain
<'_, T, A>
where
    A:
Allocator
,
Source
§
impl<T, A>
FusedIterator
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
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::collections::binary_heap::
IntoIter
<T, A>
where
    A:
Allocator
,
Source
§
impl<T, A>
FusedIterator
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
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::collections::btree_set::
Difference
<'_, T, A>
where
    T:
Ord
,
    A:
Allocator
+
Clone
,
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::collections::btree_set::
Intersection
<'_, T, A>
where
    T:
Ord
,
    A:
Allocator
+
Clone
,
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::collections::btree_set::
IntoIter
<T, A>
where
    A:
Allocator
+
Clone
,
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::collections::linked_list::
IntoIter
<T, A>
where
    A:
Allocator
,
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::collections::vec_deque::
Drain
<'_, T, A>
where
    A:
Allocator
,
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::collections::vec_deque::
IntoIter
<T, A>
where
    A:
Allocator
,
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::vec::
Drain
<'_, T, A>
where
    A:
Allocator
,
1.26.0
·
Source
§
impl<T, A>
FusedIterator
for std::vec::
IntoIter
<T, A>
where
    A:
Allocator
,
1.34.0
·
Source
§
impl<T, F>
FusedIterator
for
Successors
<T, F>
where
    F:
FnMut
(
&T
) ->
Option
<T>,
Source
§
impl<T, F, A>
FusedIterator
for std::collections::btree_set::
ExtractIf
<'_, T, F, A>
where
    A:
Allocator
+
Clone
,
    F:
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
impl<T, P>
FusedIterator
for std::slice::
RSplit
<'_, T, P>
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
impl<T, P>
FusedIterator
for
RSplitMut
<'_, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.26.0
·
Source
§
impl<T, P>
FusedIterator
for std::slice::
Split
<'_, T, P>
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
impl<T, P>
FusedIterator
for std::slice::
SplitInclusive
<'_, T, P>
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
impl<T, P>
FusedIterator
for
SplitInclusiveMut
<'_, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.26.0
·
Source
§
impl<T, P>
FusedIterator
for
SplitMut
<'_, T, P>
where
    P:
FnMut
(
&T
) ->
bool
,
1.26.0
·
Source
§
impl<T, S>
FusedIterator
for std::collections::hash_set::
Difference
<'_, T, S>
where
    T:
Eq
+
Hash
,
    S:
BuildHasher
,
1.26.0
·
Source
§
impl<T, S>
FusedIterator
for std::collections::hash_set::
Intersection
<'_, T, S>
where
    T:
Eq
+
Hash
,
    S:
BuildHasher
,
1.26.0
·
Source
§
impl<T, S>
FusedIterator
for std::collections::hash_set::
SymmetricDifference
<'_, T, S>
where
    T:
Eq
+
Hash
,
    S:
BuildHasher
,
1.26.0
·
Source
§
impl<T, S>
FusedIterator
for std::collections::hash_set::
Union
<'_, T, S>
where
    T:
Eq
+
Hash
,
    S:
BuildHasher
,
1.40.0
·
Source
§
impl<T, const N:
usize
>
FusedIterator
for std::array::
IntoIter
<T, N>
Source
§
impl<T, const N:
usize
>
FusedIterator
for std::slice::
ArrayChunks
<'_, T, N>
Source
§
impl<T, const N:
usize
>
FusedIterator
for
ArrayChunksMut
<'_, T, N>