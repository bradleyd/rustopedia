Extend in std::iter - Rust
std
::
iter
Trait
Extend
Copy item path
1.0.0
·
Source
pub trait Extend<A> {
    // Required method
    fn
extend
<T>(&mut self, iter: T)
where T:
IntoIterator
<Item = A>
;

    // Provided methods
    fn
extend_one
(&mut self, item: A) { ... }
fn
extend_reserve
(&mut self, additional:
usize
) { ... }
}
Expand description
Extend a collection with the contents of an iterator.
Iterators produce a series of values, and collections can also be thought
of as a series of values. The
Extend
trait bridges this gap, allowing you
to extend a collection by including the contents of that iterator. When
extending a collection with an already existing key, that entry is updated
or, in the case of collections that permit multiple entries with equal
keys, that entry is inserted.
§
Examples
Basic usage:
// You can extend a String with some chars:
let
mut
message = String::from(
"The first three letters are: "
);

message.extend(
&
[
'a'
,
'b'
,
'c'
]);
assert_eq!
(
"abc"
,
&
message[
29
..
32
]);
Implementing
Extend
:
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
// since MyCollection has a list of i32s, we implement Extend for i32
impl
Extend<i32>
for
MyCollection {
// This is a bit simpler with the concrete type signature: we can call
    // extend on anything which can be turned into an Iterator which gives
    // us i32s. Because we need i32s to put into MyCollection.
fn
extend<T: IntoIterator<Item=i32>>(
&mut
self
, iter: T) {
// The implementation is very straightforward: loop through the
        // iterator, and add() each element to ourselves.
for
elem
in
iter {
self
.add(elem);
        }
    }
}
let
mut
c = MyCollection::new();

c.add(
5
);
c.add(
6
);
c.add(
7
);
// let's extend our collection with three more numbers
c.extend(
vec!
[
1
,
2
,
3
]);
// we've added these elements onto the end
assert_eq!
(
"MyCollection([5, 6, 7, 1, 2, 3])"
,
format!
(
"{c:?}"
));
Required Methods
§
1.0.0
·
Source
fn
extend
<T>(&mut self, iter: T)
where
    T:
IntoIterator
<Item = A>,
Extends a collection with the contents of an iterator.
As this is the only required method for this trait, the
trait-level
docs
contain more details.
§
Examples
// You can extend a String with some chars:
let
mut
message = String::from(
"abc"
);

message.extend([
'd'
,
'e'
,
'f'
].iter());
assert_eq!
(
"abcdef"
,
&
message);
Provided Methods
§
Source
fn
extend_one
(&mut self, item: A)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
fn
extend_reserve
(&mut self, additional:
usize
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Reserves capacity in a collection for the given number of additional elements.
The default implementation does nothing.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
Source
§
impl
Extend
<
AsciiChar
> for
String
1.0.0
·
Source
§
impl
Extend
<
char
> for
String
1.28.0
·
Source
§
impl
Extend
<
()
> for
()
1.52.0
·
Source
§
impl
Extend
<
OsString
> for
OsString
1.4.0
·
Source
§
impl
Extend
<
String
> for
String
Source
§
impl<'a>
Extend
<&'a
AsciiChar
> for
String
1.2.0
·
Source
§
impl<'a>
Extend
<&'a
char
> for
String
1.0.0
·
Source
§
impl<'a>
Extend
<&'a
str
> for
String
1.52.0
·
Source
§
impl<'a>
Extend
<&'a
OsStr
> for
OsString
1.19.0
·
Source
§
impl<'a>
Extend
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
Extend
<
Cow
<'a,
OsStr
>> for
OsString
1.2.0
·
Source
§
impl<'a, K, V, A>
Extend
<(
&'a K
,
&'a V
)> for
BTreeMap
<K, V, A>
where
    K:
Ord
+
Copy
,
    V:
Copy
,
    A:
Allocator
+
Clone
,
1.4.0
·
Source
§
impl<'a, K, V, S>
Extend
<(
&'a K
,
&'a V
)> for
HashMap
<K, V, S>
where
    K:
Eq
+
Hash
+
Copy
,
    V:
Copy
,
    S:
BuildHasher
,
1.2.0
·
Source
§
impl<'a, T, A>
Extend
<
&'a T
> for
BTreeSet
<T, A>
where
    T: 'a +
Ord
+
Copy
,
    A:
Allocator
+
Clone
,
1.2.0
·
Source
§
impl<'a, T, A>
Extend
<
&'a T
> for
BinaryHeap
<T, A>
where
    T: 'a +
Ord
+
Copy
,
    A:
Allocator
,
1.2.0
·
Source
§
impl<'a, T, A>
Extend
<
&'a T
> for
LinkedList
<T, A>
where
    T: 'a +
Copy
,
    A:
Allocator
,
1.2.0
·
Source
§
impl<'a, T, A>
Extend
<
&'a T
> for
VecDeque
<T, A>
where
    T: 'a +
Copy
,
    A:
Allocator
,
1.2.0
·
Source
§
impl<'a, T, A>
Extend
<
&'a T
> for
Vec
<T, A>
where
    T:
Copy
+ 'a,
    A:
Allocator
,
Extend implementation that copies elements out of references before pushing them onto the Vec.
This implementation is specialized for slice iterators, where it uses
copy_from_slice
to
append the entire slice at once.
1.4.0
·
Source
§
impl<'a, T, S>
Extend
<
&'a T
> for
HashSet
<T, S>
where
    T: 'a +
Eq
+
Hash
+
Copy
,
    S:
BuildHasher
,
1.45.0
·
Source
§
impl<A>
Extend
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
1.56.0
·
Source
§
impl<A, EA>
Extend
<
(A₁, A₂, …, Aₙ)
> for
(EA₁, EA₂, …, EAₙ)
where
    EA:
Extend
<A>,
This trait is implemented for tuples up to twelve items long. The
impl
s for 1- and 3- through 12-ary tuples were stabilized after 2-tuples, in 1.85.0.
1.0.0
·
Source
§
impl<K, V, A>
Extend
<
(K, V)
> for
BTreeMap
<K, V, A>
where
    K:
Ord
,
    A:
Allocator
+
Clone
,
1.0.0
·
Source
§
impl<K, V, S>
Extend
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
,
Inserts all new key-values from the iterator and replaces values with existing
keys with new values returned from the iterator.
1.0.0
·
Source
§
impl<P:
AsRef
<
Path
>>
Extend
<P> for
PathBuf
1.0.0
·
Source
§
impl<T, A>
Extend
<T> for
BTreeSet
<T, A>
where
    T:
Ord
,
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
Extend
<T> for
BinaryHeap
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
Extend
<T> for
LinkedList
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
Extend
<T> for
VecDeque
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
Extend
<T> for
Vec
<T, A>
where
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, S>
Extend
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
,