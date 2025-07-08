BTreeSet in std::collections::btree_set - Rust
std
::
collections
::
btree_set
Struct
BTreeSet
Copy item path
1.0.0
·
Source
pub struct BTreeSet<T, A =
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
An ordered set based on a B-Tree.
See
BTreeMap
’s documentation for a detailed discussion of this collection’s performance
benefits and drawbacks.
It is a logic error for an item to be modified in such a way that the item’s ordering relative
to any other item, as determined by the
Ord
trait, changes while it is in the set. This is
normally only possible through
Cell
,
RefCell
, global state, I/O, or unsafe code.
The behavior resulting from such a logic error is not specified, but will be encapsulated to the
BTreeSet
that observed the logic error and not result in undefined behavior. This could
include panics, incorrect results, aborts, memory leaks, and non-termination.
Iterators returned by
BTreeSet::iter
and
BTreeSet::into_iter
produce their items in order, and take worst-case
logarithmic and amortized constant time per item returned.
§
Examples
use
std::collections::BTreeSet;
// Type inference lets us omit an explicit type signature (which
// would be `BTreeSet<&str>` in this example).
let
mut
books = BTreeSet::new();
// Add some books.
books.insert(
"A Dance With Dragons"
);
books.insert(
"To Kill a Mockingbird"
);
books.insert(
"The Odyssey"
);
books.insert(
"The Great Gatsby"
);
// Check for a specific one.
if
!books.contains(
"The Winds of Winter"
) {
println!
(
"We have {} books, but The Winds of Winter ain't one."
,
             books.len());
}
// Remove a book.
books.remove(
"The Odyssey"
);
// Iterate over everything.
for
book
in
&
books {
println!
(
"{book}"
);
}
A
BTreeSet
with a known list of items can be initialized from an array:
use
std::collections::BTreeSet;
let
set = BTreeSet::from([
1
,
2
,
3
]);
Implementations
§
Source
§
impl<T>
BTreeSet
<T>
1.0.0 (const: 1.66.0)
·
Source
pub const fn
new
() ->
BTreeSet
<T>
Makes a new, empty
BTreeSet
.
Does not allocate anything on its own.
§
Examples
use
std::collections::BTreeSet;
let
mut
set: BTreeSet<i32> = BTreeSet::new();
Source
§
impl<T, A>
BTreeSet
<T, A>
where
    A:
Allocator
+
Clone
,
Source
pub const fn
new_in
(alloc: A) ->
BTreeSet
<T, A>
🔬
This is a nightly-only experimental API. (
btreemap_alloc
#32838
)
Makes a new
BTreeSet
with a reasonable choice of B.
§
Examples
use
std::collections::BTreeSet;
use
std::alloc::Global;
let
mut
set: BTreeSet<i32> = BTreeSet::new_in(Global);
1.17.0
·
Source
pub fn
range
<K, R>(&self, range: R) ->
Range
<'_, T>
ⓘ
where
    K:
Ord
+ ?
Sized
,
    T:
Borrow
<K> +
Ord
,
    R:
RangeBounds
<K>,
Constructs a double-ended iterator over a sub-range of elements in the set.
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
std::collections::BTreeSet;
use
std::ops::Bound::Included;
let
mut
set = BTreeSet::new();
set.insert(
3
);
set.insert(
5
);
set.insert(
8
);
for
&
elem
in
set.range((Included(
&
4
), Included(
&
8
))) {
println!
(
"{elem}"
);
}
assert_eq!
(
Some
(
&
5
), set.range(
4
..).next());
1.0.0
·
Source
pub fn
difference
<'a>(
    &'a self,
    other: &'a
BTreeSet
<T, A>,
) ->
Difference
<'a, T, A>
ⓘ
where
    T:
Ord
,
Visits the elements representing the difference,
i.e., the elements that are in
self
but not in
other
,
in ascending order.
§
Examples
use
std::collections::BTreeSet;
let
mut
a = BTreeSet::new();
a.insert(
1
);
a.insert(
2
);
let
mut
b = BTreeSet::new();
b.insert(
2
);
b.insert(
3
);
let
diff: Vec<
_
> = a.difference(
&
b).cloned().collect();
assert_eq!
(diff, [
1
]);
1.0.0
·
Source
pub fn
symmetric_difference
<'a>(
    &'a self,
    other: &'a
BTreeSet
<T, A>,
) ->
SymmetricDifference
<'a, T>
ⓘ
where
    T:
Ord
,
Visits the elements representing the symmetric difference,
i.e., the elements that are in
self
or in
other
but not in both,
in ascending order.
§
Examples
use
std::collections::BTreeSet;
let
mut
a = BTreeSet::new();
a.insert(
1
);
a.insert(
2
);
let
mut
b = BTreeSet::new();
b.insert(
2
);
b.insert(
3
);
let
sym_diff: Vec<
_
> = a.symmetric_difference(
&
b).cloned().collect();
assert_eq!
(sym_diff, [
1
,
3
]);
1.0.0
·
Source
pub fn
intersection
<'a>(
    &'a self,
    other: &'a
BTreeSet
<T, A>,
) ->
Intersection
<'a, T, A>
ⓘ
where
    T:
Ord
,
Visits the elements representing the intersection,
i.e., the elements that are both in
self
and
other
,
in ascending order.
§
Examples
use
std::collections::BTreeSet;
let
mut
a = BTreeSet::new();
a.insert(
1
);
a.insert(
2
);
let
mut
b = BTreeSet::new();
b.insert(
2
);
b.insert(
3
);
let
intersection: Vec<
_
> = a.intersection(
&
b).cloned().collect();
assert_eq!
(intersection, [
2
]);
1.0.0
·
Source
pub fn
union
<'a>(&'a self, other: &'a
BTreeSet
<T, A>) ->
Union
<'a, T>
ⓘ
where
    T:
Ord
,
Visits the elements representing the union,
i.e., all the elements in
self
or
other
, without duplicates,
in ascending order.
§
Examples
use
std::collections::BTreeSet;
let
mut
a = BTreeSet::new();
a.insert(
1
);
let
mut
b = BTreeSet::new();
b.insert(
2
);
let union
: Vec<
_
> = a.union(
&
b).cloned().collect();
assert_eq!
(union, [
1
,
2
]);
1.0.0
·
Source
pub fn
clear
(&mut self)
where
    A:
Clone
,
Clears the set, removing all elements.
§
Examples
use
std::collections::BTreeSet;
let
mut
v = BTreeSet::new();
v.insert(
1
);
v.clear();
assert!
(v.is_empty());
1.0.0
·
Source
pub fn
contains
<Q>(&self, value:
&Q
) ->
bool
where
    T:
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
if the set contains an element equal to the value.
The value may be any borrowed form of the set’s element type,
but the ordering on the borrowed form
must
match the
ordering on the element type.
§
Examples
use
std::collections::BTreeSet;
let
set = BTreeSet::from([
1
,
2
,
3
]);
assert_eq!
(set.contains(
&
1
),
true
);
assert_eq!
(set.contains(
&
4
),
false
);
1.9.0
·
Source
pub fn
get
<Q>(&self, value:
&Q
) ->
Option
<
&T
>
where
    T:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Returns a reference to the element in the set, if any, that is equal to
the value.
The value may be any borrowed form of the set’s element type,
but the ordering on the borrowed form
must
match the
ordering on the element type.
§
Examples
use
std::collections::BTreeSet;
let
set = BTreeSet::from([
1
,
2
,
3
]);
assert_eq!
(set.get(
&
2
),
Some
(
&
2
));
assert_eq!
(set.get(
&
4
),
None
);
1.0.0
·
Source
pub fn
is_disjoint
(&self, other: &
BTreeSet
<T, A>) ->
bool
where
    T:
Ord
,
Returns
true
if
self
has no elements in common with
other
.
This is equivalent to checking for an empty intersection.
§
Examples
use
std::collections::BTreeSet;
let
a = BTreeSet::from([
1
,
2
,
3
]);
let
mut
b = BTreeSet::new();
assert_eq!
(a.is_disjoint(
&
b),
true
);
b.insert(
4
);
assert_eq!
(a.is_disjoint(
&
b),
true
);
b.insert(
1
);
assert_eq!
(a.is_disjoint(
&
b),
false
);
1.0.0
·
Source
pub fn
is_subset
(&self, other: &
BTreeSet
<T, A>) ->
bool
where
    T:
Ord
,
Returns
true
if the set is a subset of another,
i.e.,
other
contains at least all the elements in
self
.
§
Examples
use
std::collections::BTreeSet;
let
sup = BTreeSet::from([
1
,
2
,
3
]);
let
mut
set = BTreeSet::new();
assert_eq!
(set.is_subset(
&
sup),
true
);
set.insert(
2
);
assert_eq!
(set.is_subset(
&
sup),
true
);
set.insert(
4
);
assert_eq!
(set.is_subset(
&
sup),
false
);
1.0.0
·
Source
pub fn
is_superset
(&self, other: &
BTreeSet
<T, A>) ->
bool
where
    T:
Ord
,
Returns
true
if the set is a superset of another,
i.e.,
self
contains at least all the elements in
other
.
§
Examples
use
std::collections::BTreeSet;
let
sub = BTreeSet::from([
1
,
2
]);
let
mut
set = BTreeSet::new();
assert_eq!
(set.is_superset(
&
sub),
false
);

set.insert(
0
);
set.insert(
1
);
assert_eq!
(set.is_superset(
&
sub),
false
);

set.insert(
2
);
assert_eq!
(set.is_superset(
&
sub),
true
);
1.66.0
·
Source
pub fn
first
(&self) ->
Option
<
&T
>
where
    T:
Ord
,
Returns a reference to the first element in the set, if any.
This element is always the minimum of all elements in the set.
§
Examples
Basic usage:
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::new();
assert_eq!
(set.first(),
None
);
set.insert(
1
);
assert_eq!
(set.first(),
Some
(
&
1
));
set.insert(
2
);
assert_eq!
(set.first(),
Some
(
&
1
));
1.66.0
·
Source
pub fn
last
(&self) ->
Option
<
&T
>
where
    T:
Ord
,
Returns a reference to the last element in the set, if any.
This element is always the maximum of all elements in the set.
§
Examples
Basic usage:
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::new();
assert_eq!
(set.last(),
None
);
set.insert(
1
);
assert_eq!
(set.last(),
Some
(
&
1
));
set.insert(
2
);
assert_eq!
(set.last(),
Some
(
&
2
));
1.66.0
·
Source
pub fn
pop_first
(&mut self) ->
Option
<T>
where
    T:
Ord
,
Removes the first element from the set and returns it, if any.
The first element is always the minimum element in the set.
§
Examples
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::new();

set.insert(
1
);
while let
Some
(n) = set.pop_first() {
assert_eq!
(n,
1
);
}
assert!
(set.is_empty());
1.66.0
·
Source
pub fn
pop_last
(&mut self) ->
Option
<T>
where
    T:
Ord
,
Removes the last element from the set and returns it, if any.
The last element is always the maximum element in the set.
§
Examples
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::new();

set.insert(
1
);
while let
Some
(n) = set.pop_last() {
assert_eq!
(n,
1
);
}
assert!
(set.is_empty());
1.0.0
·
Source
pub fn
insert
(&mut self, value: T) ->
bool
where
    T:
Ord
,
Adds a value to the set.
Returns whether the value was newly inserted. That is:
If the set did not previously contain an equal value,
true
is
returned.
If the set already contained an equal value,
false
is returned, and
the entry is not updated.
See the
module-level documentation
for more.
§
Examples
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::new();
assert_eq!
(set.insert(
2
),
true
);
assert_eq!
(set.insert(
2
),
false
);
assert_eq!
(set.len(),
1
);
1.9.0
·
Source
pub fn
replace
(&mut self, value: T) ->
Option
<T>
where
    T:
Ord
,
Adds a value to the set, replacing the existing element, if any, that is
equal to the value. Returns the replaced element.
§
Examples
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::new();
set.insert(Vec::<i32>::new());
assert_eq!
(set.get(
&
[][..]).unwrap().capacity(),
0
);
set.replace(Vec::with_capacity(
10
));
assert_eq!
(set.get(
&
[][..]).unwrap().capacity(),
10
);
Source
pub fn
get_or_insert
(&mut self, value: T) ->
&T
where
    T:
Ord
,
🔬
This is a nightly-only experimental API. (
btree_set_entry
#133549
)
Inserts the given
value
into the set if it is not present, then
returns a reference to the value in the set.
§
Examples
#![feature(btree_set_entry)]
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::from([
1
,
2
,
3
]);
assert_eq!
(set.len(),
3
);
assert_eq!
(set.get_or_insert(
2
),
&
2
);
assert_eq!
(set.get_or_insert(
100
),
&
100
);
assert_eq!
(set.len(),
4
);
// 100 was inserted
Source
pub fn
get_or_insert_with
<Q, F>(&mut self, value:
&Q
, f: F) ->
&T
where
    T:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
    F:
FnOnce
(
&Q
) -> T,
🔬
This is a nightly-only experimental API. (
btree_set_entry
#133549
)
Inserts a value computed from
f
into the set if the given
value
is
not present, then returns a reference to the value in the set.
§
Examples
#![feature(btree_set_entry)]
use
std::collections::BTreeSet;
let
mut
set: BTreeSet<String> = [
"cat"
,
"dog"
,
"horse"
]
    .iter().map(|
&
pet| pet.to_owned()).collect();
assert_eq!
(set.len(),
3
);
for
&
pet
in
&
[
"cat"
,
"dog"
,
"fish"
] {
let
value = set.get_or_insert_with(pet, str::to_owned);
assert_eq!
(value, pet);
}
assert_eq!
(set.len(),
4
);
// a new "fish" was inserted
Source
pub fn
entry
(&mut self, value: T) ->
Entry
<'_, T, A>
where
    T:
Ord
,
🔬
This is a nightly-only experimental API. (
btree_set_entry
#133549
)
Gets the given value’s corresponding entry in the set for in-place manipulation.
§
Examples
#![feature(btree_set_entry)]
use
std::collections::BTreeSet;
use
std::collections::btree_set::Entry::
*
;
let
mut
singles = BTreeSet::new();
let
mut
dupes = BTreeSet::new();
for
ch
in
"a short treatise on fungi"
.chars() {
if let
Vacant(dupe_entry) = dupes.entry(ch) {
// We haven't already seen a duplicate, so
        // check if we've at least seen it once.
match
singles.entry(ch) {
            Vacant(single_entry) => {
// We found a new character for the first time.
single_entry.insert()
            }
            Occupied(single_entry) => {
// We've already seen this once, "move" it to dupes.
single_entry.remove();
                dupe_entry.insert();
            }
        }
    }
}
assert!
(!singles.contains(
&
't'
) && dupes.contains(
&
't'
));
assert!
(singles.contains(
&
'u'
) && !dupes.contains(
&
'u'
));
assert!
(!singles.contains(
&
'v'
) && !dupes.contains(
&
'v'
));
1.0.0
·
Source
pub fn
remove
<Q>(&mut self, value:
&Q
) ->
bool
where
    T:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
If the set contains an element equal to the value, removes it from the
set and drops it. Returns whether such an element was present.
The value may be any borrowed form of the set’s element type,
but the ordering on the borrowed form
must
match the
ordering on the element type.
§
Examples
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::new();

set.insert(
2
);
assert_eq!
(set.remove(
&
2
),
true
);
assert_eq!
(set.remove(
&
2
),
false
);
1.9.0
·
Source
pub fn
take
<Q>(&mut self, value:
&Q
) ->
Option
<T>
where
    T:
Borrow
<Q> +
Ord
,
    Q:
Ord
+ ?
Sized
,
Removes and returns the element in the set, if any, that is equal to
the value.
The value may be any borrowed form of the set’s element type,
but the ordering on the borrowed form
must
match the
ordering on the element type.
§
Examples
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::from([
1
,
2
,
3
]);
assert_eq!
(set.take(
&
2
),
Some
(
2
));
assert_eq!
(set.take(
&
2
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
    T:
Ord
,
    F:
FnMut
(
&T
) ->
bool
,
Retains only the elements specified by the predicate.
In other words, remove all elements
e
for which
f(&e)
returns
false
.
The elements are visited in ascending order.
§
Examples
use
std::collections::BTreeSet;
let
mut
set = BTreeSet::from([
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
]);
// Keep only the even numbers.
set.retain(|
&
k| k %
2
==
0
);
assert!
(set.iter().eq([
2
,
4
,
6
].iter()));
1.11.0
·
Source
pub fn
append
(&mut self, other: &mut
BTreeSet
<T, A>)
where
    T:
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
§
Examples
use
std::collections::BTreeSet;
let
mut
a = BTreeSet::new();
a.insert(
1
);
a.insert(
2
);
a.insert(
3
);
let
mut
b = BTreeSet::new();
b.insert(
3
);
b.insert(
4
);
b.insert(
5
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
assert!
(a.contains(
&
1
));
assert!
(a.contains(
&
2
));
assert!
(a.contains(
&
3
));
assert!
(a.contains(
&
4
));
assert!
(a.contains(
&
5
));
1.11.0
·
Source
pub fn
split_off
<Q>(&mut self, value:
&Q
) ->
BTreeSet
<T, A>
where
    Q:
Ord
+ ?
Sized
,
    T:
Borrow
<Q> +
Ord
,
    A:
Clone
,
Splits the collection into two at the value. Returns a new collection
with all elements greater than or equal to the value.
§
Examples
Basic usage:
use
std::collections::BTreeSet;
let
mut
a = BTreeSet::new();
a.insert(
1
);
a.insert(
2
);
a.insert(
3
);
a.insert(
17
);
a.insert(
41
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
assert!
(a.contains(
&
1
));
assert!
(a.contains(
&
2
));
assert!
(b.contains(
&
3
));
assert!
(b.contains(
&
17
));
assert!
(b.contains(
&
41
));
Source
pub fn
extract_if
<'a, F>(&'a mut self, pred: F) ->
ExtractIf
<'a, T, F, A>
ⓘ
where
    T:
Ord
,
    F: 'a +
FnMut
(
&T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
btree_extract_if
#70530
)
Creates an iterator that visits all elements in ascending order and
uses a closure to determine if an element should be removed.
If the closure returns
true
, the element is removed from the set and
yielded. If the closure returns
false
, or panics, the element remains
in the set and will not be yielded.
If the returned
ExtractIf
is not exhausted, e.g. because it is dropped without iterating
or the iteration short-circuits, then the remaining elements will be retained.
Use
retain
with a negated predicate if you do not need the returned iterator.
§
Examples
Splitting a set into even and odd values, reusing the original set:
#![feature(btree_extract_if)]
use
std::collections::BTreeSet;
let
mut
set: BTreeSet<i32> = (
0
..
8
).collect();
let
evens: BTreeSet<
_
> = set.extract_if(|v| v %
2
==
0
).collect();
let
odds = set;
assert_eq!
(evens.into_iter().collect::<Vec<
_
>>(),
vec!
[
0
,
2
,
4
,
6
]);
assert_eq!
(odds.into_iter().collect::<Vec<
_
>>(),
vec!
[
1
,
3
,
5
,
7
]);
1.0.0
·
Source
pub fn
iter
(&self) ->
Iter
<'_, T>
ⓘ
Gets an iterator that visits the elements in the
BTreeSet
in ascending
order.
§
Examples
use
std::collections::BTreeSet;
let
set = BTreeSet::from([
3
,
1
,
2
]);
let
mut
set_iter = set.iter();
assert_eq!
(set_iter.next(),
Some
(
&
1
));
assert_eq!
(set_iter.next(),
Some
(
&
2
));
assert_eq!
(set_iter.next(),
Some
(
&
3
));
assert_eq!
(set_iter.next(),
None
);
1.0.0 (const:
unstable
)
·
Source
pub fn
len
(&self) ->
usize
Returns the number of elements in the set.
§
Examples
use
std::collections::BTreeSet;
let
mut
v = BTreeSet::new();
assert_eq!
(v.len(),
0
);
v.insert(
1
);
assert_eq!
(v.len(),
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
if the set contains no elements.
§
Examples
use
std::collections::BTreeSet;
let
mut
v = BTreeSet::new();
assert!
(v.is_empty());
v.insert(
1
);
assert!
(!v.is_empty());
Source
pub fn
lower_bound
<Q>(&self, bound:
Bound
<
&Q
>) ->
Cursor
<'_, T>
where
    T:
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
pointing at the gap before the smallest element
greater than the given bound.
Passing
Bound::Included(x)
will return a cursor pointing to the
gap before the smallest element greater than or equal to
x
.
Passing
Bound::Excluded(x)
will return a cursor pointing to the
gap before the smallest element greater than
x
.
Passing
Bound::Unbounded
will return a cursor pointing to the
gap before the smallest element in the set.
§
Examples
#![feature(btree_cursors)]
use
std::collections::BTreeSet;
use
std::ops::Bound;
let
set = BTreeSet::from([
1
,
2
,
3
,
4
]);
let
cursor = set.lower_bound(Bound::Included(
&
2
));
assert_eq!
(cursor.peek_prev(),
Some
(
&
1
));
assert_eq!
(cursor.peek_next(),
Some
(
&
2
));
let
cursor = set.lower_bound(Bound::Excluded(
&
2
));
assert_eq!
(cursor.peek_prev(),
Some
(
&
2
));
assert_eq!
(cursor.peek_next(),
Some
(
&
3
));
let
cursor = set.lower_bound(Bound::Unbounded);
assert_eq!
(cursor.peek_prev(),
None
);
assert_eq!
(cursor.peek_next(),
Some
(
&
1
));
Source
pub fn
lower_bound_mut
<Q>(&mut self, bound:
Bound
<
&Q
>) ->
CursorMut
<'_, T, A>
where
    T:
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
pointing at the gap before the smallest element
greater than the given bound.
Passing
Bound::Included(x)
will return a cursor pointing to the
gap before the smallest element greater than or equal to
x
.
Passing
Bound::Excluded(x)
will return a cursor pointing to the
gap before the smallest element greater than
x
.
Passing
Bound::Unbounded
will return a cursor pointing to the
gap before the smallest element in the set.
§
Examples
#![feature(btree_cursors)]
use
std::collections::BTreeSet;
use
std::ops::Bound;
let
mut
set = BTreeSet::from([
1
,
2
,
3
,
4
]);
let
mut
cursor = set.lower_bound_mut(Bound::Included(
&
2
));
assert_eq!
(cursor.peek_prev(),
Some
(
&
1
));
assert_eq!
(cursor.peek_next(),
Some
(
&
2
));
let
mut
cursor = set.lower_bound_mut(Bound::Excluded(
&
2
));
assert_eq!
(cursor.peek_prev(),
Some
(
&
2
));
assert_eq!
(cursor.peek_next(),
Some
(
&
3
));
let
mut
cursor = set.lower_bound_mut(Bound::Unbounded);
assert_eq!
(cursor.peek_prev(),
None
);
assert_eq!
(cursor.peek_next(),
Some
(
&
1
));
Source
pub fn
upper_bound
<Q>(&self, bound:
Bound
<
&Q
>) ->
Cursor
<'_, T>
where
    T:
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
pointing at the gap after the greatest element
smaller than the given bound.
Passing
Bound::Included(x)
will return a cursor pointing to the
gap after the greatest element smaller than or equal to
x
.
Passing
Bound::Excluded(x)
will return a cursor pointing to the
gap after the greatest element smaller than
x
.
Passing
Bound::Unbounded
will return a cursor pointing to the
gap after the greatest element in the set.
§
Examples
#![feature(btree_cursors)]
use
std::collections::BTreeSet;
use
std::ops::Bound;
let
set = BTreeSet::from([
1
,
2
,
3
,
4
]);
let
cursor = set.upper_bound(Bound::Included(
&
3
));
assert_eq!
(cursor.peek_prev(),
Some
(
&
3
));
assert_eq!
(cursor.peek_next(),
Some
(
&
4
));
let
cursor = set.upper_bound(Bound::Excluded(
&
3
));
assert_eq!
(cursor.peek_prev(),
Some
(
&
2
));
assert_eq!
(cursor.peek_next(),
Some
(
&
3
));
let
cursor = set.upper_bound(Bound::Unbounded);
assert_eq!
(cursor.peek_prev(),
Some
(
&
4
));
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
<'_, T, A>
where
    T:
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
pointing at the gap after the greatest element
smaller than the given bound.
Passing
Bound::Included(x)
will return a cursor pointing to the
gap after the greatest element smaller than or equal to
x
.
Passing
Bound::Excluded(x)
will return a cursor pointing to the
gap after the greatest element smaller than
x
.
Passing
Bound::Unbounded
will return a cursor pointing to the
gap after the greatest element in the set.
§
Examples
#![feature(btree_cursors)]
use
std::collections::BTreeSet;
use
std::ops::Bound;
let
mut
set = BTreeSet::from([
1
,
2
,
3
,
4
]);
let
mut
cursor = set.upper_bound_mut(Bound::Included(
&
3
));
assert_eq!
(cursor.peek_prev(),
Some
(
&
3
));
assert_eq!
(cursor.peek_next(),
Some
(
&
4
));
let
mut
cursor = set.upper_bound_mut(Bound::Excluded(
&
3
));
assert_eq!
(cursor.peek_prev(),
Some
(
&
2
));
assert_eq!
(cursor.peek_next(),
Some
(
&
3
));
let
mut
cursor = set.upper_bound_mut(Bound::Unbounded);
assert_eq!
(cursor.peek_prev(),
Some
(
&
4
));
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
impl<T, A>
BitAnd
<&
BTreeSet
<T, A>> for &
BTreeSet
<T, A>
where
    T:
Ord
+
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
bitand
(self, rhs: &
BTreeSet
<T, A>) ->
BTreeSet
<T, A>
Returns the intersection of
self
and
rhs
as a new
BTreeSet<T>
.
§
Examples
use
std::collections::BTreeSet;
let
a = BTreeSet::from([
1
,
2
,
3
]);
let
b = BTreeSet::from([
2
,
3
,
4
]);
let
result =
&
a &
&
b;
assert_eq!
(result, BTreeSet::from([
2
,
3
]));
Source
§
type
Output
=
BTreeSet
<T, A>
The resulting type after applying the
&
operator.
1.0.0
·
Source
§
impl<T, A>
BitOr
<&
BTreeSet
<T, A>> for &
BTreeSet
<T, A>
where
    T:
Ord
+
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
bitor
(self, rhs: &
BTreeSet
<T, A>) ->
BTreeSet
<T, A>
Returns the union of
self
and
rhs
as a new
BTreeSet<T>
.
§
Examples
use
std::collections::BTreeSet;
let
a = BTreeSet::from([
1
,
2
,
3
]);
let
b = BTreeSet::from([
3
,
4
,
5
]);
let
result =
&
a |
&
b;
assert_eq!
(result, BTreeSet::from([
1
,
2
,
3
,
4
,
5
]));
Source
§
type
Output
=
BTreeSet
<T, A>
The resulting type after applying the
|
operator.
1.0.0
·
Source
§
impl<T, A>
BitXor
<&
BTreeSet
<T, A>> for &
BTreeSet
<T, A>
where
    T:
Ord
+
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
bitxor
(self, rhs: &
BTreeSet
<T, A>) ->
BTreeSet
<T, A>
Returns the symmetric difference of
self
and
rhs
as a new
BTreeSet<T>
.
§
Examples
use
std::collections::BTreeSet;
let
a = BTreeSet::from([
1
,
2
,
3
]);
let
b = BTreeSet::from([
2
,
3
,
4
]);
let
result =
&
a ^
&
b;
assert_eq!
(result, BTreeSet::from([
1
,
4
]));
Source
§
type
Output
=
BTreeSet
<T, A>
The resulting type after applying the
^
operator.
1.0.0
·
Source
§
impl<T, A>
Clone
for
BTreeSet
<T, A>
where
    T:
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
BTreeSet
<T, A>
Returns a copy of the value.
Read more
Source
§
fn
clone_from
(&mut self, source: &
BTreeSet
<T, A>)
Performs copy-assignment from
source
.
Read more
1.0.0
·
Source
§
impl<T, A>
Debug
for
BTreeSet
<T, A>
where
    T:
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
impl<T>
Default
for
BTreeSet
<T>
Source
§
fn
default
() ->
BTreeSet
<T>
Creates an empty
BTreeSet
.
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
Source
§
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item =
&'a T
>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, _:
&'a T
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
Source
§
fn
extend
<Iter>(&mut self, iter: Iter)
where
    Iter:
IntoIterator
<Item = T>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, elem: T)
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
impl<T, const N:
usize
>
From
<
[T; N]
> for
BTreeSet
<T>
where
    T:
Ord
,
Source
§
fn
from
(arr:
[T; N]
) ->
BTreeSet
<T>
Converts a
[T; N]
into a
BTreeSet<T>
.
If the array contains any equal values,
all but one will be dropped.
§
Examples
use
std::collections::BTreeSet;
let
set1 = BTreeSet::from([
1
,
2
,
3
,
4
]);
let
set2: BTreeSet<
_
> = [
1
,
2
,
3
,
4
].into();
assert_eq!
(set1, set2);
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
Source
§
fn
from_iter
<I>(iter: I) ->
BTreeSet
<T>
where
    I:
IntoIterator
<Item = T>,
Creates a value from an iterator.
Read more
1.0.0
·
Source
§
impl<T, A>
Hash
for
BTreeSet
<T, A>
where
    T:
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
The type of the elements being iterated over.
Source
§
type
IntoIter
=
Iter
<'a, T>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
Iter
<'a, T>
ⓘ
Creates an iterator from a value.
Read more
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
fn
into_iter
(self) ->
IntoIter
<T, A>
ⓘ
Gets an iterator for moving out the
BTreeSet
’s contents in ascending order.
§
Examples
use
std::collections::BTreeSet;
let
set = BTreeSet::from([
1
,
2
,
3
,
4
]);
let
v: Vec<
_
> = set.into_iter().collect();
assert_eq!
(v, [
1
,
2
,
3
,
4
]);
Source
§
type
Item
= T
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IntoIter
<T, A>
Which kind of iterator are we turning this into?
1.0.0
·
Source
§
impl<T, A>
Ord
for
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
Source
§
fn
cmp
(&self, other: &
BTreeSet
<T, A>) ->
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
impl<T, A>
PartialEq
for
BTreeSet
<T, A>
where
    T:
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
BTreeSet
<T, A>) ->
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
impl<T, A>
PartialOrd
for
BTreeSet
<T, A>
where
    T:
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
BTreeSet
<T, A>) ->
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
impl<T, A>
Sub
<&
BTreeSet
<T, A>> for &
BTreeSet
<T, A>
where
    T:
Ord
+
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
sub
(self, rhs: &
BTreeSet
<T, A>) ->
BTreeSet
<T, A>
Returns the difference of
self
and
rhs
as a new
BTreeSet<T>
.
§
Examples
use
std::collections::BTreeSet;
let
a = BTreeSet::from([
1
,
2
,
3
]);
let
b = BTreeSet::from([
3
,
4
,
5
]);
let
result =
&
a -
&
b;
assert_eq!
(result, BTreeSet::from([
1
,
2
]));
Source
§
type
Output
=
BTreeSet
<T, A>
The resulting type after applying the
-
operator.
1.0.0
·
Source
§
impl<T, A>
Eq
for
BTreeSet
<T, A>
where
    T:
Eq
,
    A:
Allocator
+
Clone
,
Auto Trait Implementations
§
§
impl<T, A>
Freeze
for
BTreeSet
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
RefUnwindSafe
for
BTreeSet
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<T, A>
Send
for
BTreeSet
<T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<T, A>
Sync
for
BTreeSet
<T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<T, A>
Unpin
for
BTreeSet
<T, A>
where
    A:
Unpin
,
§
impl<T, A>
UnwindSafe
for
BTreeSet
<T, A>
where
    A:
UnwindSafe
,
    T:
RefUnwindSafe
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