Box in std::boxed - Rust
std
::
boxed
Struct
Box
Copy item path
1.0.0
·
Source
pub struct Box<T, A =
Global
>(
/* private fields */
)
where
    A:
Allocator
,
    T: ?
Sized
;
Expand description
A pointer type that uniquely owns a heap allocation of type
T
.
See the
module-level documentation
for more.
Implementations
§
Source
§
impl<A>
Box
<dyn
Any
, A>
where
    A:
Allocator
,
1.0.0
·
Source
pub fn
downcast
<T>(self) ->
Result
<
Box
<T, A>,
Box
<dyn
Any
, A>>
where
    T:
Any
,
Attempts to downcast the box to a concrete type.
§
Examples
use
std::any::Any;
fn
print_if_string(value: Box<
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
print_if_string(Box::new(my_string));
print_if_string(Box::new(
0i8
));
Source
pub unsafe fn
downcast_unchecked
<T>(self) ->
Box
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
Downcasts the box to a concrete type.
For a safe alternative see
downcast
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any> = Box::new(
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
Source
§
impl<A>
Box
<dyn
Any
+
Send
, A>
where
    A:
Allocator
,
1.0.0
·
Source
pub fn
downcast
<T>(self) ->
Result
<
Box
<T, A>,
Box
<dyn
Any
+
Send
, A>>
where
    T:
Any
,
Attempts to downcast the box to a concrete type.
§
Examples
use
std::any::Any;
fn
print_if_string(value: Box<
dyn
Any + Send>) {
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
print_if_string(Box::new(my_string));
print_if_string(Box::new(
0i8
));
Source
pub unsafe fn
downcast_unchecked
<T>(self) ->
Box
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
Downcasts the box to a concrete type.
For a safe alternative see
downcast
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any + Send> = Box::new(
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
Source
§
impl<A>
Box
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
1.51.0
·
Source
pub fn
downcast
<T>(self) ->
Result
<
Box
<T, A>,
Box
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
,
Attempts to downcast the box to a concrete type.
§
Examples
use
std::any::Any;
fn
print_if_string(value: Box<
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
print_if_string(Box::new(my_string));
print_if_string(Box::new(
0i8
));
Source
pub unsafe fn
downcast_unchecked
<T>(self) ->
Box
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
Downcasts the box to a concrete type.
For a safe alternative see
downcast
.
§
Examples
#![feature(downcast_unchecked)]
use
std::any::Any;
let
x: Box<
dyn
Any + Send + Sync> = Box::new(
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
Source
§
impl<T>
Box
<T>
1.0.0
·
Source
pub fn
new
(x: T) ->
Box
<T>
Allocates memory on the heap and then places
x
into it.
This doesn’t actually allocate if
T
is zero-sized.
§
Examples
let
five = Box::new(
5
);
1.82.0
·
Source
pub fn
new_uninit
() ->
Box
<
MaybeUninit
<T>>
Constructs a new box with uninitialized contents.
§
Examples
let
mut
five = Box::<u32>::new_uninit();
// Deferred initialization:
five.write(
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
Box
<
MaybeUninit
<T>>
🔬
This is a nightly-only experimental API. (
new_zeroed_alloc
#129396
)
Constructs a new
Box
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
let
zero = Box::<u32>::new_zeroed();
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
(x: T) ->
Pin
<
Box
<T>>
Constructs a new
Pin<Box<T>>
. If
T
does not implement
Unpin
, then
x
will be pinned in memory and unable to be moved.
Constructing and pinning of the
Box
can also be done in two steps:
Box::pin(x)
does the same as
Box::into_pin
(
Box::new
(x))
. Consider using
into_pin
if you already have a
Box<T>
, or if you want to
construct a (pinned)
Box
in a different way than with
Box::new
.
Source
pub fn
try_new
(x: T) ->
Result
<
Box
<T>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Allocates memory on the heap then places
x
into it,
returning an error if the allocation fails
This doesn’t actually allocate if
T
is zero-sized.
§
Examples
#![feature(allocator_api)]
let
five = Box::try_new(
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
Box
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
Constructs a new box with uninitialized contents on the heap,
returning an error if the allocation fails
§
Examples
#![feature(allocator_api)]
let
mut
five = Box::<u32>::try_new_uninit()
?
;
// Deferred initialization:
five.write(
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
Box
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
Box
with uninitialized contents, with the memory
being filled with
0
bytes on the heap
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(allocator_api)]
let
zero = Box::<u32>::try_new_zeroed()
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
Box
<T, A>
where
    A:
Allocator
,
Source
pub fn
new_in
(x: T, alloc: A) ->
Box
<T, A>
where
    A:
Allocator
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Allocates memory in the given allocator then places
x
into it.
This doesn’t actually allocate if
T
is zero-sized.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
five = Box::new_in(
5
, System);
Source
pub fn
try_new_in
(x: T, alloc: A) ->
Result
<
Box
<T, A>,
AllocError
>
where
    A:
Allocator
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Allocates memory in the given allocator then places
x
into it,
returning an error if the allocation fails
This doesn’t actually allocate if
T
is zero-sized.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
five = Box::try_new_in(
5
, System)
?
;
Source
pub fn
new_uninit_in
(alloc: A) ->
Box
<
MaybeUninit
<T>, A>
where
    A:
Allocator
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new box with uninitialized contents in the provided allocator.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
mut
five = Box::<u32,
_
>::new_uninit_in(System);
// Deferred initialization:
five.write(
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
try_new_uninit_in
(alloc: A) ->
Result
<
Box
<
MaybeUninit
<T>, A>,
AllocError
>
where
    A:
Allocator
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new box with uninitialized contents in the provided allocator,
returning an error if the allocation fails
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
mut
five = Box::<u32,
_
>::try_new_uninit_in(System)
?
;
// Deferred initialization:
five.write(
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
new_zeroed_in
(alloc: A) ->
Box
<
MaybeUninit
<T>, A>
where
    A:
Allocator
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Box
with uninitialized contents, with the memory
being filled with
0
bytes in the provided allocator.
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
zero = Box::<u32,
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
try_new_zeroed_in
(alloc: A) ->
Result
<
Box
<
MaybeUninit
<T>, A>,
AllocError
>
where
    A:
Allocator
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Box
with uninitialized contents, with the memory
being filled with
0
bytes in the provided allocator,
returning an error if the allocation fails,
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
zero = Box::<u32,
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
(x: T, alloc: A) ->
Pin
<
Box
<T, A>>
where
    A: 'static +
Allocator
,
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new
Pin<Box<T, A>>
. If
T
does not implement
Unpin
, then
x
will be pinned in memory and unable to be moved.
Constructing and pinning of the
Box
can also be done in two steps:
Box::pin_in(x, alloc)
does the same as
Box::into_pin
(
Box::new_in
(x, alloc))
. Consider using
into_pin
if you already have a
Box<T, A>
, or if you want to
construct a (pinned)
Box
in a different way than with
Box::new_in
.
Source
pub fn
into_boxed_slice
(boxed:
Box
<T, A>) ->
Box
<
[T]
, A>
🔬
This is a nightly-only experimental API. (
box_into_boxed_slice
#71582
)
Converts a
Box<T>
into a
Box<[T]>
This conversion does not allocate on the heap and happens in place.
Source
pub fn
into_inner
(boxed:
Box
<T, A>) -> T
🔬
This is a nightly-only experimental API. (
box_into_inner
#80437
)
Consumes the
Box
, returning the wrapped value.
§
Examples
#![feature(box_into_inner)]
let
c = Box::new(
5
);
assert_eq!
(Box::into_inner(c),
5
);
Source
§
impl<T>
Box
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
Box
<[
MaybeUninit
<T>]>
Constructs a new boxed slice with uninitialized contents.
§
Examples
let
mut
values = Box::<[u32]>::new_uninit_slice(
3
);
// Deferred initialization:
values[
0
].write(
1
);
values[
1
].write(
2
);
values[
2
].write(
3
);
let
values =
unsafe
{values.assume_init() };
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
Box
<[
MaybeUninit
<T>]>
🔬
This is a nightly-only experimental API. (
new_zeroed_alloc
#129396
)
Constructs a new boxed slice with uninitialized contents, with the memory
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
let
values = Box::<[u32]>::new_zeroed_slice(
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
try_new_uninit_slice
(
    len:
usize
,
) ->
Result
<
Box
<[
MaybeUninit
<T>]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new boxed slice with uninitialized contents. Returns an error if
the allocation fails.
§
Examples
#![feature(allocator_api)]
let
mut
values = Box::<[u32]>::try_new_uninit_slice(
3
)
?
;
// Deferred initialization:
values[
0
].write(
1
);
values[
1
].write(
2
);
values[
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
]);
Source
pub fn
try_new_zeroed_slice
(
    len:
usize
,
) ->
Result
<
Box
<[
MaybeUninit
<T>]>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new boxed slice with uninitialized contents, with the memory
being filled with
0
bytes. Returns an error if the allocation fails.
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(allocator_api)]
let
values = Box::<[u32]>::try_new_zeroed_slice(
3
)
?
;
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
]);
Source
pub fn
into_array
<const N:
usize
>(self) ->
Option
<
Box
<
[T; N]
>>
🔬
This is a nightly-only experimental API. (
slice_as_array
#133508
)
Converts the boxed slice into a boxed array.
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
Box
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
Box
<[
MaybeUninit
<T>], A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new boxed slice with uninitialized contents in the provided allocator.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
mut
values = Box::<[u32],
_
>::new_uninit_slice_in(
3
, System);
// Deferred initialization:
values[
0
].write(
1
);
values[
1
].write(
2
);
values[
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
new_zeroed_slice_in
(len:
usize
, alloc: A) ->
Box
<[
MaybeUninit
<T>], A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new boxed slice with uninitialized contents in the provided allocator,
with the memory being filled with
0
bytes.
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
values = Box::<[u32],
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
pub fn
try_new_uninit_slice_in
(
    len:
usize
,
    alloc: A,
) ->
Result
<
Box
<[
MaybeUninit
<T>], A>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new boxed slice with uninitialized contents in the provided allocator. Returns an error if
the allocation fails.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
mut
values = Box::<[u32],
_
>::try_new_uninit_slice_in(
3
, System)
?
;
// Deferred initialization:
values[
0
].write(
1
);
values[
1
].write(
2
);
values[
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
]);
Source
pub fn
try_new_zeroed_slice_in
(
    len:
usize
,
    alloc: A,
) ->
Result
<
Box
<[
MaybeUninit
<T>], A>,
AllocError
>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a new boxed slice with uninitialized contents in the provided allocator, with the memory
being filled with
0
bytes. Returns an error if the allocation fails.
See
MaybeUninit::zeroed
for examples of correct and incorrect usage
of this method.
§
Examples
#![feature(allocator_api)]
use
std::alloc::System;
let
values = Box::<[u32],
_
>::try_new_zeroed_slice_in(
3
, System)
?
;
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
]);
Source
§
impl<T, A>
Box
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
Box
<T, A>
Converts to
Box<T, A>
.
§
Safety
As with
MaybeUninit::assume_init
,
it is up to the caller to guarantee that the value
really is in an initialized state.
Calling this when the content is not yet fully initialized
causes immediate undefined behavior.
§
Examples
let
mut
five = Box::<u32>::new_uninit();
// Deferred initialization:
five.write(
5
);
let
five: Box<u32> =
unsafe
{ five.assume_init() };
assert_eq!
(
*
five,
5
)
1.87.0
·
Source
pub fn
write
(boxed:
Box
<
MaybeUninit
<T>, A>, value: T) ->
Box
<T, A>
Writes the value and converts to
Box<T, A>
.
This method converts the box similarly to
Box::assume_init
but
writes
value
into it before conversion thus guaranteeing safety.
In some scenarios use of this method may improve performance because
the compiler may be able to optimize copying from stack.
§
Examples
let
big_box = Box::<[usize;
1024
]>::new_uninit();
let
mut
array = [
0
;
1024
];
for
(i, place)
in
array.iter_mut().enumerate() {
*
place = i;
}
// The optimizer may be able to elide this copy, so previous code writes
// to heap directly.
let
big_box = Box::write(big_box, array);
for
(i, x)
in
big_box.iter().enumerate() {
assert_eq!
(
*
x, i);
}
Source
§
impl<T, A>
Box
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
Box
<
[T]
, A>
Converts to
Box<[T], A>
.
§
Safety
As with
MaybeUninit::assume_init
,
it is up to the caller to guarantee that the values
really are in an initialized state.
Calling this when the content is not yet fully initialized
causes immediate undefined behavior.
§
Examples
let
mut
values = Box::<[u32]>::new_uninit_slice(
3
);
// Deferred initialization:
values[
0
].write(
1
);
values[
1
].write(
2
);
values[
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
Box
<T>
where
    T: ?
Sized
,
1.4.0
·
Source
pub unsafe fn
from_raw
(raw:
*mut T
) ->
Box
<T>
Constructs a box from a raw pointer.
After calling this function, the raw pointer is owned by the
resulting
Box
. Specifically, the
Box
destructor will call
the destructor of
T
and free the allocated memory. For this
to be safe, the memory must have been allocated in accordance
with the
memory layout
used by
Box
.
§
Safety
This function is unsafe because improper use may lead to
memory problems. For example, a double-free may occur if the
function is called twice on the same raw pointer.
The raw pointer must point to a block of memory allocated by the global allocator.
The safety conditions are described in the
memory layout
section.
§
Examples
Recreate a
Box
which was previously converted to a raw pointer
using
Box::into_raw
:
let
x = Box::new(
5
);
let
ptr = Box::into_raw(x);
let
x =
unsafe
{ Box::from_raw(ptr) };
Manually create a
Box
from scratch by using the global allocator:
use
std::alloc::{alloc, Layout};
unsafe
{
let
ptr = alloc(Layout::new::<i32>())
as
*mut
i32;
// In general .write is required to avoid attempting to destruct
    // the (uninitialized) previous contents of `ptr`, though for this
    // simple example `*ptr = 5` would have worked as well.
ptr.write(
5
);
let
x = Box::from_raw(ptr);
}
Source
pub unsafe fn
from_non_null
(ptr:
NonNull
<T>) ->
Box
<T>
🔬
This is a nightly-only experimental API. (
box_vec_non_null
#130364
)
Constructs a box from a
NonNull
pointer.
After calling this function, the
NonNull
pointer is owned by
the resulting
Box
. Specifically, the
Box
destructor will call
the destructor of
T
and free the allocated memory. For this
to be safe, the memory must have been allocated in accordance
with the
memory layout
used by
Box
.
§
Safety
This function is unsafe because improper use may lead to
memory problems. For example, a double-free may occur if the
function is called twice on the same
NonNull
pointer.
The non-null pointer must point to a block of memory allocated by the global allocator.
The safety conditions are described in the
memory layout
section.
§
Examples
Recreate a
Box
which was previously converted to a
NonNull
pointer using
Box::into_non_null
:
#![feature(box_vec_non_null)]
let
x = Box::new(
5
);
let
non_null = Box::into_non_null(x);
let
x =
unsafe
{ Box::from_non_null(non_null) };
Manually create a
Box
from scratch by using the global allocator:
#![feature(box_vec_non_null)]
use
std::alloc::{alloc, Layout};
use
std::ptr::NonNull;
unsafe
{
let
non_null = NonNull::new(alloc(Layout::new::<i32>()).cast::<i32>())
        .expect(
"allocation failed"
);
// In general .write is required to avoid attempting to destruct
    // the (uninitialized) previous contents of `non_null`.
non_null.write(
5
);
let
x = Box::from_non_null(non_null);
}
Source
§
impl<T, A>
Box
<T, A>
where
    A:
Allocator
,
    T: ?
Sized
,
Source
pub unsafe fn
from_raw_in
(raw:
*mut T
, alloc: A) ->
Box
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a box from a raw pointer in the given allocator.
After calling this function, the raw pointer is owned by the
resulting
Box
. Specifically, the
Box
destructor will call
the destructor of
T
and free the allocated memory. For this
to be safe, the memory must have been allocated in accordance
with the
memory layout
used by
Box
.
§
Safety
This function is unsafe because improper use may lead to
memory problems. For example, a double-free may occur if the
function is called twice on the same raw pointer.
The raw pointer must point to a block of memory allocated by
alloc
.
§
Examples
Recreate a
Box
which was previously converted to a raw pointer
using
Box::into_raw_with_allocator
:
#![feature(allocator_api)]
use
std::alloc::System;
let
x = Box::new_in(
5
, System);
let
(ptr, alloc) = Box::into_raw_with_allocator(x);
let
x =
unsafe
{ Box::from_raw_in(ptr, alloc) };
Manually create a
Box
from scratch by using the system allocator:
#![feature(allocator_api, slice_ptr_get)]
use
std::alloc::{Allocator, Layout, System};
unsafe
{
let
ptr = System.allocate(Layout::new::<i32>())
?
.as_mut_ptr()
as
*mut
i32;
// In general .write is required to avoid attempting to destruct
    // the (uninitialized) previous contents of `ptr`, though for this
    // simple example `*ptr = 5` would have worked as well.
ptr.write(
5
);
let
x = Box::from_raw_in(ptr, System);
}
Source
pub unsafe fn
from_non_null_in
(raw:
NonNull
<T>, alloc: A) ->
Box
<T, A>
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Constructs a box from a
NonNull
pointer in the given allocator.
After calling this function, the
NonNull
pointer is owned by
the resulting
Box
. Specifically, the
Box
destructor will call
the destructor of
T
and free the allocated memory. For this
to be safe, the memory must have been allocated in accordance
with the
memory layout
used by
Box
.
§
Safety
This function is unsafe because improper use may lead to
memory problems. For example, a double-free may occur if the
function is called twice on the same raw pointer.
The non-null pointer must point to a block of memory allocated by
alloc
.
§
Examples
Recreate a
Box
which was previously converted to a
NonNull
pointer
using
Box::into_non_null_with_allocator
:
#![feature(allocator_api, box_vec_non_null)]
use
std::alloc::System;
let
x = Box::new_in(
5
, System);
let
(non_null, alloc) = Box::into_non_null_with_allocator(x);
let
x =
unsafe
{ Box::from_non_null_in(non_null, alloc) };
Manually create a
Box
from scratch by using the system allocator:
#![feature(allocator_api, box_vec_non_null, slice_ptr_get)]
use
std::alloc::{Allocator, Layout, System};
unsafe
{
let
non_null = System.allocate(Layout::new::<i32>())
?
.cast::<i32>();
// In general .write is required to avoid attempting to destruct
    // the (uninitialized) previous contents of `non_null`.
non_null.write(
5
);
let
x = Box::from_non_null_in(non_null, System);
}
1.4.0
·
Source
pub fn
into_raw
(b:
Box
<T, A>) ->
*mut T
Consumes the
Box
, returning a wrapped raw pointer.
The pointer will be properly aligned and non-null.
After calling this function, the caller is responsible for the
memory previously managed by the
Box
. In particular, the
caller should properly destroy
T
and release the memory, taking
into account the
memory layout
used by
Box
. The easiest way to
do this is to convert the raw pointer back into a
Box
with the
Box::from_raw
function, allowing the
Box
destructor to perform
the cleanup.
Note: this is an associated function, which means that you have
to call it as
Box::into_raw(b)
instead of
b.into_raw()
. This
is so that there is no conflict with a method on the inner type.
§
Examples
Converting the raw pointer back into a
Box
with
Box::from_raw
for automatic cleanup:
let
x = Box::new(String::from(
"Hello"
));
let
ptr = Box::into_raw(x);
let
x =
unsafe
{ Box::from_raw(ptr) };
Manual cleanup by explicitly running the destructor and deallocating
the memory:
use
std::alloc::{dealloc, Layout};
use
std::ptr;
let
x = Box::new(String::from(
"Hello"
));
let
ptr = Box::into_raw(x);
unsafe
{
    ptr::drop_in_place(ptr);
    dealloc(ptr
as
*mut
u8, Layout::new::<String>());
}
Note: This is equivalent to the following:
let
x = Box::new(String::from(
"Hello"
));
let
ptr = Box::into_raw(x);
unsafe
{
    drop(Box::from_raw(ptr));
}
Source
pub fn
into_non_null
(b:
Box
<T, A>) ->
NonNull
<T>
🔬
This is a nightly-only experimental API. (
box_vec_non_null
#130364
)
Consumes the
Box
, returning a wrapped
NonNull
pointer.
The pointer will be properly aligned.
After calling this function, the caller is responsible for the
memory previously managed by the
Box
. In particular, the
caller should properly destroy
T
and release the memory, taking
into account the
memory layout
used by
Box
. The easiest way to
do this is to convert the
NonNull
pointer back into a
Box
with the
Box::from_non_null
function, allowing the
Box
destructor to
perform the cleanup.
Note: this is an associated function, which means that you have
to call it as
Box::into_non_null(b)
instead of
b.into_non_null()
.
This is so that there is no conflict with a method on the inner type.
§
Examples
Converting the
NonNull
pointer back into a
Box
with
Box::from_non_null
for automatic cleanup:
#![feature(box_vec_non_null)]
let
x = Box::new(String::from(
"Hello"
));
let
non_null = Box::into_non_null(x);
let
x =
unsafe
{ Box::from_non_null(non_null) };
Manual cleanup by explicitly running the destructor and deallocating
the memory:
#![feature(box_vec_non_null)]
use
std::alloc::{dealloc, Layout};
let
x = Box::new(String::from(
"Hello"
));
let
non_null = Box::into_non_null(x);
unsafe
{
    non_null.drop_in_place();
    dealloc(non_null.as_ptr().cast::<u8>(), Layout::new::<String>());
}
Note: This is equivalent to the following:
#![feature(box_vec_non_null)]
let
x = Box::new(String::from(
"Hello"
));
let
non_null = Box::into_non_null(x);
unsafe
{
    drop(Box::from_non_null(non_null));
}
Source
pub fn
into_raw_with_allocator
(b:
Box
<T, A>) -> (
*mut T
, A)
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Consumes the
Box
, returning a wrapped raw pointer and the allocator.
The pointer will be properly aligned and non-null.
After calling this function, the caller is responsible for the
memory previously managed by the
Box
. In particular, the
caller should properly destroy
T
and release the memory, taking
into account the
memory layout
used by
Box
. The easiest way to
do this is to convert the raw pointer back into a
Box
with the
Box::from_raw_in
function, allowing the
Box
destructor to perform
the cleanup.
Note: this is an associated function, which means that you have
to call it as
Box::into_raw_with_allocator(b)
instead of
b.into_raw_with_allocator()
. This
is so that there is no conflict with a method on the inner type.
§
Examples
Converting the raw pointer back into a
Box
with
Box::from_raw_in
for automatic cleanup:
#![feature(allocator_api)]
use
std::alloc::System;
let
x = Box::new_in(String::from(
"Hello"
), System);
let
(ptr, alloc) = Box::into_raw_with_allocator(x);
let
x =
unsafe
{ Box::from_raw_in(ptr, alloc) };
Manual cleanup by explicitly running the destructor and deallocating
the memory:
#![feature(allocator_api)]
use
std::alloc::{Allocator, Layout, System};
use
std::ptr::{
self
, NonNull};
let
x = Box::new_in(String::from(
"Hello"
), System);
let
(ptr, alloc) = Box::into_raw_with_allocator(x);
unsafe
{
    ptr::drop_in_place(ptr);
let
non_null = NonNull::new_unchecked(ptr);
    alloc.deallocate(non_null.cast(), Layout::new::<String>());
}
Source
pub fn
into_non_null_with_allocator
(b:
Box
<T, A>) -> (
NonNull
<T>, A)
🔬
This is a nightly-only experimental API. (
allocator_api
#32838
)
Consumes the
Box
, returning a wrapped
NonNull
pointer and the allocator.
The pointer will be properly aligned.
After calling this function, the caller is responsible for the
memory previously managed by the
Box
. In particular, the
caller should properly destroy
T
and release the memory, taking
into account the
memory layout
used by
Box
. The easiest way to
do this is to convert the
NonNull
pointer back into a
Box
with the
Box::from_non_null_in
function, allowing the
Box
destructor to
perform the cleanup.
Note: this is an associated function, which means that you have
to call it as
Box::into_non_null_with_allocator(b)
instead of
b.into_non_null_with_allocator()
. This is so that there is no
conflict with a method on the inner type.
§
Examples
Converting the
NonNull
pointer back into a
Box
with
Box::from_non_null_in
for automatic cleanup:
#![feature(allocator_api, box_vec_non_null)]
use
std::alloc::System;
let
x = Box::new_in(String::from(
"Hello"
), System);
let
(non_null, alloc) = Box::into_non_null_with_allocator(x);
let
x =
unsafe
{ Box::from_non_null_in(non_null, alloc) };
Manual cleanup by explicitly running the destructor and deallocating
the memory:
#![feature(allocator_api, box_vec_non_null)]
use
std::alloc::{Allocator, Layout, System};
let
x = Box::new_in(String::from(
"Hello"
), System);
let
(non_null, alloc) = Box::into_non_null_with_allocator(x);
unsafe
{
    non_null.drop_in_place();
    alloc.deallocate(non_null.cast::<u8>(), Layout::new::<String>());
}
Source
pub fn
as_mut_ptr
(b: &mut
Box
<T, A>) ->
*mut T
🔬
This is a nightly-only experimental API. (
box_as_ptr
#129090
)
Returns a raw mutable pointer to the
Box
’s contents.
The caller must ensure that the
Box
outlives the pointer this
function returns, or else it will end up dangling.
This method guarantees that for the purpose of the aliasing model, this method
does not materialize a reference to the underlying memory, and thus the returned pointer
will remain valid when mixed with other calls to
as_ptr
and
as_mut_ptr
.
Note that calling other methods that materialize references to the memory
may still invalidate this pointer.
See the example below for how this guarantee can be used.
§
Examples
Due to the aliasing guarantee, the following code is legal:
#![feature(box_as_ptr)]
unsafe
{
let
mut
b = Box::new(
0
);
let
ptr1 = Box::as_mut_ptr(
&mut
b);
    ptr1.write(
1
);
let
ptr2 = Box::as_mut_ptr(
&mut
b);
    ptr2.write(
2
);
// Notably, the write to `ptr2` did *not* invalidate `ptr1`:
ptr1.write(
3
);
}
Source
pub fn
as_ptr
(b: &
Box
<T, A>) ->
*const T
🔬
This is a nightly-only experimental API. (
box_as_ptr
#129090
)
Returns a raw pointer to the
Box
’s contents.
The caller must ensure that the
Box
outlives the pointer this
function returns, or else it will end up dangling.
The caller must also ensure that the memory the pointer (non-transitively) points to
is never written to (except inside an
UnsafeCell
) using this pointer or any pointer
derived from it. If you need to mutate the contents of the
Box
, use
as_mut_ptr
.
This method guarantees that for the purpose of the aliasing model, this method
does not materialize a reference to the underlying memory, and thus the returned pointer
will remain valid when mixed with other calls to
as_ptr
and
as_mut_ptr
.
Note that calling other methods that materialize mutable references to the memory,
as well as writing to this memory, may still invalidate this pointer.
See the example below for how this guarantee can be used.
§
Examples
Due to the aliasing guarantee, the following code is legal:
#![feature(box_as_ptr)]
unsafe
{
let
mut
v = Box::new(
0
);
let
ptr1 = Box::as_ptr(
&
v);
let
ptr2 = Box::as_mut_ptr(
&mut
v);
let
_val = ptr2.read();
// No write to this memory has happened yet, so `ptr1` is still valid.
let
_val = ptr1.read();
// However, once we do a write...
ptr2.write(
1
);
// ... `ptr1` is no longer valid.
    // This would be UB: let _val = ptr1.read();
}
Source
pub fn
allocator
(b: &
Box
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
Box::allocator(&b)
instead of
b.allocator()
. This
is so that there is no conflict with a method on the inner type.
1.26.0
·
Source
pub fn
leak
<'a>(b:
Box
<T, A>) ->
&'a mut T
where
    A: 'a,
Consumes and leaks the
Box
, returning a mutable reference,
&'a mut T
.
Note that the type
T
must outlive the chosen lifetime
'a
. If the type
has only static references, or none at all, then this may be chosen to be
'static
.
This function is mainly useful for data that lives for the remainder of
the program’s life. Dropping the returned reference will cause a memory
leak. If this is not acceptable, the reference should first be wrapped
with the
Box::from_raw
function producing a
Box
. This
Box
can
then be dropped which will properly destroy
T
and release the
allocated memory.
Note: this is an associated function, which means that you have
to call it as
Box::leak(b)
instead of
b.leak()
. This
is so that there is no conflict with a method on the inner type.
§
Examples
Simple usage:
let
x = Box::new(
41
);
let
static_ref:
&
'static
mut
usize = Box::leak(x);
*
static_ref +=
1
;
assert_eq!
(
*
static_ref,
42
);
Unsized data:
let
x =
vec!
[
1
,
2
,
3
].into_boxed_slice();
let
static_ref = Box::leak(x);
static_ref[
0
] =
4
;
assert_eq!
(
*
static_ref, [
4
,
2
,
3
]);
1.63.0
·
Source
pub fn
into_pin
(boxed:
Box
<T, A>) ->
Pin
<
Box
<T, A>>
where
    A: 'static,
Converts a
Box<T>
into a
Pin<Box<T>>
. If
T
does not implement
Unpin
, then
*boxed
will be pinned in memory and unable to be moved.
This conversion does not allocate on the heap and happens in place.
This is also available via
From
.
Constructing and pinning a
Box
with
Box::into_pin(
Box::new
(x))
can also be written more concisely using
Box::pin
(x)
.
This
into_pin
method is useful if you already have a
Box<T>
, or you are
constructing a (pinned)
Box
in a different way than with
Box::new
.
§
Notes
It’s not recommended that crates add an impl like
From<Box<T>> for Pin<T>
,
as it’ll introduce an ambiguity when calling
Pin::from
.
A demonstration of such a poor impl is shown below.
ⓘ
struct
Foo;
// A type defined in this crate.
impl
From<Box<()>>
for
Pin<Foo> {
fn
from(
_
: Box<()>) -> Pin<Foo> {
        Pin::new(Foo)
    }
}
let
foo = Box::new(());
let
bar = Pin::from(foo);
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
Box
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
Box
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
1.5.0
·
Source
§
impl<T, A>
AsMut
<T> for
Box
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
as_mut
(&mut self) ->
&mut T
Converts this type into a mutable reference of the (usually inferred) input type.
1.63.0
·
Source
§
impl<T:
AsRawFd
>
AsRawFd
for
Box
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
Box
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
Box
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
1.85.0
·
Source
§
impl<Args, F, A>
AsyncFn
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
AsyncFn
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
extern "rust-call" fn
async_call
(
    &self,
    args: Args,
) -> <
Box
<F, A> as
AsyncFnMut
<Args>>::
CallRefFuture
<'_>
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Call the
AsyncFn
, returning a future which may borrow from the called closure.
1.85.0
·
Source
§
impl<Args, F, A>
AsyncFnMut
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
AsyncFnMut
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
type
CallRefFuture
<'a> = <F as
AsyncFnMut
<Args>>::
CallRefFuture
<'a>
where
Box
<F, A>: 'a
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Future returned by
AsyncFnMut::async_call_mut
and
AsyncFn::async_call
.
Source
§
extern "rust-call" fn
async_call_mut
(
    &mut self,
    args: Args,
) -> <
Box
<F, A> as
AsyncFnMut
<Args>>::
CallRefFuture
<'_>
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Call the
AsyncFnMut
, returning a future which may borrow from the called closure.
1.85.0
·
Source
§
impl<Args, F, A>
AsyncFnOnce
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
AsyncFnOnce
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
type
Output
= <F as
AsyncFnOnce
<Args>>::
Output
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Output type of the called closure’s future.
Source
§
type
CallOnceFuture
= <F as
AsyncFnOnce
<Args>>::
CallOnceFuture
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Future returned by
AsyncFnOnce::async_call_once
.
Source
§
extern "rust-call" fn
async_call_once
(
    self,
    args: Args,
) -> <
Box
<F, A> as
AsyncFnOnce
<Args>>::
CallOnceFuture
🔬
This is a nightly-only experimental API. (
async_fn_traits
)
Call the
AsyncFnOnce
, returning a future which may move out of the called closure.
Source
§
impl<S>
AsyncIterator
for
Box
<S>
where
    S:
AsyncIterator
+
Unpin
+ ?
Sized
,
Source
§
type
Item
= <S as
AsyncIterator
>::
Item
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of items yielded by the async iterator.
Source
§
fn
poll_next
(
    self:
Pin
<&mut
Box
<S>>,
    cx: &mut
Context
<'_>,
) ->
Poll
<
Option
<<
Box
<S> as
AsyncIterator
>::
Item
>>
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Attempts to pull out the next value of this async iterator, registering the
current task for wakeup if the value is not yet available, and returning
None
if the async iterator is exhausted.
Read more
Source
§
fn
size_hint
(&self) -> (
usize
,
Option
<
usize
>)
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Returns the bounds on the remaining length of the async iterator.
Read more
1.1.0
·
Source
§
impl<T, A>
Borrow
<T> for
Box
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
1.1.0
·
Source
§
impl<T, A>
BorrowMut
<T> for
Box
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
borrow_mut
(&mut self) ->
&mut T
Mutably borrows from an owned value.
Read more
1.0.0
·
Source
§
impl<B:
BufRead
+ ?
Sized
>
BufRead
for
Box
<B>
Source
§
fn
fill_buf
(&mut self) ->
Result
<&[
u8
]>
Returns the contents of the internal buffer, filling it with more data, via
Read
methods, if empty.
Read more
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
1.3.0
·
Source
§
impl<T, A>
Clone
for
Box
<
[T]
, A>
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
Box
<
[T]
, A>)
Copies
source
’s contents into
self
without creating a new allocation,
so long as the two are of the same length.
§
Examples
let
x = Box::new([
5
,
6
,
7
]);
let
mut
y = Box::new([
8
,
9
,
10
]);
let
yp:
*const
[i32] =
&*
y;

y.clone_from(
&
x);
// The value is the same
assert_eq!
(x, y);
// And no allocation occurred
assert_eq!
(yp,
&*
y);
Source
§
fn
clone
(&self) ->
Box
<
[T]
, A>
Returns a copy of the value.
Read more
Source
§
impl
Clone
for
Box
<
ByteStr
>
Source
§
fn
clone
(&self) ->
Box
<
ByteStr
>
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
1.29.0
·
Source
§
impl
Clone
for
Box
<
CStr
>
Source
§
fn
clone
(&self) ->
Box
<
CStr
>
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
1.29.0
·
Source
§
impl
Clone
for
Box
<
OsStr
>
Source
§
fn
clone
(&self) -> Self
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
1.29.0
·
Source
§
impl
Clone
for
Box
<
Path
>
Source
§
fn
clone
(&self) -> Self
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
impl<T, A>
Clone
for
Box
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
clone
(&self) ->
Box
<T, A>
Returns a new box with a
clone()
of this box’s contents.
§
Examples
let
x = Box::new(
5
);
let
y = x.clone();
// The value is the same
assert_eq!
(x, y);
// But they are unique objects
assert_ne!
(
&*
x
as
*const
i32,
&*
y
as
*const
i32);
Source
§
fn
clone_from
(&mut self, source: &
Box
<T, A>)
Copies
source
’s contents into
self
without creating a new allocation.
§
Examples
let
x = Box::new(
5
);
let
mut
y = Box::new(
10
);
let
yp:
*const
i32 =
&*
y;

y.clone_from(
&
x);
// The value is the same
assert_eq!
(x, y);
// And no allocation occurred
assert_eq!
(yp,
&*
y);
1.3.0
·
Source
§
impl
Clone
for
Box
<
str
>
Source
§
fn
clone
(&self) ->
Box
<
str
>
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
Source
§
impl<G, R, A>
Coroutine
<R> for
Box
<G, A>
where
    G:
Coroutine
<R> +
Unpin
+ ?
Sized
,
    A:
Allocator
,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine yields.
Read more
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine returns.
Read more
Source
§
fn
resume
(
    self:
Pin
<&mut
Box
<G, A>>,
    arg: R,
) ->
CoroutineState
<<
Box
<G, A> as
Coroutine
<R>>::
Yield
, <
Box
<G, A> as
Coroutine
<R>>::
Return
>
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
Resumes the execution of this coroutine.
Read more
Source
§
impl<G, R, A>
Coroutine
<R> for
Pin
<
Box
<G, A>>
where
    G:
Coroutine
<R> + ?
Sized
,
    A:
Allocator
+ 'static,
Source
§
type
Yield
= <G as
Coroutine
<R>>::
Yield
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine yields.
Read more
Source
§
type
Return
= <G as
Coroutine
<R>>::
Return
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
The type of value this coroutine returns.
Read more
Source
§
fn
resume
(
    self:
Pin
<&mut
Pin
<
Box
<G, A>>>,
    arg: R,
) ->
CoroutineState
<<
Pin
<
Box
<G, A>> as
Coroutine
<R>>::
Yield
, <
Pin
<
Box
<G, A>> as
Coroutine
<R>>::
Return
>
🔬
This is a nightly-only experimental API. (
coroutine_trait
#43122
)
Resumes the execution of this coroutine.
Read more
1.0.0
·
Source
§
impl<T, A>
Debug
for
Box
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
1.0.0
·
Source
§
impl<T>
Default
for
Box
<
[T]
>
Source
§
fn
default
() ->
Box
<
[T]
>
Returns the “default value” for a type.
Read more
1.17.0
·
Source
§
impl
Default
for
Box
<
CStr
>
Source
§
fn
default
() ->
Box
<
CStr
>
Returns the “default value” for a type.
Read more
1.17.0
·
Source
§
impl
Default
for
Box
<
OsStr
>
Source
§
fn
default
() ->
Box
<
OsStr
>
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl<T>
Default
for
Box
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
Box
<T>
Creates a
Box<T>
, with the
Default
value for T.
1.17.0
·
Source
§
impl
Default
for
Box
<
str
>
Source
§
fn
default
() ->
Box
<
str
>
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl<T, A>
Deref
for
Box
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
DerefMut
for
Box
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
deref_mut
(&mut self) ->
&mut T
Mutably dereferences the value.
1.0.0
·
Source
§
impl<T, A>
Display
for
Box
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
impl<I, A>
DoubleEndedIterator
for
Box
<I, A>
where
    I:
DoubleEndedIterator
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
next_back
(&mut self) ->
Option
<<I as
Iterator
>::
Item
>
Removes and returns an element from the end of the iterator.
Read more
Source
§
fn
nth_back
(&mut self, n:
usize
) ->
Option
<<I as
Iterator
>::
Item
>
Returns the
n
th element from the end of the iterator.
Read more
Source
§
fn
advance_back_by
(&mut self, n:
usize
) ->
Result
<
()
,
NonZero
<
usize
>>
🔬
This is a nightly-only experimental API. (
iter_advance_by
#77404
)
Advances the iterator from the back by
n
elements.
Read more
1.27.0
·
Source
§
fn
try_rfold
<B, F, R>(&mut self, init: B, f: F) -> R
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> R,
    R:
Try
<Output = B>,
This is the reverse version of
Iterator::try_fold()
: it takes
elements starting from the back of the iterator.
Read more
1.27.0
·
Source
§
fn
rfold
<B, F>(self, init: B, f: F) -> B
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> B,
An iterator method that reduces the iterator’s elements to a single,
final value, starting from the back.
Read more
1.27.0
·
Source
§
fn
rfind
<P>(&mut self, predicate: P) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Searches for an element of an iterator from the back that satisfies a predicate.
Read more
1.0.0
·
Source
§
impl<T, A>
Drop
for
Box
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
Executes the destructor for this type.
Read more
1.8.0
·
Source
§
impl<E>
Error
for
Box
<E>
where
    E:
Error
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
<'b>(&'b self, request: &mut
Request
<'b>)
🔬
This is a nightly-only experimental API. (
error_generic_member_access
#99301
)
Provides type-based access to context intended for error reports.
Read more
1.0.0
·
Source
§
impl<I, A>
ExactSizeIterator
for
Box
<I, A>
where
    I:
ExactSizeIterator
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
len
(&self) ->
usize
Returns the exact remaining length of the iterator.
Read more
Source
§
fn
is_empty
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
exact_size_is_empty
#35428
)
Returns
true
if the iterator is empty.
Read more
1.45.0
·
Source
§
impl<A>
Extend
<
Box
<
str
, A>> for
String
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
<Item =
Box
<
str
, A>>,
Extends a collection with the contents of an iterator.
Read more
Source
§
fn
extend_one
(&mut self, item: A)
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
1.35.0
·
Source
§
impl<Args, F, A>
Fn
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
Fn
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
extern "rust-call" fn
call
(
    &self,
    args: Args,
) -> <
Box
<F, A> as
FnOnce
<Args>>::
Output
🔬
This is a nightly-only experimental API. (
fn_traits
#29625
)
Performs the call operation.
1.35.0
·
Source
§
impl<Args, F, A>
FnMut
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
FnMut
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
extern "rust-call" fn
call_mut
(
    &mut self,
    args: Args,
) -> <
Box
<F, A> as
FnOnce
<Args>>::
Output
🔬
This is a nightly-only experimental API. (
fn_traits
#29625
)
Performs the call operation.
1.35.0
·
Source
§
impl<Args, F, A>
FnOnce
<Args> for
Box
<F, A>
where
    Args:
Tuple
,
    F:
FnOnce
<Args> + ?
Sized
,
    A:
Allocator
,
Source
§
type
Output
= <F as
FnOnce
<Args>>::
Output
The returned type after the call operator is used.
Source
§
extern "rust-call" fn
call_once
(
    self,
    args: Args,
) -> <
Box
<F, A> as
FnOnce
<Args>>::
Output
🔬
This is a nightly-only experimental API. (
fn_traits
#29625
)
Performs the call operation.
1.17.0
·
Source
§
impl<T>
From
<&
[T]
> for
Box
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
(slice: &
[T]
) ->
Box
<
[T]
>
Converts a
&[T]
into a
Box<[T]>
This conversion allocates on the heap
and performs a copy of
slice
and its contents.
§
Examples
// create a &[u8] which will be used to create a Box<[u8]>
let
slice:
&
[u8] =
&
[
104
,
101
,
108
,
108
,
111
];
let
boxed_slice: Box<[u8]> = Box::from(slice);
println!
(
"{boxed_slice:?}"
);
1.17.0
·
Source
§
impl
From
<&
CStr
> for
Box
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
Box
<
CStr
>
Converts a
&CStr
into a
Box<CStr>
,
by copying the contents into a newly allocated
Box
.
1.17.0
·
Source
§
impl
From
<&
OsStr
> for
Box
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
Box
<
OsStr
>
Copies the string into a newly allocated
Box
<
OsStr
>
.
1.17.0
·
Source
§
impl
From
<&
Path
> for
Box
<
Path
>
Source
§
fn
from
(path: &
Path
) ->
Box
<
Path
>
Creates a boxed
Path
from a reference.
This will allocate and clone
path
to it.
1.84.0
·
Source
§
impl<T>
From
<&mut
[T]
> for
Box
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
(slice: &mut
[T]
) ->
Box
<
[T]
>
Converts a
&mut [T]
into a
Box<[T]>
This conversion allocates on the heap
and performs a copy of
slice
and its contents.
§
Examples
// create a &mut [u8] which will be used to create a Box<[u8]>
let
mut
array = [
104
,
101
,
108
,
108
,
111
];
let
slice:
&mut
[u8] =
&mut
array;
let
boxed_slice: Box<[u8]> = Box::from(slice);
println!
(
"{boxed_slice:?}"
);
1.84.0
·
Source
§
impl
From
<&mut
CStr
> for
Box
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
Box
<
CStr
>
Converts a
&mut CStr
into a
Box<CStr>
,
by copying the contents into a newly allocated
Box
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
Box
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
Box
<
OsStr
>
Copies the string into a newly allocated
Box
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
Box
<
Path
>
Source
§
fn
from
(path: &mut
Path
) ->
Box
<
Path
>
Creates a boxed
Path
from a reference.
This will allocate and clone
path
to it.
1.84.0
·
Source
§
impl
From
<&mut
str
> for
Box
<
str
>
Source
§
fn
from
(s: &mut
str
) ->
Box
<
str
>
Converts a
&mut str
into a
Box<str>
This conversion allocates on the heap
and performs a copy of
s
.
§
Examples
let
mut
original = String::from(
"hello"
);
let
original:
&mut
str =
&mut
original;
let
boxed: Box<str> = Box::from(original);
println!
(
"{boxed}"
);
1.6.0
·
Source
§
impl<'a>
From
<&
str
> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(err: &
str
) ->
Box
<dyn
Error
+ 'a>
Converts a
str
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
let
a_str_error =
"a str error"
;
let
a_boxed_error = Box::<
dyn
Error>::from(a_str_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a>
From
<&
str
> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Source
§
fn
from
(err: &
str
) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a
str
into a box of dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
let
a_str_error =
"a str error"
;
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_str_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.17.0
·
Source
§
impl
From
<&
str
> for
Box
<
str
>
Source
§
fn
from
(s: &
str
) ->
Box
<
str
>
Converts a
&str
into a
Box<str>
This conversion allocates on the heap
and performs a copy of
s
.
§
Examples
let
boxed: Box<str> = Box::from(
"hello"
);
println!
(
"{boxed}"
);
1.45.0
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
Box
<
[T]
>
Source
§
fn
from
(array:
[T; N]
) ->
Box
<
[T]
>
Converts a
[T; N]
into a
Box<[T]>
This conversion moves the array to newly heap-allocated memory.
§
Examples
let
boxed: Box<[u8]> = Box::from([
4
,
2
]);
println!
(
"{boxed:?}"
);
1.18.0
·
Source
§
impl<T, A>
From
<
Box
<
[T]
, A>> for
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
(s:
Box
<
[T]
, A>) ->
Vec
<T, A>
Converts a boxed slice into a vector by transferring ownership of
the existing heap allocation.
§
Examples
let
b: Box<[i32]> =
vec!
[
1
,
2
,
3
].into_boxed_slice();
assert_eq!
(Vec::from(b),
vec!
[
1
,
2
,
3
]);
Source
§
impl
From
<
Box
<[
u8
]>> for
Box
<
ByteStr
>
Source
§
fn
from
(s:
Box
<[
u8
]>) ->
Box
<
ByteStr
>
Converts to this type from the input type.
Source
§
impl
From
<
Box
<
ByteStr
>> for
Box
<[
u8
]>
Source
§
fn
from
(s:
Box
<
ByteStr
>) ->
Box
<[
u8
]>
Converts to this type from the input type.
1.18.0
·
Source
§
impl
From
<
Box
<
CStr
>> for
CString
Source
§
fn
from
(s:
Box
<
CStr
>) ->
CString
Converts a
Box
<
CStr
>
into a
CString
without copying or allocating.
1.18.0
·
Source
§
impl
From
<
Box
<
OsStr
>> for
OsString
Source
§
fn
from
(boxed:
Box
<
OsStr
>) ->
OsString
Converts a
Box
<
OsStr
>
into an
OsString
without copying or
allocating.
1.18.0
·
Source
§
impl
From
<
Box
<
Path
>> for
PathBuf
Source
§
fn
from
(boxed:
Box
<
Path
>) ->
PathBuf
Converts a
Box
<
Path
>
into a
PathBuf
.
This conversion does not allocate or copy memory.
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
1.33.0
·
Source
§
impl<T, A>
From
<
Box
<T, A>> for
Pin
<
Box
<T, A>>
where
    A:
Allocator
+ 'static,
    T: ?
Sized
,
Source
§
fn
from
(boxed:
Box
<T, A>) ->
Pin
<
Box
<T, A>>
Converts a
Box<T>
into a
Pin<Box<T>>
. If
T
does not implement
Unpin
, then
*boxed
will be pinned in memory and unable to be moved.
This conversion does not allocate on the heap and happens in place.
This is also available via
Box::into_pin
.
Constructing and pinning a
Box
with
<Pin<Box<T>>>::from(
Box::new
(x))
can also be written more concisely using
Box::pin
(x)
.
This
From
implementation is useful if you already have a
Box<T>
, or you are
constructing a (pinned)
Box
in a different way than with
Box::new
.
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
1.18.0
·
Source
§
impl
From
<
Box
<
str
>> for
String
Source
§
fn
from
(s:
Box
<
str
>) ->
String
Converts the given boxed
str
slice to a
String
.
It is notable that the
str
slice is owned.
§
Examples
let
s1: String = String::from(
"hello world"
);
let
s2: Box<str> = s1.into_boxed_str();
let
s3: String = String::from(s2);
assert_eq!
(
"hello world"
, s3)
1.19.0
·
Source
§
impl<A>
From
<
Box
<
str
, A>> for
Box
<[
u8
], A>
where
    A:
Allocator
,
Source
§
fn
from
(s:
Box
<
str
, A>) ->
Box
<[
u8
], A>
Converts a
Box<str>
into a
Box<[u8]>
This conversion does not allocate on the heap and happens in place.
§
Examples
// create a Box<str> which will be used to create a Box<[u8]>
let
boxed: Box<str> = Box::from(
"hello"
);
let
boxed_str: Box<[u8]> = Box::from(boxed);
// create a &[u8] which will be used to create a Box<[u8]>
let
slice:
&
[u8] =
&
[
104
,
101
,
108
,
108
,
111
];
let
boxed_slice = Box::from(slice);
assert_eq!
(boxed_slice, boxed_str);
1.20.0
·
Source
§
impl
From
<
CString
> for
Box
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
Box
<
CStr
>
Converts a
CString
into a
Box
<
CStr
>
without copying or allocating.
1.45.0
·
Source
§
impl<T>
From
<
Cow
<'_,
[T]
>> for
Box
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
(cow:
Cow
<'_,
[T]
>) ->
Box
<
[T]
>
Converts a
Cow<'_, [T]>
into a
Box<[T]>
When
cow
is the
Cow::Borrowed
variant, this
conversion allocates on the heap and copies the
underlying slice. Otherwise, it will try to reuse the owned
Vec
’s allocation.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
CStr
>> for
Box
<
CStr
>
Source
§
fn
from
(cow:
Cow
<'_,
CStr
>) ->
Box
<
CStr
>
Converts a
Cow<'a, CStr>
into a
Box<CStr>
,
by copying the contents if they are borrowed.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
OsStr
>> for
Box
<
OsStr
>
Source
§
fn
from
(cow:
Cow
<'_,
OsStr
>) ->
Box
<
OsStr
>
Converts a
Cow<'a, OsStr>
into a
Box
<
OsStr
>
,
by copying the contents if they are borrowed.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
Path
>> for
Box
<
Path
>
Source
§
fn
from
(cow:
Cow
<'_,
Path
>) ->
Box
<
Path
>
Creates a boxed
Path
from a clone-on-write pointer.
Converting from a
Cow::Owned
does not clone or allocate.
1.45.0
·
Source
§
impl
From
<
Cow
<'_,
str
>> for
Box
<
str
>
Source
§
fn
from
(cow:
Cow
<'_,
str
>) ->
Box
<
str
>
Converts a
Cow<'_, str>
into a
Box<str>
When
cow
is the
Cow::Borrowed
variant, this
conversion allocates on the heap and copies the
underlying
str
. Otherwise, it will try to reuse the owned
String
’s allocation.
§
Examples
use
std::borrow::Cow;
let
unboxed = Cow::Borrowed(
"hello"
);
let
boxed: Box<str> = Box::from(unboxed);
println!
(
"{boxed}"
);
let
unboxed = Cow::Owned(
"hello"
.to_string());
let
boxed: Box<str> = Box::from(unboxed);
println!
(
"{boxed}"
);
1.22.0
·
Source
§
impl<'a, 'b>
From
<
Cow
<'b,
str
>> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(err:
Cow
<'b,
str
>) ->
Box
<dyn
Error
+ 'a>
Converts a
Cow
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
use
std::borrow::Cow;
let
a_cow_str_error = Cow::from(
"a str error"
);
let
a_boxed_error = Box::<
dyn
Error>::from(a_cow_str_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.22.0
·
Source
§
impl<'a, 'b>
From
<
Cow
<'b,
str
>> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Source
§
fn
from
(err:
Cow
<'b,
str
>) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a
Cow
into a box of dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
use
std::borrow::Cow;
let
a_cow_str_error = Cow::from(
"a str error"
);
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_cow_str_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a, E>
From
<E> for
Box
<dyn
Error
+ 'a>
where
    E:
Error
+ 'a,
Source
§
fn
from
(err: E) ->
Box
<dyn
Error
+ 'a>
Converts a type of
Error
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
use
std::fmt;
#[derive(Debug)]
struct
AnError;
impl
fmt::Display
for
AnError {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"An error"
)
    }
}
impl
Error
for
AnError {}
let
an_error = AnError;
assert!
(
0
== size_of_val(
&
an_error));
let
a_boxed_error = Box::<
dyn
Error>::from(an_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a, E>
From
<E> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
where
    E:
Error
+
Send
+
Sync
+ 'a,
Source
§
fn
from
(err: E) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a type of
Error
+
Send
+
Sync
into a box of
dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
use
std::fmt;
#[derive(Debug)]
struct
AnError;
impl
fmt::Display
for
AnError {
fn
fmt(
&
self
, f:
&mut
fmt::Formatter<
'_
>) -> fmt::Result {
write!
(f,
"An error"
)
    }
}
impl
Error
for
AnError {}
unsafe impl
Send
for
AnError {}
unsafe impl
Sync
for
AnError {}
let
an_error = AnError;
assert!
(
0
== size_of_val(
&
an_error));
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(an_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.20.0
·
Source
§
impl
From
<
OsString
> for
Box
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
Box
<
OsStr
>
Converts an
OsString
into a
Box
<
OsStr
>
without copying or allocating.
1.20.0
·
Source
§
impl
From
<
PathBuf
> for
Box
<
Path
>
Source
§
fn
from
(p:
PathBuf
) ->
Box
<
Path
>
Converts a
PathBuf
into a
Box
<
Path
>
.
This conversion currently should not allocate memory,
but this behavior is not guaranteed on all platforms or in all future versions.
1.6.0
·
Source
§
impl<'a>
From
<
String
> for
Box
<dyn
Error
+ 'a>
Source
§
fn
from
(str_err:
String
) ->
Box
<dyn
Error
+ 'a>
Converts a
String
into a box of dyn
Error
.
§
Examples
use
std::error::Error;
let
a_string_error =
"a string error"
.to_string();
let
a_boxed_error = Box::<
dyn
Error>::from(a_string_error);
assert!
(size_of::<Box<
dyn
Error>>() == size_of_val(
&
a_boxed_error))
1.0.0
·
Source
§
impl<'a>
From
<
String
> for
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Source
§
fn
from
(err:
String
) ->
Box
<dyn
Error
+
Sync
+
Send
+ 'a>
Converts a
String
into a box of dyn
Error
+
Send
+
Sync
.
§
Examples
use
std::error::Error;
let
a_string_error =
"a string error"
.to_string();
let
a_boxed_error = Box::<
dyn
Error + Send + Sync>::from(a_string_error);
assert!
(
    size_of::<Box<
dyn
Error + Send + Sync>>() == size_of_val(
&
a_boxed_error))
1.20.0
·
Source
§
impl
From
<
String
> for
Box
<
str
>
Source
§
fn
from
(s:
String
) ->
Box
<
str
>
Converts the given
String
to a boxed
str
slice that is owned.
§
Examples
let
s1: String = String::from(
"hello world"
);
let
s2: Box<str> = Box::from(s1);
let
s3: String = String::from(s2);
assert_eq!
(
"hello world"
, s3)
1.6.0
·
Source
§
impl<T>
From
<T> for
Box
<T>
Source
§
fn
from
(t: T) ->
Box
<T>
Converts a
T
into a
Box<T>
The conversion allocates on the heap and moves
t
from the stack into it.
§
Examples
let
x =
5
;
let
boxed = Box::new(
5
);
assert_eq!
(Box::from(x), boxed);
1.20.0
·
Source
§
impl<T, A>
From
<
Vec
<T, A>> for
Box
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
Box
<
[T]
, A>
Converts a vector into a boxed slice.
Before doing the conversion, this method discards excess capacity like
Vec::shrink_to_fit
.
§
Examples
assert_eq!
(Box::from(
vec!
[
1
,
2
,
3
]),
vec!
[
1
,
2
,
3
].into_boxed_slice());
Any excess capacity is removed:
let
mut
vec = Vec::with_capacity(
10
);
vec.extend([
1
,
2
,
3
]);
assert_eq!
(Box::from(vec),
vec!
[
1
,
2
,
3
].into_boxed_slice());
1.80.0
·
Source
§
impl<'a>
FromIterator
<&'a
char
> for
Box
<
str
>
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
str
>
where
    T:
IntoIterator
<Item = &'a
char
>,
Creates a value from an iterator.
Read more
1.80.0
·
Source
§
impl<'a>
FromIterator
<&'a
str
> for
Box
<
str
>
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
str
>
where
    T:
IntoIterator
<Item = &'a
str
>,
Creates a value from an iterator.
Read more
1.80.0
·
Source
§
impl<A>
FromIterator
<
Box
<
str
, A>> for
Box
<
str
>
where
    A:
Allocator
,
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
str
>
where
    T:
IntoIterator
<Item =
Box
<
str
, A>>,
Creates a value from an iterator.
Read more
1.45.0
·
Source
§
impl<A>
FromIterator
<
Box
<
str
, A>> for
String
where
    A:
Allocator
,
Source
§
fn
from_iter
<I>(iter: I) ->
String
where
    I:
IntoIterator
<Item =
Box
<
str
, A>>,
Creates a value from an iterator.
Read more
1.80.0
·
Source
§
impl<'a>
FromIterator
<
Cow
<'a,
str
>> for
Box
<
str
>
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
str
>
where
    T:
IntoIterator
<Item =
Cow
<'a,
str
>>,
Creates a value from an iterator.
Read more
1.32.0
·
Source
§
impl<I>
FromIterator
<I> for
Box
<
[I]
>
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
[I]
>
where
    T:
IntoIterator
<Item = I>,
Creates a value from an iterator.
Read more
1.80.0
·
Source
§
impl
FromIterator
<
String
> for
Box
<
str
>
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
str
>
where
    T:
IntoIterator
<Item =
String
>,
Creates a value from an iterator.
Read more
1.80.0
·
Source
§
impl
FromIterator
<
char
> for
Box
<
str
>
Source
§
fn
from_iter
<T>(iter: T) ->
Box
<
str
>
where
    T:
IntoIterator
<Item =
char
>,
Creates a value from an iterator.
Read more
1.36.0
·
Source
§
impl<F, A>
Future
for
Box
<F, A>
where
    F:
Future
+
Unpin
+ ?
Sized
,
    A:
Allocator
,
Source
§
type
Output
= <F as
Future
>::
Output
The type of value produced on completion.
Source
§
fn
poll
(
    self:
Pin
<&mut
Box
<F, A>>,
    cx: &mut
Context
<'_>,
) ->
Poll
<<
Box
<F, A> as
Future
>::
Output
>
Attempts to resolve the future to a final value, registering
the current task for wakeup if the value is not yet available.
Read more
1.0.0
·
Source
§
impl<T, A>
Hash
for
Box
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
1.22.0
·
Source
§
impl<T, A>
Hasher
for
Box
<T, A>
where
    T:
Hasher
+ ?
Sized
,
    A:
Allocator
,
Source
§
fn
finish
(&self) ->
u64
Returns the hash value for the values written so far.
Read more
Source
§
fn
write
(&mut self, bytes: &[
u8
])
Writes some data into this
Hasher
.
Read more
Source
§
fn
write_u8
(&mut self, i:
u8
)
Writes a single
u8
into this hasher.
Source
§
fn
write_u16
(&mut self, i:
u16
)
Writes a single
u16
into this hasher.
Source
§
fn
write_u32
(&mut self, i:
u32
)
Writes a single
u32
into this hasher.
Source
§
fn
write_u64
(&mut self, i:
u64
)
Writes a single
u64
into this hasher.
Source
§
fn
write_u128
(&mut self, i:
u128
)
Writes a single
u128
into this hasher.
Source
§
fn
write_usize
(&mut self, i:
usize
)
Writes a single
usize
into this hasher.
Source
§
fn
write_i8
(&mut self, i:
i8
)
Writes a single
i8
into this hasher.
Source
§
fn
write_i16
(&mut self, i:
i16
)
Writes a single
i16
into this hasher.
Source
§
fn
write_i32
(&mut self, i:
i32
)
Writes a single
i32
into this hasher.
Source
§
fn
write_i64
(&mut self, i:
i64
)
Writes a single
i64
into this hasher.
Source
§
fn
write_i128
(&mut self, i:
i128
)
Writes a single
i128
into this hasher.
Source
§
fn
write_isize
(&mut self, i:
isize
)
Writes a single
isize
into this hasher.
Source
§
fn
write_length_prefix
(&mut self, len:
usize
)
🔬
This is a nightly-only experimental API. (
hasher_prefixfree_extras
#96762
)
Writes a length prefix into this hasher, as part of being prefix-free.
Read more
Source
§
fn
write_str
(&mut self, s: &
str
)
🔬
This is a nightly-only experimental API. (
hasher_prefixfree_extras
#96762
)
Writes a single
str
into this hasher.
Read more
1.80.0
·
Source
§
impl<'a, I, A>
IntoIterator
for &'a
Box
<
[I]
, A>
where
    A:
Allocator
,
Source
§
type
IntoIter
=
Iter
<'a, I>
Which kind of iterator are we turning this into?
Source
§
type
Item
=
&'a I
The type of the elements being iterated over.
Source
§
fn
into_iter
(self) ->
Iter
<'a, I>
ⓘ
Creates an iterator from a value.
Read more
1.80.0
·
Source
§
impl<'a, I, A>
IntoIterator
for &'a mut
Box
<
[I]
, A>
where
    A:
Allocator
,
Source
§
type
IntoIter
=
IterMut
<'a, I>
Which kind of iterator are we turning this into?
Source
§
type
Item
=
&'a mut I
The type of the elements being iterated over.
Source
§
fn
into_iter
(self) ->
IterMut
<'a, I>
ⓘ
Creates an iterator from a value.
Read more
1.80.0
·
Source
§
impl<I, A>
IntoIterator
for
Box
<
[I]
, A>
where
    A:
Allocator
,
Source
§
type
IntoIter
=
IntoIter
<I, A>
Which kind of iterator are we turning this into?
Source
§
type
Item
= I
The type of the elements being iterated over.
Source
§
fn
into_iter
(self) ->
IntoIter
<I, A>
ⓘ
Creates an iterator from a value.
Read more
1.0.0
·
Source
§
impl<I, A>
Iterator
for
Box
<I, A>
where
    I:
Iterator
+ ?
Sized
,
    A:
Allocator
,
Source
§
type
Item
= <I as
Iterator
>::
Item
The type of the elements being iterated over.
Source
§
fn
next
(&mut self) ->
Option
<<I as
Iterator
>::
Item
>
Advances the iterator and returns the next value.
Read more
Source
§
fn
size_hint
(&self) -> (
usize
,
Option
<
usize
>)
Returns the bounds on the remaining length of the iterator.
Read more
Source
§
fn
nth
(&mut self, n:
usize
) ->
Option
<<I as
Iterator
>::
Item
>
Returns the
n
th element of the iterator.
Read more
Source
§
fn
last
(self) ->
Option
<<I as
Iterator
>::
Item
>
Consumes the iterator, returning the last element.
Read more
Source
§
fn
next_chunk
<const N:
usize
>(
    &mut self,
) ->
Result
<[Self::
Item
;
N
],
IntoIter
<Self::
Item
, N>>
where
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
iter_next_chunk
#98326
)
Advances the iterator and returns an array containing the next
N
values.
Read more
1.0.0
·
Source
§
fn
count
(self) ->
usize
where
    Self:
Sized
,
Consumes the iterator, counting the number of iterations and returning it.
Read more
Source
§
fn
advance_by
(&mut self, n:
usize
) ->
Result
<
()
,
NonZero
<
usize
>>
🔬
This is a nightly-only experimental API. (
iter_advance_by
#77404
)
Advances the iterator by
n
elements.
Read more
1.28.0
·
Source
§
fn
step_by
(self, step:
usize
) ->
StepBy
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator starting at the same point, but stepping by
the given amount at each iteration.
Read more
1.0.0
·
Source
§
fn
chain
<U>(self, other: U) ->
Chain
<Self, <U as
IntoIterator
>::
IntoIter
>
ⓘ
where
    Self:
Sized
,
    U:
IntoIterator
<Item = Self::
Item
>,
Takes two iterators and creates a new iterator over both in sequence.
Read more
1.0.0
·
Source
§
fn
zip
<U>(self, other: U) ->
Zip
<Self, <U as
IntoIterator
>::
IntoIter
>
ⓘ
where
    Self:
Sized
,
    U:
IntoIterator
,
‘Zips up’ two iterators into a single iterator of pairs.
Read more
Source
§
fn
intersperse
(self, separator: Self::
Item
) ->
Intersperse
<Self>
ⓘ
where
    Self:
Sized
,
    Self::
Item
:
Clone
,
🔬
This is a nightly-only experimental API. (
iter_intersperse
#79524
)
Creates a new iterator which places a copy of
separator
between adjacent
items of the original iterator.
Read more
Source
§
fn
intersperse_with
<G>(self, separator: G) ->
IntersperseWith
<Self, G>
ⓘ
where
    Self:
Sized
,
    G:
FnMut
() -> Self::
Item
,
🔬
This is a nightly-only experimental API. (
iter_intersperse
#79524
)
Creates a new iterator which places an item generated by
separator
between adjacent items of the original iterator.
Read more
1.0.0
·
Source
§
fn
map
<B, F>(self, f: F) ->
Map
<Self, F>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) -> B,
Takes a closure and creates an iterator which calls that closure on each
element.
Read more
1.21.0
·
Source
§
fn
for_each
<F>(self, f: F)
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
),
Calls a closure on each element of an iterator.
Read more
1.0.0
·
Source
§
fn
filter
<P>(self, predicate: P) ->
Filter
<Self, P>
ⓘ
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Creates an iterator which uses a closure to determine if an element
should be yielded.
Read more
1.0.0
·
Source
§
fn
filter_map
<B, F>(self, f: F) ->
FilterMap
<Self, F>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) ->
Option
<B>,
Creates an iterator that both filters and maps.
Read more
1.0.0
·
Source
§
fn
enumerate
(self) ->
Enumerate
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator which gives the current iteration count as well as
the next value.
Read more
1.0.0
·
Source
§
fn
peekable
(self) ->
Peekable
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator which can use the
peek
and
peek_mut
methods
to look at the next element of the iterator without consuming it. See
their documentation for more information.
Read more
1.0.0
·
Source
§
fn
skip_while
<P>(self, predicate: P) ->
SkipWhile
<Self, P>
ⓘ
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Creates an iterator that
skip
s elements based on a predicate.
Read more
1.0.0
·
Source
§
fn
take_while
<P>(self, predicate: P) ->
TakeWhile
<Self, P>
ⓘ
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Creates an iterator that yields elements based on a predicate.
Read more
1.57.0
·
Source
§
fn
map_while
<B, P>(self, predicate: P) ->
MapWhile
<Self, P>
ⓘ
where
    Self:
Sized
,
    P:
FnMut
(Self::
Item
) ->
Option
<B>,
Creates an iterator that both yields elements based on a predicate and maps.
Read more
1.0.0
·
Source
§
fn
skip
(self, n:
usize
) ->
Skip
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator that skips the first
n
elements.
Read more
1.0.0
·
Source
§
fn
take
(self, n:
usize
) ->
Take
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator that yields the first
n
elements, or fewer
if the underlying iterator ends sooner.
Read more
1.0.0
·
Source
§
fn
scan
<St, B, F>(self, initial_state: St, f: F) ->
Scan
<Self, St, F>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(
&mut St
, Self::
Item
) ->
Option
<B>,
An iterator adapter which, like
fold
, holds internal state, but
unlike
fold
, produces a new iterator.
Read more
1.0.0
·
Source
§
fn
flat_map
<U, F>(self, f: F) ->
FlatMap
<Self, U, F>
ⓘ
where
    Self:
Sized
,
    U:
IntoIterator
,
    F:
FnMut
(Self::
Item
) -> U,
Creates an iterator that works like map, but flattens nested structure.
Read more
1.29.0
·
Source
§
fn
flatten
(self) ->
Flatten
<Self>
ⓘ
where
    Self:
Sized
,
    Self::
Item
:
IntoIterator
,
Creates an iterator that flattens nested structure.
Read more
Source
§
fn
map_windows
<F, R, const N:
usize
>(self, f: F) ->
MapWindows
<Self, F, N>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(&[Self::
Item
;
N
]) -> R,
🔬
This is a nightly-only experimental API. (
iter_map_windows
#87155
)
Calls the given function
f
for each contiguous window of size
N
over
self
and returns an iterator over the outputs of
f
. Like
slice::windows()
,
the windows during mapping overlap as well.
Read more
1.0.0
·
Source
§
fn
fuse
(self) ->
Fuse
<Self>
ⓘ
where
    Self:
Sized
,
Creates an iterator which ends after the first
None
.
Read more
1.0.0
·
Source
§
fn
inspect
<F>(self, f: F) ->
Inspect
<Self, F>
ⓘ
where
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
),
Does something with each element of an iterator, passing the value on.
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
Iterator
.
Read more
1.0.0
·
Source
§
fn
collect
<B>(self) -> B
where
    B:
FromIterator
<Self::
Item
>,
    Self:
Sized
,
Transforms an iterator into a collection.
Read more
Source
§
fn
try_collect
<B>(
    &mut self,
) -> <<Self::
Item
as
Try
>::
Residual
as
Residual
<B>>::
TryType
where
    Self:
Sized
,
    Self::
Item
:
Try
,
    <Self::
Item
as
Try
>::
Residual
:
Residual
<B>,
    B:
FromIterator
<<Self::
Item
as
Try
>::
Output
>,
🔬
This is a nightly-only experimental API. (
iterator_try_collect
#94047
)
Fallibly transforms an iterator into a collection, short circuiting if
a failure is encountered.
Read more
Source
§
fn
collect_into
<E>(self, collection:
&mut E
) ->
&mut E
where
    E:
Extend
<Self::
Item
>,
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
iter_collect_into
#94780
)
Collects all the items from an iterator into a collection.
Read more
1.0.0
·
Source
§
fn
partition
<B, F>(self, f: F) ->
(B, B)
where
    Self:
Sized
,
    B:
Default
+
Extend
<Self::
Item
>,
    F:
FnMut
(&Self::
Item
) ->
bool
,
Consumes an iterator, creating two collections from it.
Read more
Source
§
fn
partition_in_place
<'a, T, P>(self, predicate: P) ->
usize
where
    T: 'a,
    Self:
Sized
+
DoubleEndedIterator
<Item =
&'a mut T
>,
    P:
FnMut
(
&T
) ->
bool
,
🔬
This is a nightly-only experimental API. (
iter_partition_in_place
#62543
)
Reorders the elements of this iterator
in-place
according to the given predicate,
such that all those that return
true
precede all those that return
false
.
Returns the number of
true
elements found.
Read more
Source
§
fn
is_partitioned
<P>(self, predicate: P) ->
bool
where
    Self:
Sized
,
    P:
FnMut
(Self::
Item
) ->
bool
,
🔬
This is a nightly-only experimental API. (
iter_is_partitioned
#62544
)
Checks if the elements of this iterator are partitioned according to the given predicate,
such that all those that return
true
precede all those that return
false
.
Read more
1.27.0
·
Source
§
fn
try_fold
<B, F, R>(&mut self, init: B, f: F) -> R
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> R,
    R:
Try
<Output = B>,
An iterator method that applies a function as long as it returns
successfully, producing a single, final value.
Read more
1.27.0
·
Source
§
fn
try_for_each
<F, R>(&mut self, f: F) -> R
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) -> R,
    R:
Try
<Output =
()
>,
An iterator method that applies a fallible function to each item in the
iterator, stopping at the first error and returning that error.
Read more
1.0.0
·
Source
§
fn
fold
<B, F>(self, init: B, f: F) -> B
where
    Self:
Sized
,
    F:
FnMut
(B, Self::
Item
) -> B,
Folds every element into an accumulator by applying an operation,
returning the final result.
Read more
1.51.0
·
Source
§
fn
reduce
<F>(self, f: F) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
, Self::
Item
) -> Self::
Item
,
Reduces the elements to a single one, by repeatedly applying a reducing
operation.
Read more
Source
§
fn
try_reduce
<R>(
    &mut self,
    f: impl
FnMut
(Self::
Item
, Self::
Item
) -> R,
) -> <<R as
Try
>::
Residual
as
Residual
<
Option
<<R as
Try
>::
Output
>>>::
TryType
where
    Self:
Sized
,
    R:
Try
<Output = Self::
Item
>,
    <R as
Try
>::
Residual
:
Residual
<
Option
<Self::
Item
>>,
🔬
This is a nightly-only experimental API. (
iterator_try_reduce
#87053
)
Reduces the elements to a single one by repeatedly applying a reducing operation. If the
closure returns a failure, the failure is propagated back to the caller immediately.
Read more
1.0.0
·
Source
§
fn
all
<F>(&mut self, f: F) ->
bool
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) ->
bool
,
Tests if every element of the iterator matches a predicate.
Read more
1.0.0
·
Source
§
fn
any
<F>(&mut self, f: F) ->
bool
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) ->
bool
,
Tests if any element of the iterator matches a predicate.
Read more
1.0.0
·
Source
§
fn
find
<P>(&mut self, predicate: P) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    P:
FnMut
(&Self::
Item
) ->
bool
,
Searches for an element of an iterator that satisfies a predicate.
Read more
1.30.0
·
Source
§
fn
find_map
<B, F>(&mut self, f: F) ->
Option
<B>
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) ->
Option
<B>,
Applies function to the elements of iterator and returns
the first non-none result.
Read more
Source
§
fn
try_find
<R>(
    &mut self,
    f: impl
FnMut
(&Self::
Item
) -> R,
) -> <<R as
Try
>::
Residual
as
Residual
<
Option
<Self::
Item
>>>::
TryType
where
    Self:
Sized
,
    R:
Try
<Output =
bool
>,
    <R as
Try
>::
Residual
:
Residual
<
Option
<Self::
Item
>>,
🔬
This is a nightly-only experimental API. (
try_find
#63178
)
Applies function to the elements of iterator and returns
the first true result or the first error.
Read more
1.0.0
·
Source
§
fn
position
<P>(&mut self, predicate: P) ->
Option
<
usize
>
where
    Self:
Sized
,
    P:
FnMut
(Self::
Item
) ->
bool
,
Searches for an element in an iterator, returning its index.
Read more
1.0.0
·
Source
§
fn
rposition
<P>(&mut self, predicate: P) ->
Option
<
usize
>
where
    P:
FnMut
(Self::
Item
) ->
bool
,
    Self:
Sized
+
ExactSizeIterator
+
DoubleEndedIterator
,
Searches for an element in an iterator from the right, returning its
index.
Read more
1.0.0
·
Source
§
fn
max
(self) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    Self::
Item
:
Ord
,
Returns the maximum element of an iterator.
Read more
1.0.0
·
Source
§
fn
min
(self) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    Self::
Item
:
Ord
,
Returns the minimum element of an iterator.
Read more
1.6.0
·
Source
§
fn
max_by_key
<B, F>(self, f: F) ->
Option
<Self::
Item
>
where
    B:
Ord
,
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
) -> B,
Returns the element that gives the maximum value from the
specified function.
Read more
1.15.0
·
Source
§
fn
max_by
<F>(self, compare: F) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
, &Self::
Item
) ->
Ordering
,
Returns the element that gives the maximum value with respect to the
specified comparison function.
Read more
1.6.0
·
Source
§
fn
min_by_key
<B, F>(self, f: F) ->
Option
<Self::
Item
>
where
    B:
Ord
,
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
) -> B,
Returns the element that gives the minimum value from the
specified function.
Read more
1.15.0
·
Source
§
fn
min_by
<F>(self, compare: F) ->
Option
<Self::
Item
>
where
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
, &Self::
Item
) ->
Ordering
,
Returns the element that gives the minimum value with respect to the
specified comparison function.
Read more
1.0.0
·
Source
§
fn
rev
(self) ->
Rev
<Self>
ⓘ
where
    Self:
Sized
+
DoubleEndedIterator
,
Reverses an iterator’s direction.
Read more
1.0.0
·
Source
§
fn
unzip
<A, B, FromA, FromB>(self) ->
(FromA, FromB)
where
    FromA:
Default
+
Extend
<A>,
    FromB:
Default
+
Extend
<B>,
    Self:
Sized
+
Iterator
<Item =
(A, B)
>,
Converts an iterator of pairs into a pair of containers.
Read more
1.36.0
·
Source
§
fn
copied
<'a, T>(self) ->
Copied
<Self>
ⓘ
where
    T: 'a +
Copy
,
    Self:
Sized
+
Iterator
<Item =
&'a T
>,
Creates an iterator which copies all of its elements.
Read more
1.0.0
·
Source
§
fn
cloned
<'a, T>(self) ->
Cloned
<Self>
ⓘ
where
    T: 'a +
Clone
,
    Self:
Sized
+
Iterator
<Item =
&'a T
>,
Creates an iterator which
clone
s all of its elements.
Read more
1.0.0
·
Source
§
fn
cycle
(self) ->
Cycle
<Self>
ⓘ
where
    Self:
Sized
+
Clone
,
Repeats an iterator endlessly.
Read more
Source
§
fn
array_chunks
<const N:
usize
>(self) ->
ArrayChunks
<Self, N>
ⓘ
where
    Self:
Sized
,
🔬
This is a nightly-only experimental API. (
iter_array_chunks
#100450
)
Returns an iterator over
N
elements of the iterator at a time.
Read more
1.11.0
·
Source
§
fn
sum
<S>(self) -> S
where
    Self:
Sized
,
    S:
Sum
<Self::
Item
>,
Sums the elements of an iterator.
Read more
1.11.0
·
Source
§
fn
product
<P>(self) -> P
where
    Self:
Sized
,
    P:
Product
<Self::
Item
>,
Iterates over the entire iterator, multiplying all the elements
Read more
1.5.0
·
Source
§
fn
cmp
<I>(self, other: I) ->
Ordering
where
    I:
IntoIterator
<Item = Self::
Item
>,
    Self::
Item
:
Ord
,
    Self:
Sized
,
Lexicographically
compares the elements of this
Iterator
with those
of another.
Read more
Source
§
fn
cmp_by
<I, F>(self, other: I, cmp: F) ->
Ordering
where
    Self:
Sized
,
    I:
IntoIterator
,
    F:
FnMut
(Self::
Item
, <I as
IntoIterator
>::
Item
) ->
Ordering
,
🔬
This is a nightly-only experimental API. (
iter_order_by
#64295
)
Lexicographically
compares the elements of this
Iterator
with those
of another with respect to the specified comparison function.
Read more
1.5.0
·
Source
§
fn
partial_cmp
<I>(self, other: I) ->
Option
<
Ordering
>
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Lexicographically
compares the
PartialOrd
elements of
this
Iterator
with those of another. The comparison works like short-circuit
evaluation, returning a result without comparing the remaining elements.
As soon as an order can be determined, the evaluation stops and a result is returned.
Read more
Source
§
fn
partial_cmp_by
<I, F>(self, other: I, partial_cmp: F) ->
Option
<
Ordering
>
where
    Self:
Sized
,
    I:
IntoIterator
,
    F:
FnMut
(Self::
Item
, <I as
IntoIterator
>::
Item
) ->
Option
<
Ordering
>,
🔬
This is a nightly-only experimental API. (
iter_order_by
#64295
)
Lexicographically
compares the elements of this
Iterator
with those
of another with respect to the specified comparison function.
Read more
1.5.0
·
Source
§
fn
eq
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialEq
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are equal to those of
another.
Read more
Source
§
fn
eq_by
<I, F>(self, other: I, eq: F) ->
bool
where
    Self:
Sized
,
    I:
IntoIterator
,
    F:
FnMut
(Self::
Item
, <I as
IntoIterator
>::
Item
) ->
bool
,
🔬
This is a nightly-only experimental API. (
iter_order_by
#64295
)
Determines if the elements of this
Iterator
are equal to those of
another with respect to the specified equality function.
Read more
1.5.0
·
Source
§
fn
ne
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialEq
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are not equal to those of
another.
Read more
1.5.0
·
Source
§
fn
lt
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are
lexicographically
less than those of another.
Read more
1.5.0
·
Source
§
fn
le
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are
lexicographically
less or equal to those of another.
Read more
1.5.0
·
Source
§
fn
gt
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are
lexicographically
greater than those of another.
Read more
1.5.0
·
Source
§
fn
ge
<I>(self, other: I) ->
bool
where
    I:
IntoIterator
,
    Self::
Item
:
PartialOrd
<<I as
IntoIterator
>::
Item
>,
    Self:
Sized
,
Determines if the elements of this
Iterator
are
lexicographically
greater than or equal to those of another.
Read more
1.82.0
·
Source
§
fn
is_sorted
(self) ->
bool
where
    Self:
Sized
,
    Self::
Item
:
PartialOrd
,
Checks if the elements of this iterator are sorted.
Read more
1.82.0
·
Source
§
fn
is_sorted_by
<F>(self, compare: F) ->
bool
where
    Self:
Sized
,
    F:
FnMut
(&Self::
Item
, &Self::
Item
) ->
bool
,
Checks if the elements of this iterator are sorted using the given comparator function.
Read more
1.82.0
·
Source
§
fn
is_sorted_by_key
<F, K>(self, f: F) ->
bool
where
    Self:
Sized
,
    F:
FnMut
(Self::
Item
) -> K,
    K:
PartialOrd
,
Checks if the elements of this iterator are sorted using the given key extraction
function.
Read more
1.0.0
·
Source
§
impl<T, A>
Ord
for
Box
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
Box
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
Box
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
Box
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
Box
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
Box
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
Box
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
Source
§
fn
lt
(&self, other: &
Box
<T, A>) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
Source
§
fn
le
(&self, other: &
Box
<T, A>) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
Source
§
fn
ge
(&self, other: &
Box
<T, A>) ->
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
Source
§
fn
gt
(&self, other: &
Box
<T, A>) ->
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
impl<T, A>
Pointer
for
Box
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
1.0.0
·
Source
§
impl<R:
Read
+ ?
Sized
>
Read
for
Box
<R>
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
1.0.0
·
Source
§
impl<S:
Seek
+ ?
Sized
>
Seek
for
Box
<S>
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
impl<T, const N:
usize
>
TryFrom
<
Box
<
[T]
>> for
Box
<
[T; N]
>
Source
§
fn
try_from
(
    boxed_slice:
Box
<
[T]
>,
) ->
Result
<
Box
<
[T; N]
>, <
Box
<
[T; N]
> as
TryFrom
<
Box
<
[T]
>>>::
Error
>
Attempts to convert a
Box<[T]>
into a
Box<[T; N]>
.
The conversion occurs in-place and does not require a
new memory allocation.
§
Errors
Returns the old
Box<[T]>
in the
Err
variant if
boxed_slice.len()
does not equal
N
.
Source
§
type
Error
=
Box
<
[T]
>
The type returned in the event of a conversion error.
1.66.0
·
Source
§
impl<T, const N:
usize
>
TryFrom
<
Vec
<T>> for
Box
<
[T; N]
>
Source
§
fn
try_from
(
    vec:
Vec
<T>,
) ->
Result
<
Box
<
[T; N]
>, <
Box
<
[T; N]
> as
TryFrom
<
Vec
<T>>>::
Error
>
Attempts to convert a
Vec<T>
into a
Box<[T; N]>
.
Like
Vec::into_boxed_slice
, this is in-place if
vec.capacity() == N
,
but will require a reallocation otherwise.
§
Errors
Returns the original
Vec<T>
in the
Err
variant if
boxed_slice.len()
does not equal
N
.
§
Examples
This can be used with
vec!
to create an array on the heap:
let
state: Box<[f32;
100
]> =
vec!
[
1.0
;
100
].try_into().unwrap();
assert_eq!
(state.len(),
100
);
Source
§
type
Error
=
Vec
<T>
The type returned in the event of a conversion error.
1.0.0
·
Source
§
impl<W:
Write
+ ?
Sized
>
Write
for
Box
<W>
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
write_fmt
(&mut self, fmt:
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
Box
<U, A>> for
Box
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
Box
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
Box
<U>> for
Box
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
Box
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
1.26.0
·
Source
§
impl<I, A>
FusedIterator
for
Box
<I, A>
where
    I:
FusedIterator
+ ?
Sized
,
    A:
Allocator
,
1.80.0
·
Source
§
impl<'a, I, A> !
Iterator
for &'a
Box
<
[I]
, A>
where
    A:
Allocator
,
This implementation is required to make sure that the
&Box<[I]>: IntoIterator
implementation doesn’t overlap with
IntoIterator for T where T: Iterator
blanket.
1.80.0
·
Source
§
impl<'a, I, A> !
Iterator
for &'a mut
Box
<
[I]
, A>
where
    A:
Allocator
,
This implementation is required to make sure that the
&mut Box<[I]>: IntoIterator
implementation doesn’t overlap with
IntoIterator for T where T: Iterator
blanket.
1.80.0
·
Source
§
impl<I, A> !
Iterator
for
Box
<
[I]
, A>
where
    A:
Allocator
,
This implementation is required to make sure that the
Box<[I]>: IntoIterator
implementation doesn’t overlap with
IntoIterator for T where T: Iterator
blanket.
Source
§
impl<T, A>
PinCoerceUnsized
for
Box
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
impl<T>
PointerLike
for
Box
<T>
1.33.0
·
Source
§
impl<T, A>
Unpin
for
Box
<T, A>
where
    A:
Allocator
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
Box
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
Box
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
§
impl<T, A>
Send
for
Box
<T, A>
where
    A:
Send
,
    T:
Send
+ ?
Sized
,
§
impl<T, A>
Sync
for
Box
<T, A>
where
    A:
Sync
,
    T:
Sync
+ ?
Sized
,
§
impl<T, A>
UnwindSafe
for
Box
<T, A>
where
    A:
UnwindSafe
,
    T:
UnwindSafe
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
impl<I>
IntoAsyncIterator
for I
where
    I:
AsyncIterator
,
Source
§
type
Item
= <I as
AsyncIterator
>::
Item
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of the item yielded by the iterator
Source
§
type
IntoAsyncIter
= I
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
The type of the resulting iterator
Source
§
fn
into_async_iter
(self) -> <I as
IntoAsyncIterator
>::
IntoAsyncIter
🔬
This is a nightly-only experimental API. (
async_iterator
#79024
)
Converts
self
into an async iterator
Source
§
impl<F>
IntoFuture
for F
where
    F:
Future
,
Source
§
type
Output
= <F as
Future
>::
Output
The output that the future will produce on completion.
Source
§
type
IntoFuture
= F
Which kind of future are we turning this into?
Source
§
fn
into_future
(self) -> <F as
IntoFuture
>::
IntoFuture
Creates a future from a value.
Read more
Source
§
impl<I>
IntoIterator
for I
where
    I:
Iterator
,
Source
§
type
Item
= <I as
Iterator
>::
Item
The type of the elements being iterated over.
Source
§
type
IntoIter
= I
Which kind of iterator are we turning this into?
Source
§
fn
into_iter
(self) -> I
Creates an iterator from a value.
Read more
Source
§
impl<F>
Pattern
for F
where
    F:
FnMut
(
char
) ->
bool
,
Source
§
type
Searcher
<'a> =
CharPredicateSearcher
<'a, F>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Associated searcher for this pattern
Source
§
fn
into_searcher
<'a>(self, haystack: &'a
str
) ->
CharPredicateSearcher
<'a, F>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Constructs the associated searcher from
self
and the
haystack
to search in.
Source
§
fn
is_contained_in
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches anywhere in the haystack
Source
§
fn
is_prefix_of
<'a>(self, haystack: &'a
str
) ->
bool
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the front of the haystack
Source
§
fn
strip_prefix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the front of haystack, if it matches.
Source
§
fn
is_suffix_of
<'a>(self, haystack: &'a
str
) ->
bool
where
CharPredicateSearcher
<'a, F>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Checks whether the pattern matches at the back of the haystack
Source
§
fn
strip_suffix_of
<'a>(self, haystack: &'a
str
) ->
Option
<&'a
str
>
where
CharPredicateSearcher
<'a, F>:
ReverseSearcher
<'a>,
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Removes the pattern from the back of haystack, if it matches.
Source
§
fn
as_utf8_pattern
(&self) ->
Option
<
Utf8Pattern
<'_>>
🔬
This is a nightly-only experimental API. (
pattern
#27721
)
Returns the pattern as utf-8 bytes if possible.
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