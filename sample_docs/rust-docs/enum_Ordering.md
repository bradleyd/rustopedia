Ordering in std::sync::atomic - Rust
std
::
sync
::
atomic
Enum
Ordering
Copy item path
1.0.0
·
Source
#[non_exhaustive]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
Expand description
Atomic memory orderings
Memory orderings specify the way atomic operations synchronize memory.
In its weakest
Ordering::Relaxed
, only the memory directly touched by the
operation is synchronized. On the other hand, a store-load pair of
Ordering::SeqCst
operations synchronize other memory while additionally preserving a total order of such
operations across all threads.
Rust’s memory orderings are
the same as those of
C++20
.
For more information see the
nomicon
.
Variants (Non-exhaustive)
§
This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
1.0.0
Relaxed
No ordering constraints, only atomic operations.
Corresponds to
memory_order_relaxed
in C++20.
§
1.0.0
Release
When coupled with a store, all previous operations become ordered
before any load of this value with
Acquire
(or stronger) ordering.
In particular, all previous writes become visible to all threads
that perform an
Acquire
(or stronger) load of this value.
Notice that using this ordering for an operation that combines loads
and stores leads to a
Relaxed
load operation!
This ordering is only applicable for operations that can perform a store.
Corresponds to
memory_order_release
in C++20.
§
1.0.0
Acquire
When coupled with a load, if the loaded value was written by a store operation with
Release
(or stronger) ordering, then all subsequent operations
become ordered after that store. In particular, all subsequent loads will see data
written before the store.
Notice that using this ordering for an operation that combines loads
and stores leads to a
Relaxed
store operation!
This ordering is only applicable for operations that can perform a load.
Corresponds to
memory_order_acquire
in C++20.
§
1.0.0
AcqRel
Has the effects of both
Acquire
and
Release
together:
For loads it uses
Acquire
ordering. For stores it uses the
Release
ordering.
Notice that in the case of
compare_and_swap
, it is possible that the operation ends up
not performing any store and hence it has just
Acquire
ordering. However,
AcqRel
will never perform
Relaxed
accesses.
This ordering is only applicable for operations that combine both loads and stores.
Corresponds to
memory_order_acq_rel
in C++20.
§
1.0.0
SeqCst
Like
Acquire
/
Release
/
AcqRel
(for load, store, and load-with-store
operations, respectively) with the additional guarantee that all threads see all
sequentially consistent operations in the same order.
Corresponds to
memory_order_seq_cst
in C++20.
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
Ordering
Source
§
fn
clone
(&self) ->
Ordering
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
impl
Debug
for
Ordering
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
impl
Hash
for
Ordering
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
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
impl
PartialEq
for
Ordering
Source
§
fn
eq
(&self, other: &
Ordering
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
1.0.0
·
Source
§
impl
Copy
for
Ordering
1.0.0
·
Source
§
impl
Eq
for
Ordering
1.0.0
·
Source
§
impl
StructuralPartialEq
for
Ordering
Auto Trait Implementations
§
§
impl
Freeze
for
Ordering
§
impl
RefUnwindSafe
for
Ordering
§
impl
Send
for
Ordering
§
impl
Sync
for
Ordering
§
impl
Unpin
for
Ordering
§
impl
UnwindSafe
for
Ordering
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