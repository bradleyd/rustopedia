IntoIterator in std::iter - Rust
std
::
iter
Trait
IntoIterator
Copy item path
1.0.0
·
Source
pub trait IntoIterator {
    type
Item
;
    type
IntoIter
:
Iterator
<Item = Self::
Item
>;

    // Required method
    fn
into_iter
(self) -> Self::
IntoIter
;
}
Expand description
Conversion into an
Iterator
.
By implementing
IntoIterator
for a type, you define how it will be
converted to an iterator. This is common for types which describe a
collection of some kind.
One benefit of implementing
IntoIterator
is that your type will
work
with Rust’s
for
loop syntax
.
See also:
FromIterator
.
§
Examples
Basic usage:
let
v = [
1
,
2
,
3
];
let
mut
iter = v.into_iter();
assert_eq!
(
Some
(
1
), iter.next());
assert_eq!
(
Some
(
2
), iter.next());
assert_eq!
(
Some
(
3
), iter.next());
assert_eq!
(
None
, iter.next());
Implementing
IntoIterator
for your type:
// A sample collection, that's just a wrapper over Vec<T>
#[derive(Debug)]
struct
MyCollection(Vec<i32>);
// Let's give it some methods so we can create one and add things
// to it.
impl
MyCollection {
fn
new() -> MyCollection {
        MyCollection(Vec::new())
    }
fn
add(
&mut
self
, elem: i32) {
self
.
0
.push(elem);
    }
}
// and we'll implement IntoIterator
impl
IntoIterator
for
MyCollection {
type
Item = i32;
type
IntoIter = std::vec::IntoIter<
Self
::Item>;
fn
into_iter(
self
) ->
Self
::IntoIter {
self
.
0
.into_iter()
    }
}
// Now we can make a new collection...
let
mut
c = MyCollection::new();
// ... add some stuff to it ...
c.add(
0
);
c.add(
1
);
c.add(
2
);
// ... and then turn it into an Iterator:
for
(i, n)
in
c.into_iter().enumerate() {
assert_eq!
(i
as
i32, n);
}
It is common to use
IntoIterator
as a trait bound. This allows
the input collection type to change, so long as it is still an
iterator. Additional bounds can be specified by restricting on
Item
:
fn
collect_as_strings<T>(collection: T) -> Vec<String>
where
T: IntoIterator,
    T::Item: std::fmt::Debug,
{
    collection
        .into_iter()
        .map(|item|
format!
(
"{item:?}"
))
        .collect()
}
Required Associated Types
§
1.0.0
·
Source
type
Item
The type of the elements being iterated over.
1.0.0
·
Source
type
IntoIter
:
Iterator
<Item = Self::
Item
>
Which kind of iterator are we turning this into?
Required Methods
§
1.0.0
·
Source
fn
into_iter
(self) -> Self::
IntoIter
Creates an iterator from a value.
See the
module-level documentation
for more.
§
Examples
let
v = [
1
,
2
,
3
];
let
mut
iter = v.into_iter();
assert_eq!
(
Some
(
1
), iter.next());
assert_eq!
(
Some
(
2
), iter.next());
assert_eq!
(
Some
(
3
), iter.next());
assert_eq!
(
None
, iter.next());
Implementors
§
1.10.0
·
Source
§
impl<'a>
IntoIterator
for &'a
UnixListener
Available on
Unix
only.
Source
§
type
Item
=
Result
<
UnixStream
,
Error
>
Source
§
type
IntoIter
=
Incoming
<'a>
1.6.0
·
Source
§
impl<'a>
IntoIterator
for &'a
Path
Source
§
type
Item
= &'a
OsStr
Source
§
type
IntoIter
=
Iter
<'a>
1.6.0
·
Source
§
impl<'a>
IntoIterator
for &'a
PathBuf
Source
§
type
Item
= &'a
OsStr
Source
§
type
IntoIter
=
Iter
<'a>
1.80.0
·
Source
§
impl<'a, I, A>
IntoIterator
for &'a
Box
<
[I]
, A>
where
    A:
Allocator
,
Source
§
type
IntoIter
=
Iter
<'a, I>
Source
§
type
Item
=
&'a I
1.80.0
·
Source
§
impl<'a, I, A>
IntoIterator
for &'a mut
Box
<
[I]
, A>
where
    A:
Allocator
,
Source
§
type
IntoIter
=
IterMut
<'a, I>
Source
§
type
Item
=
&'a mut I
1.0.0
·
Source
§
impl<'a, K, V, A>
IntoIterator
for &'a
BTreeMap
<K, V, A>
where
    A:
Allocator
+
Clone
,
Source
§
type
Item
= (
&'a K
,
&'a V
)
Source
§
type
IntoIter
=
Iter
<'a, K, V>
1.0.0
·
Source
§
impl<'a, K, V, A>
IntoIterator
for &'a mut
BTreeMap
<K, V, A>
where
    A:
Allocator
+
Clone
,
Source
§
type
Item
= (
&'a K
,
&'a mut V
)
Source
§
type
IntoIter
=
IterMut
<'a, K, V>
1.0.0
·
Source
§
impl<'a, K, V, S>
IntoIterator
for &'a
HashMap
<K, V, S>
Source
§
type
Item
= (
&'a K
,
&'a V
)
Source
§
type
IntoIter
=
Iter
<'a, K, V>
1.0.0
·
Source
§
impl<'a, K, V, S>
IntoIterator
for &'a mut
HashMap
<K, V, S>
Source
§
type
Item
= (
&'a K
,
&'a mut V
)
Source
§
type
IntoIter
=
IterMut
<'a, K, V>
1.4.0
·
Source
§
impl<'a, T>
IntoIterator
for &'a
Option
<T>
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
IntoIterator
for &'a
[T]
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
Source
§
impl<'a, T>
IntoIterator
for &'a std::sync::mpmc::
Receiver
<T>
Source
§
type
Item
= T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.1.0
·
Source
§
impl<'a, T>
IntoIterator
for &'a std::sync::mpsc::
Receiver
<T>
Source
§
type
Item
= T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.4.0
·
Source
§
impl<'a, T>
IntoIterator
for &'a mut
Option
<T>
Source
§
type
Item
=
&'a mut T
Source
§
type
IntoIter
=
IterMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T>
IntoIterator
for &'a mut
[T]
Source
§
type
Item
=
&'a mut T
Source
§
type
IntoIter
=
IterMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T, A>
IntoIterator
for &'a
BTreeSet
<T, A>
where
    A:
Allocator
+
Clone
,
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T, A>
IntoIterator
for &'a
BinaryHeap
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T, A>
IntoIterator
for &'a
LinkedList
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T, A>
IntoIterator
for &'a
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T, A>
IntoIterator
for &'a
Vec
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T, A>
IntoIterator
for &'a mut
LinkedList
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
=
&'a mut T
Source
§
type
IntoIter
=
IterMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T, A>
IntoIterator
for &'a mut
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
=
&'a mut T
Source
§
type
IntoIter
=
IterMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T, A>
IntoIterator
for &'a mut
Vec
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
=
&'a mut T
Source
§
type
IntoIter
=
IterMut
<'a, T>
1.4.0
·
Source
§
impl<'a, T, E>
IntoIterator
for &'a
Result
<T, E>
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.4.0
·
Source
§
impl<'a, T, E>
IntoIterator
for &'a mut
Result
<T, E>
Source
§
type
Item
=
&'a mut T
Source
§
type
IntoIter
=
IterMut
<'a, T>
1.0.0
·
Source
§
impl<'a, T, S>
IntoIterator
for &'a
HashSet
<T, S>
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T, const N:
usize
>
IntoIterator
for &'a
[T; N]
Source
§
type
Item
=
&'a T
Source
§
type
IntoIter
=
Iter
<'a, T>
1.0.0
·
Source
§
impl<'a, T, const N:
usize
>
IntoIterator
for &'a mut
[T; N]
Source
§
type
Item
=
&'a mut T
Source
§
type
IntoIter
=
IterMut
<'a, T>
Source
§
impl<A>
IntoIterator
for
Range
<A>
where
    A:
Step
,
Source
§
type
Item
= A
Source
§
type
IntoIter
=
IterRange
<A>
Source
§
impl<A>
IntoIterator
for
RangeFrom
<A>
where
    A:
Step
,
Source
§
type
Item
= A
Source
§
type
IntoIter
=
IterRangeFrom
<A>
Source
§
impl<A>
IntoIterator
for
RangeInclusive
<A>
where
    A:
Step
,
Source
§
type
Item
= A
Source
§
type
IntoIter
=
IterRangeInclusive
<A>
1.0.0
·
Source
§
impl<I>
IntoIterator
for I
where
    I:
Iterator
,
Source
§
type
Item
= <I as
Iterator
>::
Item
Source
§
type
IntoIter
= I
1.80.0
·
Source
§
impl<I, A>
IntoIterator
for
Box
<
[I]
, A>
where
    A:
Allocator
,
Source
§
type
IntoIter
=
IntoIter
<I, A>
Source
§
type
Item
= I
1.0.0
·
Source
§
impl<K, V, A>
IntoIterator
for
BTreeMap
<K, V, A>
where
    A:
Allocator
+
Clone
,
Source
§
type
Item
=
(K, V)
Source
§
type
IntoIter
=
IntoIter
<K, V, A>
1.0.0
·
Source
§
impl<K, V, S>
IntoIterator
for
HashMap
<K, V, S>
Source
§
type
Item
=
(K, V)
Source
§
type
IntoIter
=
IntoIter
<K, V>
1.0.0
·
Source
§
impl<T>
IntoIterator
for
Option
<T>
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T>
Source
§
impl<T>
IntoIterator
for std::sync::mpmc::
Receiver
<T>
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T>
1.1.0
·
Source
§
impl<T>
IntoIterator
for std::sync::mpsc::
Receiver
<T>
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T>
1.0.0
·
Source
§
impl<T, A>
IntoIterator
for
BTreeSet
<T, A>
where
    A:
Allocator
+
Clone
,
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T, A>
1.0.0
·
Source
§
impl<T, A>
IntoIterator
for
BinaryHeap
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T, A>
1.0.0
·
Source
§
impl<T, A>
IntoIterator
for
LinkedList
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T, A>
1.0.0
·
Source
§
impl<T, A>
IntoIterator
for
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T, A>
1.0.0
·
Source
§
impl<T, A>
IntoIterator
for
Vec
<T, A>
where
    A:
Allocator
,
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T, A>
1.0.0
·
Source
§
impl<T, E>
IntoIterator
for
Result
<T, E>
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T>
1.0.0
·
Source
§
impl<T, S>
IntoIterator
for
HashSet
<T, S>
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T>
1.53.0
·
Source
§
impl<T, const N:
usize
>
IntoIterator
for
[T; N]
Source
§
type
Item
= T
Source
§
type
IntoIter
=
IntoIter
<T, N>