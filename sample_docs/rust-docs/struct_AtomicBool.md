AtomicBool in std::sync::atomic - Rust
std
::
sync
::
atomic
Struct
AtomicBool
Copy item path
1.0.0
·
Source
#[repr(C, align(1))]
pub struct AtomicBool {
/* private fields */
}
Expand description
A boolean type which can be safely shared between threads.
This type has the same size, alignment, and bit validity as a
bool
.
Note
: This type is only available on platforms that support atomic
loads and stores of
u8
.
Implementations
§
Source
§
impl
AtomicBool
1.0.0 (const: 1.24.0)
·
Source
pub const fn
new
(v:
bool
) ->
AtomicBool
Creates a new
AtomicBool
.
§
Examples
use
std::sync::atomic::AtomicBool;
let
atomic_true = AtomicBool::new(
true
);
let
atomic_false = AtomicBool::new(
false
);
1.75.0 (const: 1.84.0)
·
Source
pub const unsafe fn
from_ptr
<'a>(ptr:
*mut
bool
) -> &'a
AtomicBool
Creates a new
AtomicBool
from a pointer.
§
Examples
use
std::sync::atomic::{
self
, AtomicBool};
// Get a pointer to an allocated value
let
ptr:
*mut
bool = Box::into_raw(Box::new(
false
));
assert!
(ptr.cast::<AtomicBool>().is_aligned());

{
// Create an atomic view of the allocated value
let
atomic =
unsafe
{ AtomicBool::from_ptr(ptr) };
// Use `atomic` for atomic operations, possibly share it with other threads
atomic.store(
true
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
true
);
// Deallocate the value
unsafe
{ drop(Box::from_raw(ptr)) }
§
Safety
ptr
must be aligned to
align_of::<AtomicBool>()
(note that this is always true, since
align_of::<AtomicBool>() == 1
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
bool
Returns a mutable reference to the underlying
bool
.
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
mut
some_bool = AtomicBool::new(
true
);
assert_eq!
(
*
some_bool.get_mut(),
true
);
*
some_bool.get_mut() =
false
;
assert_eq!
(some_bool.load(Ordering::SeqCst),
false
);
Source
pub fn
from_mut
(v: &mut
bool
) -> &mut
AtomicBool
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Gets atomic access to a
&mut bool
.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicBool, Ordering};
let
mut
some_bool =
true
;
let
a = AtomicBool::from_mut(
&mut
some_bool);
a.store(
false
, Ordering::Relaxed);
assert_eq!
(some_bool,
false
);
Source
pub fn
get_mut_slice
(this: &mut [
AtomicBool
]) -> &mut [
bool
]
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Gets non-atomic access to a
&mut [AtomicBool]
slice.
This is safe because the mutable reference guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicBool, Ordering};
let
mut
some_bools = [
const
{ AtomicBool::new(
false
) };
10
];
let
view:
&mut
[bool] = AtomicBool::get_mut_slice(
&mut
some_bools);
assert_eq!
(view, [
false
;
10
]);
view[..
5
].copy_from_slice(
&
[
true
;
5
]);

std::thread::scope(|s| {
for
t
in
&
some_bools[..
5
] {
        s.spawn(
move
||
assert_eq!
(t.load(Ordering::Relaxed),
true
));
    }
for
f
in
&
some_bools[
5
..] {
        s.spawn(
move
||
assert_eq!
(f.load(Ordering::Relaxed),
false
));
    }
});
Source
pub fn
from_mut_slice
(v: &mut [
bool
]) -> &mut [
AtomicBool
]
🔬
This is a nightly-only experimental API. (
atomic_from_mut
#76314
)
Gets atomic access to a
&mut [bool]
slice.
§
Examples
#![feature(atomic_from_mut)]
use
std::sync::atomic::{AtomicBool, Ordering};
let
mut
some_bools = [
false
;
10
];
let
a =
&*
AtomicBool::from_mut_slice(
&mut
some_bools);
std::thread::scope(|s| {
for
i
in
0
..a.len() {
        s.spawn(
move
|| a[i].store(
true
, Ordering::Relaxed));
    }
});
assert_eq!
(some_bools, [
true
;
10
]);
1.15.0 (const: 1.79.0)
·
Source
pub const fn
into_inner
(self) ->
bool
Consumes the atomic and returns the contained value.
This is safe because passing
self
by value guarantees that no other threads are
concurrently accessing the atomic data.
§
Examples
use
std::sync::atomic::AtomicBool;
let
some_bool = AtomicBool::new(
true
);
assert_eq!
(some_bool.into_inner(),
true
);
1.0.0
·
Source
pub fn
load
(&self, order:
Ordering
) ->
bool
Loads a value from the bool.
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
std::sync::atomic::{AtomicBool, Ordering};
let
some_bool = AtomicBool::new(
true
);
assert_eq!
(some_bool.load(Ordering::Relaxed),
true
);
1.0.0
·
Source
pub fn
store
(&self, val:
bool
, order:
Ordering
)
Stores a value into the bool.
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
std::sync::atomic::{AtomicBool, Ordering};
let
some_bool = AtomicBool::new(
true
);

some_bool.store(
false
, Ordering::Relaxed);
assert_eq!
(some_bool.load(Ordering::Relaxed),
false
);
1.0.0
·
Source
pub fn
swap
(&self, val:
bool
, order:
Ordering
) ->
bool
Stores a value into the bool, returning the previous value.
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
operations on
u8
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
some_bool = AtomicBool::new(
true
);
assert_eq!
(some_bool.swap(
false
, Ordering::Relaxed),
true
);
assert_eq!
(some_bool.load(Ordering::Relaxed),
false
);
1.0.0
·
Source
pub fn
compare_and_swap
(
    &self,
    current:
bool
,
    new:
bool
,
    order:
Ordering
,
) ->
bool
👎
Deprecated since 1.50.0: Use
compare_exchange
or
compare_exchange_weak
instead
Stores a value into the
bool
if the current value is the same as the
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
operations on
u8
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
std::sync::atomic::{AtomicBool, Ordering};
let
some_bool = AtomicBool::new(
true
);
assert_eq!
(some_bool.compare_and_swap(
true
,
false
, Ordering::Relaxed),
true
);
assert_eq!
(some_bool.load(Ordering::Relaxed),
false
);
assert_eq!
(some_bool.compare_and_swap(
true
,
true
, Ordering::Relaxed),
false
);
assert_eq!
(some_bool.load(Ordering::Relaxed),
false
);
1.10.0
·
Source
pub fn
compare_exchange
(
    &self,
    current:
bool
,
    new:
bool
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
bool
,
bool
>
Stores a value into the
bool
if the current value is the same as the
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
operations on
u8
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
some_bool = AtomicBool::new(
true
);
assert_eq!
(some_bool.compare_exchange(
true
,
false
,
                                      Ordering::Acquire,
                                      Ordering::Relaxed),
Ok
(
true
));
assert_eq!
(some_bool.load(Ordering::Relaxed),
false
);
assert_eq!
(some_bool.compare_exchange(
true
,
true
,
                                      Ordering::SeqCst,
                                      Ordering::Acquire),
Err
(
false
));
assert_eq!
(some_bool.load(Ordering::Relaxed),
false
);
1.10.0
·
Source
pub fn
compare_exchange_weak
(
    &self,
    current:
bool
,
    new:
bool
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
bool
,
bool
>
Stores a value into the
bool
if the current value is the same as the
current
value.
Unlike
AtomicBool::compare_exchange
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
operations on
u8
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
val = AtomicBool::new(
false
);
let
new =
true
;
let
mut
old = val.load(Ordering::Relaxed);
loop
{
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
1.0.0
·
Source
pub fn
fetch_and
(&self, val:
bool
, order:
Ordering
) ->
bool
Logical “and” with a boolean value.
Performs a logical “and” operation on the current value and the argument
val
, and sets
the new value to the result.
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
Note:
This method is only available on platforms that support atomic
operations on
u8
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_and(
false
, Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst),
false
);
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_and(
true
, Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst),
true
);
let
foo = AtomicBool::new(
false
);
assert_eq!
(foo.fetch_and(
false
, Ordering::SeqCst),
false
);
assert_eq!
(foo.load(Ordering::SeqCst),
false
);
1.0.0
·
Source
pub fn
fetch_nand
(&self, val:
bool
, order:
Ordering
) ->
bool
Logical “nand” with a boolean value.
Performs a logical “nand” operation on the current value and the argument
val
, and sets
the new value to the result.
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
Note:
This method is only available on platforms that support atomic
operations on
u8
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_nand(
false
, Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst),
true
);
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_nand(
true
, Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst)
as
usize,
0
);
assert_eq!
(foo.load(Ordering::SeqCst),
false
);
let
foo = AtomicBool::new(
false
);
assert_eq!
(foo.fetch_nand(
false
, Ordering::SeqCst),
false
);
assert_eq!
(foo.load(Ordering::SeqCst),
true
);
1.0.0
·
Source
pub fn
fetch_or
(&self, val:
bool
, order:
Ordering
) ->
bool
Logical “or” with a boolean value.
Performs a logical “or” operation on the current value and the argument
val
, and sets the
new value to the result.
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
Note:
This method is only available on platforms that support atomic
operations on
u8
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_or(
false
, Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst),
true
);
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_or(
true
, Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst),
true
);
let
foo = AtomicBool::new(
false
);
assert_eq!
(foo.fetch_or(
false
, Ordering::SeqCst),
false
);
assert_eq!
(foo.load(Ordering::SeqCst),
false
);
1.0.0
·
Source
pub fn
fetch_xor
(&self, val:
bool
, order:
Ordering
) ->
bool
Logical “xor” with a boolean value.
Performs a logical “xor” operation on the current value and the argument
val
, and sets
the new value to the result.
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
Note:
This method is only available on platforms that support atomic
operations on
u8
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_xor(
false
, Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst),
true
);
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_xor(
true
, Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst),
false
);
let
foo = AtomicBool::new(
false
);
assert_eq!
(foo.fetch_xor(
false
, Ordering::SeqCst),
false
);
assert_eq!
(foo.load(Ordering::SeqCst),
false
);
1.81.0
·
Source
pub fn
fetch_not
(&self, order:
Ordering
) ->
bool
Logical “not” with a boolean value.
Performs a logical “not” operation on the current value, and sets
the new value to the result.
Returns the previous value.
fetch_not
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
operations on
u8
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
foo = AtomicBool::new(
true
);
assert_eq!
(foo.fetch_not(Ordering::SeqCst),
true
);
assert_eq!
(foo.load(Ordering::SeqCst),
false
);
let
foo = AtomicBool::new(
false
);
assert_eq!
(foo.fetch_not(Ordering::SeqCst),
false
);
assert_eq!
(foo.load(Ordering::SeqCst),
true
);
1.70.0 (const: 1.70.0)
·
Source
pub const fn
as_ptr
(&self) ->
*mut
bool
Returns a mutable pointer to the underlying
bool
.
Doing non-atomic reads and writes on the resulting boolean can be a data race.
This method is mostly useful for FFI, where the function signature may use
*mut bool
instead of
&AtomicBool
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
std::sync::atomic::AtomicBool;
extern
"C"
{
fn
my_atomic_op(arg:
*mut
bool);
}
let
mut
atomic = AtomicBool::new(
true
);
unsafe
{
    my_atomic_op(atomic.as_ptr());
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
bool
,
bool
>
where
    F:
FnMut
(
bool
) ->
Option
<
bool
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
AtomicBool::compare_exchange
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
operations on
u8
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicBool::compare_exchange_weak
, and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
use
std::sync::atomic::{AtomicBool, Ordering};
let
x = AtomicBool::new(
false
);
assert_eq!
(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |
_
|
None
),
Err
(
false
));
assert_eq!
(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x|
Some
(!x)),
Ok
(
false
));
assert_eq!
(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x|
Some
(!x)),
Ok
(
true
));
assert_eq!
(x.load(Ordering::SeqCst),
false
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
bool
) ->
Option
<
bool
>,
) ->
Result
<
bool
,
bool
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
AtomicBool::compare_exchange
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
operations on
u8
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicBool::compare_exchange_weak
, and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
#![feature(atomic_try_update)]
use
std::sync::atomic::{AtomicBool, Ordering};
let
x = AtomicBool::new(
false
);
assert_eq!
(x.try_update(Ordering::SeqCst, Ordering::SeqCst, |
_
|
None
),
Err
(
false
));
assert_eq!
(x.try_update(Ordering::SeqCst, Ordering::SeqCst, |x|
Some
(!x)),
Ok
(
false
));
assert_eq!
(x.try_update(Ordering::SeqCst, Ordering::SeqCst, |x|
Some
(!x)),
Ok
(
true
));
assert_eq!
(x.load(Ordering::SeqCst),
false
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
bool
) ->
bool
,
) ->
bool
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
AtomicBool::compare_exchange
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
This method is only available on platforms that support atomic operations on
u8
.
§
Considerations
This method is not magic; it is not provided by the hardware.
It is implemented in terms of
AtomicBool::compare_exchange_weak
, and suffers from the same drawbacks.
In particular, this method will not circumvent the
ABA Problem
.
§
Examples
#![feature(atomic_try_update)]
use
std::sync::atomic::{AtomicBool, Ordering};
let
x = AtomicBool::new(
false
);
assert_eq!
(x.update(Ordering::SeqCst, Ordering::SeqCst, |x| !x),
false
);
assert_eq!
(x.update(Ordering::SeqCst, Ordering::SeqCst, |x| !x),
true
);
assert_eq!
(x.load(Ordering::SeqCst),
false
);
Trait Implementations
§
1.3.0
·
Source
§
impl
Debug
for
AtomicBool
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
Default
for
AtomicBool
Source
§
fn
default
() ->
AtomicBool
Creates an
AtomicBool
initialized to
false
.
1.24.0
·
Source
§
impl
From
<
bool
> for
AtomicBool
Source
§
fn
from
(b:
bool
) ->
AtomicBool
Converts a
bool
into an
AtomicBool
.
§
Examples
use
std::sync::atomic::AtomicBool;
let
atomic_bool = AtomicBool::from(
true
);
assert_eq!
(
format!
(
"{atomic_bool:?}"
),
"true"
)
1.14.0
·
Source
§
impl
RefUnwindSafe
for
AtomicBool
1.0.0
·
Source
§
impl
Sync
for
AtomicBool
Auto Trait Implementations
§
§
impl !
Freeze
for
AtomicBool
§
impl
Send
for
AtomicBool
§
impl
Unpin
for
AtomicBool
§
impl
UnwindSafe
for
AtomicBool
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