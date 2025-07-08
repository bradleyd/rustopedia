FromIterator in std::iter - Rust
std
::
iter
Trait
FromIterator
Copy item path
1.0.0
·
Source
pub trait FromIterator<A>:
Sized
{
    // Required method
    fn
from_iter
<T>(iter: T) -> Self
where T:
IntoIterator
<Item = A>
;
}
Expand description
Conversion from an
Iterator
.
By implementing
FromIterator
for a type, you define how it will be
created from an iterator. This is common for types which describe a
collection of some kind.
If you want to create a collection from the contents of an iterator, the
Iterator::collect()
method is preferred. However, when you need to
specify the container type,
FromIterator::from_iter()
can be more
readable than using a turbofish (e.g.
::<Vec<_>>()
). See the
Iterator::collect()
documentation for more examples of its use.
See also:
IntoIterator
.
§
Examples
Basic usage:
let
five_fives = std::iter::repeat(
5
).take(
5
);
let
v = Vec::from_iter(five_fives);
assert_eq!
(v,
vec!
[
5
,
5
,
5
,
5
,
5
]);
Using
Iterator::collect()
to implicitly use
FromIterator
:
let
five_fives = std::iter::repeat(
5
).take(
5
);
let
v: Vec<i32> = five_fives.collect();
assert_eq!
(v,
vec!
[
5
,
5
,
5
,
5
,
5
]);
Using
FromIterator::from_iter()
as a more readable alternative to
Iterator::collect()
:
use
std::collections::VecDeque;
let
first = (
0
..
10
).collect::<VecDeque<i32>>();
let
second = VecDeque::from_iter(
0
..
10
);
assert_eq!
(first, second);
Implementing
FromIterator
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
// and we'll implement FromIterator
impl
FromIterator<i32>
for
MyCollection {
fn
from_iter<I: IntoIterator<Item=i32>>(iter: I) ->
Self
{
let
mut
c = MyCollection::new();
for
i
in
iter {
            c.add(i);
        }

        c
    }
}
// Now we can make a new iterator...
let
iter = (
0
..
5
).into_iter();
// ... and make a MyCollection out of it
let
c = MyCollection::from_iter(iter);
assert_eq!
(c.
0
,
vec!
[
0
,
1
,
2
,
3
,
4
]);
// collect works too!
let
iter = (
0
..
5
).into_iter();
let
c: MyCollection = iter.collect();
assert_eq!
(c.
0
,
vec!
[
0
,
1
,
2
,
3
,
4
]);
Required Methods
§
1.0.0
·
Source
fn
from_iter
<T>(iter: T) -> Self
where
    T:
IntoIterator
<Item = A>,
Creates a value from an iterator.
See the
module-level documentation
for more.
§
Examples
let
five_fives = std::iter::repeat(
5
).take(
5
);
let
v = Vec::from_iter(five_fives);
assert_eq!
(v,
vec!
[
5
,
5
,
5
,
5
,
5
]);
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.80.0
·
Source
§
impl
FromIterator
<
char
> for
Box
<
str
>
Source
§
impl
FromIterator
<
char
> for
ByteString
1.0.0
·
Source
§
impl
FromIterator
<
char
> for
String
Source
§
impl
FromIterator
<
u8
> for
ByteString
1.23.0
·
Source
§
impl
FromIterator
<
()
> for
()
Collapses all unit items from an iterator into one.
This is more useful when combined with higher-level abstractions, like
collecting to a
Result<(), E>
where you only care about errors:
use
std::io::
*
;
let
data =
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
];
let
res:
Result
<()> = data.iter()
    .map(|x|
writeln!
(stdout(),
"{x}"
))
    .collect();
assert!
(res.is_ok());
Source
§
impl
FromIterator
<
ByteString
> for
ByteString
1.52.0
·
Source
§
impl
FromIterator
<
OsString
> for
OsString
1.80.0
·
Source
§
impl
FromIterator
<
String
> for
Box
<
str
>
1.4.0
·
Source
§
impl
FromIterator
<
String
> for
String
1.80.0
·
Source
§
impl<'a>
FromIterator
<&'a
char
> for
Box
<
str
>
1.17.0
·
Source
§
impl<'a>
FromIterator
<&'a
char
> for
String
1.80.0
·
Source
§
impl<'a>
FromIterator
<&'a
str
> for
Box
<
str
>
Source
§
impl<'a>
FromIterator
<&'a
str
> for
ByteString
1.0.0
·
Source
§
impl<'a>
FromIterator
<&'a
str
> for
String
Source
§
impl<'a>
FromIterator
<&'a
ByteStr
> for
ByteString
1.52.0
·
Source
§
impl<'a>
FromIterator
<&'a
OsStr
> for
OsString
Source
§
impl<'a>
FromIterator
<&'a [
u8
]> for
ByteString
1.80.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
str
>> for
Box
<
str
>
1.19.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
str
>> for
String
1.52.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
OsStr
>> for
OsString
1.12.0
·
Source
§
impl<'a>
FromIterator
<
char
> for
Cow
<'a,
str
>
1.12.0
·
Source
§
impl<'a>
FromIterator
<
String
> for
Cow
<'a,
str
>
1.12.0
·
Source
§
impl<'a, 'b>
FromIterator
<&'b
str
> for
Cow
<'a,
str
>
1.0.0
·
Source
§
impl<'a, T>
FromIterator
<T> for
Cow
<'a,
[T]
>
where
    T:
Clone
,
1.80.0
·
Source
§
impl<A>
FromIterator
<
Box
<
str
, A>> for
Box
<
str
>
where
    A:
Allocator
,
1.45.0
·
Source
§
impl<A>
FromIterator
<
Box
<
str
, A>> for
String
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<A, E, V>
FromIterator
<
Result
<A, E>> for
Result
<V, E>
where
    V:
FromIterator
<A>,
1.79.0
·
Source
§
impl<A, EA>
FromIterator
<
(EA₁, EA₂, …, EAₙ)
> for
(A₁, A₂, …, Aₙ)
where
    A:
Default
+
Extend
<EA>,
This implementation turns an iterator of tuples into a tuple of types which implement
Default
and
Extend
.
This is similar to
Iterator::unzip
, but is also composable with other
FromIterator
implementations:
let
string =
"1,2,123,4"
;
// Example given for a 2-tuple, but 1- through 12-tuples are supported
let
(numbers, lengths): (Vec<
_
>, Vec<
_
>) = string
    .split(
','
)
    .map(|s| s.parse().map(|n: u32| (n, s.len())))
    .collect::<
Result
<
_
,
_
>>()
?
;
assert_eq!
(numbers, [
1
,
2
,
123
,
4
]);
assert_eq!
(lengths, [
1
,
1
,
3
,
1
]);
This trait is implemented for tuples up to twelve items long. The
impl
s for 1- and 3- through 12-ary tuples were stabilized after 2-tuples, in 1.85.0.
1.0.0
·
Source
§
impl<A, V>
FromIterator
<
Option
<A>> for
Option
<V>
where
    V:
FromIterator
<A>,
1.32.0
·
Source
§
impl<I>
FromIterator
<I> for
Box
<
[I]
>
1.0.0
·
Source
§
impl<K, V>
FromIterator
<
(K, V)
> for
BTreeMap
<K, V>
where
    K:
Ord
,
1.0.0
·
Source
§
impl<K, V, S>
FromIterator
<
(K, V)
> for
HashMap
<K, V, S>
where
    K:
Eq
+
Hash
,
    S:
BuildHasher
+
Default
,
1.0.0
·
Source
§
impl<P:
AsRef
<
Path
>>
FromIterator
<P> for
PathBuf
1.0.0
·
Source
§
impl<T>
FromIterator
<T> for
BTreeSet
<T>
where
    T:
Ord
,
1.0.0
·
Source
§
impl<T>
FromIterator
<T> for
BinaryHeap
<T>
where
    T:
Ord
,
1.0.0
·
Source
§
impl<T>
FromIterator
<T> for
LinkedList
<T>
1.0.0
·
Source
§
impl<T>
FromIterator
<T> for
VecDeque
<T>
1.37.0
·
Source
§
impl<T>
FromIterator
<T> for
Rc
<
[T]
>
1.37.0
·
Source
§
impl<T>
FromIterator
<T> for
Arc
<
[T]
>
1.0.0
·
Source
§
impl<T>
FromIterator
<T> for
Vec
<T>
Collects an iterator into a Vec, commonly called via
Iterator::collect()
§
Allocation behavior
In general
Vec
does not guarantee any particular growth or allocation strategy.
That also applies to this trait impl.
Note:
This section covers implementation details and is therefore exempt from
stability guarantees.
Vec may use any or none of the following strategies,
depending on the supplied iterator:
preallocate based on
Iterator::size_hint()
and panic if the number of items is outside the provided lower/upper bounds
use an amortized growth strategy similar to
pushing
one item at a time
perform the iteration in-place on the original allocation backing the iterator
The last case warrants some attention. It is an optimization that in many cases reduces peak memory
consumption and improves cache locality. But when big, short-lived allocations are created,
only a small fraction of their items get collected, no further use is made of the spare capacity
and the resulting
Vec
is moved into a longer-lived structure, then this can lead to the large
allocations having their lifetimes unnecessarily extended which can result in increased memory
footprint.
In cases where this is an issue, the excess capacity can be discarded with
Vec::shrink_to()
,
Vec::shrink_to_fit()
or by collecting into
Box<[T]>
instead, which additionally reduces
the size of the long-lived struct.
static
LONG_LIVED: Mutex<Vec<Vec<u16>>> = Mutex::new(Vec::new());
for
i
in
0
..
10
{
let
big_temporary: Vec<u16> = (
0
..
1024
).collect();
// discard most items
let
mut
result: Vec<
_
> = big_temporary.into_iter().filter(|i| i %
100
==
0
).collect();
// without this a lot of unused capacity might be moved into the global
result.shrink_to_fit();
    LONG_LIVED.lock().unwrap().push(result);
}
1.0.0
·
Source
§
impl<T, S>
FromIterator
<T> for
HashSet
<T, S>
where
    T:
Eq
+
Hash
,
    S:
BuildHasher
+
Default
,