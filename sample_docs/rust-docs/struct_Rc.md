Rc in std::rc - Rust
std
::
rc
Struct
Rc
Copy item path
1.0.0
·
Source
pub struct Rc<T, A =
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
A single-threaded reference-counting pointer. ‘Rc’ stands for ‘Reference
Counted’.
See the
module-level documentation
for more details.
The inherent methods of
Rc
are all associated functions, which means
that you have to call them as e.g.,
Rc::get_mut(&mut value)
instead of
value.get_mut()
. This avoids conflicts with methods of the inner type
T
.
Implementations
§
Source
§
impl<T>
Rc
<T>
1.0.0
·
Source
pub fn
new
(value: T) ->
Rc
<T>
Constructs a new
Rc<T>
.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
1.60.0
·
Source
pub fn
new_cyclic
<F>(data_fn: F) ->
Rc
<T>
where
    F:
FnOnce
(&
Weak
<T>) -> T,
Constructs a new
Rc<T>
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
Rc<T>
is created, such that you can
clone and store it inside the
T
.
new_cyclic
first allocates the managed allocation for the
Rc<T>
,
then calls your closure, giving it a
Weak<T>
to this allocation,
and only afterwards completes the construction of the
Rc<T>
by placing
the
T
returned from your closure into the allocation.
Since the new
Rc<T>
is not fully-constructed until
Rc<T>::new_cyclic
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
Examples
use
std::rc::{Rc, Weak};
struct
Gadget {
    me: Weak<Gadget>,
}
impl
Gadget {
/// Constructs a reference counted Gadget.
fn
new() -> Rc<
Self
> {
// `me` is a `Weak<Gadget>` pointing at the new allocation of the
        // `Rc` we're constructing.
Rc::new_cyclic(|me| {
// Create the actual struct here.
Gadget { me: me.clone() }
        })
    }
/// Returns a reference counted pointer to Self.
fn
me(
&
self
) -> Rc<
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
Rc
<
MaybeUninit
<T>>
Constructs a new
Rc
with uninitialized contents.
§
Examples
#![feature(get_mut_unchecked)]
use
std::rc::Rc;
let
mut
five = Rc::<u32>::new_uninit();
// Deferred initialization:
Rc::get_mut(
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
Rc
<
MaybeUninit
<T>>
🔬
This is a nightly-only experimental API. (
new_zeroed_alloc
#129396
)
Constructs a new
Rc
with uninitialized contents, with the memory
being filled with
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
std::rc::Rc;
let
zero = Rc::<u32>::new_zeroed();
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
try_new
(value: T) ->
Result
<
Rc
<T>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Rc<T>
, returning an error if the allocation fails
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
let
five = Rc::try_new(
5
);
Source
pub fn
try_new_uninit
() ->
Result
<
Rc
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
Rc
with uninitialized contents, returning an error if the allocation fails
§
Examples
#![feature(allocator_api)]
#![feature(get_mut_unchecked)]
use
std::rc::Rc;
let
mut
five = Rc::<u32>::try_new_uninit()
?
;
// Deferred initialization:
Rc::get_mut(
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
Rc
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
Rc
with uninitialized contents, with the memory
being filled with
0
bytes, returning an error if the allocation fails
See
MaybeUninit::zeroed
for examples of correct and
incorrect usage of this method.
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
let
zero = Rc::<u32>::try_new_zeroed()
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
1.33.0
·
Source
pub fn
pin
(value: T) ->
Pin
<
Rc
<T>>
Constructs a new
Pin<Rc<T>>
. If
T
does not implement
Unpin
, then
value
will be pinned in memory and unable to be moved.
Source
§
impl<T, A>
Rc
<T, A>
where
    A:
Allocator
,
Source
pub fn
new_in
(value: T, alloc: A) ->
Rc
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Rc
in the provided allocator.
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
five = Rc::new_in(
5
, System);
Source
pub fn
new_uninit_in
(alloc: A) ->
Rc
<
MaybeUninit
<T>, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Rc
with uninitialized contents in the provided allocator.
§
Examples
#![feature(get_mut_unchecked)]
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
mut
five = Rc::<u32,
_
>::new_uninit_in(System);
let
five =
unsafe
{
// Deferred initialization:
Rc::get_mut_unchecked(
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
Rc
<
MaybeUninit
<T>, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Rc
with uninitialized contents, with the memory
being filled with
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
std::rc::Rc;
use
std::alloc::System;
let
zero = Rc::<u32,
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
Rc
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
Rc<T, A>
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
Rc<T, A>
is created, such that you can
clone and store it inside the
T
.
new_cyclic_in
first allocates the managed allocation for the
Rc<T, A>
,
then calls your closure, giving it a
Weak<T, A>
to this allocation,
and only afterwards completes the construction of the
Rc<T, A>
by placing
the
T
returned from your closure into the allocation.
Since the new
Rc<T, A>
is not fully-constructed until
Rc<T, A>::new_cyclic_in
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
Weak<T, A>
is dropped normally.
§
Examples
See
new_cyclic
.
Source
pub fn
try_new_in
(value: T, alloc: A) ->
Result
<
Rc
<T, A>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Rc<T>
in the provided allocator, returning an error if the allocation
fails
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
five = Rc::try_new_in(
5
, System);
Source
pub fn
try_new_uninit_in
(alloc: A) ->
Result
<
Rc
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
Rc
with uninitialized contents, in the provided allocator, returning an
error if the allocation fails
§
Examples
#![feature(allocator_api)]
#![feature(get_mut_unchecked)]
use
std::rc::Rc;
use
std::alloc::System;
let
mut
five = Rc::<u32,
_
>::try_new_uninit_in(System)
?
;
let
five =
unsafe
{
// Deferred initialization:
Rc::get_mut_unchecked(
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
Rc
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
Rc
with uninitialized contents, with the memory
being filled with
0
bytes, in the provided allocator, returning an error if the allocation
fails
See
MaybeUninit::zeroed
for examples of correct and
incorrect usage of this method.
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
zero = Rc::<u32,
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
Source
pub fn
pin_in
(value: T, alloc: A) ->
Pin
<
Rc
<T, A>>
where
    A: 'static,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Pin<Rc<T>>
in the provided allocator. If
T
does not implement
Unpin
, then
value
will be pinned in memory and unable to be moved.
1.4.0
·
Source
pub fn
try_unwrap
(this:
Rc
<T, A>) ->
Result
<T,
Rc
<T, A>>
Returns the inner value, if the
Rc
has exactly one strong reference.
Otherwise, an
Err
is returned with the same
Rc
that was
passed in.
This will succeed even if there are outstanding weak references.
§
Examples
use
std::rc::Rc;
let
x = Rc::new(
3
);
assert_eq!
(Rc::try_unwrap(x),
Ok
(
3
));
let
x = Rc::new(
4
);
let
_y = Rc::clone(
&
x);
assert_eq!
(
*
Rc::try_unwrap(x).unwrap_err(),
4
);
1.70.0
·
Source
pub fn
into_inner
(this:
Rc
<T, A>) ->
Option
<T>
Returns the inner value, if the
Rc
has exactly one strong reference.
Otherwise,
None
is returned and the
Rc
is dropped.
This will succeed even if there are outstanding weak references.
If
Rc::into_inner
is called on every clone of this
Rc
,
it is guaranteed that exactly one of the calls returns the inner value.
This means in particular that the inner value is not dropped.
Rc::try_unwrap
is conceptually similar to
Rc::into_inner
.
And while they are meant for different use-cases,
Rc::into_inner(this)
is in fact equivalent to
Rc::try_unwrap
(this).
ok
()
.
(Note that the same kind of equivalence does
not
hold true for
Arc
, due to race conditions that do not apply to
Rc
!)
§
Examples
use
std::rc::Rc;
let
x = Rc::new(
3
);
assert_eq!
(Rc::into_inner(x),
Some
(
3
));
let
x = Rc::new(
4
);
let
y = Rc::clone(
&
x);
assert_eq!
(Rc::into_inner(y),
None
);
assert_eq!
(Rc::into_inner(x),
Some
(
4
));
Source
§
impl<T>
Rc
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
Rc
<[
MaybeUninit
<T>]>
Constructs a new reference-counted slice with uninitialized contents.
§
Examples
#![feature(get_mut_unchecked)]
use
std::rc::Rc;
let
mut
values = Rc::<[u32]>::new_uninit_slice(
3
);
// Deferred initialization:
let
data = Rc::get_mut(
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
Rc
<[
MaybeUninit
<T>]>
🔬
This is a nightly-only experimental API. (
new_zeroed_alloc
#129396
)
Constructs a new reference-counted slice with uninitialized contents, with the memory being
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
std::rc::Rc;
let
values = Rc::<[u32]>::new_zeroed_slice(
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
Rc
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
Rc
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
Rc
<[
MaybeUninit
<T>], A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new reference-counted slice with uninitialized contents.
§
Examples
#![feature(get_mut_unchecked)]
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
mut
values = Rc::<[u32],
_
>::new_uninit_slice_in(
3
, System);
let
values =
unsafe
{
// Deferred initialization:
Rc::get_mut_unchecked(
&mut
values)[
0
].as_mut_ptr().write(
1
);
    Rc::get_mut_unchecked(
&mut
values)[
1
].as_mut_ptr().write(
2
);
    Rc::get_mut_unchecked(
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
Rc
<[
MaybeUninit
<T>], A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new reference-counted slice with uninitialized contents, with the memory being
filled with
0
bytes.
See
MaybeUninit::zeroed
for examples of correct and
incorrect usage of this method.
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
values = Rc::<[u32],
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
Rc
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
Rc
<T, A>
Converts to
Rc<T>
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
std::rc::Rc;
let
mut
five = Rc::<u32>::new_uninit();
// Deferred initialization:
Rc::get_mut(
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
Rc
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
Rc
<
[T]
, A>
Converts to
Rc<[T]>
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
std::rc::Rc;
let
mut
values = Rc::<[u32]>::new_uninit_slice(
3
);
// Deferred initialization:
let
data = Rc::get_mut(
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
Rc
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
Rc
<T>
Constructs an
Rc<T>
from a raw pointer.
The raw pointer must have been previously returned by a call to
Rc<U>::into_raw
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
Rc<U>
was constructed
through
Rc<T>
and then converted to
Rc<U>
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
The raw pointer must point to a block of memory allocated by the global allocator
The user of
from_raw
has to make sure a specific value of
T
is only
dropped once.
This function is unsafe because improper use may lead to memory unsafety,
even if the returned
Rc<T>
is never accessed.
§
Examples
use
std::rc::Rc;
let
x = Rc::new(
"hello"
.to_owned());
let
x_ptr = Rc::into_raw(x);
unsafe
{
// Convert back to an `Rc` to prevent leak.
let
x = Rc::from_raw(x_ptr);
assert_eq!
(
&*
x,
"hello"
);
// Further calls to `Rc::from_raw(x_ptr)` would be memory-unsafe.
}
// The memory was freed when `x` went out of scope above, so `x_ptr` is now dangling!
Convert a slice back into its original array:
use
std::rc::Rc;
let
x: Rc<[u32]> = Rc::new([
1
,
2
,
3
]);
let
x_ptr:
*const
[u32] = Rc::into_raw(x);
unsafe
{
let
x: Rc<[u32;
3
]> = Rc::from_raw(x_ptr.cast::<[u32;
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
1.53.0
·
Source
pub unsafe fn
increment_strong_count
(ptr:
*const T
)
Increments the strong reference count on the
Rc<T>
associated with the
provided pointer by one.
§
Safety
The pointer must have been obtained through
Rc::into_raw
and must satisfy the
same layout requirements specified in
Rc::from_raw_in
.
The associated
Rc
instance must be valid (i.e. the strong count must be at
least 1) for the duration of this method, and
ptr
must point to a block of memory
allocated by the global allocator.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
unsafe
{
let
ptr = Rc::into_raw(five);
    Rc::increment_strong_count(ptr);
let
five = Rc::from_raw(ptr);
assert_eq!
(
2
, Rc::strong_count(
&
five));
}
1.53.0
·
Source
pub unsafe fn
decrement_strong_count
(ptr:
*const T
)
Decrements the strong reference count on the
Rc<T>
associated with the
provided pointer by one.
§
Safety
The pointer must have been obtained through
Rc::into_raw
and must satisfy the
same layout requirements specified in
Rc::from_raw_in
.
The associated
Rc
instance must be valid (i.e. the strong count must be at
least 1) when invoking this method, and
ptr
must point to a block of memory
allocated by the global allocator. This method can be used to release the final
Rc
and
backing storage, but
should not
be called after the final
Rc
has been released.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
unsafe
{
let
ptr = Rc::into_raw(five);
    Rc::increment_strong_count(ptr);
let
five = Rc::from_raw(ptr);
assert_eq!
(
2
, Rc::strong_count(
&
five));
    Rc::decrement_strong_count(ptr);
assert_eq!
(
1
, Rc::strong_count(
&
five));
}
Source
§
impl<T, A>
Rc
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
Rc
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
Rc::allocator(&r)
instead of
r.allocator()
. This
is so that there is no conflict with a method on the inner type.
1.17.0
·
Source
pub fn
into_raw
(this:
Rc
<T, A>) ->
*const T
Consumes the
Rc
, returning the wrapped pointer.
To avoid a memory leak the pointer must be converted back to an
Rc
using
Rc::from_raw
.
§
Examples
use
std::rc::Rc;
let
x = Rc::new(
"hello"
.to_owned());
let
x_ptr = Rc::into_raw(x);
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
Rc
<T, A>) -> (
*const T
, A)
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Consumes the
Rc
, returning the wrapped pointer and allocator.
To avoid a memory leak the pointer must be converted back to an
Rc
using
Rc::from_raw_in
.
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
x = Rc::new_in(
"hello"
.to_owned(), System);
let
(ptr, alloc) = Rc::into_raw_with_allocator(x);
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
{ Rc::from_raw_in(ptr, alloc) };
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
Rc
<T, A>) ->
*const T
Provides a raw pointer to the data.
The counts are not affected in any way and the
Rc
is not consumed. The pointer is valid
for as long as there are strong counts in the
Rc
.
§
Examples
use
std::rc::Rc;
let
x = Rc::new(
0
);
let
y = Rc::clone(
&
x);
let
x_ptr = Rc::as_ptr(
&
x);
assert_eq!
(x_ptr, Rc::as_ptr(
&
y));
assert_eq!
(
unsafe
{
*
x_ptr },
0
);
Source
pub unsafe fn
from_raw_in
(ptr:
*const T
, alloc: A) ->
Rc
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs an
Rc<T, A>
from a raw pointer in the provided allocator.
The raw pointer must have been previously returned by a call to
Rc<U, A>::into_raw
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
Rc<U>
was constructed
through
Rc<T>
and then converted to
Rc<U>
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
Rc<T>
is never accessed.
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
x = Rc::new_in(
"hello"
.to_owned(), System);
let
x_ptr = Rc::into_raw(x);
unsafe
{
// Convert back to an `Rc` to prevent leak.
let
x = Rc::from_raw_in(x_ptr, System);
assert_eq!
(
&*
x,
"hello"
);
// Further calls to `Rc::from_raw(x_ptr)` would be memory-unsafe.
}
// The memory was freed when `x` went out of scope above, so `x_ptr` is now dangling!
Convert a slice back into its original array:
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
x: Rc<[u32],
_
> = Rc::new_in([
1
,
2
,
3
], System);
let
x_ptr:
*const
[u32] = Rc::into_raw(x);
unsafe
{
let
x: Rc<[u32;
3
],
_
> = Rc::from_raw_in(x_ptr.cast::<[u32;
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
Rc
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
std::rc::Rc;
let
five = Rc::new(
5
);
let
weak_five = Rc::downgrade(
&
five);
1.15.0
·
Source
pub fn
weak_count
(this: &
Rc
<T, A>) ->
usize
Gets the number of
Weak
pointers to this allocation.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
let
_weak_five = Rc::downgrade(
&
five);
assert_eq!
(
1
, Rc::weak_count(
&
five));
1.15.0
·
Source
pub fn
strong_count
(this: &
Rc
<T, A>) ->
usize
Gets the number of strong (
Rc
) pointers to this allocation.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
let
_also_five = Rc::clone(
&
five);
assert_eq!
(
2
, Rc::strong_count(
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
Rc<T>
associated with the
provided pointer by one.
§
Safety
The pointer must have been obtained through
Rc::into_raw
and must satisfy the
same layout requirements specified in
Rc::from_raw_in
.
The associated
Rc
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
std::rc::Rc;
use
std::alloc::System;
let
five = Rc::new_in(
5
, System);
unsafe
{
let
ptr = Rc::into_raw(five);
    Rc::increment_strong_count_in(ptr, System);
let
five = Rc::from_raw_in(ptr, System);
assert_eq!
(
2
, Rc::strong_count(
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
Rc<T>
associated with the
provided pointer by one.
§
Safety
The pointer must have been obtained through
Rc::into_raw
and must satisfy the
same layout requirements specified in
Rc::from_raw_in
.
The associated
Rc
instance must be valid (i.e. the strong count must be at
least 1) when invoking this method, and
ptr
must point to a block of memory
allocated by
alloc
. This method can be used to release the final
Rc
and
backing storage, but
should not
be called after the final
Rc
has been released.
§
Examples
#![feature(allocator_api)]
use
std::rc::Rc;
use
std::alloc::System;
let
five = Rc::new_in(
5
, System);
unsafe
{
let
ptr = Rc::into_raw(five);
    Rc::increment_strong_count_in(ptr, System);
let
five = Rc::from_raw_in(ptr, System);
assert_eq!
(
2
, Rc::strong_count(
&
five));
    Rc::decrement_strong_count_in(ptr, System);
assert_eq!
(
1
, Rc::strong_count(
&
five));
}
1.4.0
·
Source
pub fn
get_mut
(this: &mut
Rc
<T, A>) ->
Option
<
&mut T
>
Returns a mutable reference into the given
Rc
, if there are
no other
Rc
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
Rc
pointers.
§
Examples
use
std::rc::Rc;
let
mut
x = Rc::new(
3
);
*
Rc::get_mut(
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
_y = Rc::clone(
&
x);
assert!
(Rc::get_mut(
&mut
x).is_none());
Source
pub unsafe fn
get_mut_unchecked
(this: &mut
Rc
<T, A>) ->
&mut T
🔬
This is a nightly-only experimental API. (
get_mut_unchecked
#63292
)
Returns a mutable reference into the given
Rc
,
without any check.
See also
get_mut
, which is safe and does appropriate checks.
§
Safety
If any other
Rc
or
Weak
pointers to the same allocation exist, then
they must not be dereferenced or have active borrows for the duration
of the returned borrow, and their inner type must be exactly the same as the
inner type of this Rc (including lifetimes). This is trivially the case if no
such pointers exist, for example immediately after
Rc::new
.
§
Examples
#![feature(get_mut_unchecked)]
use
std::rc::Rc;
let
mut
x = Rc::new(String::new());
unsafe
{
    Rc::get_mut_unchecked(
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
Rc
pointers to the same allocation must be to the same type.
#![feature(get_mut_unchecked)]
use
std::rc::Rc;
let
x: Rc<str> = Rc::from(
"Hello, world!"
);
let
mut
y: Rc<[u8]> = x.clone().into();
unsafe
{
// this is Undefined Behavior, because x's inner type is str, not [u8]
Rc::get_mut_unchecked(
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
Rc
pointers to the same allocation must be to the exact same type, including lifetimes.
#![feature(get_mut_unchecked)]
use
std::rc::Rc;
let
x: Rc<
&
str> = Rc::new(
"Hello, world!"
);
{
let
s = String::from(
"Oh, no!"
);
let
mut
y: Rc<
&
str> = x.clone();
unsafe
{
// this is Undefined Behavior, because x's inner type
        // is &'long str, not &'short str
*
Rc::get_mut_unchecked(
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
1.17.0
·
Source
pub fn
ptr_eq
(this: &
Rc
<T, A>, other: &
Rc
<T, A>) ->
bool
Returns
true
if the two
Rc
s point to the same allocation in a vein similar to
ptr::eq
. This function ignores the metadata of
dyn Trait
pointers.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
let
same_five = Rc::clone(
&
five);
let
other_five = Rc::new(
5
);
assert!
(Rc::ptr_eq(
&
five,
&
same_five));
assert!
(!Rc::ptr_eq(
&
five,
&
other_five));
Source
§
impl<T, A>
Rc
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
Rc
<T, A>) ->
&mut T
Makes a mutable reference into the given
Rc
.
If there are other
Rc
pointers to the same allocation, then
make_mut
will
clone
the inner value to a new allocation to ensure unique ownership.  This is also
referred to as clone-on-write.
However, if there are no other
Rc
pointers to this allocation, but some
Weak
pointers, then the
Weak
pointers will be disassociated and the inner value will not
be cloned.
See also
get_mut
, which will fail rather than cloning the inner value
or disassociating
Weak
pointers.
§
Examples
use
std::rc::Rc;
let
mut
data = Rc::new(
5
);
*
Rc::make_mut(
&mut
data) +=
1
;
// Won't clone anything
let
mut
other_data = Rc::clone(
&
data);
// Won't clone inner data
*
Rc::make_mut(
&mut
data) +=
1
;
// Clones inner data
*
Rc::make_mut(
&mut
data) +=
1
;
// Won't clone anything
*
Rc::make_mut(
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
pointers will be disassociated:
use
std::rc::Rc;
let
mut
data = Rc::new(
75
);
let
weak = Rc::downgrade(
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
Rc::make_mut(
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
Rc
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
Rc
<T, A>) -> T
If we have the only reference to
T
then unwrap it. Otherwise, clone
T
and return the
clone.
Assuming
rc_t
is of type
Rc<T>
, this function is functionally equivalent to
(*rc_t).clone()
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
rc = Rc::new(inner);
let
inner = Rc::unwrap_or_clone(rc);
// The inner value was not cloned
assert!
(ptr::eq(ptr, inner.as_ptr()));
let
rc = Rc::new(inner);
let
rc2 = rc.clone();
let
inner = Rc::unwrap_or_clone(rc);
// Because there were 2 references, we had to clone the inner value.
assert!
(!ptr::eq(ptr, inner.as_ptr()));
// `rc2` is the last reference, so when we unwrap it we get back
// the original `String`.
let
inner = Rc::unwrap_or_clone(rc2);
assert!
(ptr::eq(ptr, inner.as_ptr()));
Source
§
impl<A>
Rc
<dyn
Any
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
Rc
<T, A>,
Rc
<dyn
Any
, A>>
where
    T:
Any
,
Attempts to downcast the
Rc<dyn Any>
to a concrete type.
§
Examples
use
std::any::Any;
use
std::rc::Rc;
fn
print_if_string(value: Rc<
dyn
Any>) {
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
print_if_string(Rc::new(my_string));
print_if_string(Rc::new(
0i8
));
Source
pub unsafe fn
downcast_unchecked
<T>(self) ->
Rc
<T, A>
where
    T:
Any
,
🔬
This is a nightly-only experimental API. (
downcast_unchecked
#90850
)
Downcasts the
Rc<dyn Any>
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
std::rc::Rc;
let
x: Rc<
dyn
Any> = Rc::new(
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
1.69.0
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
Rc
<T>
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
Rc
<T>
Available on
Windows
only.
Source
§
fn
as_handle
(&self) ->
BorrowedHandle
<'_>
Borrows the handle.
Read more
1.69.0
·
Source
§
impl<T:
AsRawFd
>
AsRawFd
for
Rc
<T>
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
Rc
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
Rc
<T>
Available on
Windows
only.
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
Rc
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
Rc
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
Rc
<T, A>
Makes a clone of the
Rc
pointer.
This creates another pointer to the same allocation, increasing the
strong reference count.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
let _
= Rc::clone(
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
Rc
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
Rc
<
[T]
>
Source
§
fn
default
() ->
Rc
<
[T]
>
Creates an empty
[T]
inside an Rc
This may or may not share an allocation with other Rcs on the same thread.
1.80.0
·
Source
§
impl
Default
for
Rc
<
CStr
>
Source
§
fn
default
() ->
Rc
<
CStr
>
Creates an empty CStr inside an Rc
This may or may not share an allocation with other Rcs on the same thread.
1.0.0
·
Source
§
impl<T>
Default
for
Rc
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
Rc
<T>
Creates a new
Rc<T>
, with the
Default
value for
T
.
§
Examples
use
std::rc::Rc;
let
x: Rc<i32> = Default::default();
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
Rc
<
str
>
Source
§
fn
default
() ->
Rc
<
str
>
Creates an empty str inside an Rc
This may or may not share an allocation with other Rcs on the same thread.
1.0.0
·
Source
§
impl<T, A>
Deref
for
Rc
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
Rc
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
Rc
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
Rc
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
std::rc::Rc;
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
foo  = Rc::new(Foo);
let
foo2 = Rc::clone(
&
foo);

drop(foo);
// Doesn't print anything
drop(foo2);
// Prints "dropped!"
1.21.0
·
Source
§
impl<T>
From
<&
[T]
> for
Rc
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
Rc
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
shared: Rc<[i32]> = Rc::from(original);
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
Rc
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
Rc
<
CStr
>
Converts a
&CStr
into a
Rc<CStr>
,
by copying the contents into a newly allocated
Rc
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
Rc
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
Rc
<
OsStr
>
Copies the string into a newly allocated
Rc
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
Rc
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
Rc
<
Path
>
Converts a
Path
into an
Rc
by copying the
Path
data into a new
Rc
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
Rc
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
Rc
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
shared: Rc<[i32]> = Rc::from(original);
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
Rc
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
Rc
<
CStr
>
Converts a
&mut CStr
into a
Rc<CStr>
,
by copying the contents into a newly allocated
Rc
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
Rc
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
Rc
<
OsStr
>
Copies the string into a newly allocated
Rc
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
Rc
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
Rc
<
Path
>
Converts a
Path
into an
Rc
by copying the
Path
data into a new
Rc
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
Rc
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
Rc
<
str
>
Allocates a reference-counted string slice and copies
v
into it.
§
Example
let
mut
original = String::from(
"statue"
);
let
original:
&mut
str =
&mut
original;
let
shared: Rc<str> = Rc::from(original);
assert_eq!
(
"statue"
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
Rc
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
Rc
<
str
>
Allocates a reference-counted string slice and copies
v
into it.
§
Example
let
shared: Rc<str> = Rc::from(
"statue"
);
assert_eq!
(
"statue"
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
Rc
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
Rc
<
[T]
>
Converts a
[T; N]
into an
Rc<[T]>
.
The conversion moves the array into a newly allocated
Rc
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
shared: Rc<[i32]> = Rc::from(original);
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
1.21.0
·
Source
§
impl<T, A>
From
<
Box
<T, A>> for
Rc
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
Rc
<T, A>
Move a boxed object to a new, reference counted, allocation.
§
Example
let
original: Box<i32> = Box::new(
1
);
let
shared: Rc<i32> = Rc::from(original);
assert_eq!
(
1
,
*
shared);
1.24.0
·
Source
§
impl
From
<
CString
> for
Rc
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
Rc
<
CStr
>
Converts a
CString
into an
Rc
<
CStr
>
by moving the
CString
data into a new
Rc
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
Rc
<B>
where
    B:
ToOwned
+ ?
Sized
,
Rc
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
Rc
<B>
Creates a reference-counted pointer from a clone-on-write pointer by
copying its content.
§
Example
let
cow: Cow<
'_
, str> = Cow::Borrowed(
"eggplant"
);
let
shared: Rc<str> = Rc::from(cow);
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
Rc
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
Rc
<
OsStr
>
Converts an
OsString
into an
Rc
<
OsStr
>
by moving the
OsString
data into a new
Rc
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
Rc
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
Rc
<
Path
>
Converts a
PathBuf
into an
Rc
<
Path
>
by moving the
PathBuf
data into
a new
Rc
buffer.
Source
§
impl
From
<
Rc
<[
u8
]>> for
Rc
<
ByteStr
>
Source
§
fn
from
(s:
Rc
<[
u8
]>) ->
Rc
<
ByteStr
>
Converts to this type from the input type.
Source
§
impl
From
<
Rc
<
ByteStr
>> for
Rc
<[
u8
]>
Source
§
fn
from
(s:
Rc
<
ByteStr
>) ->
Rc
<[
u8
]>
Converts to this type from the input type.
Source
§
impl<W>
From
<
Rc
<W>> for
LocalWaker
where
    W:
LocalWake
+ 'static,
Source
§
fn
from
(waker:
Rc
<W>) ->
LocalWaker
Use a
Wake
-able type as a
LocalWaker
.
No heap allocations or atomic operations are used for this conversion.
Source
§
impl<W>
From
<
Rc
<W>> for
RawWaker
where
    W:
LocalWake
+ 'static,
Source
§
fn
from
(waker:
Rc
<W>) ->
RawWaker
Use a
Wake
-able type as a
RawWaker
.
No heap allocations or atomic operations are used for this conversion.
1.62.0
·
Source
§
impl
From
<
Rc
<
str
>> for
Rc
<[
u8
]>
Source
§
fn
from
(rc:
Rc
<
str
>) ->
Rc
<[
u8
]>
Converts a reference-counted string slice into a byte slice.
§
Example
let
string: Rc<str> = Rc::from(
"eggplant"
);
let
bytes: Rc<[u8]> = Rc::from(string);
assert_eq!
(
"eggplant"
.as_bytes(), bytes.as_ref());
1.21.0
·
Source
§
impl
From
<
String
> for
Rc
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
Rc
<
str
>
Allocates a reference-counted string slice and copies
v
into it.
§
Example
let
original: String =
"statue"
.to_owned();
let
shared: Rc<str> = Rc::from(original);
assert_eq!
(
"statue"
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
Rc
<T>
Source
§
fn
from
(t: T) ->
Rc
<T>
Converts a generic type
T
into an
Rc<T>
The conversion allocates on the heap and moves
t
from the stack into it.
§
Example
let
x =
5
;
let
rc = Rc::new(
5
);
assert_eq!
(Rc::from(x), rc);
1.21.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
Rc
<
[T]
, A>
where
    A:
Allocator
,
Source
§
fn
from
(v:
Vec
<T, A>) ->
Rc
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
shared: Rc<[i32]> = Rc::from(unique);
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
Rc
<
[T]
>
Source
§
fn
from_iter
<I>(iter: I) ->
Rc
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
Rc<[T]>
.
§
Performance characteristics
§
The general case
In the general case, collecting into
Rc<[T]>
is done by first
collecting into a
Vec<T>
. That is, when writing the following:
let
evens: Rc<[u8]> = (
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
evens: Rc<[u8]> = (
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
// A second allocation for `Rc<[T]>` happens here.
This will allocate as many times as needed for constructing the
Vec<T>
and then it will allocate once for turning the
Vec<T>
into the
Rc<[T]>
.
§
Iterators of known length
When your
Iterator
implements
TrustedLen
and is of an exact size,
a single allocation will be made for the
Rc<[T]>
. For example:
let
evens: Rc<[u8]> = (
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
Rc
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
Rc
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
Rc
<T, A>) ->
Ordering
Comparison for two
Rc
s.
The two are compared by calling
cmp()
on their inner values.
§
Examples
use
std::rc::Rc;
use
std::cmp::Ordering;
let
five = Rc::new(
5
);
assert_eq!
(Ordering::Less, five.cmp(
&
Rc::new(
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
Rc
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
Rc
<T, A>) ->
bool
Equality for two
Rc
s.
Two
Rc
s are equal if their inner values are equal, even if they are
stored in different allocation.
If
T
also implements
Eq
(implying reflexivity of equality),
two
Rc
s that point to the same allocation are
always equal.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
assert!
(five == Rc::new(
5
));
Source
§
fn
ne
(&self, other: &
Rc
<T, A>) ->
bool
Inequality for two
Rc
s.
Two
Rc
s are not equal if their inner values are not equal.
If
T
also implements
Eq
(implying reflexivity of equality),
two
Rc
s that point to the same allocation are
always equal.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
assert!
(five != Rc::new(
6
));
1.0.0
·
Source
§
impl<T, A>
PartialOrd
for
Rc
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
Rc
<T, A>) ->
Option
<
Ordering
>
Partial comparison for two
Rc
s.
The two are compared by calling
partial_cmp()
on their inner values.
§
Examples
use
std::rc::Rc;
use
std::cmp::Ordering;
let
five = Rc::new(
5
);
assert_eq!
(
Some
(Ordering::Less), five.partial_cmp(
&
Rc::new(
6
)));
Source
§
fn
lt
(&self, other: &
Rc
<T, A>) ->
bool
Less-than comparison for two
Rc
s.
The two are compared by calling
<
on their inner values.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
assert!
(five < Rc::new(
6
));
Source
§
fn
le
(&self, other: &
Rc
<T, A>) ->
bool
‘Less than or equal to’ comparison for two
Rc
s.
The two are compared by calling
<=
on their inner values.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
assert!
(five <= Rc::new(
5
));
Source
§
fn
gt
(&self, other: &
Rc
<T, A>) ->
bool
Greater-than comparison for two
Rc
s.
The two are compared by calling
>
on their inner values.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
assert!
(five > Rc::new(
4
));
Source
§
fn
ge
(&self, other: &
Rc
<T, A>) ->
bool
‘Greater than or equal to’ comparison for two
Rc
s.
The two are compared by calling
>=
on their inner values.
§
Examples
use
std::rc::Rc;
let
five = Rc::new(
5
);
assert!
(five >= Rc::new(
5
));
1.0.0
·
Source
§
impl<T, A>
Pointer
for
Rc
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
1.43.0
·
Source
§
impl<T, A, const N:
usize
>
TryFrom
<
Rc
<
[T]
, A>> for
Rc
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
Rc
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
Rc
<
[T]
, A>,
) ->
Result
<
Rc
<
[T; N]
, A>, <
Rc
<
[T; N]
, A> as
TryFrom
<
Rc
<
[T]
, A>>>::
Error
>
Performs the conversion.
Source
§
impl<T, U, A>
CoerceUnsized
<
Rc
<U, A>> for
Rc
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
Rc
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
Rc
<U>> for
Rc
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
Rc
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
Rc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.58.0
·
Source
§
impl<T, A>
RefUnwindSafe
for
Rc
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
1.0.0
·
Source
§
impl<T, A> !
Send
for
Rc
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
impl<T, A> !
Sync
for
Rc
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
1.33.0
·
Source
§
impl<T, A>
Unpin
for
Rc
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
Rc
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
Rc
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
Rc
<T, A>
where
    A:
Freeze
,
    T: ?
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