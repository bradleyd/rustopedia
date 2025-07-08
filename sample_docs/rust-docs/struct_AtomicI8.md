AtomicI8 in std::sync::atomic - Rust
std
::
sync
::
atomic
Struct
AtomicI8
Copy item path
1.34.0
·
Source
#[repr(C, align(1))]
pub struct AtomicI8 {
/* private fields */
}
Expand description
An integer type which can be safely shared between threads.
This type has the same
size, alignment, and bit validity
as the underlying integer type,
i8
.
For more about the differences between atomic types and
non-atomic types as well as information about the portability of
this type, please see the
module-level documentation
.
Note:
This type is only available on platforms that support
atomic loads and stores of
i8
.
Implementations
§
Source
§
impl
AtomicI8
1.34.0 (const: 1.34.0)
·
Source
pub const fn
new
(v:
i8
) ->
AtomicI8
Creates a new atomic integer.
§
Examples
use
std::sync::atomic::AtomicI8;
let
atomic_forty_two = AtomicI8::new(
42
);
1.75.0 (const: 1.84.0)
·
Source
pub const unsafe fn
from_ptr
<'a>(ptr:
*mut
i8
) -> &'a
AtomicI8
Creates a new reference to an atomic integer from a pointer.
§
Examples
use
std::sync::atomic::{
self
, AtomicI8};
// Get a pointer to an allocated value
let
ptr:
*mut
i8 = Box::into_raw(Box::new(
0
));
assert!
(ptr.cast::<AtomicI8>().is_aligned());

{
// Create an atomic view of the allocated value
let
atomic =
unsafe
{AtomicI8::from_ptr(ptr) };
// Use `atomic` for atomic operations, possibly share it with other threads
atomic.store(
1
, atomic::Ordering::Relaxed);
}
// It's ok to non-atomically access the value behind `ptr`,
// since the reference to the atomic ended its lifetime in the block above
assert_eq!
(
unsafe
{
*
ptr },
1
);
// Deallocate the value
unsafe
{ drop(Box::from_raw(ptr)) }
§
Safety
ptr
must be aligned to
align_of::<AtomicI8>()
(note that this is always true, since
align_of::<AtomicI8>() == 1
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
1.34.0
·
Source
pub fn
get_mut
(&mut self) -> &mut
i8
Returns a mutable reference to the underlying integer.
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
mut
some_var = AtomicI8::new(
10
);
assert_eq!
(
*
some_var.get_mut(),
10
);
*
some_var.get_mut() =
5
;
assert_eq!
(some_var.load(Ordering::SeqCst),
5
);
Source
pub fn
from_mut
(v: &mut
i8
) -> &mut
AtomicI8
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Get atomic access to a
&mut i8
.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicI8, Ordering};
let
mut
some_int =
123
;
let
a = AtomicI8::from_mut(
&mut
some_int);
a.store(
100
, Ordering::Relaxed);
assert_eq!
(some_int,
100
);
Source
pub fn
get_mut_slice
(this: &mut [
AtomicI8
]) -> &mut [
i8
]
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Get non-atomic access to a
&mut [AtomicI8]
slice
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicI8, Ordering};
let
mut
some_ints = [
const
{ AtomicI8::new(
0
) };
10
];
let
view:
&mut
[i8] = AtomicI8::get_mut_slice(
&mut
some_ints);
assert_eq!
(view, [
0
;
10
]);
view
    .iter_mut()
    .enumerate()
    .for_each(|(idx, int)|
*
int = idx
as _
);

std::thread::scope(|s| {
    some_ints
        .iter()
        .enumerate()
        .for_each(|(idx, int)| {
            s.spawn(
move
||
assert_eq!
(int.load(Ordering::Relaxed), idx
as _
));
        })
});
Source
pub fn
from_mut_slice
(v: &mut [
i8
]) -> &mut [
AtomicI8
]
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Get atomic access to a
&mut [i8]
slice.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicI8, Ordering};
let
mut
some_ints = [
0
;
10
];
let
a =
&*
AtomicI8::from_mut_slice(
&mut
some_ints);
std::thread::scope(|s| {
for
i
in
0
..a.len() {
        s.spawn(
move
|| a[i].store(i
as _
, Ordering::Relaxed));
    }
});
for
(i, n)
in
some_ints.into_iter().enumerate() {
assert_eq!
(i, n
as
usize);
}
1.34.0 (const: 1.79.0)
·
Source
pub const fn
into_inner
(self) ->
i8
Consumes the atomic and returns the contained value.
This is safe because passing
self
by value guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
use
std::sync::atomic::AtomicI8;
let
some_var = AtomicI8::new(
5
);
assert_eq!
(some_var.into_inner(),
5
);
1.34.0
·
Source
pub fn
load
(&self, order:
Ordering
) ->
i8
Loads a value from the atomic integer.
load
takes an
Ordering
argument which describes the memory ordering of this operation.
Possible values are
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
std::sync::atomic::{AtomicI8, Ordering};
let
some_var = AtomicI8::new(
5
);
assert_eq!
(some_var.load(Ordering::Relaxed),
5
);
1.34.0
·
Source
pub fn
store
(&self, val:
i8
, order:
Ordering
)
Stores a value into the atomic integer.
store
takes an
Ordering
argument which describes the memory ordering of this operation.
Possible values are
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
std::sync::atomic::{AtomicI8, Ordering};
let
some_var = AtomicI8::new(
5
);

some_var.store(
10
, Ordering::Relaxed);
assert_eq!
(some_var.load(Ordering::Relaxed),
10
);
1.34.0
·
Source
pub fn
swap
(&self, val:
i8
, order:
Ordering
) ->
i8
Stores a value into the atomic integer, returning the previous value.
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
some_var = AtomicI8::new(
5
);
assert_eq!
(some_var.swap(
10
, Ordering::Relaxed),
5
);
1.34.0
·
Source
pub fn
compare_and_swap
(&self, current:
i8
, new:
i8
, order:
Ordering
) ->
i8
👎
Deprecated since 1.50.0: Use
compare_exchange
or
compare_exchange_weak
instead
Stores a value into the atomic integer if the current value is the same as
the
current
value.
The return value is always the previous value. If it is equal to
current
, then the
value was updated.
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
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
std::sync::atomic::{AtomicI8, Ordering};
let
some_var = AtomicI8::new(
5
);
assert_eq!
(some_var.compare_and_swap(
5
,
10
, Ordering::Relaxed),
5
);
assert_eq!
(some_var.load(Ordering::Relaxed),
10
);
assert_eq!
(some_var.compare_and_swap(
6
,
12
, Ordering::Relaxed),
10
);
assert_eq!
(some_var.load(Ordering::Relaxed),
10
);
1.34.0
·
Source
pub fn
compare_exchange
(
    &self,
    current:
i8
,
    new:
i8
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
i8
,
i8
>
Stores a value into the atomic integer if the current value is the same as
the
current
value.
The return value is a result indicating whether the new value was written and
containing the previous value. On success this value is guaranteed to be equal to
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
some_var = AtomicI8::new(
5
);
assert_eq!
(some_var.compare_exchange(
5
,
10
,
                                     Ordering::Acquire,
                                     Ordering::Relaxed),
Ok
(
5
));
assert_eq!
(some_var.load(Ordering::Relaxed),
10
);
assert_eq!
(some_var.compare_exchange(
6
,
12
,
                                     Ordering::SeqCst,
                                     Ordering::Acquire),
Err
(
10
));
assert_eq!
(some_var.load(Ordering::Relaxed),
10
);
1.34.0
·
Source
pub fn
compare_exchange_weak
(
    &self,
    current:
i8
,
    new:
i8
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
i8
,
i8
>
Stores a value into the atomic integer if the current value is the same as
the
current
value.
Unlike
AtomicI8::compare_exchange
,
this function is allowed to spuriously fail even
when the comparison succeeds, which can result in more efficient code on some
platforms. The return value is a result indicating whether the new value was
written and containing the previous value.
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
val = AtomicI8::new(
4
);
let
mut
old = val.load(Ordering::Relaxed);
loop
{
let
new = old *
2
;
match
val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
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
1.34.0
·
Source
pub fn
fetch_add
(&self, val:
i8
, order:
Ordering
) ->
i8
Adds to the current value, returning the previous value.
This operation wraps around on overflow.
fetch_add
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
0
);
assert_eq!
(foo.fetch_add(
10
, Ordering::SeqCst),
0
);
assert_eq!
(foo.load(Ordering::SeqCst),
10
);
1.34.0
·
Source
pub fn
fetch_sub
(&self, val:
i8
, order:
Ordering
) ->
i8
Subtracts from the current value, returning the previous value.
This operation wraps around on overflow.
fetch_sub
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
20
);
assert_eq!
(foo.fetch_sub(
10
, Ordering::SeqCst),
20
);
assert_eq!
(foo.load(Ordering::SeqCst),
10
);
1.34.0
·
Source
pub fn
fetch_and
(&self, val:
i8
, order:
Ordering
) ->
i8
Bitwise “and” with the current value.
Performs a bitwise “and” operation on the current value and the argument
val
, and
sets the new value to the result.
Returns the previous value.
fetch_and
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
0b101101
);
assert_eq!
(foo.fetch_and(
0b110011
, Ordering::SeqCst),
0b101101
);
assert_eq!
(foo.load(Ordering::SeqCst),
0b100001
);
1.34.0
·
Source
pub fn
fetch_nand
(&self, val:
i8
, order:
Ordering
) ->
i8
Bitwise “nand” with the current value.
Performs a bitwise “nand” operation on the current value and the argument
val
, and
sets the new value to the result.
Returns the previous value.
fetch_nand
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
0x13
);
assert_eq!
(foo.fetch_nand(
0x31
, Ordering::SeqCst),
0x13
);
assert_eq!
(foo.load(Ordering::SeqCst), !(
0x13
&
0x31
));
1.34.0
·
Source
pub fn
fetch_or
(&self, val:
i8
, order:
Ordering
) ->
i8
Bitwise “or” with the current value.
Performs a bitwise “or” operation on the current value and the argument
val
, and
sets the new value to the result.
Returns the previous value.
fetch_or
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
0b101101
);
assert_eq!
(foo.fetch_or(
0b110011
, Ordering::SeqCst),
0b101101
);
assert_eq!
(foo.load(Ordering::SeqCst),
0b111111
);
1.34.0
·
Source
pub fn
fetch_xor
(&self, val:
i8
, order:
Ordering
) ->
i8
Bitwise “xor” with the current value.
Performs a bitwise “xor” operation on the current value and the argument
val
, and
sets the new value to the result.
Returns the previous value.
fetch_xor
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
0b101101
);
assert_eq!
(foo.fetch_xor(
0b110011
, Ordering::SeqCst),
0b101101
);
assert_eq!
(foo.load(Ordering::SeqCst),
0b011110
);
1.45.0
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
i8
,
i8
>
where
    F:
FnMut
(
i8
) ->
Option
<
i8
>,
Fetches the value, and applies a function to it that returns an optional
new value. Returns a
Result
of
Ok(previous_value)
if the function returned
Some(_)
, else
Err(previous_value)
.
Note: This may call the function multiple times if the value has been changed from other threads in
the meantime, as long as the function returns
Some(_)
, but the function will have been applied
only once to the stored value.
fetch_update
takes two
Ordering
arguments to describe the memory ordering of this operation.
The first describes the required ordering for when the operation finally succeeds while the second
describes the required ordering for loads. These correspond to the success and failure orderings of
AtomicI8::compare_exchange
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicI8::compare_exchange_weak
,
and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
x = AtomicI8::new(
7
);
assert_eq!
(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |
_
|
None
),
Err
(
7
));
assert_eq!
(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x|
Some
(x +
1
)),
Ok
(
7
));
assert_eq!
(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x|
Some
(x +
1
)),
Ok
(
8
));
assert_eq!
(x.load(Ordering::SeqCst),
9
);
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
i8
) ->
Option
<
i8
>,
) ->
Result
<
i8
,
i8
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
if the function returned
Some(_)
, else
Err(previous_value)
.
See also:
update
.
Note: This may call the function multiple times if the value has been changed from other threads in
the meantime, as long as the function returns
Some(_)
, but the function will have been applied
only once to the stored value.
try_update
takes two
Ordering
arguments to describe the memory ordering of this operation.
The first describes the required ordering for when the operation finally succeeds while the second
describes the required ordering for loads. These correspond to the success and failure orderings of
AtomicI8::compare_exchange
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicI8::compare_exchange_weak
,
and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
#![feature(atomic_try_update)]
use
std::sync::atomic::{AtomicI8, Ordering};
let
x = AtomicI8::new(
7
);
assert_eq!
(x.try_update(Ordering::SeqCst, Ordering::SeqCst, |
_
|
None
),
Err
(
7
));
assert_eq!
(x.try_update(Ordering::SeqCst, Ordering::SeqCst, |x|
Some
(x +
1
)),
Ok
(
7
));
assert_eq!
(x.try_update(Ordering::SeqCst, Ordering::SeqCst, |x|
Some
(x +
1
)),
Ok
(
8
));
assert_eq!
(x.load(Ordering::SeqCst),
9
);
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
i8
) ->
i8
,
) ->
i8
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
arguments to describe the memory ordering of this operation.
The first describes the required ordering for when the operation finally succeeds while the second
describes the required ordering for loads. These correspond to the success and failure orderings of
AtomicI8::compare_exchange
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicI8::compare_exchange_weak
,
and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
#![feature(atomic_try_update)]
use
std::sync::atomic::{AtomicI8, Ordering};
let
x = AtomicI8::new(
7
);
assert_eq!
(x.update(Ordering::SeqCst, Ordering::SeqCst, |x| x +
1
),
7
);
assert_eq!
(x.update(Ordering::SeqCst, Ordering::SeqCst, |x| x +
1
),
8
);
assert_eq!
(x.load(Ordering::SeqCst),
9
);
1.45.0
·
Source
pub fn
fetch_max
(&self, val:
i8
, order:
Ordering
) ->
i8
Maximum with the current value.
Finds the maximum of the current value and the argument
val
, and
sets the new value to the result.
Returns the previous value.
fetch_max
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
23
);
assert_eq!
(foo.fetch_max(
42
, Ordering::SeqCst),
23
);
assert_eq!
(foo.load(Ordering::SeqCst),
42
);
If you want to obtain the maximum value in one step, you can use the following:
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
23
);
let
bar =
42
;
let
max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);
assert!
(max_foo ==
42
);
1.45.0
·
Source
pub fn
fetch_min
(&self, val:
i8
, order:
Ordering
) ->
i8
Minimum with the current value.
Finds the minimum of the current value and the argument
val
, and
sets the new value to the result.
Returns the previous value.
fetch_min
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
Note
: This method is only available on platforms that support atomic operations on
i8
.
§
Examples
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
23
);
assert_eq!
(foo.fetch_min(
42
, Ordering::Relaxed),
23
);
assert_eq!
(foo.load(Ordering::Relaxed),
23
);
assert_eq!
(foo.fetch_min(
22
, Ordering::Relaxed),
23
);
assert_eq!
(foo.load(Ordering::Relaxed),
22
);
If you want to obtain the minimum value in one step, you can use the following:
use
std::sync::atomic::{AtomicI8, Ordering};
let
foo = AtomicI8::new(
23
);
let
bar =
12
;
let
min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);
assert_eq!
(min_foo,
12
);
1.70.0 (const: 1.70.0)
·
Source
pub const fn
as_ptr
(&self) ->
*mut
i8
Returns a mutable pointer to the underlying integer.
Doing non-atomic reads and writes on the resulting integer can be a data race.
This method is mostly useful for FFI, where the function signature may use
*mut i8
instead of
&AtomicI8
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
std::sync::atomic::AtomicI8;
extern
"C"
{
fn
my_atomic_op(arg:
*mut
i8);
}
let
atomic = AtomicI8::new(
1
);
// SAFETY: Safe as long as `my_atomic_op` is atomic.
unsafe
{
    my_atomic_op(atomic.as_ptr());
}
Trait Implementations
§
1.34.0
·
Source
§
impl
Debug
for
AtomicI8
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
1.34.0
·
Source
§
impl
Default
for
AtomicI8
Source
§
fn
default
() ->
AtomicI8
Returns the “default value” for a type.
Read more
1.34.0
·
Source
§
impl
From
<
i8
> for
AtomicI8
Source
§
fn
from
(v:
i8
) ->
AtomicI8
Converts an
i8
into an
AtomicI8
.
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicI8
1.34.0
·
Source
§
impl
Sync
for
AtomicI8
Auto Trait Implementations
§
§
impl !
Freeze
for
AtomicI8
§
impl
Send
for
AtomicI8
§
impl
Unpin
for
AtomicI8
§
impl
UnwindSafe
for
AtomicI8
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