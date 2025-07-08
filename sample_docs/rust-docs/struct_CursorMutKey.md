CursorMutKey in std::collections::btree_set - Rust
std
::
collections
::
btree_set
Struct
CursorMutKey
Copy item path
Source
pub struct CursorMutKey<'a, K, A =
Global
>
where
    K: 'a,
{
/* private fields */
}
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Expand description
A cursor over a
BTreeSet
with editing operations, and which allows
mutating elements.
A
Cursor
is like an iterator, except that it can freely seek back-and-forth, and can
safely mutate the set during iteration. This is because the lifetime of its yielded
references is tied to its own lifetime, instead of just the underlying set. This means
cursors cannot yield multiple elements at once.
Cursors always point to a gap between two elements in the set, and can
operate on the two immediately adjacent elements.
A
CursorMutKey
is created from a
CursorMut
with the
CursorMut::with_mutable_key
method.
§
Safety
Since this cursor allows mutating elements, you must ensure that the
BTreeSet
invariants are maintained. Specifically:
The newly inserted element must be unique in the tree.
All elements in the tree must remain in sorted order.
Implementations
§
Source
§
impl<'a, T, A>
CursorMutKey
<'a, T, A>
Source
pub fn
next
(&mut self) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Advances the cursor to the next gap, returning the  element that it
moved over.
If the cursor is already at the end of the set then
None
is returned
and the cursor is not moved.
Source
pub fn
prev
(&mut self) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Advances the cursor to the previous gap, returning the element that it
moved over.
If the cursor is already at the start of the set then
None
is returned
and the cursor is not moved.
Source
pub fn
peek_next
(&mut self) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Returns a reference to the next element without moving the cursor.
If the cursor is at the end of the set then
None
is returned
Source
pub fn
peek_prev
(&mut self) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Returns a reference to the previous element without moving the cursor.
If the cursor is at the start of the set then
None
is returned.
Source
pub fn
as_cursor
(&self) ->
Cursor
<'_, T>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Returns a read-only cursor pointing to the same location as the
CursorMutKey
.
The lifetime of the returned
Cursor
is bound to that of the
CursorMutKey
, which means it cannot outlive the
CursorMutKey
and that the
CursorMutKey
is frozen for the lifetime of the
Cursor
.
Source
§
impl<'a, T, A>
CursorMutKey
<'a, T, A>
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
pub unsafe fn
insert_after_unchecked
(&mut self, value: T)
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Inserts a new element into the set in the gap that the
cursor is currently pointing to.
After the insertion the cursor will be pointing at the gap before the
newly inserted element.
§
Safety
You must ensure that the
BTreeSet
invariants are maintained.
Specifically:
The key of the newly inserted element must be unique in the tree.
All elements in the tree must remain in sorted order.
Source
pub unsafe fn
insert_before_unchecked
(&mut self, value: T)
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Inserts a new element into the set in the gap that the
cursor is currently pointing to.
After the insertion the cursor will be pointing at the gap after the
newly inserted element.
§
Safety
You must ensure that the
BTreeSet
invariants are maintained.
Specifically:
The newly inserted element must be unique in the tree.
All elements in the tree must remain in sorted order.
Source
pub fn
insert_after
(&mut self, value: T) ->
Result
<
()
,
UnorderedKeyError
>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Inserts a new element into the set in the gap that the
cursor is currently pointing to.
After the insertion the cursor will be pointing at the gap before the
newly inserted element.
If the inserted element is not greater than the element before the
cursor (if any), or if it not less than the element after the cursor (if
any), then an
UnorderedKeyError
is returned since this would
invalidate the
Ord
invariant between the elements of the set.
Source
pub fn
insert_before
(&mut self, value: T) ->
Result
<
()
,
UnorderedKeyError
>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Inserts a new element into the set in the gap that the
cursor is currently pointing to.
After the insertion the cursor will be pointing at the gap after the
newly inserted element.
If the inserted element is not greater than the element before the
cursor (if any), or if it not less than the element after the cursor (if
any), then an
UnorderedKeyError
is returned since this would
invalidate the
Ord
invariant between the elements of the set.
Source
pub fn
remove_next
(&mut self) ->
Option
<T>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Removes the next element from the
BTreeSet
.
The element that was removed is returned. The cursor position is
unchanged (before the removed element).
Source
pub fn
remove_prev
(&mut self) ->
Option
<T>
🔬
This is a nightly-only experimental API. (
btree_cursors
#107540
)
Removes the preceding element from the
BTreeSet
.
The element that was removed is returned. The cursor position is
unchanged (after the removed element).
Trait Implementations
§
Source
§
impl<K, A>
Debug
for
CursorMutKey
<'_, K, A>
where
    K:
Debug
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
Auto Trait Implementations
§
§
impl<'a, K, A>
Freeze
for
CursorMutKey
<'a, K, A>
§
impl<'a, K, A>
RefUnwindSafe
for
CursorMutKey
<'a, K, A>
where
    A:
RefUnwindSafe
,
    K:
RefUnwindSafe
,
§
impl<'a, K, A>
Send
for
CursorMutKey
<'a, K, A>
where
    A:
Send
,
    K:
Send
,
§
impl<'a, K, A>
Sync
for
CursorMutKey
<'a, K, A>
where
    A:
Sync
,
    K:
Sync
,
§
impl<'a, K, A>
Unpin
for
CursorMutKey
<'a, K, A>
§
impl<'a, K, A =
Global
> !
UnwindSafe
for
CursorMutKey
<'a, K, A>
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