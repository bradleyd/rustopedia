PeekMut in std::collections::binary_heap - Rust
std
::
collections
::
binary_heap
Struct
PeekMut
Copy item path
1.12.0
·
Source
pub struct PeekMut<'a, T, A =
Global
>
where
    T: 'a +
Ord
,
    A:
Allocator
,
{
/* private fields */
}
Expand description
Structure wrapping a mutable reference to the greatest item on a
BinaryHeap
.
This
struct
is created by the
peek_mut
method on
BinaryHeap
. See
its documentation for more.
Implementations
§
Source
§
impl<'a, T, A>
PeekMut
<'a, T, A>
where
    T:
Ord
,
    A:
Allocator
,
Source
pub fn
refresh
(&mut self) ->
bool
🔬
This is a nightly-only experimental API. (
binary_heap_peek_mut_refresh
#138355
)
Sifts the current element to its new position.
Afterwards refers to the new element. Returns if the element changed.
§
Examples
The condition can be used to upper bound all elements in the heap. When only few elements
are affected, the heap’s sort ensures this is faster than a reconstruction from the raw
element list and requires no additional allocation.
#![feature(binary_heap_peek_mut_refresh)]
use
std::collections::BinaryHeap;
let
mut
heap: BinaryHeap<u32> = (
0
..
128
).collect();
let
mut
peek = heap.peek_mut().unwrap();
loop
{
*
peek =
99
;
if
!peek.refresh() {
break
;
    }
}
// Post condition, this is now an upper bound.
assert!
(
*
peek <
100
);
When the element remains the maximum after modification, the peek remains unchanged:
#![feature(binary_heap_peek_mut_refresh)]
use
std::collections::BinaryHeap;
let
mut
heap: BinaryHeap<u32> = [
1
,
2
,
3
].into();
let
mut
peek = heap.peek_mut().unwrap();
assert_eq!
(
*
peek,
3
);
*
peek =
42
;
// When we refresh, the peek is updated to the new maximum.
assert!
(!peek.refresh(),
"42 is even larger than 3"
);
assert_eq!
(
*
peek,
42
);
1.18.0
·
Source
pub fn
pop
(this:
PeekMut
<'a, T, A>) -> T
Removes the peeked value from the heap and returns it.
Trait Implementations
§
1.17.0
·
Source
§
impl<T, A>
Debug
for
PeekMut
<'_, T, A>
where
    T:
Ord
+
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
1.12.0
·
Source
§
impl<T, A>
Deref
for
PeekMut
<'_, T, A>
where
    T:
Ord
,
    A:
Allocator
,
Source
§
type
Target
= T
The resulting type after dereferencing.
Source
§
fn
deref
(&self) ->
&T
Dereferences the value.
1.12.0
·
Source
§
impl<T, A>
DerefMut
for
PeekMut
<'_, T, A>
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
deref_mut
(&mut self) ->
&mut T
Mutably dereferences the value.
1.12.0
·
Source
§
impl<T, A>
Drop
for
PeekMut
<'_, T, A>
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
drop
(&mut self)
Executes the destructor for this type.
Read more
Auto Trait Implementations
§
§
impl<'a, T, A>
Freeze
for
PeekMut
<'a, T, A>
§
impl<'a, T, A>
RefUnwindSafe
for
PeekMut
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
Send
for
PeekMut
<'a, T, A>
where
    A:
Send
,
    T:
Send
,
§
impl<'a, T, A>
Sync
for
PeekMut
<'a, T, A>
where
    A:
Sync
,
    T:
Sync
,
§
impl<'a, T, A>
Unpin
for
PeekMut
<'a, T, A>
§
impl<'a, T, A =
Global
> !
UnwindSafe
for
PeekMut
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
impl<P, T>
Receiver
for P
where
    P:
Deref
<Target = T> + ?
Sized
,
    T: ?
Sized
,
Source
§
type
Target
= T
🔬
This is a nightly-only experimental API. (
arbitrary_self_types
#44874
)
The target type on which the method may be called.
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