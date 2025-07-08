Arc in std::sync - Rust
std
::
sync
Struct
Arc
Copy item path
1.0.0
·
Source
pub struct Arc<T, A =
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
A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically
Reference Counted’.
The type
Arc<T>
provides shared ownership of a value of type
T
,
allocated in the heap. Invoking
clone
on
Arc
produces
a new
Arc
instance, which points to the same allocation on the heap as the
source
Arc
, while increasing a reference count. When the last
Arc
pointer to a given allocation is destroyed, the value stored in that allocation (often
referred to as “inner value”) is also dropped.
Shared references in Rust disallow mutation by default, and
Arc
is no
exception: you cannot generally obtain a mutable reference to something
inside an
Arc
. If you do need to mutate through an
Arc
, you have several options:
Use interior mutability with synchronization primitives like
Mutex
,
RwLock
, or one of the
Atomic
types.
Use clone-on-write semantics with
Arc::make_mut
which provides efficient mutation
without requiring interior mutability. This approach clones the data only when
needed (when there are multiple references) and can be more efficient when mutations
are infrequent.
Use
Arc::get_mut
when you know your
Arc
is not shared (has a reference count of 1),
which provides direct mutable access to the inner value without any cloning.
use
std::sync::Arc;
let
mut
data = Arc::new(
vec!
[
1
,
2
,
3
]);
// This will clone the vector only if there are other references to it
Arc::make_mut(
&mut
data).push(
4
);
assert_eq!
(
*
data,
vec!
[
1
,
2
,
3
,
4
]);
Note
: This type is only available on platforms that support atomic
loads and stores of pointers, which includes all platforms that support
the
std
crate but not all those which only support
alloc
.
This may be detected at compile time using
#[cfg(target_has_atomic = "ptr")]
.
§
Thread Safety
Unlike
Rc<T>
,
Arc<T>
uses atomic operations for its reference
counting. This means that it is thread-safe. The disadvantage is that
atomic operations are more expensive than ordinary memory accesses. If you
are not sharing reference-counted allocations between threads, consider using
Rc<T>
for lower overhead.
Rc<T>
is a safe default, because the
compiler will catch any attempt to send an
Rc<T>
between threads.
However, a library might choose
Arc<T>
in order to give library consumers
more flexibility.
Arc<T>
will implement
Send
and
Sync
as long as the
T
implements
Send
and
Sync
. Why can’t you put a non-thread-safe type
T
in an
Arc<T>
to make it thread-safe? This may be a bit counter-intuitive at
first: after all, isn’t the point of
Arc<T>
thread safety? The key is
this:
Arc<T>
makes it thread safe to have multiple ownership of the same
data, but it  doesn’t add thread safety to its data. Consider
Arc<
RefCell<T>
>
.
RefCell<T>
isn’t
Sync
, and if
Arc<T>
was always
Send
,
Arc<
RefCell<T>
>
would be as well. But then we’d have a problem:
RefCell<T>
is not thread safe; it keeps track of the borrowing count using
non-atomic operations.
In the end, this means that you may need to pair
Arc<T>
with some sort of
std::sync
type, usually
Mutex<T>
.
§
Breaking cycles with
Weak
The
downgrade
method can be used to create a non-owning
Weak
pointer. A
Weak
pointer can be
upgrade
d
to an
Arc
, but this will return
None
if the value stored in the allocation has
already been dropped. In other words,
Weak
pointers do not keep the value
inside the allocation alive; however, they
do
keep the allocation
(the backing store for the value) alive.
A cycle between
Arc
pointers will never be deallocated. For this reason,
Weak
is used to break cycles. For example, a tree could have
strong
Arc
pointers from parent nodes to children, and
Weak
pointers from children back to their parents.
§
Cloning references
Creating a new reference from an existing reference-counted pointer is done using the
Clone
trait implemented for
Arc<T>
and
Weak<T>
.
use
std::sync::Arc;
let
foo = Arc::new(
vec!
[
1.0
,
2.0
,
3.0
]);
// The two syntaxes below are equivalent.
let
a = foo.clone();
let
b = Arc::clone(
&
foo);
// a, b, and foo are all Arcs that point to the same memory location
§
Deref
behavior
Arc<T>
automatically dereferences to
T
(via the
Deref
trait),
so you can call
T
’s methods on a value of type
Arc<T>
. To avoid name
clashes with
T
’s methods, the methods of
Arc<T>
itself are associated
functions, called using
fully qualified syntax
:
use
std::sync::Arc;
let
my_arc = Arc::new(());
let
my_weak = Arc::downgrade(
&
my_arc);
Arc<T>
’s implementations of traits like
Clone
may also be called using
fully qualified syntax. Some people prefer to use fully qualified syntax,
while others prefer using method-call syntax.
use
std::sync::Arc;
let
arc = Arc::new(());
// Method-call syntax
let
arc2 = arc.clone();
// Fully qualified syntax
let
arc3 = Arc::clone(
&
arc);
Weak<T>
does not auto-dereference to
T
, because the inner value may have
already been dropped.
§
Examples
Sharing some immutable data between threads:
use
std::sync::Arc;
use
std::thread;
let
five = Arc::new(
5
);
for _ in
0
..
10
{
let
five = Arc::clone(
&
five);

    thread::spawn(
move
|| {
println!
(
"{five:?}"
);
    });
}
Sharing a mutable
AtomicUsize
:
use
std::sync::Arc;
use
std::sync::atomic::{AtomicUsize, Ordering};
use
std::thread;
let
val = Arc::new(AtomicUsize::new(
5
));
for _ in
0
..
10
{
let
val = Arc::clone(
&
val);

    thread::spawn(
move
|| {
let
v = val.fetch_add(
1
, Ordering::Relaxed);
println!
(
"{v:?}"
);
    });
}
See the
rc
documentation
for more examples of reference
counting in general.
Implementations
§
Source
§
impl<T>
Arc
<T>
1.0.0
·
Source
pub fn
new
(data: T) ->
Arc
<T>
Constructs a new
Arc<T>
.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
1.60.0
·
Source
pub fn
new_cyclic
<F>(data_fn: F) ->
Arc
<T>
where
    F:
FnOnce
(&
Weak
<T>) -> T,
Constructs a new
Arc<T>
while giving you a
Weak<T>
to the allocation,
to allow you to construct a
T
which holds a weak pointer to itself.
Generally, a structure circularly referencing itself, either directly or
indirectly, should not hold a strong reference to itself to prevent a memory leak.
Using this function, you get access to the weak pointer during the
initialization of
T
, before the
Arc<T>
is created, such that you can
clone and store it inside the
T
.
new_cyclic
first allocates the managed allocation for the
Arc<T>
,
then calls your closure, giving it a
Weak<T>
to this allocation,
and only afterwards completes the construction of the
Arc<T>
by placing
the
T
returned from your closure into the allocation.
Since the new
Arc<T>
is not fully-constructed until
Arc<T>::new_cyclic
returns, calling
upgrade
on the weak reference inside your closure will
fail and result in a
None
value.
§
Panics
If
data_fn
panics, the panic is propagated to the caller, and the
temporary
Weak<T>
is dropped normally.
§
Example
use
std::sync::{Arc, Weak};
struct
Gadget {
    me: Weak<Gadget>,
}
impl
Gadget {
/// Constructs a reference counted Gadget.
fn
new() -> Arc<
Self
> {
// `me` is a `Weak<Gadget>` pointing at the new allocation of the
        // `Arc` we're constructing.
Arc::new_cyclic(|me| {
// Create the actual struct here.
Gadget { me: me.clone() }
        })
    }
/// Returns a reference counted pointer to Self.
fn
me(
&
self
) -> Arc<
Self
> {
self
.me.upgrade().unwrap()
    }
}
1.82.0
·
Source
pub fn
new_uninit
() ->
Arc
<
MaybeUninit
<T>>
Constructs a new
Arc
with uninitialized contents.
§
Examples
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
let
mut
five = Arc::<u32>::new_uninit();
// Deferred initialization:
Arc::get_mut(
&mut
five).unwrap().write(
5
);
let
five =
unsafe
{ five.assume_init() };
assert_eq!
(
*
five,
5
)
Source
pub fn
new_zeroed
() ->
Arc
<
MaybeUninit
<T>>
🔬
This is a nightly-only experimental API. (
new_zeroed_alloc
#129396
)
Constructs a new
Arc
with uninitialized contents, with the memory
being filled with
0
bytes.
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(new_zeroed_alloc)]
use
std::sync::Arc;
let
zero = Arc::<u32>::new_zeroed();
let
zero =
unsafe
{ zero.assume_init() };
assert_eq!
(
*
zero,
0
)
1.33.0
·
Source
pub fn
pin
(data: T) ->
Pin
<
Arc
<T>>
Constructs a new
Pin<Arc<T>>
. If
T
does not implement
Unpin
, then
data
will be pinned in memory and unable to be moved.
Source
pub fn
try_pin
(data: T) ->
Result
<
Pin
<
Arc
<T>>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Pin<Arc<T>>
, return an error if allocation fails.
Source
pub fn
try_new
(data: T) ->
Result
<
Arc
<T>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc<T>
, returning an error if allocation fails.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
let
five = Arc::try_new(
5
)
?
;
Source
pub fn
try_new_uninit
() ->
Result
<
Arc
<
MaybeUninit
<T>>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc
with uninitialized contents, returning an error
if allocation fails.
§
Examples
#![feature(allocator_api)]
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
let
mut
five = Arc::<u32>::try_new_uninit()
?
;
// Deferred initialization:
Arc::get_mut(
&mut
five).unwrap().write(
5
);
let
five =
unsafe
{ five.assume_init() };
assert_eq!
(
*
five,
5
);
Source
pub fn
try_new_zeroed
() ->
Result
<
Arc
<
MaybeUninit
<T>>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc
with uninitialized contents, with the memory
being filled with
0
bytes, returning an error if allocation fails.
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature( allocator_api)]
use
std::sync::Arc;
let
zero = Arc::<u32>::try_new_zeroed()
?
;
let
zero =
unsafe
{ zero.assume_init() };
assert_eq!
(
*
zero,
0
);
Source
§
impl<T, A>
Arc
<T, A>
where
    A:
Allocator
,
Source
pub fn
new_in
(data: T, alloc: A) ->
Arc
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc<T>
in the provided allocator.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
five = Arc::new_in(
5
, System);
Source
pub fn
new_uninit_in
(alloc: A) ->
Arc
<
MaybeUninit
<T>, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc
with uninitialized contents in the provided allocator.
§
Examples
#![feature(get_mut_unchecked)]
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
mut
five = Arc::<u32,
_
>::new_uninit_in(System);
let
five =
unsafe
{
// Deferred initialization:
Arc::get_mut_unchecked(
&mut
five).as_mut_ptr().write(
5
);

    five.assume_init()
};
assert_eq!
(
*
five,
5
)
Source
pub fn
new_zeroed_in
(alloc: A) ->
Arc
<
MaybeUninit
<T>, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc
with uninitialized contents, with the memory
being filled with
0
bytes, in the provided allocator.
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
zero = Arc::<u32,
_
>::new_zeroed_in(System);
let
zero =
unsafe
{ zero.assume_init() };
assert_eq!
(
*
zero,
0
)
Source
pub fn
new_cyclic_in
<F>(data_fn: F, alloc: A) ->
Arc
<T, A>
where
    F:
FnOnce
(&
Weak
<T, A>) -> T,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc<T, A>
in the given allocator while giving you a
Weak<T, A>
to the allocation,
to allow you to construct a
T
which holds a weak pointer to itself.
Generally, a structure circularly referencing itself, either directly or
indirectly, should not hold a strong reference to itself to prevent a memory leak.
Using this function, you get access to the weak pointer during the
initialization of
T
, before the
Arc<T, A>
is created, such that you can
clone and store it inside the
T
.
new_cyclic_in
first allocates the managed allocation for the
Arc<T, A>
,
then calls your closure, giving it a
Weak<T, A>
to this allocation,
and only afterwards completes the construction of the
Arc<T, A>
by placing
the
T
returned from your closure into the allocation.
Since the new
Arc<T, A>
is not fully-constructed until
Arc<T, A>::new_cyclic_in
returns, calling
upgrade
on the weak reference inside your closure will
fail and result in a
None
value.
§
Panics
If
data_fn
panics, the panic is propagated to the caller, and the
temporary
Weak<T>
is dropped normally.
§
Example
See
new_cyclic
Source
pub fn
pin_in
(data: T, alloc: A) ->
Pin
<
Arc
<T, A>>
where
    A: 'static,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Pin<Arc<T, A>>
in the provided allocator. If
T
does not implement
Unpin
,
then
data
will be pinned in memory and unable to be moved.
Source
pub fn
try_pin_in
(data: T, alloc: A) ->
Result
<
Pin
<
Arc
<T, A>>,
AllocError
>
where
    A: 'static,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Pin<Arc<T, A>>
in the provided allocator, return an error if allocation
fails.
Source
pub fn
try_new_in
(data: T, alloc: A) ->
Result
<
Arc
<T, A>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc<T, A>
in the provided allocator, returning an error if allocation fails.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
five = Arc::try_new_in(
5
, System)
?
;
Source
pub fn
try_new_uninit_in
(alloc: A) ->
Result
<
Arc
<
MaybeUninit
<T>, A>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc
with uninitialized contents, in the provided allocator, returning an
error if allocation fails.
§
Examples
#![feature(allocator_api)]
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
use
std::alloc::System;
let
mut
five = Arc::<u32,
_
>::try_new_uninit_in(System)
?
;
let
five =
unsafe
{
// Deferred initialization:
Arc::get_mut_unchecked(
&mut
five).as_mut_ptr().write(
5
);

    five.assume_init()
};
assert_eq!
(
*
five,
5
);
Source
pub fn
try_new_zeroed_in
(alloc: A) ->
Result
<
Arc
<
MaybeUninit
<T>, A>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Arc
with uninitialized contents, with the memory
being filled with
0
bytes, in the provided allocator, returning an error if allocation
fails.
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
zero = Arc::<u32,
_
>::try_new_zeroed_in(System)
?
;
let
zero =
unsafe
{ zero.assume_init() };
assert_eq!
(
*
zero,
0
);
1.4.0
·
Source
pub fn
try_unwrap
(this:
Arc
<T, A>) ->
Result
<T,
Arc
<T, A>>
Returns the inner value, if the
Arc
has exactly one strong reference.
Otherwise, an
Err
is returned with the same
Arc
that was
passed in.
This will succeed even if there are outstanding weak references.
It is strongly recommended to use
Arc::into_inner
instead if you don’t
keep the
Arc
in the
Err
case.
Immediately dropping the
Err
-value, as the expression
Arc::try_unwrap(this).ok()
does, can cause the strong count to
drop to zero and the inner value of the
Arc
to be dropped.
For instance, if two threads execute such an expression in parallel,
there is a race condition without the possibility of unsafety:
The threads could first both check whether they own the last instance
in
Arc::try_unwrap
, determine that they both do not, and then both
discard and drop their instance in the call to
ok
.
In this scenario, the value inside the
Arc
is safely destroyed
by exactly one of the threads, but neither thread will ever be able
to use the value.
§
Examples
use
std::sync::Arc;
let
x = Arc::new(
3
);
assert_eq!
(Arc::try_unwrap(x),
Ok
(
3
));
let
x = Arc::new(
4
);
let
_y = Arc::clone(
&
x);
assert_eq!
(
*
Arc::try_unwrap(x).unwrap_err(),
4
);
1.70.0
·
Source
pub fn
into_inner
(this:
Arc
<T, A>) ->
Option
<T>
Returns the inner value, if the
Arc
has exactly one strong reference.
Otherwise,
None
is returned and the
Arc
is dropped.
This will succeed even if there are outstanding weak references.
If
Arc::into_inner
is called on every clone of this
Arc
,
it is guaranteed that exactly one of the calls returns the inner value.
This means in particular that the inner value is not dropped.
Arc::try_unwrap
is conceptually similar to
Arc::into_inner
, but it
is meant for different use-cases. If used as a direct replacement
for
Arc::into_inner
anyway, such as with the expression
Arc::try_unwrap
(this).
ok
()
, then it does
not
give the same guarantee as described in the previous paragraph.
For more information, see the examples below and read the documentation
of
Arc::try_unwrap
.
§
Examples
Minimal example demonstrating the guarantee that
Arc::into_inner
gives.
use
std::sync::Arc;
let
x = Arc::new(
3
);
let
y = Arc::clone(
&
x);
// Two threads calling `Arc::into_inner` on both clones of an `Arc`:
let
x_thread = std::thread::spawn(|| Arc::into_inner(x));
let
y_thread = std::thread::spawn(|| Arc::into_inner(y));
let
x_inner_value = x_thread.join().unwrap();
let
y_inner_value = y_thread.join().unwrap();
// One of the threads is guaranteed to receive the inner value:
assert!
(
matches!
(
    (x_inner_value, y_inner_value),
    (
None
,
Some
(
3
)) | (
Some
(
3
),
None
)
));
// The result could also be `(None, None)` if the threads called
// `Arc::try_unwrap(x).ok()` and `Arc::try_unwrap(y).ok()` instead.
A more practical example demonstrating the need for
Arc::into_inner
:
use
std::sync::Arc;
// Definition of a simple singly linked list using `Arc`:
#[derive(Clone)]
struct
LinkedList<T>(
Option
<Arc<Node<T>>>);
struct
Node<T>(T,
Option
<Arc<Node<T>>>);
// Dropping a long `LinkedList<T>` relying on the destructor of `Arc`
// can cause a stack overflow. To prevent this, we can provide a
// manual `Drop` implementation that does the destruction in a loop:
impl
<T> Drop
for
LinkedList<T> {
fn
drop(
&mut
self
) {
let
mut
link =
self
.
0
.take();
while let
Some
(arc_node) = link.take() {
if let
Some
(Node(_value, next)) = Arc::into_inner(arc_node) {
                link = next;
            }
        }
    }
}
// Implementation of `new` and `push` omitted
impl
<T> LinkedList<T> {
/* ... */
}
// The following code could have still caused a stack overflow
// despite the manual `Drop` impl if that `Drop` impl had used
// `Arc::try_unwrap(arc).ok()` instead of `Arc::into_inner(arc)`.

// Create a long list and clone it
let
mut
x = LinkedList::new();
let
size =
100000
;
for
i
in
0
..size {
    x.push(i);
// Adds i to the front of x
}
let
y = x.clone();
// Drop the clones in parallel
let
x_thread = std::thread::spawn(|| drop(x));
let
y_thread = std::thread::spawn(|| drop(y));
x_thread.join().unwrap();
y_thread.join().unwrap();
Source
§
impl<T>
Arc
<
[T]
>
1.82.0
·
Source
pub fn
new_uninit_slice
(len:
usize
) ->
Arc
<[
MaybeUninit
<T>]>
Constructs a new atomically reference-counted slice with uninitialized contents.
§
Examples
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
let
mut
values = Arc::<[u32]>::new_uninit_slice(
3
);
// Deferred initialization:
let
data = Arc::get_mut(
&mut
values).unwrap();
data[
0
].write(
1
);
data[
1
].write(
2
);
data[
2
].write(
3
);
let
values =
unsafe
{ values.assume_init() };
assert_eq!
(
*
values, [
1
,
2
,
3
])
Source
pub fn
new_zeroed_slice
(len:
usize
) ->
Arc
<[
MaybeUninit
<T>]>
🔬
This is a nightly-only experimental API. (
new_zeroed_alloc
#129396
)
Constructs a new atomically reference-counted slice with uninitialized contents, with the memory being
filled with
0
bytes.
See
MaybeUninit::zeroed
for examples of correct and
incorrect usage of this method.
§
Examples
#![feature(new_zeroed_alloc)]
use
std::sync::Arc;
let
values = Arc::<[u32]>::new_zeroed_slice(
3
);
let
values =
unsafe
{ values.assume_init() };
assert_eq!
(
*
values, [
0
,
0
,
0
])
Source
pub fn
into_array
<const N:
usize
>(self) ->
Option
<
Arc
<
[T; N]
>>
🔬
This is a nightly-only experimental API. (
slice_as_array
#133508
)
Converts the reference-counted slice into a reference-counted array.
This operation does not reallocate; the underlying array of the slice is simply reinterpreted as an array type.
If
N
is not exactly equal to the length of
self
, then this method returns
None
.
Source
§
impl<T, A>
Arc
<
[T]
, A>
where
    A:
Allocator
,
Source
pub fn
new_uninit_slice_in
(len:
usize
, alloc: A) ->
Arc
<[
MaybeUninit
<T>], A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new atomically reference-counted slice with uninitialized contents in the
provided allocator.
§
Examples
#![feature(get_mut_unchecked)]
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
mut
values = Arc::<[u32],
_
>::new_uninit_slice_in(
3
, System);
let
values =
unsafe
{
// Deferred initialization:
Arc::get_mut_unchecked(
&mut
values)[
0
].as_mut_ptr().write(
1
);
    Arc::get_mut_unchecked(
&mut
values)[
1
].as_mut_ptr().write(
2
);
    Arc::get_mut_unchecked(
&mut
values)[
2
].as_mut_ptr().write(
3
);

    values.assume_init()
};
assert_eq!
(
*
values, [
1
,
2
,
3
])
Source
pub fn
new_zeroed_slice_in
(len:
usize
, alloc: A) ->
Arc
<[
MaybeUninit
<T>], A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new atomically reference-counted slice with uninitialized contents, with the memory being
filled with
0
bytes, in the provided allocator.
See
MaybeUninit::zeroed
for examples of correct and
incorrect usage of this method.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
values = Arc::<[u32],
_
>::new_zeroed_slice_in(
3
, System);
let
values =
unsafe
{ values.assume_init() };
assert_eq!
(
*
values, [
0
,
0
,
0
])
Source
§
impl<T, A>
Arc
<
MaybeUninit
<T>, A>
where
    A:
Allocator
,
1.82.0
·
Source
pub unsafe fn
assume_init
(self) ->
Arc
<T, A>
Converts to
Arc<T>
.
§
Safety
As with
MaybeUninit::assume_init
,
it is up to the caller to guarantee that the inner value
really is in an initialized state.
Calling this when the content is not yet fully initialized
causes immediate undefined behavior.
§
Examples
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
let
mut
five = Arc::<u32>::new_uninit();
// Deferred initialization:
Arc::get_mut(
&mut
five).unwrap().write(
5
);
let
five =
unsafe
{ five.assume_init() };
assert_eq!
(
*
five,
5
)
Source
§
impl<T, A>
Arc
<[
MaybeUninit
<T>], A>
where
    A:
Allocator
,
1.82.0
·
Source
pub unsafe fn
assume_init
(self) ->
Arc
<
[T]
, A>
Converts to
Arc<[T]>
.
§
Safety
As with
MaybeUninit::assume_init
,
it is up to the caller to guarantee that the inner value
really is in an initialized state.
Calling this when the content is not yet fully initialized
causes immediate undefined behavior.
§
Examples
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
let
mut
values = Arc::<[u32]>::new_uninit_slice(
3
);
// Deferred initialization:
let
data = Arc::get_mut(
&mut
values).unwrap();
data[
0
].write(
1
);
data[
1
].write(
2
);
data[
2
].write(
3
);
let
values =
unsafe
{ values.assume_init() };
assert_eq!
(
*
values, [
1
,
2
,
3
])
Source
§
impl<T>
Arc
<T>
where
    T: ?
Sized
,
1.17.0
·
Source
pub unsafe fn
from_raw
(ptr:
*const T
) ->
Arc
<T>
Constructs an
Arc<T>
from a raw pointer.
The raw pointer must have been previously returned by a call to
Arc<U>::into_raw
with the following requirements:
If
U
is sized, it must have the same size and alignment as
T
. This
is trivially true if
U
is
T
.
If
U
is unsized, its data pointer must have the same size and
alignment as
T
. This is trivially true if
Arc<U>
was constructed
through
Arc<T>
and then converted to
Arc<U>
through an
unsized
coercion
.
Note that if
U
or
U
’s data pointer is not
T
but has the same size
and alignment, this is basically like transmuting references of
different types. See
mem::transmute
for more information
on what restrictions apply in this case.
The raw pointer must point to a block of memory allocated by the global allocator.
The user of
from_raw
has to make sure a specific value of
T
is only
dropped once.
This function is unsafe because improper use may lead to memory unsafety,
even if the returned
Arc<T>
is never accessed.
§
Examples
use
std::sync::Arc;
let
x = Arc::new(
"hello"
.to_owned());
let
x_ptr = Arc::into_raw(x);
unsafe
{
// Convert back to an `Arc` to prevent leak.
let
x = Arc::from_raw(x_ptr);
assert_eq!
(
&*
x,
"hello"
);
// Further calls to `Arc::from_raw(x_ptr)` would be memory-unsafe.
}
// The memory was freed when `x` went out of scope above, so `x_ptr` is now dangling!
Convert a slice back into its original array:
use
std::sync::Arc;
let
x: Arc<[u32]> = Arc::new([
1
,
2
,
3
]);
let
x_ptr:
*const
[u32] = Arc::into_raw(x);
unsafe
{
let
x: Arc<[u32;
3
]> = Arc::from_raw(x_ptr.cast::<[u32;
3
]>());
assert_eq!
(
&*
x,
&
[
1
,
2
,
3
]);
}
1.51.0
·
Source
pub unsafe fn
increment_strong_count
(ptr:
*const T
)
Increments the strong reference count on the
Arc<T>
associated with the
provided pointer by one.
§
Safety
The pointer must have been obtained through
Arc::into_raw
and must satisfy the
same layout requirements specified in
Arc::from_raw_in
.
The associated
Arc
instance must be valid (i.e. the strong count must be at
least 1) for the duration of this method, and
ptr
must point to a block of memory
allocated by the global allocator.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
unsafe
{
let
ptr = Arc::into_raw(five);
    Arc::increment_strong_count(ptr);
// This assertion is deterministic because we haven't shared
    // the `Arc` between threads.
let
five = Arc::from_raw(ptr);
assert_eq!
(
2
, Arc::strong_count(
&
five));
}
1.51.0
·
Source
pub unsafe fn
decrement_strong_count
(ptr:
*const T
)
Decrements the strong reference count on the
Arc<T>
associated with the
provided pointer by one.
§
Safety
The pointer must have been obtained through
Arc::into_raw
and must satisfy the
same layout requirements specified in
Arc::from_raw_in
.
The associated
Arc
instance must be valid (i.e. the strong count must be at
least 1) when invoking this method, and
ptr
must point to a block of memory
allocated by the global allocator. This method can be used to release the final
Arc
and backing storage, but
should not
be called after the final
Arc
has been
released.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
unsafe
{
let
ptr = Arc::into_raw(five);
    Arc::increment_strong_count(ptr);
// Those assertions are deterministic because we haven't shared
    // the `Arc` between threads.
let
five = Arc::from_raw(ptr);
assert_eq!
(
2
, Arc::strong_count(
&
five));
    Arc::decrement_strong_count(ptr);
assert_eq!
(
1
, Arc::strong_count(
&
five));
}
Source
§
impl<T, A>
Arc
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
(this: &
Arc
<T, A>) ->
&A
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Returns a reference to the underlying allocator.
Note: this is an associated function, which means that you have
to call it as
Arc::allocator(&a)
instead of
a.allocator()
. This
is so that there is no conflict with a method on the inner type.
1.17.0
·
Source
pub fn
into_raw
(this:
Arc
<T, A>) ->
*const T
Consumes the
Arc
, returning the wrapped pointer.
To avoid a memory leak the pointer must be converted back to an
Arc
using
Arc::from_raw
.
§
Examples
use
std::sync::Arc;
let
x = Arc::new(
"hello"
.to_owned());
let
x_ptr = Arc::into_raw(x);
assert_eq!
(
unsafe
{
&*
x_ptr },
"hello"
);
Source
pub fn
into_raw_with_allocator
(this:
Arc
<T, A>) -> (
*const T
, A)
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Consumes the
Arc
, returning the wrapped pointer and allocator.
To avoid a memory leak the pointer must be converted back to an
Arc
using
Arc::from_raw_in
.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
x = Arc::new_in(
"hello"
.to_owned(), System);
let
(ptr, alloc) = Arc::into_raw_with_allocator(x);
assert_eq!
(
unsafe
{
&*
ptr },
"hello"
);
let
x =
unsafe
{ Arc::from_raw_in(ptr, alloc) };
assert_eq!
(
&*
x,
"hello"
);
1.45.0
·
Source
pub fn
as_ptr
(this: &
Arc
<T, A>) ->
*const T
Provides a raw pointer to the data.
The counts are not affected in any way and the
Arc
is not consumed. The pointer is valid for
as long as there are strong counts in the
Arc
.
§
Examples
use
std::sync::Arc;
let
x = Arc::new(
"hello"
.to_owned());
let
y = Arc::clone(
&
x);
let
x_ptr = Arc::as_ptr(
&
x);
assert_eq!
(x_ptr, Arc::as_ptr(
&
y));
assert_eq!
(
unsafe
{
&*
x_ptr },
"hello"
);
Source
pub unsafe fn
from_raw_in
(ptr:
*const T
, alloc: A) ->
Arc
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs an
Arc<T, A>
from a raw pointer.
The raw pointer must have been previously returned by a call to
Arc<U, A>::into_raw
with the following requirements:
If
U
is sized, it must have the same size and alignment as
T
. This
is trivially true if
U
is
T
.
If
U
is unsized, its data pointer must have the same size and
alignment as
T
. This is trivially true if
Arc<U>
was constructed
through
Arc<T>
and then converted to
Arc<U>
through an
unsized
coercion
.
Note that if
U
or
U
’s data pointer is not
T
but has the same size
and alignment, this is basically like transmuting references of
different types. See
mem::transmute
for more information
on what restrictions apply in this case.
The raw pointer must point to a block of memory allocated by
alloc
The user of
from_raw
has to make sure a specific value of
T
is only
dropped once.
This function is unsafe because improper use may lead to memory unsafety,
even if the returned
Arc<T>
is never accessed.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
x = Arc::new_in(
"hello"
.to_owned(), System);
let
x_ptr = Arc::into_raw(x);
unsafe
{
// Convert back to an `Arc` to prevent leak.
let
x = Arc::from_raw_in(x_ptr, System);
assert_eq!
(
&*
x,
"hello"
);
// Further calls to `Arc::from_raw(x_ptr)` would be memory-unsafe.
}
// The memory was freed when `x` went out of scope above, so `x_ptr` is now dangling!
Convert a slice back into its original array:
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
x: Arc<[u32],
_
> = Arc::new_in([
1
,
2
,
3
], System);
let
x_ptr:
*const
[u32] = Arc::into_raw(x);
unsafe
{
let
x: Arc<[u32;
3
],
_
> = Arc::from_raw_in(x_ptr.cast::<[u32;
3
]>(), System);
assert_eq!
(
&*
x,
&
[
1
,
2
,
3
]);
}
1.4.0
·
Source
pub fn
downgrade
(this: &
Arc
<T, A>) ->
Weak
<T, A>
where
    A:
Clone
,
Creates a new
Weak
pointer to this allocation.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
let
weak_five = Arc::downgrade(
&
five);
1.15.0
·
Source
pub fn
weak_count
(this: &
Arc
<T, A>) ->
usize
Gets the number of
Weak
pointers to this allocation.
§
Safety
This method by itself is safe, but using it correctly requires extra care.
Another thread can change the weak count at any time,
including potentially between calling this method and acting on the result.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
let
_weak_five = Arc::downgrade(
&
five);
// This assertion is deterministic because we haven't shared
// the `Arc` or `Weak` between threads.
assert_eq!
(
1
, Arc::weak_count(
&
five));
1.15.0
·
Source
pub fn
strong_count
(this: &
Arc
<T, A>) ->
usize
Gets the number of strong (
Arc
) pointers to this allocation.
§
Safety
This method by itself is safe, but using it correctly requires extra care.
Another thread can change the strong count at any time,
including potentially between calling this method and acting on the result.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
let
_also_five = Arc::clone(
&
five);
// This assertion is deterministic because we haven't shared
// the `Arc` between threads.
assert_eq!
(
2
, Arc::strong_count(
&
five));
Source
pub unsafe fn
increment_strong_count_in
(ptr:
*const T
, alloc: A)
where
    A:
Clone
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Increments the strong reference count on the
Arc<T>
associated with the
provided pointer by one.
§
Safety
The pointer must have been obtained through
Arc::into_raw
and must satisfy the
same layout requirements specified in
Arc::from_raw_in
.
The associated
Arc
instance must be valid (i.e. the strong count must be at
least 1) for the duration of this method, and
ptr
must point to a block of memory
allocated by
alloc
.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
five = Arc::new_in(
5
, System);
unsafe
{
let
ptr = Arc::into_raw(five);
    Arc::increment_strong_count_in(ptr, System);
// This assertion is deterministic because we haven't shared
    // the `Arc` between threads.
let
five = Arc::from_raw_in(ptr, System);
assert_eq!
(
2
, Arc::strong_count(
&
five));
}
Source
pub unsafe fn
decrement_strong_count_in
(ptr:
*const T
, alloc: A)
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Decrements the strong reference count on the
Arc<T>
associated with the
provided pointer by one.
§
Safety
The pointer must have been obtained through
Arc::into_raw
and must satisfy the
same layout requirements specified in
Arc::from_raw_in
.
The associated
Arc
instance must be valid (i.e. the strong count must be at
least 1) when invoking this method, and
ptr
must point to a block of memory
allocated by
alloc
. This method can be used to release the final
Arc
and backing storage, but
should not
be called after the final
Arc
has been
released.
§
Examples
#![feature(allocator_api)]
use
std::sync::Arc;
use
std::alloc::System;
let
five = Arc::new_in(
5
, System);
unsafe
{
let
ptr = Arc::into_raw(five);
    Arc::increment_strong_count_in(ptr, System);
// Those assertions are deterministic because we haven't shared
    // the `Arc` between threads.
let
five = Arc::from_raw_in(ptr, System);
assert_eq!
(
2
, Arc::strong_count(
&
five));
    Arc::decrement_strong_count_in(ptr, System);
assert_eq!
(
1
, Arc::strong_count(
&
five));
}
1.17.0
·
Source
pub fn
ptr_eq
(this: &
Arc
<T, A>, other: &
Arc
<T, A>) ->
bool
Returns
true
if the two
Arc
s point to the same allocation in a vein similar to
ptr::eq
. This function ignores the metadata of
dyn Trait
pointers.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
let
same_five = Arc::clone(
&
five);
let
other_five = Arc::new(
5
);
assert!
(Arc::ptr_eq(
&
five,
&
same_five));
assert!
(!Arc::ptr_eq(
&
five,
&
other_five));
Source
§
impl<T, A>
Arc
<T, A>
where
    T:
CloneToUninit
+ ?
Sized
,
    A:
Allocator
+
Clone
,
1.4.0
·
Source
pub fn
make_mut
(this: &mut
Arc
<T, A>) ->
&mut T
Makes a mutable reference into the given
Arc
.
If there are other
Arc
pointers to the same allocation, then
make_mut
will
clone
the inner value to a new allocation to ensure unique ownership.  This is also
referred to as clone-on-write.
However, if there are no other
Arc
pointers to this allocation, but some
Weak
pointers, then the
Weak
pointers will be dissociated and the inner value will not
be cloned.
See also
get_mut
, which will fail rather than cloning the inner value
or dissociating
Weak
pointers.
§
Examples
use
std::sync::Arc;
let
mut
data = Arc::new(
5
);
*
Arc::make_mut(
&mut
data) +=
1
;
// Won't clone anything
let
mut
other_data = Arc::clone(
&
data);
// Won't clone inner data
*
Arc::make_mut(
&mut
data) +=
1
;
// Clones inner data
*
Arc::make_mut(
&mut
data) +=
1
;
// Won't clone anything
*
Arc::make_mut(
&mut
other_data)
*
=
2
;
// Won't clone anything

// Now `data` and `other_data` point to different allocations.
assert_eq!
(
*
data,
8
);
assert_eq!
(
*
other_data,
12
);
Weak
pointers will be dissociated:
use
std::sync::Arc;
let
mut
data = Arc::new(
75
);
let
weak = Arc::downgrade(
&
data);
assert!
(
75
==
*
data);
assert!
(
75
==
*
weak.upgrade().unwrap());
*
Arc::make_mut(
&mut
data) +=
1
;
assert!
(
76
==
*
data);
assert!
(weak.upgrade().is_none());
Source
§
impl<T, A>
Arc
<T, A>
where
    T:
Clone
,
    A:
Allocator
,
1.76.0
·
Source
pub fn
unwrap_or_clone
(this:
Arc
<T, A>) -> T
If we have the only reference to
T
then unwrap it. Otherwise, clone
T
and return the
clone.
Assuming
arc_t
is of type
Arc<T>
, this function is functionally equivalent to
(*arc_t).clone()
, but will avoid cloning the inner value where possible.
§
Examples
let
inner = String::from(
"test"
);
let
ptr = inner.as_ptr();
let
arc = Arc::new(inner);
let
inner = Arc::unwrap_or_clone(arc);
// The inner value was not cloned
assert!
(ptr::eq(ptr, inner.as_ptr()));
let
arc = Arc::new(inner);
let
arc2 = arc.clone();
let
inner = Arc::unwrap_or_clone(arc);
// Because there were 2 references, we had to clone the inner value.
assert!
(!ptr::eq(ptr, inner.as_ptr()));
// `arc2` is the last reference, so when we unwrap it we get back
// the original `String`.
let
inner = Arc::unwrap_or_clone(arc2);
assert!
(ptr::eq(ptr, inner.as_ptr()));
Source
§
impl<T, A>
Arc
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
pub fn
get_mut
(this: &mut
Arc
<T, A>) ->
Option
<
&mut T
>
Returns a mutable reference into the given
Arc
, if there are
no other
Arc
or
Weak
pointers to the same allocation.
Returns
None
otherwise, because it is not safe to
mutate a shared value.
See also
make_mut
, which will
clone
the inner value when there are other
Arc
pointers.
§
Examples
use
std::sync::Arc;
let
mut
x = Arc::new(
3
);
*
Arc::get_mut(
&mut
x).unwrap() =
4
;
assert_eq!
(
*
x,
4
);
let
_y = Arc::clone(
&
x);
assert!
(Arc::get_mut(
&mut
x).is_none());
Source
pub unsafe fn
get_mut_unchecked
(this: &mut
Arc
<T, A>) ->
&mut T
🔬
This is a nightly-only experimental API. (
get_mut_unchecked
#63292
)
Returns a mutable reference into the given
Arc
,
without any check.
See also
get_mut
, which is safe and does appropriate checks.
§
Safety
If any other
Arc
or
Weak
pointers to the same allocation exist, then
they must not be dereferenced or have active borrows for the duration
of the returned borrow, and their inner type must be exactly the same as the
inner type of this Rc (including lifetimes). This is trivially the case if no
such pointers exist, for example immediately after
Arc::new
.
§
Examples
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
let
mut
x = Arc::new(String::new());
unsafe
{
    Arc::get_mut_unchecked(
&mut
x).push_str(
"foo"
)
}
assert_eq!
(
*
x,
"foo"
);
Other
Arc
pointers to the same allocation must be to the same type.
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
let
x: Arc<str> = Arc::from(
"Hello, world!"
);
let
mut
y: Arc<[u8]> = x.clone().into();
unsafe
{
// this is Undefined Behavior, because x's inner type is str, not [u8]
Arc::get_mut_unchecked(
&mut
y).fill(
0xff
);
// 0xff is invalid in UTF-8
}
println!
(
"{}"
,
&*
x);
// Invalid UTF-8 in a str
Other
Arc
pointers to the same allocation must be to the exact same type, including lifetimes.
#![feature(get_mut_unchecked)]
use
std::sync::Arc;
let
x: Arc<
&
str> = Arc::new(
"Hello, world!"
);
{
let
s = String::from(
"Oh, no!"
);
let
mut
y: Arc<
&
str> = x.clone();
unsafe
{
// this is Undefined Behavior, because x's inner type
        // is &'long str, not &'short str
*
Arc::get_mut_unchecked(
&mut
y) =
&
s;
    }
}
println!
(
"{}"
,
&*
x);
// Use-after-free
Source
§
impl<A>
Arc
<dyn
Any
+
Sync
+
Send
, A>
where
    A:
Allocator
,
1.29.0
·
Source
pub fn
downcast
<T>(self) ->
Result
<
Arc
<T, A>,
Arc
<dyn
Any
+
Sync
+
Send
, A>>
where
    T:
Any
+
Send
+
Sync
,
Attempts to downcast the
Arc<dyn Any + Send + Sync>
to a concrete type.
§
Examples
use
std::any::Any;
use
std::sync::Arc;
fn
print_if_string(value: Arc<
dyn
Any + Send + Sync>) {
if let
Ok
(string) = value.downcast::<String>() {
println!
(
"String ({}): {}"
, string.len(), string);
    }
}
let
my_string =
"Hello World"
.to_string();
print_if_string(Arc::new(my_string));
print_if_string(Arc::new(
0i8
));
Source
pub unsafe fn
downcast_unchecked
<T>(self) ->
Arc
<T, A>
where
    T:
Any
+
Send
+
Sync
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Downcasts the
Arc<dyn Any + Send + Sync>
to a concrete type.
For a safe alternative see
downcast
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
use
std::sync::Arc;
let
x: Arc<
dyn
Any + Send + Sync> = Arc::new(
1_usize
);
unsafe
{
assert_eq!
(
*
x.downcast_unchecked::<usize>(),
1
);
}
§
Safety
The contained value must be of type
T
. Calling this method
with the incorrect type is
undefined behavior
.
Trait Implementations
§
1.64.0
·
Source
§
impl<T:
AsFd
+ ?
Sized
>
AsFd
for
Arc
<T>
This impl allows implementing traits that require
AsFd
on Arc.
use
std::net::UdpSocket;
use
std::sync::Arc;
trait
MyTrait: AsFd {}
impl
MyTrait
for
Arc<UdpSocket> {}
impl
MyTrait
for
Box<UdpSocket> {}
Source
§
fn
as_fd
(&self) ->
BorrowedFd
<'_>
Borrows the file descriptor.
Read more
1.71.0
·
Source
§
impl<T:
AsHandle
+ ?
Sized
>
AsHandle
for
Arc
<T>
Available on
Windows
only.
This impl allows implementing traits that require
AsHandle
on Arc.
use
std::fs::File;
use
std::sync::Arc;
trait
MyTrait: AsHandle {}
impl
MyTrait
for
Arc<File> {}
impl
MyTrait
for
Box<File> {}
Source
§
fn
as_handle
(&self) ->
BorrowedHandle
<'_>
Borrows the handle.
Read more
1.63.0
·
Source
§
impl<T:
AsRawFd
>
AsRawFd
for
Arc
<T>
This impl allows implementing traits that require
AsRawFd
on Arc.
use
std::net::UdpSocket;
use
std::sync::Arc;
trait
MyTrait: AsRawFd {
}
impl
MyTrait
for
Arc<UdpSocket> {}
impl
MyTrait
for
Box<UdpSocket> {}
Source
§
fn
as_raw_fd
(&self) ->
RawFd
Extracts the raw file descriptor.
Read more
1.5.0
·
Source
§
impl<T, A>
AsRef
<T> for
Arc
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
as_ref
(&self) ->
&T
Converts this type into a shared reference of the (usually inferred) input type.
1.71.0
·
Source
§
impl<T:
AsSocket
>
AsSocket
for
Arc
<T>
Available on
Windows
only.
This impl allows implementing traits that require
AsSocket
on Arc.
use
std::net::UdpSocket;
use
std::sync::Arc;
trait
MyTrait: AsSocket {}
impl
MyTrait
for
Arc<UdpSocket> {}
impl
MyTrait
for
Box<UdpSocket> {}
Source
§
fn
as_socket
(&self) ->
BorrowedSocket
<'_>
Borrows the socket.
1.0.0
·
Source
§
impl<T, A>
Borrow
<T> for
Arc
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
borrow
(&self) ->
&T
Immutably borrows from an owned value.
Read more
1.0.0
·
Source
§
impl<T, A>
Clone
for
Arc
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
Arc
<T, A>
Makes a clone of the
Arc
pointer.
This creates another pointer to the same allocation, increasing the
strong reference count.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
let _
= Arc::clone(
&
five);
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
impl<T, A>
Debug
for
Arc
<T, A>
where
    T:
Debug
+ ?
Sized
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
1.80.0
·
Source
§
impl<T>
Default
for
Arc
<
[T]
>
Source
§
fn
default
() ->
Arc
<
[T]
>
Creates an empty
[T]
inside an Arc
This may or may not share an allocation with other Arcs.
1.80.0
·
Source
§
impl
Default
for
Arc
<
CStr
>
Source
§
fn
default
() ->
Arc
<
CStr
>
Creates an empty CStr inside an Arc
This may or may not share an allocation with other Arcs.
1.0.0
·
Source
§
impl<T>
Default
for
Arc
<T>
where
    T:
Default
,
Source
§
fn
default
() ->
Arc
<T>
Creates a new
Arc<T>
, with the
Default
value for
T
.
§
Examples
use
std::sync::Arc;
let
x: Arc<i32> = Default::default();
assert_eq!
(
*
x,
0
);
1.80.0
·
Source
§
impl
Default
for
Arc
<
str
>
Source
§
fn
default
() ->
Arc
<
str
>
Creates an empty str inside an Arc
This may or may not share an allocation with other Arcs.
1.0.0
·
Source
§
impl<T, A>
Deref
for
Arc
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
1.0.0
·
Source
§
impl<T, A>
Display
for
Arc
<T, A>
where
    T:
Display
+ ?
Sized
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
impl<T, A>
Drop
for
Arc
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
Arc
.
This will decrement the strong reference count. If the strong reference
count reaches zero then the only other references (if any) are
Weak
, so we
drop
the inner value.
§
Examples
use
std::sync::Arc;
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
foo  = Arc::new(Foo);
let
foo2 = Arc::clone(
&
foo);

drop(foo);
// Doesn't print anything
drop(foo2);
// Prints "dropped!"
1.52.0
·
Source
§
impl<T>
Error
for
Arc
<T>
where
    T:
Error
+ ?
Sized
,
Source
§
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
Read more
Source
§
fn
cause
(&self) ->
Option
<&dyn
Error
>
👎
Deprecated since 1.33.0: replaced by Error::source, which can support downcasting
Source
§
fn
source
(&self) ->
Option
<&(dyn
Error
+ 'static)>
Returns the lower-level source of this error, if any.
Read more
Source
§
fn
provide
<'a>(&'a self, req: &mut
Request
<'a>)
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides type-based access to context intended for error reports.
Read more
1.21.0
·
Source
§
impl<T>
From
<&
[T]
> for
Arc
<
[T]
>
where
    T:
Clone
,
Source
§
fn
from
(v: &
[T]
) ->
Arc
<
[T]
>
Allocates a reference-counted slice and fills it by cloning
v
’s items.
§
Example
let
original:
&
[i32] =
&
[
1
,
2
,
3
];
let
shared: Arc<[i32]> = Arc::from(original);
assert_eq!
(
&
[
1
,
2
,
3
],
&
shared[..]);
1.24.0
·
Source
§
impl
From
<&
CStr
> for
Arc
<
CStr
>
Source
§
fn
from
(s: &
CStr
) ->
Arc
<
CStr
>
Converts a
&CStr
into a
Arc<CStr>
,
by copying the contents into a newly allocated
Arc
.
1.24.0
·
Source
§
impl
From
<&
OsStr
> for
Arc
<
OsStr
>
Source
§
fn
from
(s: &
OsStr
) ->
Arc
<
OsStr
>
Copies the string into a newly allocated
Arc
<
OsStr
>
.
1.24.0
·
Source
§
impl
From
<&
Path
> for
Arc
<
Path
>
Source
§
fn
from
(s: &
Path
) ->
Arc
<
Path
>
Converts a
Path
into an
Arc
by copying the
Path
data into a new
Arc
buffer.
1.84.0
·
Source
§
impl<T>
From
<&mut
[T]
> for
Arc
<
[T]
>
where
    T:
Clone
,
Source
§
fn
from
(v: &mut
[T]
) ->
Arc
<
[T]
>
Allocates a reference-counted slice and fills it by cloning
v
’s items.
§
Example
let
mut
original = [
1
,
2
,
3
];
let
original:
&mut
[i32] =
&mut
original;
let
shared: Arc<[i32]> = Arc::from(original);
assert_eq!
(
&
[
1
,
2
,
3
],
&
shared[..]);
1.84.0
·
Source
§
impl
From
<&mut
CStr
> for
Arc
<
CStr
>
Source
§
fn
from
(s: &mut
CStr
) ->
Arc
<
CStr
>
Converts a
&mut CStr
into a
Arc<CStr>
,
by copying the contents into a newly allocated
Arc
.
1.84.0
·
Source
§
impl
From
<&mut
OsStr
> for
Arc
<
OsStr
>
Source
§
fn
from
(s: &mut
OsStr
) ->
Arc
<
OsStr
>
Copies the string into a newly allocated
Arc
<
OsStr
>
.
1.84.0
·
Source
§
impl
From
<&mut
Path
> for
Arc
<
Path
>
Source
§
fn
from
(s: &mut
Path
) ->
Arc
<
Path
>
Converts a
Path
into an
Arc
by copying the
Path
data into a new
Arc
buffer.
1.84.0
·
Source
§
impl
From
<&mut
str
> for
Arc
<
str
>
Source
§
fn
from
(v: &mut
str
) ->
Arc
<
str
>
Allocates a reference-counted
str
and copies
v
into it.
§
Example
let
mut
original = String::from(
"eggplant"
);
let
original:
&mut
str =
&mut
original;
let
shared: Arc<str> = Arc::from(original);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.21.0
·
Source
§
impl
From
<&
str
> for
Arc
<
str
>
Source
§
fn
from
(v: &
str
) ->
Arc
<
str
>
Allocates a reference-counted
str
and copies
v
into it.
§
Example
let
shared: Arc<str> = Arc::from(
"eggplant"
);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.74.0
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
Arc
<
[T]
>
Source
§
fn
from
(v:
[T; N]
) ->
Arc
<
[T]
>
Converts a
[T; N]
into an
Arc<[T]>
.
The conversion moves the array into a newly allocated
Arc
.
§
Example
let
original: [i32;
3
] = [
1
,
2
,
3
];
let
shared: Arc<[i32]> = Arc::from(original);
assert_eq!
(
&
[
1
,
2
,
3
],
&
shared[..]);
Source
§
impl
From
<
Arc
<[
u8
]>> for
Arc
<
ByteStr
>
Source
§
fn
from
(s:
Arc
<[
u8
]>) ->
Arc
<
ByteStr
>
Converts to this type from the input type.
Source
§
impl
From
<
Arc
<
ByteStr
>> for
Arc
<[
u8
]>
Source
§
fn
from
(s:
Arc
<
ByteStr
>) ->
Arc
<[
u8
]>
Converts to this type from the input type.
1.51.0
·
Source
§
impl<W>
From
<
Arc
<W>> for
RawWaker
where
    W:
Wake
+
Send
+
Sync
+ 'static,
Source
§
fn
from
(waker:
Arc
<W>) ->
RawWaker
Use a
Wake
-able type as a
RawWaker
.
No heap allocations or atomic operations are used for this conversion.
1.51.0
·
Source
§
impl<W>
From
<
Arc
<W>> for
Waker
where
    W:
Wake
+
Send
+
Sync
+ 'static,
Source
§
fn
from
(waker:
Arc
<W>) ->
Waker
Use a
Wake
-able type as a
Waker
.
No heap allocations or atomic operations are used for this conversion.
1.62.0
·
Source
§
impl
From
<
Arc
<
str
>> for
Arc
<[
u8
]>
Source
§
fn
from
(rc:
Arc
<
str
>) ->
Arc
<[
u8
]>
Converts an atomically reference-counted string slice into a byte slice.
§
Example
let
string: Arc<str> = Arc::from(
"eggplant"
);
let
bytes: Arc<[u8]> = Arc::from(string);
assert_eq!
(
"eggplant"
.as_bytes(), bytes.as_ref());
1.21.0
·
Source
§
impl<T, A>
From
<
Box
<T, A>> for
Arc
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
from
(v:
Box
<T, A>) ->
Arc
<T, A>
Move a boxed object to a new, reference-counted allocation.
§
Example
let
unique: Box<str> = Box::from(
"eggplant"
);
let
shared: Arc<str> = Arc::from(unique);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.24.0
·
Source
§
impl
From
<
CString
> for
Arc
<
CStr
>
Source
§
fn
from
(s:
CString
) ->
Arc
<
CStr
>
Converts a
CString
into an
Arc
<
CStr
>
by moving the
CString
data into a new
Arc
buffer.
1.45.0
·
Source
§
impl<'a, B>
From
<
Cow
<'a, B>> for
Arc
<B>
where
    B:
ToOwned
+ ?
Sized
,
Arc
<B>:
From
<
&'a B
> +
From
<<B as
ToOwned
>::
Owned
>,
Source
§
fn
from
(cow:
Cow
<'a, B>) ->
Arc
<B>
Creates an atomically reference-counted pointer from a clone-on-write
pointer by copying its content.
§
Example
let
cow: Cow<
'_
, str> = Cow::Borrowed(
"eggplant"
);
let
shared: Arc<str> = Arc::from(cow);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.24.0
·
Source
§
impl
From
<
OsString
> for
Arc
<
OsStr
>
Source
§
fn
from
(s:
OsString
) ->
Arc
<
OsStr
>
Converts an
OsString
into an
Arc
<
OsStr
>
by moving the
OsString
data into a new
Arc
buffer.
1.24.0
·
Source
§
impl
From
<
PathBuf
> for
Arc
<
Path
>
Source
§
fn
from
(s:
PathBuf
) ->
Arc
<
Path
>
Converts a
PathBuf
into an
Arc
<
Path
>
by moving the
PathBuf
data
into a new
Arc
buffer.
1.21.0
·
Source
§
impl
From
<
String
> for
Arc
<
str
>
Source
§
fn
from
(v:
String
) ->
Arc
<
str
>
Allocates a reference-counted
str
and copies
v
into it.
§
Example
let
unique: String =
"eggplant"
.to_owned();
let
shared: Arc<str> = Arc::from(unique);
assert_eq!
(
"eggplant"
,
&
shared[..]);
1.6.0
·
Source
§
impl<T>
From
<T> for
Arc
<T>
Source
§
fn
from
(t: T) ->
Arc
<T>
Converts a
T
into an
Arc<T>
The conversion moves the value into a
newly allocated
Arc
. It is equivalent to
calling
Arc::new(t)
.
§
Example
let
x =
5
;
let
arc = Arc::new(
5
);
assert_eq!
(Arc::from(x), arc);
1.21.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
Arc
<
[T]
, A>
where
    A:
Allocator
+
Clone
,
Source
§
fn
from
(v:
Vec
<T, A>) ->
Arc
<
[T]
, A>
Allocates a reference-counted slice and moves
v
’s items into it.
§
Example
let
unique: Vec<i32> =
vec!
[
1
,
2
,
3
];
let
shared: Arc<[i32]> = Arc::from(unique);
assert_eq!
(
&
[
1
,
2
,
3
],
&
shared[..]);
1.37.0
·
Source
§
impl<T>
FromIterator
<T> for
Arc
<
[T]
>
Source
§
fn
from_iter
<I>(iter: I) ->
Arc
<
[T]
>
where
    I:
IntoIterator
<Item = T>,
Takes each element in the
Iterator
and collects it into an
Arc<[T]>
.
§
Performance characteristics
§
The general case
In the general case, collecting into
Arc<[T]>
is done by first
collecting into a
Vec<T>
. That is, when writing the following:
let
evens: Arc<[u8]> = (
0
..
10
).filter(|
&
x| x %
2
==
0
).collect();
this behaves as if we wrote:
let
evens: Arc<[u8]> = (
0
..
10
).filter(|
&
x| x %
2
==
0
)
    .collect::<Vec<
_
>>()
// The first set of allocations happens here.
.into();
// A second allocation for `Arc<[T]>` happens here.
This will allocate as many times as needed for constructing the
Vec<T>
and then it will allocate once for turning the
Vec<T>
into the
Arc<[T]>
.
§
Iterators of known length
When your
Iterator
implements
TrustedLen
and is of an exact size,
a single allocation will be made for the
Arc<[T]>
. For example:
let
evens: Arc<[u8]> = (
0
..
10
).collect();
// Just a single allocation happens here.
1.0.0
·
Source
§
impl<T, A>
Hash
for
Arc
<T, A>
where
    T:
Hash
+ ?
Sized
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
Ord
for
Arc
<T, A>
where
    T:
Ord
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
cmp
(&self, other: &
Arc
<T, A>) ->
Ordering
Comparison for two
Arc
s.
The two are compared by calling
cmp()
on their inner values.
§
Examples
use
std::sync::Arc;
use
std::cmp::Ordering;
let
five = Arc::new(
5
);
assert_eq!
(Ordering::Less, five.cmp(
&
Arc::new(
6
)));
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
Arc
<T, A>
where
    T:
PartialEq
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
eq
(&self, other: &
Arc
<T, A>) ->
bool
Equality for two
Arc
s.
Two
Arc
s are equal if their inner values are equal, even if they are
stored in different allocation.
If
T
also implements
Eq
(implying reflexivity of equality),
two
Arc
s that point to the same allocation are always equal.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
assert!
(five == Arc::new(
5
));
Source
§
fn
ne
(&self, other: &
Arc
<T, A>) ->
bool
Inequality for two
Arc
s.
Two
Arc
s are not equal if their inner values are not equal.
If
T
also implements
Eq
(implying reflexivity of equality),
two
Arc
s that point to the same value are always equal.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
assert!
(five != Arc::new(
6
));
1.0.0
·
Source
§
impl<T, A>
PartialOrd
for
Arc
<T, A>
where
    T:
PartialOrd
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
partial_cmp
(&self, other: &
Arc
<T, A>) ->
Option
<
Ordering
>
Partial comparison for two
Arc
s.
The two are compared by calling
partial_cmp()
on their inner values.
§
Examples
use
std::sync::Arc;
use
std::cmp::Ordering;
let
five = Arc::new(
5
);
assert_eq!
(
Some
(Ordering::Less), five.partial_cmp(
&
Arc::new(
6
)));
Source
§
fn
lt
(&self, other: &
Arc
<T, A>) ->
bool
Less-than comparison for two
Arc
s.
The two are compared by calling
<
on their inner values.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
assert!
(five < Arc::new(
6
));
Source
§
fn
le
(&self, other: &
Arc
<T, A>) ->
bool
‘Less than or equal to’ comparison for two
Arc
s.
The two are compared by calling
<=
on their inner values.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
assert!
(five <= Arc::new(
5
));
Source
§
fn
gt
(&self, other: &
Arc
<T, A>) ->
bool
Greater-than comparison for two
Arc
s.
The two are compared by calling
>
on their inner values.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
assert!
(five > Arc::new(
4
));
Source
§
fn
ge
(&self, other: &
Arc
<T, A>) ->
bool
‘Greater than or equal to’ comparison for two
Arc
s.
The two are compared by calling
>=
on their inner values.
§
Examples
use
std::sync::Arc;
let
five = Arc::new(
5
);
assert!
(five >= Arc::new(
5
));
1.0.0
·
Source
§
impl<T, A>
Pointer
for
Arc
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
1.73.0
·
Source
§
impl
Read
for
Arc
<
File
>
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
Pull some bytes from this source into the specified buffer, returning
how many bytes were read.
Read more
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
1.6.0
·
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
1.73.0
·
Source
§
impl
Seek
for
Arc
<
File
>
Source
§
fn
seek
(&mut self, pos:
SeekFrom
) ->
Result
<
u64
>
Seek to an offset, in bytes, in a stream.
Read more
Source
§
fn
stream_position
(&mut self) ->
Result
<
u64
>
Returns the current seek position from the start of the stream.
Read more
1.55.0
·
Source
§
fn
rewind
(&mut self) ->
Result
<
()
>
Rewind to the beginning of a stream.
Read more
Source
§
fn
stream_len
(&mut self) ->
Result
<
u64
>
🔬
This is a nightly-only experimental API. (
seek_stream_len
#59359
)
Returns the length of this stream (in bytes).
Read more
1.80.0
·
Source
§
fn
seek_relative
(&mut self, offset:
i64
) ->
Result
<
()
>
Seeks relative to the current position.
Read more
1.43.0
·
Source
§
impl<T, A, const N:
usize
>
TryFrom
<
Arc
<
[T]
, A>> for
Arc
<
[T; N]
, A>
where
    A:
Allocator
,
Source
§
type
Error
=
Arc
<
[T]
, A>
The type returned in the event of a conversion error.
Source
§
fn
try_from
(
    boxed_slice:
Arc
<
[T]
, A>,
) ->
Result
<
Arc
<
[T; N]
, A>, <
Arc
<
[T; N]
, A> as
TryFrom
<
Arc
<
[T]
, A>>>::
Error
>
Performs the conversion.
1.73.0
·
Source
§
impl
Write
for
Arc
<
File
>
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
Source
§
impl<T, U, A>
CoerceUnsized
<
Arc
<U, A>> for
Arc
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
impl<T, A>
DerefPure
for
Arc
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
impl<T, U>
DispatchFromDyn
<
Arc
<U>> for
Arc
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
1.0.0
·
Source
§
impl<T, A>
Eq
for
Arc
<T, A>
where
    T:
Eq
+ ?
Sized
,
    A:
Allocator
,
Source
§
impl<T, A>
PinCoerceUnsized
for
Arc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.0.0
·
Source
§
impl<T, A>
Send
for
Arc
<T, A>
where
    T:
Sync
+
Send
+ ?
Sized
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
Arc
<T, A>
where
    T:
Sync
+
Send
+ ?
Sized
,
    A:
Allocator
+
Sync
,
1.33.0
·
Source
§
impl<T, A>
Unpin
for
Arc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.9.0
·
Source
§
impl<T, A>
UnwindSafe
for
Arc
<T, A>
where
    T:
RefUnwindSafe
+ ?
Sized
,
    A:
Allocator
+
UnwindSafe
,
Source
§
impl<T, A>
UseCloned
for
Arc
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
Arc
<T, A>
where
    A:
Freeze
,
    T: ?
Sized
,
§
impl<T, A>
RefUnwindSafe
for
Arc
<T, A>
where
    A:
RefUnwindSafe
,
    T:
RefUnwindSafe
+ ?
Sized
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
<
!
> for T
Source
§
fn
from
(t:
!
) -> T
Converts to this type from the input type.
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
impl<T>
ToString
for T
where
    T:
Display
+ ?
Sized
,
Source
§
fn
to_string
(&self) ->
String
Converts the given value to a
String
.
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