LinkedList in std::collections::linked_list - Rust
std
::
collections
::
linked_list
Struct
LinkedList
Copy item path
1.0.0
·
Source
pub struct LinkedList<T, A =
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
A doubly-linked list with owned nodes.
The
LinkedList
allows pushing and popping elements at either end
in constant time.
A
LinkedList
with a known list of items can be initialized from an array:
use
std::collections::LinkedList;
let
list = LinkedList::from([
1
,
2
,
3
]);
NOTE: It is almost always better to use
Vec
or
VecDeque
because
array-based containers are generally faster,
more memory efficient, and make better use of CPU cache.
Implementations
§
Source
§
impl<T>
LinkedList
<T>
1.0.0 (const: 1.39.0)
·
Source
pub const fn
new
() ->
LinkedList
<T>
Creates an empty
LinkedList
.
§
Examples
use
std::collections::LinkedList;
let
list: LinkedList<u32> = LinkedList::new();
1.0.0
·
Source
pub fn
append
(&mut self, other: &mut
LinkedList
<T>)
Moves all elements from
other
to the end of the list.
This reuses all the nodes from
other
and moves them into
self
. After
this operation,
other
becomes empty.
This operation should compute in
O
(1) time and
O
(1) memory.
§
Examples
use
std::collections::LinkedList;
let
mut
list1 = LinkedList::new();
list1.push_back(
'a'
);
let
mut
list2 = LinkedList::new();
list2.push_back(
'b'
);
list2.push_back(
'c'
);

list1.append(
&mut
list2);
let
mut
iter = list1.iter();
assert_eq!
(iter.next(),
Some
(
&
'a'
));
assert_eq!
(iter.next(),
Some
(
&
'b'
));
assert_eq!
(iter.next(),
Some
(
&
'c'
));
assert!
(iter.next().is_none());
assert!
(list2.is_empty());
Source
§
impl<T, A>
LinkedList
<T, A>
where
    A:
Allocator
,
Source
pub const fn
new_in
(alloc: A) ->
LinkedList
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs an empty
LinkedList<T, A>
.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
use
std::collections::LinkedList;
let
list: LinkedList<u32,
_
> = LinkedList::new_in(System);
1.0.0
·
Source
pub fn
iter
(&self) ->
Iter
<'_, T>
ⓘ
Provides a forward iterator.
§
Examples
use
std::collections::LinkedList;
let
mut
list: LinkedList<u32> = LinkedList::new();

list.push_back(
0
);
list.push_back(
1
);
list.push_back(
2
);
let
mut
iter = list.iter();
assert_eq!
(iter.next(),
Some
(
&
0
));
assert_eq!
(iter.next(),
Some
(
&
1
));
assert_eq!
(iter.next(),
Some
(
&
2
));
assert_eq!
(iter.next(),
None
);
1.0.0
·
Source
pub fn
iter_mut
(&mut self) ->
IterMut
<'_, T>
ⓘ
Provides a forward iterator with mutable references.
§
Examples
use
std::collections::LinkedList;
let
mut
list: LinkedList<u32> = LinkedList::new();

list.push_back(
0
);
list.push_back(
1
);
list.push_back(
2
);
for
element
in
list.iter_mut() {
*
element +=
10
;
}
let
mut
iter = list.iter();
assert_eq!
(iter.next(),
Some
(
&
10
));
assert_eq!
(iter.next(),
Some
(
&
11
));
assert_eq!
(iter.next(),
Some
(
&
12
));
assert_eq!
(iter.next(),
None
);
Source
pub fn
cursor_front
(&self) ->
Cursor
<'_, T, A>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a cursor at the front element.
The cursor is pointing to the “ghost” non-element if the list is empty.
Source
pub fn
cursor_front_mut
(&mut self) ->
CursorMut
<'_, T, A>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a cursor with editing operations at the front element.
The cursor is pointing to the “ghost” non-element if the list is empty.
Source
pub fn
cursor_back
(&self) ->
Cursor
<'_, T, A>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a cursor at the back element.
The cursor is pointing to the “ghost” non-element if the list is empty.
Source
pub fn
cursor_back_mut
(&mut self) ->
CursorMut
<'_, T, A>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a cursor with editing operations at the back element.
The cursor is pointing to the “ghost” non-element if the list is empty.
1.0.0
·
Source
pub fn
is_empty
(&self) ->
bool
Returns
true
if the
LinkedList
is empty.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();
assert!
(dl.is_empty());

dl.push_front(
"foo"
);
assert!
(!dl.is_empty());
1.0.0
·
Source
pub fn
len
(&self) ->
usize
Returns the length of the
LinkedList
.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();

dl.push_front(
2
);
assert_eq!
(dl.len(),
1
);

dl.push_front(
1
);
assert_eq!
(dl.len(),
2
);

dl.push_back(
3
);
assert_eq!
(dl.len(),
3
);
1.0.0
·
Source
pub fn
clear
(&mut self)
Removes all elements from the
LinkedList
.
This operation should compute in
O
(
n
) time.
§
Examples
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();

dl.push_front(
2
);
dl.push_front(
1
);
assert_eq!
(dl.len(),
2
);
assert_eq!
(dl.front(),
Some
(
&
1
));

dl.clear();
assert_eq!
(dl.len(),
0
);
assert_eq!
(dl.front(),
None
);
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
if the
LinkedList
contains an element equal to the
given value.
This operation should compute linearly in
O
(
n
) time.
§
Examples
use
std::collections::LinkedList;
let
mut
list: LinkedList<u32> = LinkedList::new();

list.push_back(
0
);
list.push_back(
1
);
list.push_back(
2
);
assert_eq!
(list.contains(
&
0
),
true
);
assert_eq!
(list.contains(
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
if the list is
empty.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();
assert_eq!
(dl.front(),
None
);

dl.push_front(
1
);
assert_eq!
(dl.front(),
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
if the list
is empty.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();
assert_eq!
(dl.front(),
None
);

dl.push_front(
1
);
assert_eq!
(dl.front(),
Some
(
&
1
));
match
dl.front_mut() {
None
=> {},
Some
(x) =>
*
x =
5
,
}
assert_eq!
(dl.front(),
Some
(
&
5
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
if the list is
empty.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();
assert_eq!
(dl.back(),
None
);

dl.push_back(
1
);
assert_eq!
(dl.back(),
Some
(
&
1
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
if the list
is empty.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();
assert_eq!
(dl.back(),
None
);

dl.push_back(
1
);
assert_eq!
(dl.back(),
Some
(
&
1
));
match
dl.back_mut() {
None
=> {},
Some
(x) =>
*
x =
5
,
}
assert_eq!
(dl.back(),
Some
(
&
5
));
1.0.0
·
Source
pub fn
push_front
(&mut self, elt: T)
Adds an element first in the list.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();

dl.push_front(
2
);
assert_eq!
(dl.front().unwrap(),
&
2
);

dl.push_front(
1
);
assert_eq!
(dl.front().unwrap(),
&
1
);
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
if the list is
empty.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
d = LinkedList::new();
assert_eq!
(d.pop_front(),
None
);

d.push_front(
1
);
d.push_front(
3
);
assert_eq!
(d.pop_front(),
Some
(
3
));
assert_eq!
(d.pop_front(),
Some
(
1
));
assert_eq!
(d.pop_front(),
None
);
1.0.0
·
Source
pub fn
push_back
(&mut self, elt: T)
Appends an element to the back of a list.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
d = LinkedList::new();
d.push_back(
1
);
d.push_back(
3
);
assert_eq!
(
3
,
*
d.back().unwrap());
1.0.0
·
Source
pub fn
pop_back
(&mut self) ->
Option
<T>
Removes the last element from a list and returns it, or
None
if
it is empty.
This operation should compute in
O
(1) time.
§
Examples
use
std::collections::LinkedList;
let
mut
d = LinkedList::new();
assert_eq!
(d.pop_back(),
None
);
d.push_back(
1
);
d.push_back(
3
);
assert_eq!
(d.pop_back(),
Some
(
3
));
1.0.0
·
Source
pub fn
split_off
(&mut self, at:
usize
) ->
LinkedList
<T, A>
where
    A:
Clone
,
Splits the list into two at the given index. Returns everything after the given index,
including the index.
This operation should compute in
O
(
n
) time.
§
Panics
Panics if
at > len
.
§
Examples
use
std::collections::LinkedList;
let
mut
d = LinkedList::new();

d.push_front(
1
);
d.push_front(
2
);
d.push_front(
3
);
let
mut
split = d.split_off(
2
);
assert_eq!
(split.pop_front(),
Some
(
1
));
assert_eq!
(split.pop_front(),
None
);
Source
pub fn
remove
(&mut self, at:
usize
) -> T
🔬
This is a nightly-only experimental API. (
linked_list_remove
#69210
)
Removes the element at the given index and returns it.
This operation should compute in
O
(
n
) time.
§
Panics
Panics if at >= len
§
Examples
#![feature(linked_list_remove)]
use
std::collections::LinkedList;
let
mut
d = LinkedList::new();

d.push_front(
1
);
d.push_front(
2
);
d.push_front(
3
);
assert_eq!
(d.remove(
1
),
2
);
assert_eq!
(d.remove(
0
),
3
);
assert_eq!
(d.remove(
0
),
1
);
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
🔬
This is a nightly-only experimental API. (
linked_list_retain
#114135
)
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
#![feature(linked_list_retain)]
use
std::collections::LinkedList;
let
mut
d = LinkedList::new();

d.push_front(
1
);
d.push_front(
2
);
d.push_front(
3
);

d.retain(|
&
x| x %
2
==
0
);
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
Because the elements are visited exactly once in the original order,
external state may be used to decide which elements to keep.
#![feature(linked_list_retain)]
use
std::collections::LinkedList;
let
mut
d = LinkedList::new();

d.push_front(
1
);
d.push_front(
2
);
d.push_front(
3
);
let
keep = [
false
,
true
,
false
];
let
mut
iter = keep.iter();
d.retain(|
_
|
*
iter.next().unwrap());
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
🔬
This is a nightly-only experimental API. (
linked_list_retain
#114135
)
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
#![feature(linked_list_retain)]
use
std::collections::LinkedList;
let
mut
d = LinkedList::new();

d.push_front(
1
);
d.push_front(
2
);
d.push_front(
3
);

d.retain_mut(|x|
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
(d.pop_front(),
Some
(
3
));
assert_eq!
(d.pop_front(),
None
);
1.87.0
·
Source
pub fn
extract_if
<F>(&mut self, filter: F) ->
ExtractIf
<'_, T, F, A>
ⓘ
where
    F:
FnMut
(
&mut T
) ->
bool
,
Creates an iterator which uses a closure to determine if an element should be removed.
If the closure returns true, then the element is removed and yielded.
If the closure returns false, the element will remain in the list and will not be yielded
by the iterator.
If the returned
ExtractIf
is not exhausted, e.g. because it is dropped without iterating
or the iteration short-circuits, then the remaining elements will be retained.
Use
extract_if().for_each(drop)
if you do not need the returned iterator.
Note that
extract_if
lets you mutate every element in the filter closure, regardless of
whether you choose to keep or remove it.
§
Examples
Splitting a list into evens and odds, reusing the original list:
use
std::collections::LinkedList;
let
mut
numbers: LinkedList<u32> = LinkedList::new();
numbers.extend(
&
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
,
6
,
8
,
9
,
11
,
13
,
14
,
15
]);
let
evens = numbers.extract_if(|x|
*
x %
2
==
0
).collect::<LinkedList<
_
>>();
let
odds = numbers;
assert_eq!
(evens.into_iter().collect::<Vec<
_
>>(),
vec!
[
2
,
4
,
6
,
8
,
14
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
9
,
11
,
13
,
15
]);
Trait Implementations
§
1.0.0
·
Source
§
impl<T, A>
Clone
for
LinkedList
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
LinkedList
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
as it avoids reallocation of the nodes of the linked list. Additionally,
if the element type
T
overrides
clone_from()
, this will reuse the
resources of
self
’s elements as well.
Source
§
fn
clone
(&self) ->
LinkedList
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
LinkedList
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
LinkedList
<T>
Source
§
fn
default
() ->
LinkedList
<T>
Creates an empty
LinkedList<T>
.
1.0.0
·
Source
§
impl<T, A>
Drop
for
LinkedList
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
LinkedList
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
LinkedList
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
LinkedList
<T>
Source
§
fn
from
(arr:
[T; N]
) ->
LinkedList
<T>
Converts a
[T; N]
into a
LinkedList<T>
.
use
std::collections::LinkedList;
let
list1 = LinkedList::from([
1
,
2
,
3
,
4
]);
let
list2: LinkedList<
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
(list1, list2);
1.0.0
·
Source
§
impl<T>
FromIterator
<T> for
LinkedList
<T>
Source
§
fn
from_iter
<I>(iter: I) ->
LinkedList
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
LinkedList
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
LinkedList
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
Consumes the list into an iterator yielding elements by value.
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
LinkedList
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
LinkedList
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
LinkedList
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
LinkedList
<T, A>) ->
bool
Tests for
self
and
other
values to be equal, and is used by
==
.
Source
§
fn
ne
(&self, other: &
LinkedList
<T, A>) ->
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
LinkedList
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
LinkedList
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
Eq
for
LinkedList
<T, A>
where
    T:
Eq
,
    A:
Allocator
,
1.0.0
·
Source
§
impl<T, A>
Send
for
LinkedList
<T, A>
where
    T:
Send
,
    A:
Allocator
+
Send
,
1.0.0
·
Source
§
impl<T, A>
Sync
for
LinkedList
<T, A>
where
    T:
Sync
,
    A:
Allocator
+
Sync
,
Auto Trait Implementations
§
§
impl<T, A>
Freeze
for
LinkedList
<T, A>
where
    A:
Freeze
,
§
impl<T, A>
RefUnwindSafe
for
LinkedList
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
Unpin
for
LinkedList
<T, A>
where
    A:
Unpin
,
§
impl<T, A>
UnwindSafe
for
LinkedList
<T, A>
where
    A:
UnwindSafe
,
    T:
RefUnwindSafe
+
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