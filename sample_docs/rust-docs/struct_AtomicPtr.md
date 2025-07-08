AtomicPtr in std::sync::atomic - Rust
std
::
sync
::
atomic
Struct
AtomicPtr
Copy item path
1.0.0
·
Source
#[repr(C, align(8))]
pub struct AtomicPtr<T> {
/* private fields */
}
Expand description
A raw pointer type which can be safely shared between threads.
This type has the same size and bit validity as a
*mut T
.
Note
: This type is only available on platforms that support atomic
loads and stores of pointers. Its size depends on the target pointer’s size.
Implementations
§
Source
§
impl<T>
AtomicPtr
<T>
1.0.0 (const: 1.24.0)
·
Source
pub const fn
new
(p:
*mut T
) ->
AtomicPtr
<T>
Creates a new
AtomicPtr
.
§
Examples
use
std::sync::atomic::AtomicPtr;
let
ptr =
&mut
5
;
let
atomic_ptr = AtomicPtr::new(ptr);
1.75.0 (const: 1.84.0)
·
Source
pub const unsafe fn
from_ptr
<'a>(ptr:
*mut
*mut T
) -> &'a
AtomicPtr
<T>
Creates a new
AtomicPtr
from a pointer.
§
Examples
use
std::sync::atomic::{
self
, AtomicPtr};
// Get a pointer to an allocated value
let
ptr:
*mut *mut
u8 = Box::into_raw(Box::new(std::ptr::null_mut()));
assert!
(ptr.cast::<AtomicPtr<u8>>().is_aligned());

{
// Create an atomic view of the allocated value
let
atomic =
unsafe
{ AtomicPtr::from_ptr(ptr) };
// Use `atomic` for atomic operations, possibly share it with other threads
atomic.store(std::ptr::NonNull::dangling().as_ptr(), atomic::Ordering::Relaxed);
}
// It's ok to non-atomically access the value behind `ptr`,
// since the reference to the atomic ended its lifetime in the block above
assert!
(!
unsafe
{
*
ptr }.is_null());
// Deallocate the value
unsafe
{ drop(Box::from_raw(ptr)) }
§
Safety
ptr
must be aligned to
align_of::<AtomicPtr<T>>()
(note that on some platforms this
can be bigger than
align_of::<*mut T>()
).
ptr
must be
valid
for both reads and writes for the whole lifetime
'a
.
You must adhere to the
Memory model for atomic accesses
. In particular, it is not
allowed to mix atomic and non-atomic accesses, or atomic accesses of different sizes,
without synchronization.
1.15.0
·
Source
pub fn
get_mut
(&mut self) -> &mut
*mut T
Returns a mutable reference to the underlying pointer.
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
use
std::sync::atomic::{AtomicPtr, Ordering};
let
mut
data =
10
;
let
mut
atomic_ptr = AtomicPtr::new(
&mut
data);
let
mut
other_data =
5
;
*
atomic_ptr.get_mut() =
&mut
other_data;
assert_eq!
(
unsafe
{
*
atomic_ptr.load(Ordering::SeqCst) },
5
);
Source
pub fn
from_mut
(v: &mut
*mut T
) -> &mut
AtomicPtr
<T>
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Gets atomic access to a pointer.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicPtr, Ordering};
let
mut
data =
123
;
let
mut
some_ptr =
&mut
data
as
*mut
i32;
let
a = AtomicPtr::from_mut(
&mut
some_ptr);
let
mut
other_data =
456
;
a.store(
&mut
other_data, Ordering::Relaxed);
assert_eq!
(
unsafe
{
*
some_ptr },
456
);
Source
pub fn
get_mut_slice
(this: &mut [
AtomicPtr
<T>]) -> &mut [
*mut T
]
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Gets non-atomic access to a
&mut [AtomicPtr]
slice.
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
#![feature(atomic_from_mut)]
use
std::ptr::null_mut;
use
std::sync::atomic::{AtomicPtr, Ordering};
let
mut
some_ptrs = [
const
{ AtomicPtr::new(null_mut::<String>()) };
10
];
let
view:
&mut
[
*mut
String] = AtomicPtr::get_mut_slice(
&mut
some_ptrs);
assert_eq!
(view, [null_mut::<String>();
10
]);
view
    .iter_mut()
    .enumerate()
    .for_each(|(i, ptr)|
*
ptr = Box::into_raw(Box::new(
format!
(
"iteration#{i}"
))));

std::thread::scope(|s| {
for
ptr
in
&
some_ptrs {
        s.spawn(
move
|| {
let
ptr = ptr.load(Ordering::Relaxed);
assert!
(!ptr.is_null());
let
name =
unsafe
{ Box::from_raw(ptr) };
println!
(
"Hello, {name}!"
);
        });
    }
});
Source
pub fn
from_mut_slice
(v: &mut [
*mut T
]) -> &mut [
AtomicPtr
<T>]
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Gets atomic access to a slice of pointers.
§
Examples
#![feature(atomic_from_mut)]
use
std::ptr::null_mut;
use
std::sync::atomic::{AtomicPtr, Ordering};
let
mut
some_ptrs = [null_mut::<String>();
10
];
let
a =
&*
AtomicPtr::from_mut_slice(
&mut
some_ptrs);
std::thread::scope(|s| {
for
i
in
0
..a.len() {
        s.spawn(
move
|| {
let
name = Box::new(
format!
(
"thread{i}"
));
            a[i].store(Box::into_raw(name), Ordering::Relaxed);
        });
    }
});
for
p
in
some_ptrs {
assert!
(!p.is_null());
let
name =
unsafe
{ Box::from_raw(p) };
println!
(
"Hello, {name}!"
);
}
1.15.0 (const: 1.79.0)
·
Source
pub const fn
into_inner
(self) ->
*mut T
Consumes the atomic and returns the contained value.
This is safe because passing
self
by value guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
use
std::sync::atomic::AtomicPtr;
let
mut
data =
5
;
let
atomic_ptr = AtomicPtr::new(
&mut
data);
assert_eq!
(
unsafe
{
*
atomic_ptr.into_inner() },
5
);
1.0.0
·
Source
pub fn
load
(&self, order:
Ordering
) ->
*mut T
Loads a value from the pointer.
load
takes an
Ordering
argument which describes the memory ordering
of this operation. Possible values are
SeqCst
,
Acquire
and
Relaxed
.
§
Panics
Panics if
order
is
Release
or
AcqRel
.
§
Examples
use
std::sync::atomic::{AtomicPtr, Ordering};
let
ptr =
&mut
5
;
let
some_ptr = AtomicPtr::new(ptr);
let
value = some_ptr.load(Ordering::Relaxed);
1.0.0
·
Source
pub fn
store
(&self, ptr:
*mut T
, order:
Ordering
)
Stores a value into the pointer.
store
takes an
Ordering
argument which describes the memory ordering
of this operation. Possible values are
SeqCst
,
Release
and
Relaxed
.
§
Panics
Panics if
order
is
Acquire
or
AcqRel
.
§
Examples
use
std::sync::atomic::{AtomicPtr, Ordering};
let
ptr =
&mut
5
;
let
some_ptr = AtomicPtr::new(ptr);
let
other_ptr =
&mut
10
;

some_ptr.store(other_ptr, Ordering::Relaxed);
1.0.0
·
Source
pub fn
swap
(&self, ptr:
*mut T
, order:
Ordering
) ->
*mut T
Stores a value into the pointer, returning the previous value.
swap
takes an
Ordering
argument which describes the memory ordering
of this operation. All ordering modes are possible. Note that using
Acquire
makes the store part of this operation
Relaxed
, and
using
Release
makes the load part
Relaxed
.
Note:
This method is only available on platforms that support atomic
operations on pointers.
§
Examples
use
std::sync::atomic::{AtomicPtr, Ordering};
let
ptr =
&mut
5
;
let
some_ptr = AtomicPtr::new(ptr);
let
other_ptr =
&mut
10
;
let
value = some_ptr.swap(other_ptr, Ordering::Relaxed);
1.0.0
·
Source
pub fn
compare_and_swap
(
    &self,
    current:
*mut T
,
    new:
*mut T
,
    order:
Ordering
,
) ->
*mut T
👎
Deprecated since 1.50.0: Use
compare_exchange
or
compare_exchange_weak
instead
Stores a value into the pointer if the current value is the same as the
current
value.
The return value is always the previous value. If it is equal to
current
, then the value
was updated.
compare_and_swap
also takes an
Ordering
argument which describes the memory
ordering of this operation. Notice that even when using
AcqRel
, the operation
might fail and hence just perform an
Acquire
load, but not have
Release
semantics.
Using
Acquire
makes the store part of this operation
Relaxed
if it
happens, and using
Release
makes the load part
Relaxed
.
Note:
This method is only available on platforms that support atomic
operations on pointers.
§
Migrating to
compare_exchange
and
compare_exchange_weak
compare_and_swap
is equivalent to
compare_exchange
with the following mapping for
memory orderings:
Original
Success
Failure
Relaxed
Relaxed
Relaxed
Acquire
Acquire
Acquire
Release
Release
Relaxed
AcqRel
AcqRel
Acquire
SeqCst
SeqCst
SeqCst
compare_and_swap
and
compare_exchange
also differ in their return type. You can use
compare_exchange(...).unwrap_or_else(|x| x)
to recover the behavior of
compare_and_swap
,
but in most cases it is more idiomatic to check whether the return value is
Ok
or
Err
rather than to infer success vs failure based on the value that was read.
During migration, consider whether it makes sense to use
compare_exchange_weak
instead.
compare_exchange_weak
is allowed to fail spuriously even when the comparison succeeds,
which allows the compiler to generate better assembly code when the compare and swap
is used in a loop.
§
Examples
use
std::sync::atomic::{AtomicPtr, Ordering};
let
ptr =
&mut
5
;
let
some_ptr = AtomicPtr::new(ptr);
let
other_ptr =
&mut
10
;
let
value = some_ptr.compare_and_swap(ptr, other_ptr, Ordering::Relaxed);
1.10.0
·
Source
pub fn
compare_exchange
(
    &self,
    current:
*mut T
,
    new:
*mut T
,
    success:
Ordering
,
    failure:
Ordering
,
) ->
Result
<
*mut T
,
*mut T
>
Stores a value into the pointer if the current value is the same as the
current
value.
The return value is a result indicating whether the new value was written and containing
the previous value. On success this value is guaranteed to be equal to
current
.
compare_exchange
takes two
Ordering
arguments to describe the memory
ordering of this operation.
success
describes the required ordering for the
read-modify-write operation that takes place if the comparison with
current
succeeds.
failure
describes the required ordering for the load operation that takes place when
the comparison fails. Using
Acquire
as success ordering makes the store part
of this operation
Relaxed
, and using
Release
makes the successful load
Relaxed
. The failure ordering can only be
SeqCst
,
Acquire
or
Relaxed
.
Note:
This method is only available on platforms that support atomic
operations on pointers.
§
Examples
use
std::sync::atomic::{AtomicPtr, Ordering};
let
ptr =
&mut
5
;
let
some_ptr = AtomicPtr::new(ptr);
let
other_ptr =
&mut
10
;
let
value = some_ptr.compare_exchange(ptr, other_ptr,
                                      Ordering::SeqCst, Ordering::Relaxed);
1.10.0
·
Source
pub fn
compare_exchange_weak
(
    &self,
    current:
*mut T
,
    new:
*mut T
,
    success:
Ordering
,
    failure:
Ordering
,
) ->
Result
<
*mut T
,
*mut T
>
Stores a value into the pointer if the current value is the same as the
current
value.
Unlike
AtomicPtr::compare_exchange
, this function is allowed to spuriously fail even when the
comparison succeeds, which can result in more efficient code on some platforms. The
return value is a result indicating whether the new value was written and containing the
previous value.
compare_exchange_weak
takes two
Ordering
arguments to describe the memory
ordering of this operation.
success
describes the required ordering for the
read-modify-write operation that takes place if the comparison with
current
succeeds.
failure
describes the required ordering for the load operation that takes place when
the comparison fails. Using
Acquire
as success ordering makes the store part
of this operation
Relaxed
, and using
Release
makes the successful load
Relaxed
. The failure ordering can only be
SeqCst
,
Acquire
or
Relaxed
.
Note:
This method is only available on platforms that support atomic
operations on pointers.
§
Examples
use
std::sync::atomic::{AtomicPtr, Ordering};
let
some_ptr = AtomicPtr::new(
&mut
5
);
let
new =
&mut
10
;
let
mut
old = some_ptr.load(Ordering::Relaxed);
loop
{
match
some_ptr.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
Ok
(
_
) =>
break
,
Err
(x) => old = x,
    }
}
1.53.0
·
Source
pub fn
fetch_update
<F>(
    &self,
    set_order:
Ordering
,
    fetch_order:
Ordering
,
    f: F,
) ->
Result
<
*mut T
,
*mut T
>
where
    F:
FnMut
(
*mut T
) ->
Option
<
*mut T
>,
Fetches the value, and applies a function to it that returns an optional
new value. Returns a
Result
of
Ok(previous_value)
if the function
returned
Some(_)
, else
Err(previous_value)
.
Note: This may call the function multiple times if the value has been
changed from other threads in the meantime, as long as the function
returns
Some(_)
, but the function will have been applied only once to
the stored value.
fetch_update
takes two
Ordering
arguments to describe the memory
ordering of this operation. The first describes the required ordering for
when the operation finally succeeds while the second describes the
required ordering for loads. These correspond to the success and failure
orderings of
AtomicPtr::compare_exchange
respectively.
Using
Acquire
as success ordering makes the store part of this
operation
Relaxed
, and using
Release
makes the final successful
load
Relaxed
. The (failed) load ordering can only be
SeqCst
,
Acquire
or
Relaxed
.
Note:
This method is only available on platforms that support atomic
operations on pointers.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicPtr::compare_exchange_weak
, and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
use
std::sync::atomic::{AtomicPtr, Ordering};
let
ptr:
*mut
_
=
&mut
5
;
let
some_ptr = AtomicPtr::new(ptr);
let
new:
*mut
_
=
&mut
10
;
assert_eq!
(some_ptr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |
_
|
None
),
Err
(ptr));
let
result = some_ptr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
if
x == ptr {
Some
(new)
    }
else
{
None
}
});
assert_eq!
(result,
Ok
(ptr));
assert_eq!
(some_ptr.load(Ordering::SeqCst), new);
Source
pub fn
try_update
(
    &self,
    set_order:
Ordering
,
    fetch_order:
Ordering
,
    f: impl
FnMut
(
*mut T
) ->
Option
<
*mut T
>,
) ->
Result
<
*mut T
,
*mut T
>
🔬
This is a nightly-only experimental API. (
atomic_try_update
#135894
)
Fetches the value, and applies a function to it that returns an optional
new value. Returns a
Result
of
Ok(previous_value)
if the function
returned
Some(_)
, else
Err(previous_value)
.
See also:
update
.
Note: This may call the function multiple times if the value has been
changed from other threads in the meantime, as long as the function
returns
Some(_)
, but the function will have been applied only once to
the stored value.
try_update
takes two
Ordering
arguments to describe the memory
ordering of this operation. The first describes the required ordering for
when the operation finally succeeds while the second describes the
required ordering for loads. These correspond to the success and failure
orderings of
AtomicPtr::compare_exchange
respectively.
Using
Acquire
as success ordering makes the store part of this
operation
Relaxed
, and using
Release
makes the final successful
load
Relaxed
. The (failed) load ordering can only be
SeqCst
,
Acquire
or
Relaxed
.
Note:
This method is only available on platforms that support atomic
operations on pointers.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicPtr::compare_exchange_weak
, and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
#![feature(atomic_try_update)]
use
std::sync::atomic::{AtomicPtr, Ordering};
let
ptr:
*mut
_
=
&mut
5
;
let
some_ptr = AtomicPtr::new(ptr);
let
new:
*mut
_
=
&mut
10
;
assert_eq!
(some_ptr.try_update(Ordering::SeqCst, Ordering::SeqCst, |
_
|
None
),
Err
(ptr));
let
result = some_ptr.try_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
if
x == ptr {
Some
(new)
    }
else
{
None
}
});
assert_eq!
(result,
Ok
(ptr));
assert_eq!
(some_ptr.load(Ordering::SeqCst), new);
Source
pub fn
update
(
    &self,
    set_order:
Ordering
,
    fetch_order:
Ordering
,
    f: impl
FnMut
(
*mut T
) ->
*mut T
,
) ->
*mut T
🔬
This is a nightly-only experimental API. (
atomic_try_update
#135894
)
Fetches the value, applies a function to it that it return a new value.
The new value is stored and the old value is returned.
See also:
try_update
.
Note: This may call the function multiple times if the value has been changed from other threads in
the meantime, but the function will have been applied only once to the stored value.
update
takes two
Ordering
arguments to describe the memory
ordering of this operation. The first describes the required ordering for
when the operation finally succeeds while the second describes the
required ordering for loads. These correspond to the success and failure
orderings of
AtomicPtr::compare_exchange
respectively.
Using
Acquire
as success ordering makes the store part
of this operation
Relaxed
, and using
Release
makes the final successful load
Relaxed
. The (failed) load ordering can only be
SeqCst
,
Acquire
or
Relaxed
.
Note:
This method is only available on platforms that support atomic
operations on pointers.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicPtr::compare_exchange_weak
, and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
#![feature(atomic_try_update)]
use
std::sync::atomic::{AtomicPtr, Ordering};
let
ptr:
*mut
_
=
&mut
5
;
let
some_ptr = AtomicPtr::new(ptr);
let
new:
*mut
_
=
&mut
10
;
let
result = some_ptr.update(Ordering::SeqCst, Ordering::SeqCst, |
_
| new);
assert_eq!
(result, ptr);
assert_eq!
(some_ptr.load(Ordering::SeqCst), new);
Source
pub fn
fetch_ptr_add
(&self, val:
usize
, order:
Ordering
) ->
*mut T
🔬
This is a nightly-only experimental API. (
strict_provenance_atomic_ptr
#99108
)
Offsets the pointer’s address by adding
val
(in units of
T
),
returning the previous pointer.
This is equivalent to using
wrapping_add
to atomically perform the
equivalent of
ptr = ptr.wrapping_add(val);
.
This method operates in units of
T
, which means that it cannot be used
to offset the pointer by an amount which is not a multiple of
size_of::<T>()
. This can sometimes be inconvenient, as you may want to
work with a deliberately misaligned pointer. In such cases, you may use
the
fetch_byte_add
method instead.
fetch_ptr_add
takes an
Ordering
argument which describes the
memory ordering of this operation. All ordering modes are possible. Note
that using
Acquire
makes the store part of this operation
Relaxed
, and using
Release
makes the load part
Relaxed
.
Note
: This method is only available on platforms that support atomic
operations on
AtomicPtr
.
§
Examples
#![feature(strict_provenance_atomic_ptr)]
use
core::sync::atomic::{AtomicPtr, Ordering};
let
atom = AtomicPtr::<i64>::new(core::ptr::null_mut());
assert_eq!
(atom.fetch_ptr_add(
1
, Ordering::Relaxed).addr(),
0
);
// Note: units of `size_of::<i64>()`.
assert_eq!
(atom.load(Ordering::Relaxed).addr(),
8
);
Source
pub fn
fetch_ptr_sub
(&self, val:
usize
, order:
Ordering
) ->
*mut T
🔬
This is a nightly-only experimental API. (
strict_provenance_atomic_ptr
#99108
)
Offsets the pointer’s address by subtracting
val
(in units of
T
),
returning the previous pointer.
This is equivalent to using
wrapping_sub
to atomically perform the
equivalent of
ptr = ptr.wrapping_sub(val);
.
This method operates in units of
T
, which means that it cannot be used
to offset the pointer by an amount which is not a multiple of
size_of::<T>()
. This can sometimes be inconvenient, as you may want to
work with a deliberately misaligned pointer. In such cases, you may use
the
fetch_byte_sub
method instead.
fetch_ptr_sub
takes an
Ordering
argument which describes the memory
ordering of this operation. All ordering modes are possible. Note that
using
Acquire
makes the store part of this operation
Relaxed
,
and using
Release
makes the load part
Relaxed
.
Note
: This method is only available on platforms that support atomic
operations on
AtomicPtr
.
§
Examples
#![feature(strict_provenance_atomic_ptr)]
use
core::sync::atomic::{AtomicPtr, Ordering};
let
array = [
1i32
,
2i32
];
let
atom = AtomicPtr::new(array.as_ptr().wrapping_add(
1
)
as
*mut
_
);
assert!
(core::ptr::eq(
    atom.fetch_ptr_sub(
1
, Ordering::Relaxed),
&
array[
1
],
));
assert!
(core::ptr::eq(atom.load(Ordering::Relaxed),
&
array[
0
]));
Source
pub fn
fetch_byte_add
(&self, val:
usize
, order:
Ordering
) ->
*mut T
🔬
This is a nightly-only experimental API. (
strict_provenance_atomic_ptr
#99108
)
Offsets the pointer’s address by adding
val
bytes
, returning the
previous pointer.
This is equivalent to using
wrapping_byte_add
to atomically
perform
ptr = ptr.wrapping_byte_add(val)
.
fetch_byte_add
takes an
Ordering
argument which describes the
memory ordering of this operation. All ordering modes are possible. Note
that using
Acquire
makes the store part of this operation
Relaxed
, and using
Release
makes the load part
Relaxed
.
Note
: This method is only available on platforms that support atomic
operations on
AtomicPtr
.
§
Examples
#![feature(strict_provenance_atomic_ptr)]
use
core::sync::atomic::{AtomicPtr, Ordering};
let
atom = AtomicPtr::<i64>::new(core::ptr::null_mut());
assert_eq!
(atom.fetch_byte_add(
1
, Ordering::Relaxed).addr(),
0
);
// Note: in units of bytes, not `size_of::<i64>()`.
assert_eq!
(atom.load(Ordering::Relaxed).addr(),
1
);
Source
pub fn
fetch_byte_sub
(&self, val:
usize
, order:
Ordering
) ->
*mut T
🔬
This is a nightly-only experimental API. (
strict_provenance_atomic_ptr
#99108
)
Offsets the pointer’s address by subtracting
val
bytes
, returning the
previous pointer.
This is equivalent to using
wrapping_byte_sub
to atomically
perform
ptr = ptr.wrapping_byte_sub(val)
.
fetch_byte_sub
takes an
Ordering
argument which describes the
memory ordering of this operation. All ordering modes are possible. Note
that using
Acquire
makes the store part of this operation
Relaxed
, and using
Release
makes the load part
Relaxed
.
Note
: This method is only available on platforms that support atomic
operations on
AtomicPtr
.
§
Examples
#![feature(strict_provenance_atomic_ptr)]
use
core::sync::atomic::{AtomicPtr, Ordering};
let
atom = AtomicPtr::<i64>::new(core::ptr::without_provenance_mut(
1
));
assert_eq!
(atom.fetch_byte_sub(
1
, Ordering::Relaxed).addr(),
1
);
assert_eq!
(atom.load(Ordering::Relaxed).addr(),
0
);
Source
pub fn
fetch_or
(&self, val:
usize
, order:
Ordering
) ->
*mut T
🔬
This is a nightly-only experimental API. (
strict_provenance_atomic_ptr
#99108
)
Performs a bitwise “or” operation on the address of the current pointer,
and the argument
val
, and stores a pointer with provenance of the
current pointer and the resulting address.
This is equivalent to using
map_addr
to atomically perform
ptr = ptr.map_addr(|a| a | val)
. This can be used in tagged
pointer schemes to atomically set tag bits.
Caveat
: This operation returns the previous value. To compute the
stored value without losing provenance, you may use
map_addr
. For
example:
a.fetch_or(val).map_addr(|a| a | val)
.
fetch_or
takes an
Ordering
argument which describes the memory
ordering of this operation. All ordering modes are possible. Note that
using
Acquire
makes the store part of this operation
Relaxed
,
and using
Release
makes the load part
Relaxed
.
Note
: This method is only available on platforms that support atomic
operations on
AtomicPtr
.
This API and its claimed semantics are part of the Strict Provenance
experiment, see the
module documentation for
ptr
for
details.
§
Examples
#![feature(strict_provenance_atomic_ptr)]
use
core::sync::atomic::{AtomicPtr, Ordering};
let
pointer =
&mut
3i64
as
*mut
i64;
let
atom = AtomicPtr::<i64>::new(pointer);
// Tag the bottom bit of the pointer.
assert_eq!
(atom.fetch_or(
1
, Ordering::Relaxed).addr() &
1
,
0
);
// Extract and untag.
let
tagged = atom.load(Ordering::Relaxed);
assert_eq!
(tagged.addr() &
1
,
1
);
assert_eq!
(tagged.map_addr(|p| p & !
1
), pointer);
Source
pub fn
fetch_and
(&self, val:
usize
, order:
Ordering
) ->
*mut T
🔬
This is a nightly-only experimental API. (
strict_provenance_atomic_ptr
#99108
)
Performs a bitwise “and” operation on the address of the current
pointer, and the argument
val
, and stores a pointer with provenance of
the current pointer and the resulting address.
This is equivalent to using
map_addr
to atomically perform
ptr = ptr.map_addr(|a| a & val)
. This can be used in tagged
pointer schemes to atomically unset tag bits.
Caveat
: This operation returns the previous value. To compute the
stored value without losing provenance, you may use
map_addr
. For
example:
a.fetch_and(val).map_addr(|a| a & val)
.
fetch_and
takes an
Ordering
argument which describes the memory
ordering of this operation. All ordering modes are possible. Note that
using
Acquire
makes the store part of this operation
Relaxed
,
and using
Release
makes the load part
Relaxed
.
Note
: This method is only available on platforms that support atomic
operations on
AtomicPtr
.
This API and its claimed semantics are part of the Strict Provenance
experiment, see the
module documentation for
ptr
for
details.
§
Examples
#![feature(strict_provenance_atomic_ptr)]
use
core::sync::atomic::{AtomicPtr, Ordering};
let
pointer =
&mut
3i64
as
*mut
i64;
// A tagged pointer
let
atom = AtomicPtr::<i64>::new(pointer.map_addr(|a| a |
1
));
assert_eq!
(atom.fetch_or(
1
, Ordering::Relaxed).addr() &
1
,
1
);
// Untag, and extract the previously tagged pointer.
let
untagged = atom.fetch_and(!
1
, Ordering::Relaxed)
    .map_addr(|a| a & !
1
);
assert_eq!
(untagged, pointer);
Source
pub fn
fetch_xor
(&self, val:
usize
, order:
Ordering
) ->
*mut T
🔬
This is a nightly-only experimental API. (
strict_provenance_atomic_ptr
#99108
)
Performs a bitwise “xor” operation on the address of the current
pointer, and the argument
val
, and stores a pointer with provenance of
the current pointer and the resulting address.
This is equivalent to using
map_addr
to atomically perform
ptr = ptr.map_addr(|a| a ^ val)
. This can be used in tagged
pointer schemes to atomically toggle tag bits.
Caveat
: This operation returns the previous value. To compute the
stored value without losing provenance, you may use
map_addr
. For
example:
a.fetch_xor(val).map_addr(|a| a ^ val)
.
fetch_xor
takes an
Ordering
argument which describes the memory
ordering of this operation. All ordering modes are possible. Note that
using
Acquire
makes the store part of this operation
Relaxed
,
and using
Release
makes the load part
Relaxed
.
Note
: This method is only available on platforms that support atomic
operations on
AtomicPtr
.
This API and its claimed semantics are part of the Strict Provenance
experiment, see the
module documentation for
ptr
for
details.
§
Examples
#![feature(strict_provenance_atomic_ptr)]
use
core::sync::atomic::{AtomicPtr, Ordering};
let
pointer =
&mut
3i64
as
*mut
i64;
let
atom = AtomicPtr::<i64>::new(pointer);
// Toggle a tag bit on the pointer.
atom.fetch_xor(
1
, Ordering::Relaxed);
assert_eq!
(atom.load(Ordering::Relaxed).addr() &
1
,
1
);
1.70.0 (const: 1.70.0)
·
Source
pub const fn
as_ptr
(&self) ->
*mut
*mut T
Returns a mutable pointer to the underlying pointer.
Doing non-atomic reads and writes on the resulting pointer can be a data race.
This method is mostly useful for FFI, where the function signature may use
*mut *mut T
instead of
&AtomicPtr<T>
.
Returning an
*mut
pointer from a shared reference to this atomic is safe because the
atomic types work with interior mutability. All modifications of an atomic change the value
through a shared reference, and can do so safely as long as they use atomic operations. Any
use of the returned raw pointer requires an
unsafe
block and still has to uphold the same
restriction: operations on it must be atomic.
§
Examples
ⓘ
use
std::sync::atomic::AtomicPtr;
extern
"C"
{
fn
my_atomic_op(arg:
*mut *mut
u32);
}
let
mut
value =
17
;
let
atomic = AtomicPtr::new(
&mut
value);
// SAFETY: Safe as long as `my_atomic_op` is atomic.
unsafe
{
    my_atomic_op(atomic.as_ptr());
}
Trait Implementations
§
1.3.0
·
Source
§
impl<T>
Debug
for
AtomicPtr
<T>
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
AtomicPtr
<T>
Source
§
fn
default
() ->
AtomicPtr
<T>
Creates a null
AtomicPtr<T>
.
1.23.0
·
Source
§
impl<T>
From
<
*mut T
> for
AtomicPtr
<T>
Source
§
fn
from
(p:
*mut T
) ->
AtomicPtr
<T>
Converts a
*mut T
into an
AtomicPtr<T>
.
1.24.0
·
Source
§
impl<T>
Pointer
for
AtomicPtr
<T>
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
1.14.0
·
Source
§
impl<T>
RefUnwindSafe
for
AtomicPtr
<T>
1.0.0
·
Source
§
impl<T>
Send
for
AtomicPtr
<T>
1.0.0
·
Source
§
impl<T>
Sync
for
AtomicPtr
<T>
Auto Trait Implementations
§
§
impl<T> !
Freeze
for
AtomicPtr
<T>
§
impl<T>
Unpin
for
AtomicPtr
<T>
§
impl<T>
UnwindSafe
for
AtomicPtr
<T>
where
    T:
RefUnwindSafe
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