Weak in std::rc - Rust
std
::
rc
Struct
Weak
Copy item path
1.4.0
·
Source
pub struct Weak<T, A =
Global
>
where
    A:
Allocator
,
    T: ?
Sized
,
{
/* private fields */
}
Expand description
Weak
is a version of
Rc
that holds a non-owning reference to the
managed allocation.
The allocation is accessed by calling
upgrade
on the
Weak
pointer, which returns an
Option
<
Rc
<T>>
.
Since a
Weak
reference does not count towards ownership, it will not
prevent the value stored in the allocation from being dropped, and
Weak
itself makes no
guarantees about the value still being present. Thus it may return
None
when
upgrade
d. Note however that a
Weak
reference
does
prevent the allocation
itself (the backing store) from being deallocated.
A
Weak
pointer is useful for keeping a temporary reference to the allocation
managed by
Rc
without preventing its inner value from being dropped. It is also used to
prevent circular references between
Rc
pointers, since mutual owning references
would never allow either
Rc
to be dropped. For example, a tree could
have strong
Rc
pointers from parent nodes to children, and
Weak
pointers from children back to their parents.
The typical way to obtain a
Weak
pointer is to call
Rc::downgrade
.
Implementations
§
Source
§
impl<T>
Weak
<T>
1.10.0 (const: 1.73.0)
·
Source
pub const fn
new
() ->
Weak
<T>
Constructs a new
Weak<T>
, without allocating any memory.
Calling
upgrade
on the return value always gives
None
.
§
Examples
use
std::rc::Weak;
let
empty: Weak<i64> = Weak::new();
assert!
(empty.upgrade().is_none());
Source
§
impl<T, A>
Weak
<T, A>
where
    A:
Allocator
,
Source
pub fn
new_in
(alloc: A) ->
Weak
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Weak<T>
, without allocating any memory, technically in the provided
allocator.
Calling
upgrade
on the return value always gives
None
.
§
Examples
use
std::rc::Weak;
let
empty: Weak<i64> = Weak::new();
assert!
(empty.upgrade().is_none());
Source
§
impl<T>
Weak
<T>
where
    T: ?
Sized
,
1.45.0
·
Source
pub unsafe fn
from_raw
(ptr:
*const T
) ->
Weak
<T>
Converts a raw pointer previously created by
into_raw
back into
Weak<T>
.
This can be used to safely get a strong reference (by calling
upgrade
later) or to deallocate the weak count by dropping the
Weak<T>
.
It takes ownership of one weak reference (with the exception of pointers created by
new
,
as these don’t own anything; the method still works on them).
§
Safety
The pointer must have originated from the
into_raw
and must still own its potential
weak reference, and
ptr
must point to a block of memory allocated by the global allocator.
It is allowed for the strong count to be 0 at the time of calling this. Nevertheless, this
takes ownership of one weak reference currently represented as a raw pointer (the weak
count is not modified by this operation) and therefore it must be paired with a previous
call to
into_raw
.
§
Examples
use
std::rc::{Rc, Weak};
let
strong = Rc::new(
"hello"
.to_owned());
let
raw_1 = Rc::downgrade(
&
strong).into_raw();
let
raw_2 = Rc::downgrade(
&
strong).into_raw();
assert_eq!
(
2
, Rc::weak_count(
&
strong));
assert_eq!
(
"hello"
,
&*
unsafe
{ Weak::from_raw(raw_1) }.upgrade().unwrap());
assert_eq!
(
1
, Rc::weak_count(
&
strong));

drop(strong);
// Decrement the last weak count.
assert!
(
unsafe
{ Weak::from_raw(raw_2) }.upgrade().is_none());
Source
§
impl<T, A>
Weak
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
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
1.45.0
·
Source
pub fn
as_ptr
(&self) ->
*const T
Returns a raw pointer to the object
T
pointed to by this
Weak<T>
.
The pointer is valid only if there are some strong references. The pointer may be dangling,
unaligned or even
null
otherwise.
§
Examples
use
std::rc::Rc;
use
std::ptr;
let
strong = Rc::new(
"hello"
.to_owned());
let
weak = Rc::downgrade(
&
strong);
// Both point to the same object
assert!
(ptr::eq(
&*
strong, weak.as_ptr()));
// The strong here keeps it alive, so we can still access the object.
assert_eq!
(
"hello"
,
unsafe
{
&*
weak.as_ptr() });

drop(strong);
// But not any more. We can do weak.as_ptr(), but accessing the pointer would lead to
// undefined behavior.
// assert_eq!("hello", unsafe { &*weak.as_ptr() });
1.45.0
·
Source
pub fn
into_raw
(self) ->
*const T
Consumes the
Weak<T>
and turns it into a raw pointer.
This converts the weak pointer into a raw pointer, while still preserving the ownership of
one weak reference (the weak count is not modified by this operation). It can be turned
back into the
Weak<T>
with
from_raw
.
The same restrictions of accessing the target of the pointer as with
as_ptr
apply.
§
Examples
use
std::rc::{Rc, Weak};
let
strong = Rc::new(
"hello"
.to_owned());
let
weak = Rc::downgrade(
&
strong);
let
raw = weak.into_raw();
assert_eq!
(
1
, Rc::weak_count(
&
strong));
assert_eq!
(
"hello"
,
unsafe
{
&*
raw });

drop(
unsafe
{ Weak::from_raw(raw) });
assert_eq!
(
0
, Rc::weak_count(
&
strong));
Source
pub fn
into_raw_with_allocator
(self) -> (
*const T
, A)
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Consumes the
Weak<T>
, returning the wrapped pointer and allocator.
This converts the weak pointer into a raw pointer, while still preserving the ownership of
one weak reference (the weak count is not modified by this operation). It can be turned
back into the
Weak<T>
with
from_raw_in
.
The same restrictions of accessing the target of the pointer as with
as_ptr
apply.
§
Examples
#![feature(allocator_api)]
use
std::rc::{Rc, Weak};
use
std::alloc::System;
let
strong = Rc::new_in(
"hello"
.to_owned(), System);
let
weak = Rc::downgrade(
&
strong);
let
(raw, alloc) = weak.into_raw_with_allocator();
assert_eq!
(
1
, Rc::weak_count(
&
strong));
assert_eq!
(
"hello"
,
unsafe
{
&*
raw });

drop(
unsafe
{ Weak::from_raw_in(raw, alloc) });
assert_eq!
(
0
, Rc::weak_count(
&
strong));
Source
pub unsafe fn
from_raw_in
(ptr:
*const T
, alloc: A) ->
Weak
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Converts a raw pointer previously created by
into_raw
back into
Weak<T>
.
This can be used to safely get a strong reference (by calling
upgrade
later) or to deallocate the weak count by dropping the
Weak<T>
.
It takes ownership of one weak reference (with the exception of pointers created by
new
,
as these don’t own anything; the method still works on them).
§
Safety
The pointer must have originated from the
into_raw
and must still own its potential
weak reference, and
ptr
must point to a block of memory allocated by
alloc
.
It is allowed for the strong count to be 0 at the time of calling this. Nevertheless, this
takes ownership of one weak reference currently represented as a raw pointer (the weak
count is not modified by this operation) and therefore it must be paired with a previous
call to
into_raw
.
§
Examples
use
std::rc::{Rc, Weak};
let
strong = Rc::new(
"hello"
.to_owned());
let
raw_1 = Rc::downgrade(
&
strong).into_raw();
let
raw_2 = Rc::downgrade(
&
strong).into_raw();
assert_eq!
(
2
, Rc::weak_count(
&
strong));
assert_eq!
(
"hello"
,
&*
unsafe
{ Weak::from_raw(raw_1) }.upgrade().unwrap());
assert_eq!
(
1
, Rc::weak_count(
&
strong));

drop(strong);
// Decrement the last weak count.
assert!
(
unsafe
{ Weak::from_raw(raw_2) }.upgrade().is_none());
1.4.0
·
Source
pub fn
upgrade
(&self) ->
Option
<
Rc
<T, A>>
where
    A:
Clone
,
Attempts to upgrade the
Weak
pointer to an
Rc
, delaying
dropping of the inner value if successful.
Returns
None
if the inner value has since been dropped.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
let
weak_five = Rc::downgrade(
&
five);
let
strong_five:
Option
<Rc<
_
>> = weak_five.upgrade();
assert!
(strong_five.is_some());
// Destroy all strong pointers.
drop(strong_five);
drop(five);
assert!
(weak_five.upgrade().is_none());
1.41.0
·
Source
pub fn
strong_count
(&self) ->
usize
Gets the number of strong (
Rc
) pointers pointing to this allocation.
If
self
was created using
Weak::new
, this will return 0.
1.41.0
·
Source
pub fn
weak_count
(&self) ->
usize
Gets the number of
Weak
pointers pointing to this allocation.
If no strong pointers remain, this will return zero.
1.39.0
·
Source
pub fn
ptr_eq
(&self, other: &
Weak
<T, A>) ->
bool
Returns
true
if the two
Weak
s point to the same allocation similar to
ptr::eq
, or if
both don’t point to any allocation (because they were created with
Weak::new()
). However,
this function ignores the metadata of
dyn Trait
pointers.
§
Notes
Since this compares pointers it means that
Weak::new()
will equal each
other, even though they don’t point to any allocation.
§
Examples
use
std::rc::Rc;
let
first_rc = Rc::new(
5
);
let
first = Rc::downgrade(
&
first_rc);
let
second = Rc::downgrade(
&
first_rc);
assert!
(first.ptr_eq(
&
second));
let
third_rc = Rc::new(
5
);
let
third = Rc::downgrade(
&
third_rc);
assert!
(!first.ptr_eq(
&
third));
Comparing
Weak::new
.
use
std::rc::{Rc, Weak};
let
first = Weak::new();
let
second = Weak::new();
assert!
(first.ptr_eq(
&
second));
let
third_rc = Rc::new(());
let
third = Rc::downgrade(
&
third_rc);
assert!
(!first.ptr_eq(
&
third));
Trait Implementations
§
1.4.0
·
Source
§
impl<T, A>
Clone
for
Weak
<T, A>
where
    A:
Allocator
+
Clone
,
    T: ?
Sized
,
Source
§
fn
clone
(&self) ->
Weak
<T, A>
Makes a clone of the
Weak
pointer that points to the same allocation.
§
Examples
use
std::rc::{Rc, Weak};
let
weak_five = Rc::downgrade(
&
Rc::new(
5
));
let _
= Weak::clone(
&
weak_five);
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
1.4.0
·
Source
§
impl<T, A>
Debug
for
Weak
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
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
1.10.0
·
Source
§
impl<T>
Default
for
Weak
<T>
Source
§
fn
default
() ->
Weak
<T>
Constructs a new
Weak<T>
, without allocating any memory.
Calling
upgrade
on the return value always gives
None
.
§
Examples
use
std::rc::Weak;
let
empty: Weak<i64> = Default::default();
assert!
(empty.upgrade().is_none());
1.4.0
·
Source
§
impl<T, A>
Drop
for
Weak
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
fn
drop
(&mut self)
Drops the
Weak
pointer.
§
Examples
use
std::rc::{Rc, Weak};
struct
Foo;
impl
Drop
for
Foo {
fn
drop(
&mut
self
) {
println!
(
"dropped!"
);
    }
}
let
foo = Rc::new(Foo);
let
weak_foo = Rc::downgrade(
&
foo);
let
other_weak_foo = Weak::clone(
&
weak_foo);

drop(weak_foo);
// Doesn't print anything
drop(foo);
// Prints "dropped!"
assert!
(other_weak_foo.upgrade().is_none());
Source
§
impl<T, U, A>
CoerceUnsized
<
Weak
<U, A>> for
Weak
<T, A>
where
    T:
Unsize
<U> + ?
Sized
,
    A:
Allocator
,
    U: ?
Sized
,
Source
§
impl<T, U>
DispatchFromDyn
<
Weak
<U>> for
Weak
<T>
where
    T:
Unsize
<U> + ?
Sized
,
    U: ?
Sized
,
Source
§
impl<T, A>
PinCoerceUnsized
for
Weak
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.4.0
·
Source
§
impl<T, A> !
Send
for
Weak
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.4.0
·
Source
§
impl<T, A> !
Sync
for
Weak
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
§
impl<T, A>
UseCloned
for
Weak
<T, A>
where
    A:
Allocator
+
Clone
,
    T: ?
Sized
,
Auto Trait Implementations
§
§
impl<T, A>
Freeze
for
Weak
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A =
Global
> !
RefUnwindSafe
for
Weak
<T, A>
§
impl<T, A>
Unpin
for
Weak
<T, A>
where
    A:
Unpin
,
    T: ?
Sized
,
§
impl<T, A =
Global
> !
UnwindSafe
for
Weak
<T, A>
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