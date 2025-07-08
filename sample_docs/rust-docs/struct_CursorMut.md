CursorMut in std::collections::linked_list - Rust
std
::
collections
::
linked_list
Struct
CursorMut
Copy item path
Source
pub struct CursorMut<'a, T, A =
Global
>
where
    T: 'a,
    A:
Allocator
,
{
/* private fields */
}
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Expand description
A cursor over a
LinkedList
with editing operations.
A
Cursor
is like an iterator, except that it can freely seek back-and-forth, and can
safely mutate the list during iteration. This is because the lifetime of its yielded
references is tied to its own lifetime, instead of just the underlying list. This means
cursors cannot yield multiple elements at once.
Cursors always rest between two elements in the list, and index in a logically circular way.
To accommodate this, there is a “ghost” non-element that yields
None
between the head and
tail of the list.
Implementations
§
Source
§
impl<'a, T, A>
CursorMut
<'a, T, A>
where
    A:
Allocator
,
Source
pub fn
index
(&self) ->
Option
<
usize
>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Returns the cursor position index within the
LinkedList
.
This returns
None
if the cursor is currently pointing to the
“ghost” non-element.
Source
pub fn
move_next
(&mut self)
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Moves the cursor to the next element of the
LinkedList
.
If the cursor is pointing to the “ghost” non-element then this will move it to
the first element of the
LinkedList
. If it is pointing to the last
element of the
LinkedList
then this will move it to the “ghost” non-element.
Source
pub fn
move_prev
(&mut self)
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Moves the cursor to the previous element of the
LinkedList
.
If the cursor is pointing to the “ghost” non-element then this will move it to
the last element of the
LinkedList
. If it is pointing to the first
element of the
LinkedList
then this will move it to the “ghost” non-element.
Source
pub fn
current
(&mut self) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Returns a reference to the element that the cursor is currently
pointing to.
This returns
None
if the cursor is currently pointing to the
“ghost” non-element.
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
linked_list_cursors
#58533
)
Returns a reference to the next element.
If the cursor is pointing to the “ghost” non-element then this returns
the first element of the
LinkedList
. If it is pointing to the last
element of the
LinkedList
then this returns
None
.
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
linked_list_cursors
#58533
)
Returns a reference to the previous element.
If the cursor is pointing to the “ghost” non-element then this returns
the last element of the
LinkedList
. If it is pointing to the first
element of the
LinkedList
then this returns
None
.
Source
pub fn
as_cursor
(&self) ->
Cursor
<'_, T, A>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Returns a read-only cursor pointing to the current element.
The lifetime of the returned
Cursor
is bound to that of the
CursorMut
, which means it cannot outlive the
CursorMut
and that the
CursorMut
is frozen for the lifetime of the
Cursor
.
Source
pub fn
as_list
(&self) -> &
LinkedList
<T, A>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a read-only reference to the cursor’s parent list.
The lifetime of the returned reference is bound to that of the
CursorMut
, which means it cannot outlive the
CursorMut
and that the
CursorMut
is frozen for the lifetime of the reference.
Source
§
impl<'a, T>
CursorMut
<'a, T>
Source
pub fn
splice_after
(&mut self, list:
LinkedList
<T>)
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Inserts the elements from the given
LinkedList
after the current one.
If the cursor is pointing at the “ghost” non-element then the new elements are
inserted at the start of the
LinkedList
.
Source
pub fn
splice_before
(&mut self, list:
LinkedList
<T>)
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Inserts the elements from the given
LinkedList
before the current one.
If the cursor is pointing at the “ghost” non-element then the new elements are
inserted at the end of the
LinkedList
.
Source
§
impl<'a, T, A>
CursorMut
<'a, T, A>
where
    A:
Allocator
,
Source
pub fn
insert_after
(&mut self, item: T)
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Inserts a new element into the
LinkedList
after the current one.
If the cursor is pointing at the “ghost” non-element then the new element is
inserted at the front of the
LinkedList
.
Source
pub fn
insert_before
(&mut self, item: T)
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Inserts a new element into the
LinkedList
before the current one.
If the cursor is pointing at the “ghost” non-element then the new element is
inserted at the end of the
LinkedList
.
Source
pub fn
remove_current
(&mut self) ->
Option
<T>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Removes the current element from the
LinkedList
.
The element that was removed is returned, and the cursor is
moved to point to the next element in the
LinkedList
.
If the cursor is currently pointing to the “ghost” non-element then no element
is removed and
None
is returned.
Source
pub fn
remove_current_as_list
(&mut self) ->
Option
<
LinkedList
<T, A>>
where
    A:
Clone
,
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Removes the current element from the
LinkedList
without deallocating the list node.
The node that was removed is returned as a new
LinkedList
containing only this node.
The cursor is moved to point to the next element in the current
LinkedList
.
If the cursor is currently pointing to the “ghost” non-element then no element
is removed and
None
is returned.
Source
pub fn
split_after
(&mut self) ->
LinkedList
<T, A>
where
    A:
Clone
,
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Splits the list into two after the current element. This will return a
new list consisting of everything after the cursor, with the original
list retaining everything before.
If the cursor is pointing at the “ghost” non-element then the entire contents
of the
LinkedList
are moved.
Source
pub fn
split_before
(&mut self) ->
LinkedList
<T, A>
where
    A:
Clone
,
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Splits the list into two before the current element. This will return a
new list consisting of everything before the cursor, with the original
list retaining everything after.
If the cursor is pointing at the “ghost” non-element then the entire contents
of the
LinkedList
are moved.
Source
pub fn
push_front
(&mut self, elt: T)
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Appends an element to the front of the cursor’s parent list. The node
that the cursor points to is unchanged, even if it is the “ghost” node.
This operation should compute in
O
(1) time.
Source
pub fn
push_back
(&mut self, elt: T)
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Appends an element to the back of the cursor’s parent list. The node
that the cursor points to is unchanged, even if it is the “ghost” node.
This operation should compute in
O
(1) time.
Source
pub fn
pop_front
(&mut self) ->
Option
<T>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Removes the first element from the cursor’s parent list and returns it,
or None if the list is empty. The element the cursor points to remains
unchanged, unless it was pointing to the front element. In that case, it
points to the new front element.
This operation should compute in
O
(1) time.
Source
pub fn
pop_back
(&mut self) ->
Option
<T>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Removes the last element from the cursor’s parent list and returns it,
or None if the list is empty. The element the cursor points to remains
unchanged, unless it was pointing to the back element. In that case, it
points to the “ghost” element.
This operation should compute in
O
(1) time.
Source
pub fn
front
(&self) ->
Option
<
&T
>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a reference to the front element of the cursor’s parent list,
or None if the list is empty.
Source
pub fn
front_mut
(&mut self) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a mutable reference to the front element of the cursor’s
parent list, or None if the list is empty.
Source
pub fn
back
(&self) ->
Option
<
&T
>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a reference to the back element of the cursor’s parent list,
or None if the list is empty.
Source
pub fn
back_mut
(&mut self) ->
Option
<
&mut T
>
🔬
This is a nightly-only experimental API. (
linked_list_cursors
#58533
)
Provides a mutable reference to back element of the cursor’s parent
list, or
None
if the list is empty.
§
Examples
Building and mutating a list with a cursor, then getting the back element:
#![feature(linked_list_cursors)]
use
std::collections::LinkedList;
let
mut
dl = LinkedList::new();
dl.push_front(
3
);
dl.push_front(
2
);
dl.push_front(
1
);
let
mut
cursor = dl.cursor_front_mut();
*
cursor.current().unwrap() =
99
;
*
cursor.back_mut().unwrap() =
0
;
let
mut
contents = dl.into_iter();
assert_eq!
(contents.next(),
Some
(
99
));
assert_eq!
(contents.next(),
Some
(
2
));
assert_eq!
(contents.next(),
Some
(
0
));
assert_eq!
(contents.next(),
None
);
Trait Implementations
§
Source
§
impl<T, A>
Debug
for
CursorMut
<'_, T, A>
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
Source
§
impl<T, A>
Send
for
CursorMut
<'_, T, A>
where
    T:
Send
,
    A:
Allocator
+
Send
,
Source
§
impl<T, A>
Sync
for
CursorMut
<'_, T, A>
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
impl<'a, T, A>
Freeze
for
CursorMut
<'a, T, A>
§
impl<'a, T, A>
RefUnwindSafe
for
CursorMut
<'a, T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
,
§
impl<'a, T, A>
Unpin
for
CursorMut
<'a, T, A>
§
impl<'a, T, A =
Global
> !
UnwindSafe
for
CursorMut
<'a, T, A>
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