BinaryHeap in std::collections::binary_heap - Rust
std
::
collections
::
binary_heap
Struct
BinaryHeap
Copy item path
1.0.0
·
Source
pub struct BinaryHeap<T, A =
Global
>
where
    A:
Allocator
,
{
/* private fields */
}
Expand description
A priority queue implemented with a binary heap.
This will be a max-heap.
It is a logic error for an item to be modified in such a way that the
item’s ordering relative to any other item, as determined by the
Ord
trait, changes while it is in the heap. This is normally only possible
through interior mutability, global state, I/O, or unsafe code. The
behavior resulting from such a logic error is not specified, but will
be encapsulated to the
BinaryHeap
that observed the logic error and not
result in undefined behavior. This could include panics, incorrect results,
aborts, memory leaks, and non-termination.
As long as no elements change their relative order while being in the heap
as described above, the API of
BinaryHeap
guarantees that the heap
invariant remains intact i.e. its methods all behave as documented. For
example if a method is documented as iterating in sorted order, that’s
guaranteed to work as long as elements in the heap have not changed order,
even in the presence of closures getting unwinded out of, iterators getting
leaked, and similar foolishness.
§
Examples
use
std::collections::BinaryHeap;
// Type inference lets us omit an explicit type signature (which
// would be `BinaryHeap<i32>` in this example).
let
mut
heap = BinaryHeap::new();
// We can use peek to look at the next item in the heap. In this case,
// there's no items in there yet so we get None.
assert_eq!
(heap.peek(),
None
);
// Let's add some scores...
heap.push(
1
);
heap.push(
5
);
heap.push(
2
);
// Now peek shows the most important item in the heap.
assert_eq!
(heap.peek(),
Some
(
&
5
));
// We can check the length of a heap.
assert_eq!
(heap.len(),
3
);
// We can iterate over the items in the heap, although they are returned in
// a random order.
for
x
in
&
heap {
println!
(
"{x}"
);
}
// If we instead pop these scores, they should come back in order.
assert_eq!
(heap.pop(),
Some
(
5
));
assert_eq!
(heap.pop(),
Some
(
2
));
assert_eq!
(heap.pop(),
Some
(
1
));
assert_eq!
(heap.pop(),
None
);
// We can clear the heap of any remaining items.
heap.clear();
// The heap should now be empty.
assert!
(heap.is_empty())
A
BinaryHeap
with a known list of items can be initialized from an array:
use
std::collections::BinaryHeap;
let
heap = BinaryHeap::from([
1
,
5
,
2
]);
§
Min-heap
Either
core::cmp::Reverse
or a custom
Ord
implementation can be used to
make
BinaryHeap
a min-heap. This makes
heap.pop()
return the smallest
value instead of the greatest one.
use
std::collections::BinaryHeap;
use
std::cmp::Reverse;
let
mut
heap = BinaryHeap::new();
// Wrap values in `Reverse`
heap.push(Reverse(
1
));
heap.push(Reverse(
5
));
heap.push(Reverse(
2
));
// If we pop these scores now, they should come back in the reverse order.
assert_eq!
(heap.pop(),
Some
(Reverse(
1
)));
assert_eq!
(heap.pop(),
Some
(Reverse(
2
)));
assert_eq!
(heap.pop(),
Some
(Reverse(
5
)));
assert_eq!
(heap.pop(),
None
);
§
Time complexity
push
pop
peek
/
peek_mut
O
(1)~
O
(log(
n
))
O
(1)
The value for
push
is an expected cost; the method documentation gives a
more detailed analysis.
Implementations
§
Source
§
impl<T>
BinaryHeap
<T>
where
    T:
Ord
,
1.0.0 (const: 1.80.0)
·
Source
pub const fn
new
() ->
BinaryHeap
<T>
Creates an empty
BinaryHeap
as a max-heap.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::new();
heap.push(
4
);
1.0.0
·
Source
pub fn
with_capacity
(capacity:
usize
) ->
BinaryHeap
<T>
Creates an empty
BinaryHeap
with at least the specified capacity.
The binary heap will be able to hold at least
capacity
elements without
reallocating. This method is allowed to allocate for more elements than
capacity
. If
capacity
is zero, the binary heap will not allocate.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::with_capacity(
10
);
heap.push(
4
);
Source
§
impl<T, A>
BinaryHeap
<T, A>
where
    T:
Ord
,
    A:
Allocator
,
Source
pub const fn
new_in
(alloc: A) ->
BinaryHeap
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Creates an empty
BinaryHeap
as a max-heap, using
A
as allocator.
§
Examples
Basic usage:
#![feature(allocator_api)]
use
std::alloc::System;
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::new_in(System);
heap.push(
4
);
Source
pub fn
with_capacity_in
(capacity:
usize
, alloc: A) ->
BinaryHeap
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Creates an empty
BinaryHeap
with at least the specified capacity, using
A
as allocator.
The binary heap will be able to hold at least
capacity
elements without
reallocating. This method is allowed to allocate for more elements than
capacity
. If
capacity
is zero, the binary heap will not allocate.
§
Examples
Basic usage:
#![feature(allocator_api)]
use
std::alloc::System;
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::with_capacity_in(
10
, System);
heap.push(
4
);
1.12.0
·
Source
pub fn
peek_mut
(&mut self) ->
Option
<
PeekMut
<'_, T, A>>
Returns a mutable reference to the greatest item in the binary heap, or
None
if it is empty.
Note: If the
PeekMut
value is leaked, some heap elements might get
leaked along with it, but the remaining elements will remain a valid
heap.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::new();
assert!
(heap.peek_mut().is_none());

heap.push(
1
);
heap.push(
5
);
heap.push(
2
);
if let
Some
(
mut
val) = heap.peek_mut() {
*
val =
0
;
}
assert_eq!
(heap.peek(),
Some
(
&
2
));
§
Time complexity
If the item is modified then the worst case time complexity is
O
(log(
n
)),
otherwise it’s
O
(1).
1.0.0
·
Source
pub fn
pop
(&mut self) ->
Option
<T>
Removes the greatest item from the binary heap and returns it, or
None
if it
is empty.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::from([
1
,
3
]);
assert_eq!
(heap.pop(),
Some
(
3
));
assert_eq!
(heap.pop(),
Some
(
1
));
assert_eq!
(heap.pop(),
None
);
§
Time complexity
The worst case cost of
pop
on a heap containing
n
elements is
O
(log(
n
)).
1.0.0
·
Source
pub fn
push
(&mut self, item: T)
Pushes an item onto the binary heap.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::new();
heap.push(
3
);
heap.push(
5
);
heap.push(
1
);
assert_eq!
(heap.len(),
3
);
assert_eq!
(heap.peek(),
Some
(
&
5
));
§
Time complexity
The expected cost of
push
, averaged over every possible ordering of
the elements being pushed, and over a sufficiently large number of
pushes, is
O
(1). This is the most meaningful cost metric when pushing
elements that are
not
already in any sorted pattern.
The time complexity degrades if elements are pushed in predominantly
ascending order. In the worst case, elements are pushed in ascending
sorted order and the amortized cost per push is
O
(log(
n
)) against a heap
containing
n
elements.
The worst case cost of a
single
call to
push
is
O
(
n
). The worst case
occurs when capacity is exhausted and needs a resize. The resize cost
has been amortized in the previous figures.
1.5.0
·
Source
pub fn
into_sorted_vec
(self) ->
Vec
<T, A>
Consumes the
BinaryHeap
and returns a vector in sorted
(ascending) order.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::from([
1
,
2
,
4
,
5
,
7
]);
heap.push(
6
);
heap.push(
3
);
let
vec = heap.into_sorted_vec();
assert_eq!
(vec, [
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
,
7
]);
1.11.0
·
Source
pub fn
append
(&mut self, other: &mut
BinaryHeap
<T, A>)
Moves all the elements of
other
into
self
, leaving
other
empty.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
a = BinaryHeap::from([-
10
,
1
,
2
,
3
,
3
]);
let
mut
b = BinaryHeap::from([-
20
,
5
,
43
]);

a.append(
&mut
b);
assert_eq!
(a.into_sorted_vec(), [-
20
, -
10
,
1
,
2
,
3
,
3
,
5
,
43
]);
assert!
(b.is_empty());
Source
pub fn
drain_sorted
(&mut self) ->
DrainSorted
<'_, T, A>
ⓘ
🔬
This is a nightly-only experimental API. (
binary_heap_drain_sorted
#59278
)
Clears the binary heap, returning an iterator over the removed elements
in heap order. If the iterator is dropped before being fully consumed,
it drops the remaining elements in heap order.
The returned iterator keeps a mutable borrow on the heap to optimize
its implementation.
Note:
.drain_sorted()
is
O
(
n
* log(
n
)); much slower than
.drain()
.
You should use the latter for most cases.
§
Examples
Basic usage:
#![feature(binary_heap_drain_sorted)]
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::from([
1
,
2
,
3
,
4
,
5
]);
assert_eq!
(heap.len(),
5
);

drop(heap.drain_sorted());
// removes all elements in heap order
assert_eq!
(heap.len(),
0
);
1.70.0
·
Source
pub fn
retain
<F>(&mut self, f: F)
where
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
. The elements are visited in unsorted (and unspecified) order.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::from([-
10
, -
5
,
1
,
2
,
4
,
13
]);

heap.retain(|x| x %
2
==
0
);
// only keep even numbers
assert_eq!
(heap.into_sorted_vec(), [-
10
,
2
,
4
])
Source
§
impl<T, A>
BinaryHeap
<T, A>
where
    A:
Allocator
,
1.0.0
·
Source
pub fn
iter
(&self) ->
Iter
<'_, T>
ⓘ
Returns an iterator visiting all values in the underlying vector, in
arbitrary order.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
heap = BinaryHeap::from([
1
,
2
,
3
,
4
]);
// Print 1, 2, 3, 4 in arbitrary order
for
x
in
heap.iter() {
println!
(
"{x}"
);
}
Source
pub fn
into_iter_sorted
(self) ->
IntoIterSorted
<T, A>
ⓘ
🔬
This is a nightly-only experimental API. (
binary_heap_into_iter_sorted
#59278
)
Returns an iterator which retrieves elements in heap order.
This method consumes the original heap.
§
Examples
Basic usage:
#![feature(binary_heap_into_iter_sorted)]
use
std::collections::BinaryHeap;
let
heap = BinaryHeap::from([
1
,
2
,
3
,
4
,
5
]);
assert_eq!
(heap.into_iter_sorted().take(
2
).collect::<Vec<
_
>>(), [
5
,
4
]);
1.0.0
·
Source
pub fn
peek
(&self) ->
Option
<
&T
>
Returns the greatest item in the binary heap, or
None
if it is empty.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::new();
assert_eq!
(heap.peek(),
None
);

heap.push(
1
);
heap.push(
5
);
heap.push(
2
);
assert_eq!
(heap.peek(),
Some
(
&
5
));
§
Time complexity
Cost is
O
(1) in the worst case.
1.0.0
·
Source
pub fn
capacity
(&self) ->
usize
Returns the number of elements the binary heap can hold without reallocating.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::with_capacity(
100
);
assert!
(heap.capacity() >=
100
);
heap.push(
4
);
1.0.0
·
Source
pub fn
reserve_exact
(&mut self, additional:
usize
)
Reserves the minimum capacity for at least
additional
elements more than
the current length. Unlike
reserve
, this will not
deliberately over-allocate to speculatively avoid frequent allocations.
After calling
reserve_exact
, capacity will be greater than or equal to
self.len() + additional
. Does nothing if the capacity is already
sufficient.
§
Panics
Panics if the new capacity overflows
usize
.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::new();
heap.reserve_exact(
100
);
assert!
(heap.capacity() >=
100
);
heap.push(
4
);
1.0.0
·
Source
pub fn
reserve
(&mut self, additional:
usize
)
Reserves capacity for at least
additional
elements more than the
current length. The allocator may reserve more space to speculatively
avoid frequent allocations. After calling
reserve
,
capacity will be greater than or equal to
self.len() + additional
.
Does nothing if capacity is already sufficient.
§
Panics
Panics if the new capacity overflows
usize
.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::new();
heap.reserve(
100
);
assert!
(heap.capacity() >=
100
);
heap.push(
4
);
1.63.0
·
Source
pub fn
try_reserve_exact
(
    &mut self,
    additional:
usize
,
) ->
Result
<
()
,
TryReserveError
>
Tries to reserve the minimum capacity for at least
additional
elements
more than the current length. Unlike
try_reserve
, this will not
deliberately over-allocate to speculatively avoid frequent allocations.
After calling
try_reserve_exact
, capacity will be greater than or
equal to
self.len() + additional
if it returns
Ok(())
.
Does nothing if the capacity is already sufficient.
Note that the allocator may give the collection more space than it
requests. Therefore, capacity can not be relied upon to be precisely
minimal. Prefer
try_reserve
if future insertions are expected.
§
Errors
If the capacity overflows, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::collections::BinaryHeap;
use
std::collections::TryReserveError;
fn
find_max_slow(data:
&
[u32]) ->
Result
<
Option
<u32>, TryReserveError> {
let
mut
heap = BinaryHeap::new();
// Pre-reserve the memory, exiting if we can't
heap.try_reserve_exact(data.len())
?
;
// Now we know this can't OOM in the middle of our complex work
heap.extend(data.iter());
Ok
(heap.pop())
}
1.63.0
·
Source
pub fn
try_reserve
(&mut self, additional:
usize
) ->
Result
<
()
,
TryReserveError
>
Tries to reserve capacity for at least
additional
elements more than the
current length. The allocator may reserve more space to speculatively
avoid frequent allocations. After calling
try_reserve
, capacity will be
greater than or equal to
self.len() + additional
if it returns
Ok(())
. Does nothing if capacity is already sufficient. This method
preserves the contents even if an error occurs.
§
Errors
If the capacity overflows, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::collections::BinaryHeap;
use
std::collections::TryReserveError;
fn
find_max_slow(data:
&
[u32]) ->
Result
<
Option
<u32>, TryReserveError> {
let
mut
heap = BinaryHeap::new();
// Pre-reserve the memory, exiting if we can't
heap.try_reserve(data.len())
?
;
// Now we know this can't OOM in the middle of our complex work
heap.extend(data.iter());
Ok
(heap.pop())
}
1.0.0
·
Source
pub fn
shrink_to_fit
(&mut self)
Discards as much additional capacity as possible.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap: BinaryHeap<i32> = BinaryHeap::with_capacity(
100
);
assert!
(heap.capacity() >=
100
);
heap.shrink_to_fit();
assert!
(heap.capacity() ==
0
);
1.56.0
·
Source
pub fn
shrink_to
(&mut self, min_capacity:
usize
)
Discards capacity with a lower bound.
The capacity will remain at least as large as both the length
and the supplied value.
If the current capacity is less than the lower limit, this is a no-op.
§
Examples
use
std::collections::BinaryHeap;
let
mut
heap: BinaryHeap<i32> = BinaryHeap::with_capacity(
100
);
assert!
(heap.capacity() >=
100
);
heap.shrink_to(
10
);
assert!
(heap.capacity() >=
10
);
1.80.0
·
Source
pub fn
as_slice
(&self) -> &
[T]
Returns a slice of all values in the underlying vector, in arbitrary
order.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
use
std::io::{
self
, Write};
let
heap = BinaryHeap::from([
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
,
7
]);

io::sink().write(heap.as_slice()).unwrap();
1.5.0
·
Source
pub fn
into_vec
(self) ->
Vec
<T, A>
Consumes the
BinaryHeap
and returns the underlying vector
in arbitrary order.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
heap = BinaryHeap::from([
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
,
7
]);
let
vec = heap.into_vec();
// Will print in some order
for
x
in
vec {
println!
(
"{x}"
);
}
Source
pub fn
allocator
(&self) ->
&A
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Returns a reference to the underlying allocator.
1.0.0
·
Source
pub fn
len
(&self) ->
usize
Returns the length of the binary heap.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
heap = BinaryHeap::from([
1
,
3
]);
assert_eq!
(heap.len(),
2
);
1.0.0
·
Source
pub fn
is_empty
(&self) ->
bool
Checks if the binary heap is empty.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::new();
assert!
(heap.is_empty());

heap.push(
3
);
heap.push(
5
);
heap.push(
1
);
assert!
(!heap.is_empty());
1.6.0
·
Source
pub fn
drain
(&mut self) ->
Drain
<'_, T, A>
ⓘ
Clears the binary heap, returning an iterator over the removed elements
in arbitrary order. If the iterator is dropped before being fully
consumed, it drops the remaining elements in arbitrary order.
The returned iterator keeps a mutable borrow on the heap to optimize
its implementation.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::from([
1
,
3
]);
assert!
(!heap.is_empty());
for
x
in
heap.drain() {
println!
(
"{x}"
);
}
assert!
(heap.is_empty());
1.0.0
·
Source
pub fn
clear
(&mut self)
Drops all items from the binary heap.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
mut
heap = BinaryHeap::from([
1
,
3
]);
assert!
(!heap.is_empty());

heap.clear();
assert!
(heap.is_empty());
Trait Implementations
§
1.0.0
·
Source
§
impl<T, A>
Clone
for
BinaryHeap
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
clone_from
(&mut self, source: &
BinaryHeap
<T, A>)
Overwrites the contents of
self
with a clone of the contents of
source
.
This method is preferred over simply assigning
source.clone()
to
self
,
as it avoids reallocation if possible.
See
Vec::clone_from()
for more details.
Source
§
fn
clone
(&self) ->
BinaryHeap
<T, A>
Returns a copy of the value.
Read more
1.4.0
·
Source
§
impl<T, A>
Debug
for
BinaryHeap
<T, A>
where
    T:
Debug
,
    A:
Allocator
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
BinaryHeap
<T>
where
    T:
Ord
,
Source
§
fn
default
() ->
BinaryHeap
<T>
Creates an empty
BinaryHeap<T>
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
BinaryHeap
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
fn
extend
<I>(&mut self, iter: I)
where
    I:
IntoIterator
<Item = T>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, item: T)
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
BinaryHeap
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
BinaryHeap
<T>
use
std::collections::BinaryHeap;
let
mut
h1 = BinaryHeap::from([
1
,
4
,
2
,
3
]);
let
mut
h2: BinaryHeap<
_
> = [
1
,
4
,
2
,
3
].into();
while let
Some
((a, b)) = h1.pop().zip(h2.pop()) {
assert_eq!
(a, b);
}
1.5.0
·
Source
§
impl<T, A>
From
<
BinaryHeap
<T, A>> for
Vec
<T, A>
where
    A:
Allocator
,
Source
§
fn
from
(heap:
BinaryHeap
<T, A>) ->
Vec
<T, A>
Converts a
BinaryHeap<T>
into a
Vec<T>
.
This conversion requires no data movement or allocation, and has
constant time complexity.
1.5.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
BinaryHeap
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
fn
from
(vec:
Vec
<T, A>) ->
BinaryHeap
<T, A>
Converts a
Vec<T>
into a
BinaryHeap<T>
.
This conversion happens in-place, and has
O
(
n
) time complexity.
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
Source
§
fn
from_iter
<I>(iter: I) ->
BinaryHeap
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
BinaryHeap
<T, A>
where
    A:
Allocator
,
Source
§
fn
into_iter
(self) ->
IntoIter
<T, A>
ⓘ
Creates a consuming iterator, that is, one that moves each value out of
the binary heap in arbitrary order. The binary heap cannot be used
after calling this.
§
Examples
Basic usage:
use
std::collections::BinaryHeap;
let
heap = BinaryHeap::from([
1
,
2
,
3
,
4
]);
// Print 1, 2, 3, 4 in arbitrary order
for
x
in
heap.into_iter() {
// x has type i32, not &i32
println!
(
"{x}"
);
}
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
Auto Trait Implementations
§
§
impl<T, A>
Freeze
for
BinaryHeap
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
RefUnwindSafe
for
BinaryHeap
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
BinaryHeap
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
BinaryHeap
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
BinaryHeap
<T, A>
where
    A:
Unpin
,
    T:
Unpin
,
§
impl<T, A>
UnwindSafe
for
BinaryHeap
<T, A>
where
    A:
UnwindSafe
,
    T:
UnwindSafe
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