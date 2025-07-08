AtomicI32 in std::sync::atomic - Rust
std
::
sync
::
atomic
Struct
AtomicI32
Copy item path
1.34.0
·
Source
#[repr(C, align(4))]
pub struct AtomicI32 {
/* private fields */
}
Expand description
An integer type which can be safely shared between threads.
This type has the same
size and bit validity
as the underlying integer type,
i32
.
However, the alignment of this type is always equal to its size, even on targets where
i32
has a lesser alignment.
For more about the differences between atomic types and
non-atomic types as well as information about the portability of
this type, please see the
module-level documentation
.
Note:
This type is only available on platforms that support
atomic loads and stores of
i32
.
Implementations
§
Source
§
impl
AtomicI32
1.34.0 (const: 1.34.0)
·
Source
pub const fn
new
(v:
i32
) ->
AtomicI32
Creates a new atomic integer.
§
Examples
use
std::sync::atomic::AtomicI32;
let
atomic_forty_two = AtomicI32::new(
42
);
1.75.0 (const: 1.84.0)
·
Source
pub const unsafe fn
from_ptr
<'a>(ptr:
*mut
i32
) -> &'a
AtomicI32
Creates a new reference to an atomic integer from a pointer.
§
Examples
use
std::sync::atomic::{
self
, AtomicI32};
// Get a pointer to an allocated value
let
ptr:
*mut
i32 = Box::into_raw(Box::new(
0
));
assert!
(ptr.cast::<AtomicI32>().is_aligned());

{
// Create an atomic view of the allocated value
let
atomic =
unsafe
{AtomicI32::from_ptr(ptr) };
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
align_of::<AtomicI32>()
(note that on some platforms this can be bigger than
align_of::<i32>()
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
i32
Returns a mutable reference to the underlying integer.
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
mut
some_var = AtomicI32::new(
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
i32
) -> &mut
AtomicI32
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Get atomic access to a
&mut i32
.
Note:
This function is only available on targets where
AtomicI32
has the same alignment as
i32
.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicI32, Ordering};
let
mut
some_int =
123
;
let
a = AtomicI32::from_mut(
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
AtomicI32
]) -> &mut [
i32
]
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Get non-atomic access to a
&mut [AtomicI32]
slice
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicI32, Ordering};
let
mut
some_ints = [
const
{ AtomicI32::new(
0
) };
10
];
let
view:
&mut
[i32] = AtomicI32::get_mut_slice(
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
i32
]) -> &mut [
AtomicI32
]
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Get atomic access to a
&mut [i32]
slice.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicI32, Ordering};
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
AtomicI32::from_mut_slice(
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
i32
Consumes the atomic and returns the contained value.
This is safe because passing
self
by value guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
use
std::sync::atomic::AtomicI32;
let
some_var = AtomicI32::new(
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
i32
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
std::sync::atomic::{AtomicI32, Ordering};
let
some_var = AtomicI32::new(
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
i32
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
std::sync::atomic::{AtomicI32, Ordering};
let
some_var = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
some_var = AtomicI32::new(
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
i32
, new:
i32
, order:
Ordering
) ->
i32
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
i32
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
std::sync::atomic::{AtomicI32, Ordering};
let
some_var = AtomicI32::new(
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
i32
,
    new:
i32
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
i32
,
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
some_var = AtomicI32::new(
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
i32
,
    new:
i32
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
i32
,
i32
>
Stores a value into the atomic integer if the current value is the same as
the
current
value.
Unlike
AtomicI32::compare_exchange
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
val = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
i32
,
i32
>
where
    F:
FnMut
(
i32
) ->
Option
<
i32
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
AtomicI32::compare_exchange
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
i32
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicI32::compare_exchange_weak
,
and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
x = AtomicI32::new(
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
i32
) ->
Option
<
i32
>,
) ->
Result
<
i32
,
i32
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
AtomicI32::compare_exchange
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
i32
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicI32::compare_exchange_weak
,
and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
#![feature(atomic_try_update)]
use
std::sync::atomic::{AtomicI32, Ordering};
let
x = AtomicI32::new(
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
i32
) ->
i32
,
) ->
i32
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
AtomicI32::compare_exchange
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
i32
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicI32::compare_exchange_weak
,
and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
#![feature(atomic_try_update)]
use
std::sync::atomic::{AtomicI32, Ordering};
let
x = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
i32
, order:
Ordering
) ->
i32
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
i32
.
§
Examples
use
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
std::sync::atomic::{AtomicI32, Ordering};
let
foo = AtomicI32::new(
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
i32
Returns a mutable pointer to the underlying integer.
Doing non-atomic reads and writes on the resulting integer can be a data race.
This method is mostly useful for FFI, where the function signature may use
*mut i32
instead of
&AtomicI32
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
std::sync::atomic::AtomicI32;
extern
"C"
{
fn
my_atomic_op(arg:
*mut
i32);
}
let
atomic = AtomicI32::new(
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
AtomicI32
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
AtomicI32
Source
§
fn
default
() ->
AtomicI32
Returns the “default value” for a type.
Read more
1.34.0
·
Source
§
impl
From
<
i32
> for
AtomicI32
Source
§
fn
from
(v:
i32
) ->
AtomicI32
Converts an
i32
into an
AtomicI32
.
1.34.0
·
Source
§
impl
RefUnwindSafe
for
AtomicI32
1.34.0
·
Source
§
impl
Sync
for
AtomicI32
Auto Trait Implementations
§
§
impl !
Freeze
for
AtomicI32
§
impl
Send
for
AtomicI32
§
impl
Unpin
for
AtomicI32
§
impl
UnwindSafe
for
AtomicI32
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