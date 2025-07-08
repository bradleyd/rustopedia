ExactSizeIterator in std::iter - Rust
std
::
iter
Trait
ExactSizeIterator
Copy item path
1.0.0
·
Source
pub trait ExactSizeIterator:
Iterator
{
    // Provided methods
    fn
len
(&self) ->
usize
{ ... }
fn
is_empty
(&self) ->
bool
{ ... }
}
Expand description
An iterator that knows its exact length.
Many
Iterator
s don’t know how many times they will iterate, but some do.
If an iterator knows how many times it can iterate, providing access to
that information can be useful. For example, if you want to iterate
backwards, a good start is to know where the end is.
When implementing an
ExactSizeIterator
, you must also implement
Iterator
. When doing so, the implementation of
Iterator::size_hint
must
return the exact size of the iterator.
The
len
method has a default implementation, so you usually shouldn’t
implement it. However, you may be able to provide a more performant
implementation than the default, so overriding it in this case makes sense.
Note that this trait is a safe trait and as such does
not
and
cannot
guarantee that the returned length is correct. This means that
unsafe
code
must not
rely on the correctness of
Iterator::size_hint
. The
unstable and unsafe
TrustedLen
trait gives
this additional guarantee.
§
When
shouldn’t
an adapter be
ExactSizeIterator
?
If an adapter makes an iterator
longer
, then it’s usually incorrect for
that adapter to implement
ExactSizeIterator
.  The inner exact-sized
iterator might already be
usize::MAX
-long, and thus the length of the
longer adapted iterator would no longer be exactly representable in
usize
.
This is why
Chain<A, B>
isn’t
ExactSizeIterator
,
even when
A
and
B
are both
ExactSizeIterator
.
§
Examples
Basic usage:
// a finite range knows exactly how many times it will iterate
let
five =
0
..
5
;
assert_eq!
(
5
, five.len());
In the
module-level docs
, we implemented an
Iterator
,
Counter
.
Let’s implement
ExactSizeIterator
for it as well:
impl
ExactSizeIterator
for
Counter {
// We can easily calculate the remaining number of iterations.
fn
len(
&
self
) -> usize {
5
-
self
.count
    }
}
// And now we can use it!
let
mut
counter = Counter::new();
assert_eq!
(
5
, counter.len());
let _
= counter.next();
assert_eq!
(
4
, counter.len());
Provided Methods
§
1.0.0
·
Source
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
The implementation ensures that the iterator will return exactly
len()
more times a
Some(T)
value, before returning
None
.
This method has a default implementation, so you usually should not
implement it directly. However, if you can provide a more efficient
implementation, you can do so. See the
trait-level
docs for an
example.
This function has the same safety guarantees as the
Iterator::size_hint
function.
§
Examples
Basic usage:
// a finite range knows exactly how many times it will iterate
let
mut
range =
0
..
5
;
assert_eq!
(
5
, range.len());
let _
= range.next();
assert_eq!
(
4
, range.len());
Source
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
This method has a default implementation using
ExactSizeIterator::len()
, so you don’t need to implement it yourself.
§
Examples
Basic usage:
#![feature(exact_size_is_empty)]
let
mut
one_element = std::iter::once(
0
);
assert!
(!one_element.is_empty());
assert_eq!
(one_element.next(),
Some
(
0
));
assert!
(one_element.is_empty());
assert_eq!
(one_element.next(),
None
);
Implementors
§
1.0.0
·
Source
§
impl
ExactSizeIterator
for std::ascii::
EscapeDefault
1.20.0
·
Source
§
impl
ExactSizeIterator
for
EscapeDebug
1.11.0
·
Source
§
impl
ExactSizeIterator
for std::char::
EscapeDefault
1.11.0
·
Source
§
impl
ExactSizeIterator
for
EscapeUnicode
1.35.0
·
Source
§
impl
ExactSizeIterator
for
ToLowercase
1.35.0
·
Source
§
impl
ExactSizeIterator
for
ToUppercase
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Args
1.0.0
·
Source
§
impl
ExactSizeIterator
for
ArgsOs
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
i8
>
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
i16
>
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
i32
>
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
isize
>
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
u8
>
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
u16
>
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
u32
>
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Range
<
usize
>
1.26.0
·
Source
§
impl
ExactSizeIterator
for
RangeInclusive
<
i8
>
1.26.0
·
Source
§
impl
ExactSizeIterator
for
RangeInclusive
<
i16
>
1.26.0
·
Source
§
impl
ExactSizeIterator
for
RangeInclusive
<
u8
>
1.26.0
·
Source
§
impl
ExactSizeIterator
for
RangeInclusive
<
u16
>
Source
§
impl
ExactSizeIterator
for
IterRange
<
i8
>
Source
§
impl
ExactSizeIterator
for
IterRange
<
i16
>
Source
§
impl
ExactSizeIterator
for
IterRange
<
isize
>
Source
§
impl
ExactSizeIterator
for
IterRange
<
u8
>
Source
§
impl
ExactSizeIterator
for
IterRange
<
u16
>
Source
§
impl
ExactSizeIterator
for
IterRange
<
usize
>
Source
§
impl
ExactSizeIterator
for
IterRangeInclusive
<
i8
>
Source
§
impl
ExactSizeIterator
for
IterRangeInclusive
<
u8
>
1.0.0
·
Source
§
impl
ExactSizeIterator
for
Bytes
<'_>
1.57.0
·
Source
§
impl<'a>
ExactSizeIterator
for
CommandArgs
<'a>
1.57.0
·
Source
§
impl<'a>
ExactSizeIterator
for
CommandEnvs
<'a>
1.1.0
·
Source
§
impl<'a, I, T>
ExactSizeIterator
for
Cloned
<I>
where
    T: 'a +
Clone
,
    I:
ExactSizeIterator
<Item =
&'a T
>,
1.36.0
·
Source
§
impl<'a, I, T>
ExactSizeIterator
for
Copied
<I>
where
    T: 'a +
Copy
,
    I:
ExactSizeIterator
<Item =
&'a T
>,
1.31.0
·
Source
§
impl<'a, T>
ExactSizeIterator
for
RChunksExact
<'a, T>
1.0.0
·
Source
§
impl<A>
ExactSizeIterator
for std::option::
IntoIter
<A>
1.0.0
·
Source
§
impl<A>
ExactSizeIterator
for std::option::
Iter
<'_, A>
1.0.0
·
Source
§
impl<A>
ExactSizeIterator
for std::option::
IterMut
<'_, A>
1.82.0
·
Source
§
impl<A>
ExactSizeIterator
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
ExactSizeIterator
for
Zip
<A, B>
where
    A:
ExactSizeIterator
,
    B:
ExactSizeIterator
,
1.43.0
·
Source
§
impl<A, F>
ExactSizeIterator
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
ExactSizeIterator
for
Map
<I, F>
where
    I:
ExactSizeIterator
,
    F:
FnMut
(<I as
Iterator
>::
Item
) -> B,
1.82.0
·
Source
§
impl<F, A>
ExactSizeIterator
for
Take
<
RepeatWith
<F>>
where
    F:
FnMut
() -> A,
1.0.0
·
Source
§
impl<I>
ExactSizeIterator
for
&mut I
where
    I:
ExactSizeIterator
+ ?
Sized
,
1.0.0
·
Source
§
impl<I>
ExactSizeIterator
for
Enumerate
<I>
where
    I:
ExactSizeIterator
,
1.0.0
·
Source
§
impl<I>
ExactSizeIterator
for
Fuse
<I>
where
    I:
ExactSizeIterator
,
1.0.0
·
Source
§
impl<I>
ExactSizeIterator
for
Peekable
<I>
where
    I:
ExactSizeIterator
,
1.0.0
·
Source
§
impl<I>
ExactSizeIterator
for
Rev
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
ExactSizeIterator
for
Skip
<I>
where
    I:
ExactSizeIterator
,
1.28.0
·
Source
§
impl<I>
ExactSizeIterator
for
StepBy
<I>
where
    I:
ExactSizeIterator
,
1.0.0
·
Source
§
impl<I>
ExactSizeIterator
for
Take
<I>
where
    I:
ExactSizeIterator
,
1.0.0
·
Source
§
impl<I, A>
ExactSizeIterator
for
Box
<I, A>
where
    I:
ExactSizeIterator
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
ExactSizeIterator
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
ExactSizeIterator
for
Inspect
<I, F>
where
    I:
ExactSizeIterator
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
ExactSizeIterator
for
MapWindows
<I, F, N>
where
    I:
ExactSizeIterator
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
impl<I, const N:
usize
>
ExactSizeIterator
for std::iter::
ArrayChunks
<I, N>
where
    I:
ExactSizeIterator
,
1.0.0
·
Source
§
impl<K>
ExactSizeIterator
for std::collections::hash_set::
Drain
<'_, K>
1.0.0
·
Source
§
impl<K>
ExactSizeIterator
for std::collections::hash_set::
IntoIter
<K>
1.0.0
·
Source
§
impl<K>
ExactSizeIterator
for std::collections::hash_set::
Iter
<'_, K>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::btree_map::
Iter
<'_, K, V>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::btree_map::
IterMut
<'_, K, V>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::btree_map::
Keys
<'_, K, V>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::btree_map::
Values
<'_, K, V>
1.10.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::btree_map::
ValuesMut
<'_, K, V>
1.6.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
Drain
<'_, K, V>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
IntoIter
<K, V>
1.54.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
IntoKeys
<K, V>
1.54.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
IntoValues
<K, V>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
Iter
<'_, K, V>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
IterMut
<'_, K, V>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
Keys
<'_, K, V>
1.0.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
Values
<'_, K, V>
1.10.0
·
Source
§
impl<K, V>
ExactSizeIterator
for std::collections::hash_map::
ValuesMut
<'_, K, V>
1.0.0
·
Source
§
impl<K, V, A>
ExactSizeIterator
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
ExactSizeIterator
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
ExactSizeIterator
for std::collections::btree_map::
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
ExactSizeIterator
for std::collections::binary_heap::
Iter
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::collections::btree_set::
Iter
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::collections::linked_list::
Iter
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::collections::linked_list::
IterMut
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::collections::vec_deque::
Iter
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::collections::vec_deque::
IterMut
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::result::
IntoIter
<T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::result::
Iter
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::result::
IterMut
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for
Chunks
<'_, T>
1.31.0
·
Source
§
impl<T>
ExactSizeIterator
for
ChunksExact
<'_, T>
1.31.0
·
Source
§
impl<T>
ExactSizeIterator
for
ChunksExactMut
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for
ChunksMut
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::slice::
Iter
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for std::slice::
IterMut
<'_, T>
1.31.0
·
Source
§
impl<T>
ExactSizeIterator
for
RChunks
<'_, T>
1.31.0
·
Source
§
impl<T>
ExactSizeIterator
for
RChunksExactMut
<'_, T>
1.31.0
·
Source
§
impl<T>
ExactSizeIterator
for
RChunksMut
<'_, T>
1.0.0
·
Source
§
impl<T>
ExactSizeIterator
for
Windows
<'_, T>
1.2.0
·
Source
§
impl<T>
ExactSizeIterator
for
Empty
<T>
1.2.0
·
Source
§
impl<T>
ExactSizeIterator
for
Once
<T>
1.82.0
·
Source
§
impl<T>
ExactSizeIterator
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
ExactSizeIterator
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
ExactSizeIterator
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
1.0.0
·
Source
§
impl<T, A>
ExactSizeIterator
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
ExactSizeIterator
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
1.0.0
·
Source
§
impl<T, A>
ExactSizeIterator
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
ExactSizeIterator
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
ExactSizeIterator
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
ExactSizeIterator
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
ExactSizeIterator
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
ExactSizeIterator
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
ExactSizeIterator
for std::array::
IntoIter
<T, N>
Source
§
impl<T, const N:
usize
>
ExactSizeIterator
for std::slice::
ArrayChunks
<'_, T, N>
Source
§
impl<T, const N:
usize
>
ExactSizeIterator
for
ArrayChunksMut
<'_, T, N>
Source
§
impl<T, const N:
usize
>
ExactSizeIterator
for
ArrayWindows
<'_, T, N>