BTreeMap in std::collections::btree_map - Rust
std
::
collections
::
btree_map
Struct
BTreeMap
Copy item path
1.0.0
·
Source
pub struct BTreeMap<K, V, A =
Global
>
where
    A:
Allocator
+
Clone
,
{
/* private fields */
}
Expand description
An ordered map based on a
B-Tree
.
B-Trees represent a fundamental compromise between cache-efficiency and actually minimizing
the amount of work performed in a search. In theory, a binary search tree (BST) is the optimal
choice for a sorted map, as a perfectly balanced BST performs the theoretical minimum amount of
comparisons necessary to find an element (log
2
n). However, in practice the way this
is done is
very
inefficient for modern computer architectures. In particular, every element
is stored in its own individually heap-allocated node. This means that every single insertion
triggers a heap-allocation, and every single comparison should be a cache-miss. Since these
are both notably expensive things to do in practice, we are forced to, at the very least,
reconsider the BST strategy.
A B-Tree instead makes each node contain B-1 to 2B-1 elements in a contiguous array. By doing
this, we reduce the number of allocations by a factor of B, and improve cache efficiency in
searches. However, this does mean that searches will have to do
more
comparisons on average.
The precise number of comparisons depends on the node search strategy used. For optimal cache
efficiency, one could search the nodes linearly. For optimal comparisons, one could search
the node using binary search. As a compromise, one could also perform a linear search
that initially only checks every i
th
element for some choice of i.
Currently, our implementation simply performs naive linear search. This provides excellent
performance on
small
nodes of elements which are cheap to compare. However in the future we
would like to further explore choosing the optimal search strategy based on the choice of B,
and possibly other factors. Using linear search, searching for a random element is expected
to take B * log(n) comparisons, which is generally worse than a BST. In practice,
however, performance is excellent.
It is a logic error for a key to be modified in such a way that the key’s ordering relative to
any other key, as determined by the
Ord
trait, changes while it is in the map. This is
normally only possible through
Cell
,
RefCell
, global state, I/O, or unsafe code.
The behavior resulting from such a logic error is not specified, but will be encapsulated to the
BTreeMap
that observed the logic error and not result in undefined behavior. This could
include panics, incorrect results, aborts, memory leaks, and non-termination.
Iterators obtained from functions such as
BTreeMap::iter
,
BTreeMap::into_iter
,
BTreeMap::values
, or
BTreeMap::keys
produce their items in order by key, and take worst-case logarithmic and
amortized constant time per item returned.
§
Examples
use
std::collections::BTreeMap;
// type inference lets us omit an explicit type signature (which
// would be `BTreeMap<&str, &str>` in this example).
let
mut
movie_reviews = BTreeMap::new();
// review some movies.
movie_reviews.insert(
"Office Space"
,
"Deals with real issues in the workplace."
);
movie_reviews.insert(
"Pulp Fiction"
,
"Masterpiece."
);
movie_reviews.insert(
"The Godfather"
,
"Very enjoyable."
);
movie_reviews.insert(
"The Blues Brothers"
,
"Eye lyked it a lot."
);
// check for a specific one.
if
!movie_reviews.contains_key(
"Les Misérables"
) {
println!
(
"We've got {} reviews, but Les Misérables ain't one."
,
             movie_reviews.len());
}
// oops, this review has a lot of spelling mistakes, let's delete it.
movie_reviews.remove(
"The Blues Brothers"
);
// look up the values associated with some keys.
let
to_find = [
"Up!"
,
"Office Space"
];
for
movie
in
&
to_find {
match
movie_reviews.get(movie) {
Some
(review) =>
println!
(
"{movie}: {review}"
),
None
=>
println!
(
"{movie} is unreviewed."
)
    }
}
// Look up the value for a key (will panic if the key is not found).
println!
(
"Movie review: {}"
, movie_reviews[
"Office Space"
]);
// iterate over everything.
for
(movie, review)
in
&
movie_reviews {
println!
(
"{movie}: \"{review}\""
);
}
A
BTreeMap
with a known list of items can be initialized from an array:
use
std::collections::BTreeMap;
let
solar_distance = BTreeMap::from([
    (
"Mercury"
,
0.4
),
    (
"Venus"
,
0.7
),
    (
"Earth"
,
1.0
),
    (
"Mars"
,
1.5
),
]);
BTreeMap
implements an
Entry API
, which allows for complex
methods of getting, setting, updating and removing keys and their values:
use
std::collections::BTreeMap;
// type inference lets us omit an explicit type signature (which
// would be `BTreeMap<&str, u8>` in this example).
let
mut
player_stats = BTreeMap::new();
fn
random_stat_buff() -> u8 {
// could actually return some random value here - let's just return
    // some fixed value for now
42
}
// insert a key only if it doesn't already exist
player_stats.entry(
"health"
).or_insert(
100
);
// insert a key using a function that provides a new value only if it
// doesn't already exist
player_stats.entry(
"defence"
).or_insert_with(random_stat_buff);
// update a key, guarding against the key possibly not being set
let
stat = player_stats.entry(
"attack"
).or_insert(
100
);
*
stat += random_stat_buff();
// modify an entry before an insert with in-place mutation
player_stats.entry(
"mana"
).and_modify(|mana|
*
mana +=
200
).or_insert(
100
);
Implementations
§
Source
§
impl<K, V>
BTreeMap
<K, V>
1.0.0 (const: 1.66.0)
·
Source
pub const fn
new
() ->
BTreeMap
<K, V>
Makes a new, empty
BTreeMap
.
Does not allocate anything on its own.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
// entries can now be inserted into the empty map
map.insert(
1
,
"a"
);
Source
§
impl<K, V, A>
BTreeMap
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
pub fn
clear
(&mut self)
Clears the map, removing all elements.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
a.insert(
1
,
"a"
);
a.clear();
assert!
(a.is_empty());
Source
pub const fn
new_in
(alloc: A) ->
BTreeMap
<K, V, A>
🔬
This is a nightly-only experimental API. (
btreemap_alloc
#32838
)
Makes a new empty BTreeMap with a reasonable choice for B.
§
Examples
use
std::collections::BTreeMap;
use
std::alloc::Global;
let
mut
map = BTreeMap::new_in(Global);
// entries can now be inserted into the empty map
map.insert(
1
,
"a"
);
Source
§
impl<K, V, A>
BTreeMap
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
pub fn
get
<Q>(&self, key:
&Q
) ->
Option
<
&V
>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Returns a reference to the value corresponding to the key.
The key may be any borrowed form of the map’s key type, but the ordering
on the borrowed form
must
match the ordering on the key type.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
assert_eq!
(map.get(
&
1
),
Some
(
&
"a"
));
assert_eq!
(map.get(
&
2
),
None
);
1.40.0
·
Source
pub fn
get_key_value
<Q>(&self, k:
&Q
) ->
Option
<(
&K
,
&V
)>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Returns the key-value pair corresponding to the supplied key. This is
potentially useful:
for key types where non-identical keys can be considered equal;
for getting the
&K
stored key value from a borrowed
&Q
lookup key; or
for getting a reference to a key with the same lifetime as the collection.
The supplied key may be any borrowed form of the map’s key type, but the ordering
on the borrowed form
must
match the ordering on the key type.
§
Examples
use
std::cmp::Ordering;
use
std::collections::BTreeMap;
#[derive(Clone, Copy, Debug)]
struct
S {
    id: u32,
    name:
&
'static
str,
// ignored by equality and ordering operations
}
impl
PartialEq
for
S {
fn
eq(
&
self
, other:
&
S) -> bool {
self
.id == other.id
    }
}
impl
Eq
for
S {}
impl
PartialOrd
for
S {
fn
partial_cmp(
&
self
, other:
&
S) ->
Option
<Ordering> {
self
.id.partial_cmp(
&
other.id)
    }
}
impl
Ord
for
S {
fn
cmp(
&
self
, other:
&
S) -> Ordering {
self
.id.cmp(
&
other.id)
    }
}
let
j_a = S { id:
1
, name:
"Jessica"
};
let
j_b = S { id:
1
, name:
"Jess"
};
let
p = S { id:
2
, name:
"Paul"
};
assert_eq!
(j_a, j_b);
let
mut
map = BTreeMap::new();
map.insert(j_a,
"Paris"
);
assert_eq!
(map.get_key_value(
&
j_a),
Some
((
&
j_a,
&
"Paris"
)));
assert_eq!
(map.get_key_value(
&
j_b),
Some
((
&
j_a,
&
"Paris"
)));
// the notable case
assert_eq!
(map.get_key_value(
&
p),
None
);
1.66.0
·
Source
pub fn
first_key_value
(&self) ->
Option
<(
&K
,
&V
)>
where
    K:
Ord
,
Returns the first key-value pair in the map.
The key in this pair is the minimum key in the map.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
assert_eq!
(map.first_key_value(),
None
);
map.insert(
1
,
"b"
);
map.insert(
2
,
"a"
);
assert_eq!
(map.first_key_value(),
Some
((
&
1
,
&
"b"
)));
1.66.0
·
Source
pub fn
first_entry
(&mut self) ->
Option
<
OccupiedEntry
<'_, K, V, A>>
where
    K:
Ord
,
Returns the first entry in the map for in-place manipulation.
The key of this entry is the minimum key in the map.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
map.insert(
2
,
"b"
);
if let
Some
(
mut
entry) = map.first_entry() {
if
*
entry.key() >
0
{
        entry.insert(
"first"
);
    }
}
assert_eq!
(
*
map.get(
&
1
).unwrap(),
"first"
);
assert_eq!
(
*
map.get(
&
2
).unwrap(),
"b"
);
1.66.0
·
Source
pub fn
pop_first
(&mut self) ->
Option
<
(K, V)
>
where
    K:
Ord
,
Removes and returns the first element in the map.
The key of this element is the minimum key that was in the map.
§
Examples
Draining elements in ascending order, while keeping a usable map each iteration.
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
map.insert(
2
,
"b"
);
while let
Some
((key, _val)) = map.pop_first() {
assert!
(map.iter().all(|(k, _v)|
*
k > key));
}
assert!
(map.is_empty());
1.66.0
·
Source
pub fn
last_key_value
(&self) ->
Option
<(
&K
,
&V
)>
where
    K:
Ord
,
Returns the last key-value pair in the map.
The key in this pair is the maximum key in the map.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"b"
);
map.insert(
2
,
"a"
);
assert_eq!
(map.last_key_value(),
Some
((
&
2
,
&
"a"
)));
1.66.0
·
Source
pub fn
last_entry
(&mut self) ->
Option
<
OccupiedEntry
<'_, K, V, A>>
where
    K:
Ord
,
Returns the last entry in the map for in-place manipulation.
The key of this entry is the maximum key in the map.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
map.insert(
2
,
"b"
);
if let
Some
(
mut
entry) = map.last_entry() {
if
*
entry.key() >
0
{
        entry.insert(
"last"
);
    }
}
assert_eq!
(
*
map.get(
&
1
).unwrap(),
"a"
);
assert_eq!
(
*
map.get(
&
2
).unwrap(),
"last"
);
1.66.0
·
Source
pub fn
pop_last
(&mut self) ->
Option
<
(K, V)
>
where
    K:
Ord
,
Removes and returns the last element in the map.
The key of this element is the maximum key that was in the map.
§
Examples
Draining elements in descending order, while keeping a usable map each iteration.
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
map.insert(
2
,
"b"
);
while let
Some
((key, _val)) = map.pop_last() {
assert!
(map.iter().all(|(k, _v)|
*
k < key));
}
assert!
(map.is_empty());
1.0.0
·
Source
pub fn
contains_key
<Q>(&self, key:
&Q
) ->
bool
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Returns
true
if the map contains a value for the specified key.
The key may be any borrowed form of the map’s key type, but the ordering
on the borrowed form
must
match the ordering on the key type.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
assert_eq!
(map.contains_key(
&
1
),
true
);
assert_eq!
(map.contains_key(
&
2
),
false
);
1.0.0
·
Source
pub fn
get_mut
<Q>(&mut self, key:
&Q
) ->
Option
<
&mut V
>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Returns a mutable reference to the value corresponding to the key.
The key may be any borrowed form of the map’s key type, but the ordering
on the borrowed form
must
match the ordering on the key type.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
if let
Some
(x) = map.get_mut(
&
1
) {
*
x =
"b"
;
}
assert_eq!
(map[
&
1
],
"b"
);
1.0.0
·
Source
pub fn
insert
(&mut self, key: K, value: V) ->
Option
<V>
where
    K:
Ord
,
Inserts a key-value pair into the map.
If the map did not have this key present,
None
is returned.
If the map did have this key present, the value is updated, and the old
value is returned. The key is not updated, though; this matters for
types that can be
==
without being identical. See the
module-level
documentation
for more.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
assert_eq!
(map.insert(
37
,
"a"
),
None
);
assert_eq!
(map.is_empty(),
false
);

map.insert(
37
,
"b"
);
assert_eq!
(map.insert(
37
,
"c"
),
Some
(
"b"
));
assert_eq!
(map[
&
37
],
"c"
);
Source
pub fn
try_insert
(
    &mut self,
    key: K,
    value: V,
) ->
Result
<
&mut V
,
OccupiedError
<'_, K, V, A>>
where
    K:
Ord
,
🔬
This is a nightly-only experimental API. (
map_try_insert
#82766
)
Tries to insert a key-value pair into the map, and returns
a mutable reference to the value in the entry.
If the map already had this key present, nothing is updated, and
an error containing the occupied entry and the value is returned.
§
Examples
#![feature(map_try_insert)]
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
assert_eq!
(map.try_insert(
37
,
"a"
).unwrap(),
&
"a"
);
let
err = map.try_insert(
37
,
"b"
).unwrap_err();
assert_eq!
(err.entry.key(),
&
37
);
assert_eq!
(err.entry.get(),
&
"a"
);
assert_eq!
(err.value,
"b"
);
1.0.0
·
Source
pub fn
remove
<Q>(&mut self, key:
&Q
) ->
Option
<V>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Removes a key from the map, returning the value at the key if the key
was previously in the map.
The key may be any borrowed form of the map’s key type, but the ordering
on the borrowed form
must
match the ordering on the key type.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
assert_eq!
(map.remove(
&
1
),
Some
(
"a"
));
assert_eq!
(map.remove(
&
1
),
None
);
1.45.0
·
Source
pub fn
remove_entry
<Q>(&mut self, key:
&Q
) ->
Option
<
(K, V)
>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Removes a key from the map, returning the stored key and value if the key
was previously in the map.
The key may be any borrowed form of the map’s key type, but the ordering
on the borrowed form
must
match the ordering on the key type.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
1
,
"a"
);
assert_eq!
(map.remove_entry(
&
1
),
Some
((
1
,
"a"
)));
assert_eq!
(map.remove_entry(
&
1
),
None
);
1.53.0
·
Source
pub fn
retain
<F>(&mut self, f: F)
where
    K:
Ord
,
    F:
FnMut
(
&K
,
&mut V
) ->
bool
,
Retains only the elements specified by the predicate.
In other words, remove all pairs
(k, v)
for which
f(&k, &mut v)
returns
false
.
The elements are visited in ascending key order.
§
Examples
use
std::collections::BTreeMap;
let
mut
map: BTreeMap<i32, i32> = (
0
..
8
).map(|x| (x, x
*
10
)).collect();
// Keep only the elements with even-numbered keys.
map.retain(|
&
k,
_
| k %
2
==
0
);
assert!
(map.into_iter().eq(
vec!
[(
0
,
0
), (
2
,
20
), (
4
,
40
), (
6
,
60
)]));
1.11.0
·
Source
pub fn
append
(&mut self, other: &mut
BTreeMap
<K, V, A>)
where
    K:
Ord
,
    A:
Clone
,
Moves all elements from
other
into
self
, leaving
other
empty.
If a key from
other
is already present in
self
, the respective
value from
self
will be overwritten with the respective value from
other
.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
a.insert(
1
,
"a"
);
a.insert(
2
,
"b"
);
a.insert(
3
,
"c"
);
// Note: Key (3) also present in b.
let
mut
b = BTreeMap::new();
b.insert(
3
,
"d"
);
// Note: Key (3) also present in a.
b.insert(
4
,
"e"
);
b.insert(
5
,
"f"
);

a.append(
&mut
b);
assert_eq!
(a.len(),
5
);
assert_eq!
(b.len(),
0
);
assert_eq!
(a[
&
1
],
"a"
);
assert_eq!
(a[
&
2
],
"b"
);
assert_eq!
(a[
&
3
],
"d"
);
// Note: "c" has been overwritten.
assert_eq!
(a[
&
4
],
"e"
);
assert_eq!
(a[
&
5
],
"f"
);
1.17.0
·
Source
pub fn
range
<T, R>(&self, range: R) ->
Range
<'_, K, V>
ⓘ
where
    T:
Ord
+ ?
Sized
,
    K:
Borrow
<T> +
Ord
,
    R:
RangeBounds
<T>,
Constructs a double-ended iterator over a sub-range of elements in the map.
The simplest way is to use the range syntax
min..max
, thus
range(min..max)
will
yield elements from min (inclusive) to max (exclusive).
The range may also be entered as
(Bound<T>, Bound<T>)
, so for example
range((Excluded(4), Included(10)))
will yield a left-exclusive, right-inclusive
range from 4 to 10.
§
Panics
Panics if range
start > end
.
Panics if range
start == end
and both bounds are
Excluded
.
§
Examples
use
std::collections::BTreeMap;
use
std::ops::Bound::Included;
let
mut
map = BTreeMap::new();
map.insert(
3
,
"a"
);
map.insert(
5
,
"b"
);
map.insert(
8
,
"c"
);
for
(
&
key,
&
value)
in
map.range((Included(
&
4
), Included(
&
8
))) {
println!
(
"{key}: {value}"
);
}
assert_eq!
(
Some
((
&
5
,
&
"b"
)), map.range(
4
..).next());
1.17.0
·
Source
pub fn
range_mut
<T, R>(&mut self, range: R) ->
RangeMut
<'_, K, V>
ⓘ
where
    T:
Ord
+ ?
Sized
,
    K:
Borrow
<T> +
Ord
,
    R:
RangeBounds
<T>,
Constructs a mutable double-ended iterator over a sub-range of elements in the map.
The simplest way is to use the range syntax
min..max
, thus
range(min..max)
will
yield elements from min (inclusive) to max (exclusive).
The range may also be entered as
(Bound<T>, Bound<T>)
, so for example
range((Excluded(4), Included(10)))
will yield a left-exclusive, right-inclusive
range from 4 to 10.
§
Panics
Panics if range
start > end
.
Panics if range
start == end
and both bounds are
Excluded
.
§
Examples
use
std::collections::BTreeMap;
let
mut
map: BTreeMap<
&
str, i32> =
    [(
"Alice"
,
0
), (
"Bob"
,
0
), (
"Carol"
,
0
), (
"Cheryl"
,
0
)].into();
for
(
_
, balance)
in
map.range_mut(
"B"
..
"Cheryl"
) {
*
balance +=
100
;
}
for
(name, balance)
in
&
map {
println!
(
"{name} => {balance}"
);
}
1.0.0
·
Source
pub fn
entry
(&mut self, key: K) ->
Entry
<'_, K, V, A>
where
    K:
Ord
,
Gets the given key’s corresponding entry in the map for in-place manipulation.
§
Examples
use
std::collections::BTreeMap;
let
mut
count: BTreeMap<
&
str, usize> = BTreeMap::new();
// count the number of occurrences of letters in the vec
for
x
in
[
"a"
,
"b"
,
"a"
,
"c"
,
"a"
,
"b"
] {
    count.entry(x).and_modify(|curr|
*
curr +=
1
).or_insert(
1
);
}
assert_eq!
(count[
"a"
],
3
);
assert_eq!
(count[
"b"
],
2
);
assert_eq!
(count[
"c"
],
1
);
1.11.0
·
Source
pub fn
split_off
<Q>(&mut self, key:
&Q
) ->
BTreeMap
<K, V, A>
where
    Q:
Ord
+ ?
Sized
,
    K:
Borrow
<Q> +
Ord
,
    A:
Clone
,
Splits the collection into two at the given key. Returns everything after the given key,
including the key.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
a.insert(
1
,
"a"
);
a.insert(
2
,
"b"
);
a.insert(
3
,
"c"
);
a.insert(
17
,
"d"
);
a.insert(
41
,
"e"
);
let
b = a.split_off(
&
3
);
assert_eq!
(a.len(),
2
);
assert_eq!
(b.len(),
3
);
assert_eq!
(a[
&
1
],
"a"
);
assert_eq!
(a[
&
2
],
"b"
);
assert_eq!
(b[
&
3
],
"c"
);
assert_eq!
(b[
&
17
],
"d"
);
assert_eq!
(b[
&
41
],
"e"
);
Source
pub fn
extract_if
<F>(&mut self, pred: F) ->
ExtractIf
<'_, K, V, F, A>
ⓘ
where
    K:
Ord
,
    F:
FnMut
(
&K
,
&mut V
) ->
bool
,
🔬
This is a nightly-only experimental API. (
btree_extract_if
#70530
)
Creates an iterator that visits all elements (key-value pairs) in
ascending key order and uses a closure to determine if an element should
be removed. If the closure returns
true
, the element is removed from
the map and yielded. If the closure returns
false
, or panics, the
element remains in the map and will not be yielded.
The iterator also lets you mutate the value of each element in the
closure, regardless of whether you choose to keep or remove it.
If the returned
ExtractIf
is not exhausted, e.g. because it is dropped without iterating
or the iteration short-circuits, then the remaining elements will be retained.
Use
retain
with a negated predicate if you do not need the returned iterator.
§
Examples
Splitting a map into even and odd keys, reusing the original map:
#![feature(btree_extract_if)]
use
std::collections::BTreeMap;
let
mut
map: BTreeMap<i32, i32> = (
0
..
8
).map(|x| (x, x)).collect();
let
evens: BTreeMap<
_
,
_
> = map.extract_if(|k, _v| k %
2
==
0
).collect();
let
odds = map;
assert_eq!
(evens.keys().copied().collect::<Vec<
_
>>(), [
0
,
2
,
4
,
6
]);
assert_eq!
(odds.keys().copied().collect::<Vec<
_
>>(), [
1
,
3
,
5
,
7
]);
1.54.0
·
Source
pub fn
into_keys
(self) ->
IntoKeys
<K, V, A>
ⓘ
Creates a consuming iterator visiting all the keys, in sorted order.
The map cannot be used after calling this.
The iterator element type is
K
.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
a.insert(
2
,
"b"
);
a.insert(
1
,
"a"
);
let
keys: Vec<i32> = a.into_keys().collect();
assert_eq!
(keys, [
1
,
2
]);
1.54.0
·
Source
pub fn
into_values
(self) ->
IntoValues
<K, V, A>
ⓘ
Creates a consuming iterator visiting all the values, in order by key.
The map cannot be used after calling this.
The iterator element type is
V
.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
a.insert(
1
,
"hello"
);
a.insert(
2
,
"goodbye"
);
let
values: Vec<
&
str> = a.into_values().collect();
assert_eq!
(values, [
"hello"
,
"goodbye"
]);
Source
§
impl<K, V, A>
BTreeMap
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
pub fn
iter
(&self) ->
Iter
<'_, K, V>
ⓘ
Gets an iterator over the entries of the map, sorted by key.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::new();
map.insert(
3
,
"c"
);
map.insert(
2
,
"b"
);
map.insert(
1
,
"a"
);
for
(key, value)
in
map.iter() {
println!
(
"{key}: {value}"
);
}
let
(first_key, first_value) = map.iter().next().unwrap();
assert_eq!
((
*
first_key,
*
first_value), (
1
,
"a"
));
1.0.0
·
Source
pub fn
iter_mut
(&mut self) ->
IterMut
<'_, K, V>
ⓘ
Gets a mutable iterator over the entries of the map, sorted by key.
§
Examples
use
std::collections::BTreeMap;
let
mut
map = BTreeMap::from([
   (
"a"
,
1
),
   (
"b"
,
2
),
   (
"c"
,
3
),
]);
// add 10 to the value if the key isn't "a"
for
(key, value)
in
map.iter_mut() {
if
key !=
&
"a"
{
*
value +=
10
;
    }
}
1.0.0
·
Source
pub fn
keys
(&self) ->
Keys
<'_, K, V>
ⓘ
Gets an iterator over the keys of the map, in sorted order.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
a.insert(
2
,
"b"
);
a.insert(
1
,
"a"
);
let
keys: Vec<
_
> = a.keys().cloned().collect();
assert_eq!
(keys, [
1
,
2
]);
1.0.0
·
Source
pub fn
values
(&self) ->
Values
<'_, K, V>
ⓘ
Gets an iterator over the values of the map, in order by key.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
a.insert(
1
,
"hello"
);
a.insert(
2
,
"goodbye"
);
let
values: Vec<
&
str> = a.values().cloned().collect();
assert_eq!
(values, [
"hello"
,
"goodbye"
]);
1.10.0
·
Source
pub fn
values_mut
(&mut self) ->
ValuesMut
<'_, K, V>
ⓘ
Gets a mutable iterator over the values of the map, in order by key.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
a.insert(
1
, String::from(
"hello"
));
a.insert(
2
, String::from(
"goodbye"
));
for
value
in
a.values_mut() {
    value.push_str(
"!"
);
}
let
values: Vec<String> = a.values().cloned().collect();
assert_eq!
(values, [String::from(
"hello!"
),
                    String::from(
"goodbye!"
)]);
1.0.0 (const:
unstable
)
·
Source
pub fn
len
(&self) ->
usize
Returns the number of elements in the map.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
assert_eq!
(a.len(),
0
);
a.insert(
1
,
"a"
);
assert_eq!
(a.len(),
1
);
1.0.0 (const:
unstable
)
·
Source
pub fn
is_empty
(&self) ->
bool
Returns
true
if the map contains no elements.
§
Examples
use
std::collections::BTreeMap;
let
mut
a = BTreeMap::new();
assert!
(a.is_empty());
a.insert(
1
,
"a"
);
assert!
(!a.is_empty());
Source
pub fn
lower_bound
<Q>(&self, bound:
Bound
<
&Q
>) ->
Cursor
<'_, K, V>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Returns a
Cursor
pointing at the gap before the smallest key
greater than the given bound.
Passing
Bound::Included(x)
will return a cursor pointing to the
gap before the smallest key greater than or equal to
x
.
Passing
Bound::Excluded(x)
will return a cursor pointing to the
gap before the smallest key greater than
x
.
Passing
Bound::Unbounded
will return a cursor pointing to the
gap before the smallest key in the map.
§
Examples
#![feature(btree_cursors)]
use
std::collections::BTreeMap;
use
std::ops::Bound;
let
map = BTreeMap::from([
    (
1
,
"a"
),
    (
2
,
"b"
),
    (
3
,
"c"
),
    (
4
,
"d"
),
]);
let
cursor = map.lower_bound(Bound::Included(
&
2
));
assert_eq!
(cursor.peek_prev(),
Some
((
&
1
,
&
"a"
)));
assert_eq!
(cursor.peek_next(),
Some
((
&
2
,
&
"b"
)));
let
cursor = map.lower_bound(Bound::Excluded(
&
2
));
assert_eq!
(cursor.peek_prev(),
Some
((
&
2
,
&
"b"
)));
assert_eq!
(cursor.peek_next(),
Some
((
&
3
,
&
"c"
)));
let
cursor = map.lower_bound(Bound::Unbounded);
assert_eq!
(cursor.peek_prev(),
None
);
assert_eq!
(cursor.peek_next(),
Some
((
&
1
,
&
"a"
)));
Source
pub fn
lower_bound_mut
<Q>(&mut self, bound:
Bound
<
&Q
>) ->
CursorMut
<'_, K, V, A>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Returns a
CursorMut
pointing at the gap before the smallest key
greater than the given bound.
Passing
Bound::Included(x)
will return a cursor pointing to the
gap before the smallest key greater than or equal to
x
.
Passing
Bound::Excluded(x)
will return a cursor pointing to the
gap before the smallest key greater than
x
.
Passing
Bound::Unbounded
will return a cursor pointing to the
gap before the smallest key in the map.
§
Examples
#![feature(btree_cursors)]
use
std::collections::BTreeMap;
use
std::ops::Bound;
let
mut
map = BTreeMap::from([
    (
1
,
"a"
),
    (
2
,
"b"
),
    (
3
,
"c"
),
    (
4
,
"d"
),
]);
let
mut
cursor = map.lower_bound_mut(Bound::Included(
&
2
));
assert_eq!
(cursor.peek_prev(),
Some
((
&
1
,
&mut
"a"
)));
assert_eq!
(cursor.peek_next(),
Some
((
&
2
,
&mut
"b"
)));
let
mut
cursor = map.lower_bound_mut(Bound::Excluded(
&
2
));
assert_eq!
(cursor.peek_prev(),
Some
((
&
2
,
&mut
"b"
)));
assert_eq!
(cursor.peek_next(),
Some
((
&
3
,
&mut
"c"
)));
let
mut
cursor = map.lower_bound_mut(Bound::Unbounded);
assert_eq!
(cursor.peek_prev(),
None
);
assert_eq!
(cursor.peek_next(),
Some
((
&
1
,
&mut
"a"
)));
Source
pub fn
upper_bound
<Q>(&self, bound:
Bound
<
&Q
>) ->
Cursor
<'_, K, V>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Returns a
Cursor
pointing at the gap after the greatest key
smaller than the given bound.
Passing
Bound::Included(x)
will return a cursor pointing to the
gap after the greatest key smaller than or equal to
x
.
Passing
Bound::Excluded(x)
will return a cursor pointing to the
gap after the greatest key smaller than
x
.
Passing
Bound::Unbounded
will return a cursor pointing to the
gap after the greatest key in the map.
§
Examples
#![feature(btree_cursors)]
use
std::collections::BTreeMap;
use
std::ops::Bound;
let
map = BTreeMap::from([
    (
1
,
"a"
),
    (
2
,
"b"
),
    (
3
,
"c"
),
    (
4
,
"d"
),
]);
let
cursor = map.upper_bound(Bound::Included(
&
3
));
assert_eq!
(cursor.peek_prev(),
Some
((
&
3
,
&
"c"
)));
assert_eq!
(cursor.peek_next(),
Some
((
&
4
,
&
"d"
)));
let
cursor = map.upper_bound(Bound::Excluded(
&
3
));
assert_eq!
(cursor.peek_prev(),
Some
((
&
2
,
&
"b"
)));
assert_eq!
(cursor.peek_next(),
Some
((
&
3
,
&
"c"
)));
let
cursor = map.upper_bound(Bound::Unbounded);
assert_eq!
(cursor.peek_prev(),
Some
((
&
4
,
&
"d"
)));
assert_eq!
(cursor.peek_next(),
None
);
Source
pub fn
upper_bound_mut
<Q>(&mut self, bound:
Bound
<
&Q
>) ->
CursorMut
<'_, K, V, A>
where
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Returns a
CursorMut
pointing at the gap after the greatest key
smaller than the given bound.
Passing
Bound::Included(x)
will return a cursor pointing to the
gap after the greatest key smaller than or equal to
x
.
Passing
Bound::Excluded(x)
will return a cursor pointing to the
gap after the greatest key smaller than
x
.
Passing
Bound::Unbounded
will return a cursor pointing to the
gap after the greatest key in the map.
§
Examples
#![feature(btree_cursors)]
use
std::collections::BTreeMap;
use
std::ops::Bound;
let
mut
map = BTreeMap::from([
    (
1
,
"a"
),
    (
2
,
"b"
),
    (
3
,
"c"
),
    (
4
,
"d"
),
]);
let
mut
cursor = map.upper_bound_mut(Bound::Included(
&
3
));
assert_eq!
(cursor.peek_prev(),
Some
((
&
3
,
&mut
"c"
)));
assert_eq!
(cursor.peek_next(),
Some
((
&
4
,
&mut
"d"
)));
let
mut
cursor = map.upper_bound_mut(Bound::Excluded(
&
3
));
assert_eq!
(cursor.peek_prev(),
Some
((
&
2
,
&mut
"b"
)));
assert_eq!
(cursor.peek_next(),
Some
((
&
3
,
&mut
"c"
)));
let
mut
cursor = map.upper_bound_mut(Bound::Unbounded);
assert_eq!
(cursor.peek_prev(),
Some
((
&
4
,
&mut
"d"
)));
assert_eq!
(cursor.peek_next(),
None
);
Trait Implementations
§
1.0.0
·
Source
§
impl<K, V, A>
Clone
for
BTreeMap
<K, V, A>
where
    K:
Clone
,
    V:
Clone
,
    A:
Allocator
+
Clone
,
Source
§
fn
clone
(&self) ->
BTreeMap
<K, V, A>
Returns a copy of the value.
Read more
1.0.0
·
Source
§
fn
clone_from
(&mut self, source: &Self)
Performs copy-assignment from
source
.
Read more
1.0.0
·
Source
§
impl<K, V, A>
Debug
for
BTreeMap
<K, V, A>
where
    K:
Debug
,
    V:
Debug
,
    A:
Allocator
+
Clone
,
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
<
()
,
Error
>
Formats the value using the given formatter.
Read more
1.0.0
·
Source
§
impl<K, V>
Default
for
BTreeMap
<K, V>
Source
§
fn
default
() ->
BTreeMap
<K, V>
Creates an empty
BTreeMap
.
1.7.0
·
Source
§
impl<K, V, A>
Drop
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
fn
drop
(&mut self)
Executes the destructor for this type.
Read more
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
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item = (
&'a K
,
&'a V
)>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, _: (
&'a K
,
&'a V
))
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
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
Read more
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
Source
§
fn
extend
<T>(&mut self, iter: T)
where
    T:
IntoIterator
<Item =
(K, V)
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, _:
(K, V)
)
🔬
This is a nightly-only experimental API. (
extend_one
#72631
)
Extends a collection with exactly one element.
Source
§
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
Read more
1.56.0
·
Source
§
impl<K, V, const N:
usize
>
From
<[
(K, V)
;
N
]> for
BTreeMap
<K, V>
where
    K:
Ord
,
Source
§
fn
from
(arr: [
(K, V)
;
N
]) ->
BTreeMap
<K, V>
Converts a
[(K, V); N]
into a
BTreeMap<K, V>
.
If any entries in the array have equal keys,
all but one of the corresponding values will be dropped.
use
std::collections::BTreeMap;
let
map1 = BTreeMap::from([(
1
,
2
), (
3
,
4
)]);
let
map2: BTreeMap<
_
,
_
> = [(
1
,
2
), (
3
,
4
)].into();
assert_eq!
(map1, map2);
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
Source
§
fn
from_iter
<T>(iter: T) ->
BTreeMap
<K, V>
where
    T:
IntoIterator
<Item =
(K, V)
>,
Constructs a
BTreeMap<K, V>
from an iterator of key-value pairs.
If the iterator produces any pairs with equal keys,
all but one of the corresponding values will be dropped.
1.0.0
·
Source
§
impl<K, V, A>
Hash
for
BTreeMap
<K, V, A>
where
    K:
Hash
,
    V:
Hash
,
    A:
Allocator
+
Clone
,
Source
§
fn
hash
<H>(&self, state:
&mut H
)
where
    H:
Hasher
,
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
1.0.0
·
Source
§
impl<K, Q, V, A>
Index
<
&Q
> for
BTreeMap
<K, V, A>
where
    A:
Allocator
+
Clone
,
    K:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Source
§
fn
index
(&self, key:
&Q
) ->
&V
Returns a reference to the value corresponding to the supplied key.
§
Panics
Panics if the key is not present in the
BTreeMap
.
Source
§
type
Output
= V
The returned type after indexing.
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
The type of the elements being iterated over.
Source
§
type
IntoIter
=
Iter
<'a, K, V>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
Iter
<'a, K, V>
ⓘ
Creates an iterator from a value.
Read more
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
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IterMut
<'a, K, V>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
IterMut
<'a, K, V>
ⓘ
Creates an iterator from a value.
Read more
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
fn
into_iter
(self) ->
IntoIter
<K, V, A>
ⓘ
Gets an owning iterator over the entries of the map, sorted by key.
Source
§
type
Item
=
(K, V)
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IntoIter
<K, V, A>
Which kind of iterator are we turning this into?
1.0.0
·
Source
§
impl<K, V, A>
Ord
for
BTreeMap
<K, V, A>
where
    K:
Ord
,
    V:
Ord
,
    A:
Allocator
+
Clone
,
Source
§
fn
cmp
(&self, other: &
BTreeMap
<K, V, A>) ->
Ordering
This method returns an
Ordering
between
self
and
other
.
Read more
1.21.0
·
Source
§
fn
max
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the maximum of two values.
Read more
1.21.0
·
Source
§
fn
min
(self, other: Self) -> Self
where
    Self:
Sized
,
Compares and returns the minimum of two values.
Read more
1.50.0
·
Source
§
fn
clamp
(self, min: Self, max: Self) -> Self
where
    Self:
Sized
,
Restrict a value to a certain interval.
Read more
1.0.0
·
Source
§
impl<K, V, A>
PartialEq
for
BTreeMap
<K, V, A>
where
    K:
PartialEq
,
    V:
PartialEq
,
    A:
Allocator
+
Clone
,
Source
§
fn
eq
(&self, other: &
BTreeMap
<K, V, A>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
1.0.0
·
Source
§
fn
ne
(&self, other:
&Rhs
) ->
bool
Tests for
!=
. The default implementation is almost always sufficient,
and should not be overridden without very good reason.
1.0.0
·
Source
§
impl<K, V, A>
PartialOrd
for
BTreeMap
<K, V, A>
where
    K:
PartialOrd
,
    V:
PartialOrd
,
    A:
Allocator
+
Clone
,
Source
§
fn
partial_cmp
(&self, other: &
BTreeMap
<K, V, A>) ->
Option
<
Ordering
>
This method returns an ordering between
self
and
other
values if one exists.
Read more
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
bool
Tests greater than (for
self
and
other
) and is used by the
>
operator.
Read more
1.0.0
·
Source
§
fn
ge
(&self, other:
&Rhs
) ->
bool
Tests greater than or equal to (for
self
and
other
) and is used by
the
>=
operator.
Read more
1.0.0
·
Source
§
impl<K, V, A>
Eq
for
BTreeMap
<K, V, A>
where
    K:
Eq
,
    V:
Eq
,
    A:
Allocator
+
Clone
,
1.64.0
·
Source
§
impl<K, V, A>
UnwindSafe
for
BTreeMap
<K, V, A>
where
    A:
Allocator
+
Clone
+
UnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
Auto Trait Implementations
§
§
impl<K, V, A>
Freeze
for
BTreeMap
<K, V, A>
where
    A:
Freeze
,
§
impl<K, V, A>
RefUnwindSafe
for
BTreeMap
<K, V, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
    V:
RefUnwindSafe
,
§
impl<K, V, A>
Send
for
BTreeMap
<K, V, A>
where
    A:
Send
,
    K:
Send
,
    V:
Send
,
§
impl<K, V, A>
Sync
for
BTreeMap
<K, V, A>
where
    A:
Sync
,
    K:
Sync
,
    V:
Sync
,
§
impl<K, V, A>
Unpin
for
BTreeMap
<K, V, A>
where
    A:
Unpin
,
Blanket Implementations
§
Source
§
impl<T>
Any
for T
where
    T: 'static + ?
Sized
,
Source
§
fn
type_id
(&self) ->
TypeId
Gets the
TypeId
of
self
.
Read more
Source
§
impl<T>
Borrow
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
Source
§
impl<T>
BorrowMut
<T> for T
where
    T: ?
Sized
,
Source
§
fn
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
Source
§
impl<T>
CloneToUninit
for T
where
    T:
Clone
,
Source
§
unsafe fn
clone_to_uninit
(&self, dest:
*mut
u8
)
🔬
This is a nightly-only experimental API. (
clone_to_uninit
#126799
)
Performs copy-assignment from
self
to
dest
.
Read more
Source
§
impl<T>
From
<T> for T
Source
§
fn
from
(t: T) -> T
Returns the argument unchanged.
Source
§
impl<T, U>
Into
<U> for T
where
    U:
From
<T>,
Source
§
fn
into
(self) -> U
Calls
U::from(self)
.
That is, this conversion is whatever the implementation of
From
<T> for U
chooses to do.
Source
§
impl<T>
ToOwned
for T
where
    T:
Clone
,
Source
§
type
Owned
= T
The resulting type after obtaining ownership.
Source
§
fn
to_owned
(&self) -> T
Creates owned data from borrowed data, usually by cloning.
Read more
Source
§
fn
clone_into
(&self, target:
&mut T
)
Uses borrowed data to replace owned data, usually by cloning.
Read more
Source
§
impl<T, U>
TryFrom
<U> for T
where
    U:
Into
<T>,
Source
§
type
Error
=
Infallible
The type returned in the event of a conversion error.
Source
§
fn
try_from
(value: U) ->
Result
<T, <T as
TryFrom
<U>>::
Error
>
Performs the conversion.
Source
§
impl<T, U>
TryInto
<U> for T
where
    U:
TryFrom
<T>,
Source
§
type
Error
= <U as
TryFrom
<T>>::
Error
The type returned in the event of a conversion error.
Source
§
fn
try_into
(self) ->
Result
<U, <U as
TryFrom
<T>>::
Error
>
Performs the conversion.