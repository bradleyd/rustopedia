VecDeque in std::collections::vec_deque - Rust
std
::
collections
::
vec_deque
Struct
VecDeque
Copy item path
1.0.0
·
Source
pub struct VecDeque<T, A =
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
A double-ended queue implemented with a growable ring buffer.
The “default” usage of this type as a queue is to use
push_back
to add to
the queue, and
pop_front
to remove from the queue.
extend
and
append
push onto the back in this manner, and iterating over
VecDeque
goes front
to back.
A
VecDeque
with a known list of items can be initialized from an array:
use
std::collections::VecDeque;
let
deq = VecDeque::from([-
1
,
0
,
1
]);
Since
VecDeque
is a ring buffer, its elements are not necessarily contiguous
in memory. If you want to access the elements as a single slice, such as for
efficient sorting, you can use
make_contiguous
. It rotates the
VecDeque
so that its elements do not wrap, and returns a mutable slice to the
now-contiguous element sequence.
Implementations
§
Source
§
impl<T>
VecDeque
<T>
1.0.0 (const: 1.68.0)
·
Source
pub const fn
new
() ->
VecDeque
<T>
Creates an empty deque.
§
Examples
use
std::collections::VecDeque;
let
deque: VecDeque<u32> = VecDeque::new();
1.0.0
·
Source
pub fn
with_capacity
(capacity:
usize
) ->
VecDeque
<T>
Creates an empty deque with space for at least
capacity
elements.
§
Examples
use
std::collections::VecDeque;
let
deque: VecDeque<u32> = VecDeque::with_capacity(
10
);
Source
pub fn
try_with_capacity
(
    capacity:
usize
,
) ->
Result
<
VecDeque
<T>,
TryReserveError
>
🔬
This is a nightly-only experimental API. (
try_with_capacity
#91913
)
Creates an empty deque with space for at least
capacity
elements.
§
Errors
Returns an error if the capacity exceeds
isize::MAX
bytes
,
or if the allocator reports allocation failure.
§
Examples
use
std::collections::VecDeque;
let
deque: VecDeque<u32> = VecDeque::try_with_capacity(
10
)
?
;
Source
§
impl<T, A>
VecDeque
<T, A>
where
    A:
Allocator
,
Source
pub const fn
new_in
(alloc: A) ->
VecDeque
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Creates an empty deque.
§
Examples
use
std::collections::VecDeque;
let
deque: VecDeque<u32> = VecDeque::new();
Source
pub fn
with_capacity_in
(capacity:
usize
, alloc: A) ->
VecDeque
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Creates an empty deque with space for at least
capacity
elements.
§
Examples
use
std::collections::VecDeque;
let
deque: VecDeque<u32> = VecDeque::with_capacity(
10
);
1.0.0
·
Source
pub fn
get
(&self, index:
usize
) ->
Option
<
&T
>
Provides a reference to the element at the given index.
Element at index 0 is the front of the queue.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
3
);
buf.push_back(
4
);
buf.push_back(
5
);
buf.push_back(
6
);
assert_eq!
(buf.get(
1
),
Some
(
&
4
));
1.0.0
·
Source
pub fn
get_mut
(&mut self, index:
usize
) ->
Option
<
&mut T
>
Provides a mutable reference to the element at the given index.
Element at index 0 is the front of the queue.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
3
);
buf.push_back(
4
);
buf.push_back(
5
);
buf.push_back(
6
);
assert_eq!
(buf[
1
],
4
);
if let
Some
(elem) = buf.get_mut(
1
) {
*
elem =
7
;
}
assert_eq!
(buf[
1
],
7
);
1.0.0
·
Source
pub fn
swap
(&mut self, i:
usize
, j:
usize
)
Swaps elements at indices
i
and
j
.
i
and
j
may be equal.
Element at index 0 is the front of the queue.
§
Panics
Panics if either index is out of bounds.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
3
);
buf.push_back(
4
);
buf.push_back(
5
);
assert_eq!
(buf, [
3
,
4
,
5
]);
buf.swap(
0
,
2
);
assert_eq!
(buf, [
5
,
4
,
3
]);
1.0.0
·
Source
pub fn
capacity
(&self) ->
usize
Returns the number of elements the deque can hold without
reallocating.
§
Examples
use
std::collections::VecDeque;
let
buf: VecDeque<i32> = VecDeque::with_capacity(
10
);
assert!
(buf.capacity() >=
10
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
more elements to be inserted in the
given deque. Does nothing if the capacity is already sufficient.
Note that the allocator may give the collection more space than it requests. Therefore
capacity can not be relied upon to be precisely minimal. Prefer
reserve
if future
insertions are expected.
§
Panics
Panics if the new capacity overflows
usize
.
§
Examples
use
std::collections::VecDeque;
let
mut
buf: VecDeque<i32> = [
1
].into();
buf.reserve_exact(
10
);
assert!
(buf.capacity() >=
11
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
more elements to be inserted in the given
deque. The collection may reserve more space to speculatively avoid frequent reallocations.
§
Panics
Panics if the new capacity overflows
usize
.
§
Examples
use
std::collections::VecDeque;
let
mut
buf: VecDeque<i32> = [
1
].into();
buf.reserve(
10
);
assert!
(buf.capacity() >=
11
);
1.57.0
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
more elements to
be inserted in the given deque. After calling
try_reserve_exact
,
capacity will be greater than or equal to
self.len() + additional
if
it returns
Ok(())
. Does nothing if the capacity is already sufficient.
Note that the allocator may give the collection more space than it
requests. Therefore, capacity can not be relied upon to be precisely
minimal. Prefer
try_reserve
if future insertions are expected.
§
Errors
If the capacity overflows
usize
, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::collections::TryReserveError;
use
std::collections::VecDeque;
fn
process_data(data:
&
[u32]) ->
Result
<VecDeque<u32>, TryReserveError> {
let
mut
output = VecDeque::new();
// Pre-reserve the memory, exiting if we can't
output.try_reserve_exact(data.len())
?
;
// Now we know this can't OOM(Out-Of-Memory) in the middle of our complex work
output.extend(data.iter().map(|
&
val| {
        val *
2
+
5
// very complicated
}));
Ok
(output)
}
1.57.0
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
more elements to be inserted
in the given deque. The collection may reserve more space to speculatively avoid
frequent reallocations. After calling
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
If the capacity overflows
usize
, or the allocator reports a failure, then an error
is returned.
§
Examples
use
std::collections::TryReserveError;
use
std::collections::VecDeque;
fn
process_data(data:
&
[u32]) ->
Result
<VecDeque<u32>, TryReserveError> {
let
mut
output = VecDeque::new();
// Pre-reserve the memory, exiting if we can't
output.try_reserve(data.len())
?
;
// Now we know this can't OOM in the middle of our complex work
output.extend(data.iter().map(|
&
val| {
        val *
2
+
5
// very complicated
}));
Ok
(output)
}
1.5.0
·
Source
pub fn
shrink_to_fit
(&mut self)
Shrinks the capacity of the deque as much as possible.
It will drop down as close as possible to the length but the allocator may still inform the
deque that there is space for a few more elements.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::with_capacity(
15
);
buf.extend(
0
..
4
);
assert_eq!
(buf.capacity(),
15
);
buf.shrink_to_fit();
assert!
(buf.capacity() >=
4
);
1.56.0
·
Source
pub fn
shrink_to
(&mut self, min_capacity:
usize
)
Shrinks the capacity of the deque with a lower bound.
The capacity will remain at least as large as both the length
and the supplied value.
If the current capacity is less than the lower limit, this is a no-op.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::with_capacity(
15
);
buf.extend(
0
..
4
);
assert_eq!
(buf.capacity(),
15
);
buf.shrink_to(
6
);
assert!
(buf.capacity() >=
6
);
buf.shrink_to(
0
);
assert!
(buf.capacity() >=
4
);
1.16.0
·
Source
pub fn
truncate
(&mut self, len:
usize
)
Shortens the deque, keeping the first
len
elements and dropping
the rest.
If
len
is greater or equal to the deque’s current length, this has
no effect.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
5
);
buf.push_back(
10
);
buf.push_back(
15
);
assert_eq!
(buf, [
5
,
10
,
15
]);
buf.truncate(
1
);
assert_eq!
(buf, [
5
]);
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
iter
(&self) ->
Iter
<'_, T>
ⓘ
Returns a front-to-back iterator.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
5
);
buf.push_back(
3
);
buf.push_back(
4
);
let
b:
&
[
_
] =
&
[
&
5
,
&
3
,
&
4
];
let
c: Vec<
&
i32> = buf.iter().collect();
assert_eq!
(
&
c[..], b);
1.0.0
·
Source
pub fn
iter_mut
(&mut self) ->
IterMut
<'_, T>
ⓘ
Returns a front-to-back iterator that returns mutable references.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
5
);
buf.push_back(
3
);
buf.push_back(
4
);
for
num
in
buf.iter_mut() {
*
num =
*
num -
2
;
}
let
b:
&
[
_
] =
&
[
&mut
3
,
&mut
1
,
&mut
2
];
assert_eq!
(
&
buf.iter_mut().collect::<Vec<
&mut
i32>>()[..], b);
1.5.0
·
Source
pub fn
as_slices
(&self) -> (&
[T]
, &
[T]
)
Returns a pair of slices which contain, in order, the contents of the
deque.
If
make_contiguous
was previously called, all elements of the
deque will be in the first slice and the second slice will be empty.
§
Examples
use
std::collections::VecDeque;
let
mut
deque = VecDeque::new();

deque.push_back(
0
);
deque.push_back(
1
);
deque.push_back(
2
);
assert_eq!
(deque.as_slices(), (
&
[
0
,
1
,
2
][..],
&
[][..]));

deque.push_front(
10
);
deque.push_front(
9
);
assert_eq!
(deque.as_slices(), (
&
[
9
,
10
][..],
&
[
0
,
1
,
2
][..]));
1.5.0
·
Source
pub fn
as_mut_slices
(&mut self) -> (&mut
[T]
, &mut
[T]
)
Returns a pair of slices which contain, in order, the contents of the
deque.
If
make_contiguous
was previously called, all elements of the
deque will be in the first slice and the second slice will be empty.
§
Examples
use
std::collections::VecDeque;
let
mut
deque = VecDeque::new();

deque.push_back(
0
);
deque.push_back(
1
);

deque.push_front(
10
);
deque.push_front(
9
);

deque.as_mut_slices().
0
[
0
] =
42
;
deque.as_mut_slices().
1
[
0
] =
24
;
assert_eq!
(deque.as_slices(), (
&
[
42
,
10
][..],
&
[
24
,
1
][..]));
1.0.0
·
Source
pub fn
len
(&self) ->
usize
Returns the number of elements in the deque.
§
Examples
use
std::collections::VecDeque;
let
mut
deque = VecDeque::new();
assert_eq!
(deque.len(),
0
);
deque.push_back(
1
);
assert_eq!
(deque.len(),
1
);
1.0.0
·
Source
pub fn
is_empty
(&self) ->
bool
Returns
true
if the deque is empty.
§
Examples
use
std::collections::VecDeque;
let
mut
deque = VecDeque::new();
assert!
(deque.is_empty());
deque.push_front(
1
);
assert!
(!deque.is_empty());
1.51.0
·
Source
pub fn
range
<R>(&self, range: R) ->
Iter
<'_, T>
ⓘ
where
    R:
RangeBounds
<
usize
>,
Creates an iterator that covers the specified range in the deque.
§
Panics
Panics if the starting point is greater than the end point or if
the end point is greater than the length of the deque.
§
Examples
use
std::collections::VecDeque;
let
deque: VecDeque<
_
> = [
1
,
2
,
3
].into();
let
range = deque.range(
2
..).copied().collect::<VecDeque<
_
>>();
assert_eq!
(range, [
3
]);
// A full range covers all contents
let
all = deque.range(..);
assert_eq!
(all.len(),
3
);
1.51.0
·
Source
pub fn
range_mut
<R>(&mut self, range: R) ->
IterMut
<'_, T>
ⓘ
where
    R:
RangeBounds
<
usize
>,
Creates an iterator that covers the specified mutable range in the deque.
§
Panics
Panics if the starting point is greater than the end point or if
the end point is greater than the length of the deque.
§
Examples
use
std::collections::VecDeque;
let
mut
deque: VecDeque<
_
> = [
1
,
2
,
3
].into();
for
v
in
deque.range_mut(
2
..) {
*
v
*
=
2
;
}
assert_eq!
(deque, [
1
,
2
,
6
]);
// A full range covers all contents
for
v
in
deque.range_mut(..) {
*
v
*
=
2
;
}
assert_eq!
(deque, [
2
,
4
,
12
]);
1.6.0
·
Source
pub fn
drain
<R>(&mut self, range: R) ->
Drain
<'_, T, A>
ⓘ
where
    R:
RangeBounds
<
usize
>,
Removes the specified range from the deque in bulk, returning all
removed elements as an iterator. If the iterator is dropped before
being fully consumed, it drops the remaining removed elements.
The returned iterator keeps a mutable borrow on the queue to optimize
its implementation.
§
Panics
Panics if the starting point is greater than the end point or if
the end point is greater than the length of the deque.
§
Leaking
If the returned iterator goes out of scope without being dropped (due to
mem::forget
, for example), the deque may have lost and leaked
elements arbitrarily, including elements outside the range.
§
Examples
use
std::collections::VecDeque;
let
mut
deque: VecDeque<
_
> = [
1
,
2
,
3
].into();
let
drained = deque.drain(
2
..).collect::<VecDeque<
_
>>();
assert_eq!
(drained, [
3
]);
assert_eq!
(deque, [
1
,
2
]);
// A full range clears all contents, like `clear()` does
deque.drain(..);
assert!
(deque.is_empty());
1.0.0
·
Source
pub fn
clear
(&mut self)
Clears the deque, removing all values.
§
Examples
use
std::collections::VecDeque;
let
mut
deque = VecDeque::new();
deque.push_back(
1
);
deque.clear();
assert!
(deque.is_empty());
1.12.0
·
Source
pub fn
contains
(&self, x:
&T
) ->
bool
where
    T:
PartialEq
,
Returns
true
if the deque contains an element equal to the
given value.
This operation is
O
(
n
).
Note that if you have a sorted
VecDeque
,
binary_search
may be faster.
§
Examples
use
std::collections::VecDeque;
let
mut
deque: VecDeque<u32> = VecDeque::new();

deque.push_back(
0
);
deque.push_back(
1
);
assert_eq!
(deque.contains(
&
1
),
true
);
assert_eq!
(deque.contains(
&
10
),
false
);
1.0.0
·
Source
pub fn
front
(&self) ->
Option
<
&T
>
Provides a reference to the front element, or
None
if the deque is
empty.
§
Examples
use
std::collections::VecDeque;
let
mut
d = VecDeque::new();
assert_eq!
(d.front(),
None
);

d.push_back(
1
);
d.push_back(
2
);
assert_eq!
(d.front(),
Some
(
&
1
));
1.0.0
·
Source
pub fn
front_mut
(&mut self) ->
Option
<
&mut T
>
Provides a mutable reference to the front element, or
None
if the
deque is empty.
§
Examples
use
std::collections::VecDeque;
let
mut
d = VecDeque::new();
assert_eq!
(d.front_mut(),
None
);

d.push_back(
1
);
d.push_back(
2
);
match
d.front_mut() {
Some
(x) =>
*
x =
9
,
None
=> (),
}
assert_eq!
(d.front(),
Some
(
&
9
));
1.0.0
·
Source
pub fn
back
(&self) ->
Option
<
&T
>
Provides a reference to the back element, or
None
if the deque is
empty.
§
Examples
use
std::collections::VecDeque;
let
mut
d = VecDeque::new();
assert_eq!
(d.back(),
None
);

d.push_back(
1
);
d.push_back(
2
);
assert_eq!
(d.back(),
Some
(
&
2
));
1.0.0
·
Source
pub fn
back_mut
(&mut self) ->
Option
<
&mut T
>
Provides a mutable reference to the back element, or
None
if the
deque is empty.
§
Examples
use
std::collections::VecDeque;
let
mut
d = VecDeque::new();
assert_eq!
(d.back(),
None
);

d.push_back(
1
);
d.push_back(
2
);
match
d.back_mut() {
Some
(x) =>
*
x =
9
,
None
=> (),
}
assert_eq!
(d.back(),
Some
(
&
9
));
1.0.0
·
Source
pub fn
pop_front
(&mut self) ->
Option
<T>
Removes the first element and returns it, or
None
if the deque is
empty.
§
Examples
use
std::collections::VecDeque;
let
mut
d = VecDeque::new();
d.push_back(
1
);
d.push_back(
2
);
assert_eq!
(d.pop_front(),
Some
(
1
));
assert_eq!
(d.pop_front(),
Some
(
2
));
assert_eq!
(d.pop_front(),
None
);
1.0.0
·
Source
pub fn
pop_back
(&mut self) ->
Option
<T>
Removes the last element from the deque and returns it, or
None
if
it is empty.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
assert_eq!
(buf.pop_back(),
None
);
buf.push_back(
1
);
buf.push_back(
3
);
assert_eq!
(buf.pop_back(),
Some
(
3
));
Source
pub fn
pop_front_if
(
    &mut self,
    predicate: impl
FnOnce
(
&mut T
) ->
bool
,
) ->
Option
<T>
🔬
This is a nightly-only experimental API. (
vec_deque_pop_if
#135889
)
Removes and returns the first element from the deque if the predicate
returns
true
, or
None
if the predicate returns false or the deque
is empty (the predicate will not be called in that case).
§
Examples
#![feature(vec_deque_pop_if)]
use
std::collections::VecDeque;
let
mut
deque: VecDeque<i32> =
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
].into();
let
pred = |x:
&mut
i32|
*
x %
2
==
0
;
assert_eq!
(deque.pop_front_if(pred),
Some
(
0
));
assert_eq!
(deque, [
1
,
2
,
3
,
4
]);
assert_eq!
(deque.pop_front_if(pred),
None
);
Source
pub fn
pop_back_if
(
    &mut self,
    predicate: impl
FnOnce
(
&mut T
) ->
bool
,
) ->
Option
<T>
🔬
This is a nightly-only experimental API. (
vec_deque_pop_if
#135889
)
Removes and returns the last element from the deque if the predicate
returns
true
, or
None
if the predicate returns false or the deque
is empty (the predicate will not be called in that case).
§
Examples
#![feature(vec_deque_pop_if)]
use
std::collections::VecDeque;
let
mut
deque: VecDeque<i32> =
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
].into();
let
pred = |x:
&mut
i32|
*
x %
2
==
0
;
assert_eq!
(deque.pop_back_if(pred),
Some
(
4
));
assert_eq!
(deque, [
0
,
1
,
2
,
3
]);
assert_eq!
(deque.pop_back_if(pred),
None
);
1.0.0
·
Source
pub fn
push_front
(&mut self, value: T)
Prepends an element to the deque.
§
Examples
use
std::collections::VecDeque;
let
mut
d = VecDeque::new();
d.push_front(
1
);
d.push_front(
2
);
assert_eq!
(d.front(),
Some
(
&
2
));
1.0.0
·
Source
pub fn
push_back
(&mut self, value: T)
Appends an element to the back of the deque.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
1
);
buf.push_back(
3
);
assert_eq!
(
3
,
*
buf.back().unwrap());
1.5.0
·
Source
pub fn
swap_remove_front
(&mut self, index:
usize
) ->
Option
<T>
Removes an element from anywhere in the deque and returns it,
replacing it with the first element.
This does not preserve ordering, but is
O
(1).
Returns
None
if
index
is out of bounds.
Element at index 0 is the front of the queue.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
assert_eq!
(buf.swap_remove_front(
0
),
None
);
buf.push_back(
1
);
buf.push_back(
2
);
buf.push_back(
3
);
assert_eq!
(buf, [
1
,
2
,
3
]);
assert_eq!
(buf.swap_remove_front(
2
),
Some
(
3
));
assert_eq!
(buf, [
2
,
1
]);
1.5.0
·
Source
pub fn
swap_remove_back
(&mut self, index:
usize
) ->
Option
<T>
Removes an element from anywhere in the deque and returns it,
replacing it with the last element.
This does not preserve ordering, but is
O
(1).
Returns
None
if
index
is out of bounds.
Element at index 0 is the front of the queue.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
assert_eq!
(buf.swap_remove_back(
0
),
None
);
buf.push_back(
1
);
buf.push_back(
2
);
buf.push_back(
3
);
assert_eq!
(buf, [
1
,
2
,
3
]);
assert_eq!
(buf.swap_remove_back(
0
),
Some
(
1
));
assert_eq!
(buf, [
3
,
2
]);
1.5.0
·
Source
pub fn
insert
(&mut self, index:
usize
, value: T)
Inserts an element at
index
within the deque, shifting all elements
with indices greater than or equal to
index
towards the back.
Element at index 0 is the front of the queue.
§
Panics
Panics if
index
is strictly greater than deque’s length
§
Examples
use
std::collections::VecDeque;
let
mut
vec_deque = VecDeque::new();
vec_deque.push_back(
'a'
);
vec_deque.push_back(
'b'
);
vec_deque.push_back(
'c'
);
assert_eq!
(vec_deque,
&
[
'a'
,
'b'
,
'c'
]);

vec_deque.insert(
1
,
'd'
);
assert_eq!
(vec_deque,
&
[
'a'
,
'd'
,
'b'
,
'c'
]);

vec_deque.insert(
4
,
'e'
);
assert_eq!
(vec_deque,
&
[
'a'
,
'd'
,
'b'
,
'c'
,
'e'
]);
1.0.0
·
Source
pub fn
remove
(&mut self, index:
usize
) ->
Option
<T>
Removes and returns the element at
index
from the deque.
Whichever end is closer to the removal point will be moved to make
room, and all the affected elements will be moved to new positions.
Returns
None
if
index
is out of bounds.
Element at index 0 is the front of the queue.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
'a'
);
buf.push_back(
'b'
);
buf.push_back(
'c'
);
assert_eq!
(buf, [
'a'
,
'b'
,
'c'
]);
assert_eq!
(buf.remove(
1
),
Some
(
'b'
));
assert_eq!
(buf, [
'a'
,
'c'
]);
1.4.0
·
Source
pub fn
split_off
(&mut self, at:
usize
) ->
VecDeque
<T, A>
where
    A:
Clone
,
Splits the deque into two at the given index.
Returns a newly allocated
VecDeque
.
self
contains elements
[0, at)
,
and the returned deque contains elements
[at, len)
.
Note that the capacity of
self
does not change.
Element at index 0 is the front of the queue.
§
Panics
Panics if
at > len
.
§
Examples
use
std::collections::VecDeque;
let
mut
buf: VecDeque<
_
> = [
'a'
,
'b'
,
'c'
].into();
let
buf2 = buf.split_off(
1
);
assert_eq!
(buf, [
'a'
]);
assert_eq!
(buf2, [
'b'
,
'c'
]);
1.4.0
·
Source
pub fn
append
(&mut self, other: &mut
VecDeque
<T, A>)
Moves all the elements of
other
into
self
, leaving
other
empty.
§
Panics
Panics if the new number of elements in self overflows a
usize
.
§
Examples
use
std::collections::VecDeque;
let
mut
buf: VecDeque<
_
> = [
1
,
2
].into();
let
mut
buf2: VecDeque<
_
> = [
3
,
4
].into();
buf.append(
&mut
buf2);
assert_eq!
(buf, [
1
,
2
,
3
,
4
]);
assert_eq!
(buf2, []);
1.4.0
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
returns false.
This method operates in place, visiting each element exactly once in the
original order, and preserves the order of the retained elements.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.extend(
1
..
5
);
buf.retain(|
&
x| x %
2
==
0
);
assert_eq!
(buf, [
2
,
4
]);
Because the elements are visited exactly once in the original order,
external state may be used to decide which elements to keep.
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.extend(
1
..
6
);
let
keep = [
false
,
true
,
true
,
false
,
true
];
let
mut
iter = keep.iter();
buf.retain(|
_
|
*
iter.next().unwrap());
assert_eq!
(buf, [
2
,
3
,
5
]);
1.61.0
·
Source
pub fn
retain_mut
<F>(&mut self, f: F)
where
    F:
FnMut
(
&mut T
) ->
bool
,
Retains only the elements specified by the predicate.
In other words, remove all elements
e
for which
f(&mut e)
returns false.
This method operates in place, visiting each element exactly once in the
original order, and preserves the order of the retained elements.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.extend(
1
..
5
);
buf.retain_mut(|x|
if
*
x %
2
==
0
{
*
x +=
1
;
true
}
else
{
false
});
assert_eq!
(buf, [
3
,
5
]);
1.33.0
·
Source
pub fn
resize_with
(&mut self, new_len:
usize
, generator: impl
FnMut
() -> T)
Modifies the deque in-place so that
len()
is equal to
new_len
,
either by removing excess elements from the back or by appending
elements generated by calling
generator
to the back.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
5
);
buf.push_back(
10
);
buf.push_back(
15
);
assert_eq!
(buf, [
5
,
10
,
15
]);

buf.resize_with(
5
, Default::default);
assert_eq!
(buf, [
5
,
10
,
15
,
0
,
0
]);

buf.resize_with(
2
, ||
unreachable!
());
assert_eq!
(buf, [
5
,
10
]);
let
mut
state =
100
;
buf.resize_with(
5
, || { state +=
1
; state });
assert_eq!
(buf, [
5
,
10
,
101
,
102
,
103
]);
1.48.0
·
Source
pub fn
make_contiguous
(&mut self) -> &mut
[T]
Rearranges the internal storage of this deque so it is one contiguous
slice, which is then returned.
This method does not allocate and does not change the order of the
inserted elements. As it returns a mutable slice, this can be used to
sort a deque.
Once the internal storage is contiguous, the
as_slices
and
as_mut_slices
methods will return the entire contents of the
deque in a single slice.
§
Examples
Sorting the content of a deque.
use
std::collections::VecDeque;
let
mut
buf = VecDeque::with_capacity(
15
);

buf.push_back(
2
);
buf.push_back(
1
);
buf.push_front(
3
);
// sorting the deque
buf.make_contiguous().sort();
assert_eq!
(buf.as_slices(), (
&
[
1
,
2
,
3
]
as
&
[
_
],
&
[]
as
&
[
_
]));
// sorting it in reverse order
buf.make_contiguous().sort_by(|a, b| b.cmp(a));
assert_eq!
(buf.as_slices(), (
&
[
3
,
2
,
1
]
as
&
[
_
],
&
[]
as
&
[
_
]));
Getting immutable access to the contiguous slice.
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();

buf.push_back(
2
);
buf.push_back(
1
);
buf.push_front(
3
);

buf.make_contiguous();
if let
(slice,
&
[]) = buf.as_slices() {
// we can now be sure that `slice` contains all elements of the deque,
    // while still having immutable access to `buf`.
assert_eq!
(buf.len(), slice.len());
assert_eq!
(slice,
&
[
3
,
2
,
1
]
as
&
[
_
]);
}
1.36.0
·
Source
pub fn
rotate_left
(&mut self, n:
usize
)
Rotates the double-ended queue
n
places to the left.
Equivalently,
Rotates item
n
into the first position.
Pops the first
n
items and pushes them to the end.
Rotates
len() - n
places to the right.
§
Panics
If
n
is greater than
len()
. Note that
n == len()
does
not
panic and is a no-op rotation.
§
Complexity
Takes
*O*(min(n, len() - n))
time and no extra space.
§
Examples
use
std::collections::VecDeque;
let
mut
buf: VecDeque<
_
> = (
0
..
10
).collect();

buf.rotate_left(
3
);
assert_eq!
(buf, [
3
,
4
,
5
,
6
,
7
,
8
,
9
,
0
,
1
,
2
]);
for
i
in
1
..
10
{
assert_eq!
(i *
3
%
10
, buf[
0
]);
    buf.rotate_left(
3
);
}
assert_eq!
(buf, [
0
,
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
,
8
,
9
]);
1.36.0
·
Source
pub fn
rotate_right
(&mut self, n:
usize
)
Rotates the double-ended queue
n
places to the right.
Equivalently,
Rotates the first item into position
n
.
Pops the last
n
items and pushes them to the front.
Rotates
len() - n
places to the left.
§
Panics
If
n
is greater than
len()
. Note that
n == len()
does
not
panic and is a no-op rotation.
§
Complexity
Takes
*O*(min(n, len() - n))
time and no extra space.
§
Examples
use
std::collections::VecDeque;
let
mut
buf: VecDeque<
_
> = (
0
..
10
).collect();

buf.rotate_right(
3
);
assert_eq!
(buf, [
7
,
8
,
9
,
0
,
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
for
i
in
1
..
10
{
assert_eq!
(
0
, buf[i *
3
%
10
]);
    buf.rotate_right(
3
);
}
assert_eq!
(buf, [
0
,
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
,
8
,
9
]);
1.54.0
·
Source
pub fn
binary_search
(&self, x:
&T
) ->
Result
<
usize
,
usize
>
where
    T:
Ord
,
Binary searches this
VecDeque
for a given element.
If the
VecDeque
is not sorted, the returned result is unspecified and
meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. If the value is not found then
Result::Err
is returned, containing the index where a matching
element could be inserted while maintaining sorted order.
See also
binary_search_by
,
binary_search_by_key
, and
partition_point
.
§
Examples
Looks up a series of four elements. The first is found, with a
uniquely determined position; the second and third are not
found; the fourth could match any position in
[1, 4]
.
use
std::collections::VecDeque;
let
deque: VecDeque<
_
> = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
].into();
assert_eq!
(deque.binary_search(
&
13
),
Ok
(
9
));
assert_eq!
(deque.binary_search(
&
4
),
Err
(
7
));
assert_eq!
(deque.binary_search(
&
100
),
Err
(
13
));
let
r = deque.binary_search(
&
1
);
assert!
(
matches!
(r,
Ok
(
1
..=
4
)));
If you want to insert an item to a sorted deque, while maintaining
sort order, consider using
partition_point
:
use
std::collections::VecDeque;
let
mut
deque: VecDeque<
_
> = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
].into();
let
num =
42
;
let
idx = deque.partition_point(|
&
x| x <= num);
// If `num` is unique, `s.partition_point(|&x| x < num)` (with `<`) is equivalent to
// `s.binary_search(&num).unwrap_or_else(|x| x)`, but using `<=` may allow `insert`
// to shift less elements.
deque.insert(idx, num);
assert_eq!
(deque,
&
[
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
42
,
55
]);
1.54.0
·
Source
pub fn
binary_search_by
<'a, F>(&'a self, f: F) ->
Result
<
usize
,
usize
>
where
    F:
FnMut
(
&'a T
) ->
Ordering
,
Binary searches this
VecDeque
with a comparator function.
The comparator function should return an order code that indicates
whether its argument is
Less
,
Equal
or
Greater
the desired
target.
If the
VecDeque
is not sorted or if the comparator function does not
implement an order consistent with the sort order of the underlying
VecDeque
, the returned result is unspecified and meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. If the value is not found then
Result::Err
is returned, containing the index where a matching
element could be inserted while maintaining sorted order.
See also
binary_search
,
binary_search_by_key
, and
partition_point
.
§
Examples
Looks up a series of four elements. The first is found, with a
uniquely determined position; the second and third are not
found; the fourth could match any position in
[1, 4]
.
use
std::collections::VecDeque;
let
deque: VecDeque<
_
> = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
].into();
assert_eq!
(deque.binary_search_by(|x| x.cmp(
&
13
)),
Ok
(
9
));
assert_eq!
(deque.binary_search_by(|x| x.cmp(
&
4
)),
Err
(
7
));
assert_eq!
(deque.binary_search_by(|x| x.cmp(
&
100
)),
Err
(
13
));
let
r = deque.binary_search_by(|x| x.cmp(
&
1
));
assert!
(
matches!
(r,
Ok
(
1
..=
4
)));
1.54.0
·
Source
pub fn
binary_search_by_key
<'a, B, F>(
    &'a self,
    b:
&B
,
    f: F,
) ->
Result
<
usize
,
usize
>
where
    F:
FnMut
(
&'a T
) -> B,
    B:
Ord
,
Binary searches this
VecDeque
with a key extraction function.
Assumes that the deque is sorted by the key, for instance with
make_contiguous().sort_by_key()
using the same key extraction function.
If the deque is not sorted by the key, the returned result is
unspecified and meaningless.
If the value is found then
Result::Ok
is returned, containing the
index of the matching element. If there are multiple matches, then any
one of the matches could be returned. If the value is not found then
Result::Err
is returned, containing the index where a matching
element could be inserted while maintaining sorted order.
See also
binary_search
,
binary_search_by
, and
partition_point
.
§
Examples
Looks up a series of four elements in a slice of pairs sorted by
their second elements. The first is found, with a uniquely
determined position; the second and third are not found; the
fourth could match any position in
[1, 4]
.
use
std::collections::VecDeque;
let
deque: VecDeque<
_
> = [(
0
,
0
), (
2
,
1
), (
4
,
1
), (
5
,
1
),
         (
3
,
1
), (
1
,
2
), (
2
,
3
), (
4
,
5
), (
5
,
8
), (
3
,
13
),
         (
1
,
21
), (
2
,
34
), (
4
,
55
)].into();
assert_eq!
(deque.binary_search_by_key(
&
13
, |
&
(a, b)| b),
Ok
(
9
));
assert_eq!
(deque.binary_search_by_key(
&
4
, |
&
(a, b)| b),
Err
(
7
));
assert_eq!
(deque.binary_search_by_key(
&
100
, |
&
(a, b)| b),
Err
(
13
));
let
r = deque.binary_search_by_key(
&
1
, |
&
(a, b)| b);
assert!
(
matches!
(r,
Ok
(
1
..=
4
)));
1.54.0
·
Source
pub fn
partition_point
<P>(&self, pred: P) ->
usize
where
    P:
FnMut
(
&T
) ->
bool
,
Returns the index of the partition point according to the given predicate
(the index of the first element of the second partition).
The deque is assumed to be partitioned according to the given predicate.
This means that all elements for which the predicate returns true are at the start of the deque
and all elements for which the predicate returns false are at the end.
For example,
[7, 15, 3, 5, 4, 12, 6]
is partitioned under the predicate
x % 2 != 0
(all odd numbers are at the start, all even at the end).
If the deque is not partitioned, the returned result is unspecified and meaningless,
as this method performs a kind of binary search.
See also
binary_search
,
binary_search_by
, and
binary_search_by_key
.
§
Examples
use
std::collections::VecDeque;
let
deque: VecDeque<
_
> = [
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
6
,
7
].into();
let
i = deque.partition_point(|
&
x| x <
5
);
assert_eq!
(i,
4
);
assert!
(deque.iter().take(i).all(|
&
x| x <
5
));
assert!
(deque.iter().skip(i).all(|
&
x| !(x <
5
)));
If you want to insert an item to a sorted deque, while maintaining
sort order:
use
std::collections::VecDeque;
let
mut
deque: VecDeque<
_
> = [
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
55
].into();
let
num =
42
;
let
idx = deque.partition_point(|
&
x| x < num);
deque.insert(idx, num);
assert_eq!
(deque,
&
[
0
,
1
,
1
,
1
,
1
,
2
,
3
,
5
,
8
,
13
,
21
,
34
,
42
,
55
]);
Source
§
impl<T, A>
VecDeque
<T, A>
where
    T:
Clone
,
    A:
Allocator
,
1.16.0
·
Source
pub fn
resize
(&mut self, new_len:
usize
, value: T)
Modifies the deque in-place so that
len()
is equal to new_len,
either by removing excess elements from the back or by appending clones of
value
to the back.
§
Examples
use
std::collections::VecDeque;
let
mut
buf = VecDeque::new();
buf.push_back(
5
);
buf.push_back(
10
);
buf.push_back(
15
);
assert_eq!
(buf, [
5
,
10
,
15
]);

buf.resize(
2
,
0
);
assert_eq!
(buf, [
5
,
10
]);

buf.resize(
5
,
20
);
assert_eq!
(buf, [
5
,
10
,
20
,
20
,
20
]);
Trait Implementations
§
1.75.0
·
Source
§
impl<A:
Allocator
>
BufRead
for
VecDeque
<
u8
, A>
BufRead is implemented for
VecDeque<u8>
by reading bytes from the front of the
VecDeque
.
Source
§
fn
fill_buf
(&mut self) ->
Result
<&[
u8
]>
Returns the contents of the “front” slice as returned by
as_slices
. If the contained byte slices of the
VecDeque
are
discontiguous, multiple calls to
fill_buf
will be needed to read the entire content.
Source
§
fn
consume
(&mut self, amt:
usize
)
Marks the given
amount
of additional bytes from the internal buffer as having been read.
Subsequent calls to
read
only return bytes that have not been marked as read.
Read more
Source
§
fn
has_data_left
(&mut self) ->
Result
<
bool
>
🔬
This is a nightly-only experimental API. (
buf_read_has_data_left
#86423
)
Checks if there is any data left to be
read
.
Read more
1.0.0
·
Source
§
fn
read_until
(&mut self, byte:
u8
, buf: &mut
Vec
<
u8
>) ->
Result
<
usize
>
Reads all bytes into
buf
until the delimiter
byte
or EOF is reached.
Read more
1.83.0
·
Source
§
fn
skip_until
(&mut self, byte:
u8
) ->
Result
<
usize
>
Skips all bytes until the delimiter
byte
or EOF is reached.
Read more
1.0.0
·
Source
§
fn
read_line
(&mut self, buf: &mut
String
) ->
Result
<
usize
>
Reads all bytes until a newline (the
0xA
byte) is reached, and append
them to the provided
String
buffer.
Read more
1.0.0
·
Source
§
fn
split
(self, byte:
u8
) ->
Split
<Self>
ⓘ
where
    Self:
Sized
,
Returns an iterator over the contents of this reader split on the byte
byte
.
Read more
1.0.0
·
Source
§
fn
lines
(self) ->
Lines
<Self>
ⓘ
where
    Self:
Sized
,
Returns an iterator over the lines of this reader.
Read more
1.0.0
·
Source
§
impl<T, A>
Clone
for
VecDeque
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
VecDeque
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
Source
§
fn
clone
(&self) ->
VecDeque
<T, A>
Returns a copy of the value.
Read more
1.0.0
·
Source
§
impl<T, A>
Debug
for
VecDeque
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
VecDeque
<T>
Source
§
fn
default
() ->
VecDeque
<T>
Creates an empty deque.
1.0.0
·
Source
§
impl<T, A>
Drop
for
VecDeque
<T, A>
where
    A:
Allocator
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
VecDeque
<T, A>
where
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
VecDeque
<T>
Source
§
fn
from
(arr:
[T; N]
) ->
VecDeque
<T>
Converts a
[T; N]
into a
VecDeque<T>
.
use
std::collections::VecDeque;
let
deq1 = VecDeque::from([
1
,
2
,
3
,
4
]);
let
deq2: VecDeque<
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
(deq1, deq2);
1.10.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
fn
from
(other:
Vec
<T, A>) ->
VecDeque
<T, A>
Turn a
Vec<T>
into a
VecDeque<T>
.
This conversion is guaranteed to run in
O
(1) time
and to not re-allocate the
Vec
’s buffer or allocate
any additional memory.
1.10.0
·
Source
§
impl<T, A>
From
<
VecDeque
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
(other:
VecDeque
<T, A>) ->
Vec
<T, A>
Turn a
VecDeque<T>
into a
Vec<T>
.
This never needs to re-allocate, but does need to do
O
(
n
) data movement if
the circular buffer doesn’t happen to be at the beginning of the allocation.
§
Examples
use
std::collections::VecDeque;
// This one is *O*(1).
let
deque: VecDeque<
_
> = (
1
..
5
).collect();
let
ptr = deque.as_slices().
0
.as_ptr();
let
vec = Vec::from(deque);
assert_eq!
(vec, [
1
,
2
,
3
,
4
]);
assert_eq!
(vec.as_ptr(), ptr);
// This one needs data rearranging.
let
mut
deque: VecDeque<
_
> = (
1
..
5
).collect();
deque.push_front(
9
);
deque.push_front(
8
);
let
ptr = deque.as_slices().
1
.as_ptr();
let
vec = Vec::from(deque);
assert_eq!
(vec, [
8
,
9
,
1
,
2
,
3
,
4
]);
assert_eq!
(vec.as_ptr(), ptr);
1.0.0
·
Source
§
impl<T>
FromIterator
<T> for
VecDeque
<T>
Source
§
fn
from_iter
<I>(iter: I) ->
VecDeque
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
VecDeque
<T, A>
where
    T:
Hash
,
    A:
Allocator
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
impl<T, A>
Index
<
usize
> for
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
type
Output
= T
The returned type after indexing.
Source
§
fn
index
(&self, index:
usize
) ->
&T
Performs the indexing (
container[index]
) operation.
Read more
1.0.0
·
Source
§
impl<T, A>
IndexMut
<
usize
> for
VecDeque
<T, A>
where
    A:
Allocator
,
Source
§
fn
index_mut
(&mut self, index:
usize
) ->
&mut T
Performs the mutable indexing (
container[index]
) operation.
Read more
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
The type of the elements being iterated over.
Source
§
type
IntoIter
=
IterMut
<'a, T>
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) ->
IterMut
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
VecDeque
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
Consumes the deque into a front-to-back iterator yielding elements by
value.
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
VecDeque
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
cmp
(&self, other: &
VecDeque
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
1.17.0
·
Source
§
impl<T, U, A>
PartialEq
<&
[U]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&
[U]
) ->
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
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<&
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&
[U; N]
) ->
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
1.17.0
·
Source
§
impl<T, U, A>
PartialEq
<&mut
[U]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&mut
[U]
) ->
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
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<&mut
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &&mut
[U; N]
) ->
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
1.17.0
·
Source
§
impl<T, U, A, const N:
usize
>
PartialEq
<
[U; N]
> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
[U; N]
) ->
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
1.17.0
·
Source
§
impl<T, U, A>
PartialEq
<
Vec
<U, A>> for
VecDeque
<T, A>
where
    A:
Allocator
,
    T:
PartialEq
<U>,
Source
§
fn
eq
(&self, other: &
Vec
<U, A>) ->
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
PartialEq
for
VecDeque
<T, A>
where
    T:
PartialEq
,
    A:
Allocator
,
Source
§
fn
eq
(&self, other: &
VecDeque
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
VecDeque
<T, A>
where
    T:
PartialOrd
,
    A:
Allocator
,
Source
§
fn
partial_cmp
(&self, other: &
VecDeque
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
1.63.0
·
Source
§
impl<A:
Allocator
>
Read
for
VecDeque
<
u8
, A>
Read is implemented for
VecDeque<u8>
by consuming bytes from the front of the
VecDeque
.
Source
§
fn
read
(&mut self, buf: &mut [
u8
]) ->
Result
<
usize
>
Fill
buf
with the contents of the “front” slice as returned by
as_slices
. If the contained byte slices of the
VecDeque
are
discontiguous, multiple calls to
read
will be needed to read the entire content.
Source
§
fn
read_exact
(&mut self, buf: &mut [
u8
]) ->
Result
<
()
>
Reads the exact number of bytes required to fill
buf
.
Read more
Source
§
fn
read_buf
(&mut self, cursor:
BorrowedCursor
<'_>) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
read_buf
#78485
)
Pull some bytes from this source into the specified buffer.
Read more
Source
§
fn
read_buf_exact
(&mut self, cursor:
BorrowedCursor
<'_>) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
read_buf
#78485
)
Reads the exact number of bytes required to fill
cursor
.
Read more
Source
§
fn
read_to_end
(&mut self, buf: &mut
Vec
<
u8
>) ->
Result
<
usize
>
Reads all bytes until EOF in this source, placing them into
buf
.
Read more
Source
§
fn
read_to_string
(&mut self, buf: &mut
String
) ->
Result
<
usize
>
Reads all bytes until EOF in this source, appending them to
buf
.
Read more
1.36.0
·
Source
§
fn
read_vectored
(&mut self, bufs: &mut [
IoSliceMut
<'_>]) ->
Result
<
usize
>
Like
read
, except that it reads into a slice of buffers.
Read more
Source
§
fn
is_read_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if this
Read
er has an efficient
read_vectored
implementation.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adaptor for this instance of
Read
.
Read more
1.0.0
·
Source
§
fn
bytes
(self) ->
Bytes
<Self>
ⓘ
where
    Self:
Sized
,
Transforms this
Read
instance to an
Iterator
over its bytes.
Read more
1.0.0
·
Source
§
fn
chain
<R:
Read
>(self, next: R) ->
Chain
<Self, R>
ⓘ
where
    Self:
Sized
,
Creates an adapter which will chain this stream with another.
Read more
1.0.0
·
Source
§
fn
take
(self, limit:
u64
) ->
Take
<Self>
ⓘ
where
    Self:
Sized
,
Creates an adapter which will read at most
limit
bytes from it.
Read more
1.63.0
·
Source
§
impl<A:
Allocator
>
Write
for
VecDeque
<
u8
, A>
Write is implemented for
VecDeque<u8>
by appending to the
VecDeque
, growing it as needed.
Source
§
fn
write
(&mut self, buf: &[
u8
]) ->
Result
<
usize
>
Writes a buffer into this writer, returning how many bytes were written.
Read more
Source
§
fn
write_vectored
(&mut self, bufs: &[
IoSlice
<'_>]) ->
Result
<
usize
>
Like
write
, except that it writes from a slice of buffers.
Read more
Source
§
fn
is_write_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if this
Write
r has an efficient
write_vectored
implementation.
Read more
Source
§
fn
write_all
(&mut self, buf: &[
u8
]) ->
Result
<
()
>
Attempts to write an entire buffer into this writer.
Read more
Source
§
fn
write_all_vectored
(&mut self, bufs: &mut [
IoSlice
<'_>]) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
write_all_vectored
#70436
)
Attempts to write multiple buffers into this writer.
Read more
Source
§
fn
flush
(&mut self) ->
Result
<
()
>
Flushes this output stream, ensuring that all intermediately buffered
contents reach their destination.
Read more
1.0.0
·
Source
§
fn
write_fmt
(&mut self, args:
Arguments
<'_>) ->
Result
<
()
>
Writes a formatted string into this writer, returning any error
encountered.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adapter for this instance of
Write
.
Read more
1.0.0
·
Source
§
impl<T, A>
Eq
for
VecDeque
<T, A>
where
    T:
Eq
,
    A:
Allocator
,
Auto Trait Implementations
§
§
impl<T, A>
Freeze
for
VecDeque
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
RefUnwindSafe
for
VecDeque
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
VecDeque
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
VecDeque
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
VecDeque
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
VecDeque
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