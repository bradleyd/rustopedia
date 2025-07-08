Thread in std::thread - Rust
std
::
thread
Struct
Thread
Copy item path
1.0.0
·
Source
pub struct Thread {
/* private fields */
}
Expand description
A handle to a thread.
Threads are represented via the
Thread
type, which you can get in one of
two ways:
By spawning a new thread, e.g., using the
thread::spawn
function, and calling
thread
on the
JoinHandle
.
By requesting the current thread, using the
thread::current
function.
The
thread::current
function is available even for threads not spawned
by the APIs of this module.
There is usually no need to create a
Thread
struct yourself, one
should instead use a function like
spawn
to create new threads, see the
docs of
Builder
and
spawn
for more details.
Implementations
§
Source
§
impl
Thread
1.0.0
·
Source
pub fn
unpark
(&self)
Atomically makes the handle’s token available if it is not already.
Every thread is equipped with some basic low-level blocking support, via
the
park
function and the
unpark()
method. These can be
used as a more CPU-efficient implementation of a spinlock.
See the
park documentation
for more details.
§
Examples
use
std::thread;
use
std::time::Duration;
let
parked_thread = thread::Builder::new()
    .spawn(|| {
println!
(
"Parking thread"
);
        thread::park();
println!
(
"Thread unparked"
);
    })
    .unwrap();
// Let some time pass for the thread to be spawned.
thread::sleep(Duration::from_millis(
10
));
println!
(
"Unpark the thread"
);
parked_thread.thread().unpark();

parked_thread.join().unwrap();
1.19.0
·
Source
pub fn
id
(&self) ->
ThreadId
Gets the thread’s unique identifier.
§
Examples
use
std::thread;
let
other_thread = thread::spawn(|| {
    thread::current().id()
});
let
other_thread_id = other_thread.join().unwrap();
assert!
(thread::current().id() != other_thread_id);
1.0.0
·
Source
pub fn
name
(&self) ->
Option
<&
str
>
Gets the thread’s name.
For more information about named threads, see
this module-level documentation
.
§
Examples
Threads by default have no name specified:
use
std::thread;
let
builder = thread::Builder::new();
let
handler = builder.spawn(|| {
assert!
(thread::current().name().is_none());
}).unwrap();

handler.join().unwrap();
Thread with a specified name:
use
std::thread;
let
builder = thread::Builder::new()
    .name(
"foo"
.into());
let
handler = builder.spawn(|| {
assert_eq!
(thread::current().name(),
Some
(
"foo"
))
}).unwrap();

handler.join().unwrap();
Source
pub fn
into_raw
(self) ->
*const
()
🔬
This is a nightly-only experimental API. (
thread_raw
#97523
)
Consumes the
Thread
, returning a raw pointer.
To avoid a memory leak the pointer must be converted
back into a
Thread
using
Thread::from_raw
.
§
Examples
#![feature(thread_raw)]
use
std::thread::{
self
, Thread};
let
thread = thread::current();
let
id = thread.id();
let
ptr = Thread::into_raw(thread);
unsafe
{
assert_eq!
(Thread::from_raw(ptr).id(), id);
}
Source
pub unsafe fn
from_raw
(ptr:
*const
()
) ->
Thread
🔬
This is a nightly-only experimental API. (
thread_raw
#97523
)
Constructs a
Thread
from a raw pointer.
The raw pointer must have been previously returned
by a call to
Thread::into_raw
.
§
Safety
This function is unsafe because improper use may lead
to memory unsafety, even if the returned
Thread
is never
accessed.
Creating a
Thread
from a pointer other than one returned
from
Thread::into_raw
is
undefined behavior
.
Calling this function twice on the same raw pointer can lead
to a double-free if both
Thread
instances are dropped.
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
Thread
Source
§
fn
clone
(&self) ->
Thread
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
Thread
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
Thread
§
impl
RefUnwindSafe
for
Thread
§
impl
Send
for
Thread
§
impl
Sync
for
Thread
§
impl
Unpin
for
Thread
§
impl
UnwindSafe
for
Thread
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