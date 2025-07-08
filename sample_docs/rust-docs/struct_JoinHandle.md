JoinHandle in std::thread - Rust
std
::
thread
Struct
JoinHandle
Copy item path
1.0.0
·
Source
pub struct JoinHandle<T>(
/* private fields */
);
Expand description
An owned permission to join on a thread (block on its termination).
A
JoinHandle
detaches
the associated thread when it is dropped, which
means that there is no longer any handle to the thread and no way to
join
on it.
Due to platform restrictions, it is not possible to
Clone
this
handle: the ability to join a thread is a uniquely-owned permission.
This
struct
is created by the
thread::spawn
function and the
thread::Builder::spawn
method.
§
Examples
Creation from
thread::spawn
:
use
std::thread;
let
join_handle: thread::JoinHandle<
_
> = thread::spawn(|| {
// some work here
});
Creation from
thread::Builder::spawn
:
use
std::thread;
let
builder = thread::Builder::new();
let
join_handle: thread::JoinHandle<
_
> = builder.spawn(|| {
// some work here
}).unwrap();
A thread being detached and outliving the thread that spawned it:
use
std::thread;
use
std::time::Duration;
let
original_thread = thread::spawn(|| {
let
_detached_thread = thread::spawn(|| {
// Here we sleep to make sure that the first thread returns before.
thread::sleep(Duration::from_millis(
10
));
// This will be called, even though the JoinHandle is dropped.
println!
(
"♫ Still alive ♫"
);
    });
});

original_thread.join().expect(
"The thread being joined has panicked"
);
println!
(
"Original thread is joined."
);
// We make sure that the new thread has time to run, before the main
// thread returns.
thread::sleep(Duration::from_millis(
1000
));
Implementations
§
Source
§
impl<T>
JoinHandle
<T>
1.0.0
·
Source
pub fn
thread
(&self) -> &
Thread
Extracts a handle to the underlying thread.
§
Examples
use
std::thread;
let
builder = thread::Builder::new();
let
join_handle: thread::JoinHandle<
_
> = builder.spawn(|| {
// some work here
}).unwrap();
let
thread = join_handle.thread();
println!
(
"thread id: {:?}"
, thread.id());
1.0.0
·
Source
pub fn
join
(self) ->
Result
<T>
Waits for the associated thread to finish.
This function will return immediately if the associated thread has already finished.
In terms of
atomic memory orderings
,  the completion of the associated
thread synchronizes with this function returning. In other words, all
operations performed by that thread
happen
before
all
operations that happen after
join
returns.
If the associated thread panics,
Err
is returned with the parameter given
to
panic!
(though see the Notes below).
§
Panics
This function may panic on some platforms if a thread attempts to join
itself or otherwise may create a deadlock with joining threads.
§
Examples
use
std::thread;
let
builder = thread::Builder::new();
let
join_handle: thread::JoinHandle<
_
> = builder.spawn(|| {
// some work here
}).unwrap();
join_handle.join().expect(
"Couldn't join on the associated thread"
);
§
Notes
If a “foreign” unwinding operation (e.g. an exception thrown from C++
code, or a
panic!
in Rust code compiled or linked with a different
runtime) unwinds all the way to the thread root, the process may be
aborted; see the Notes on
thread::spawn
. If the process is not
aborted, this function will return a
Result::Err
containing an opaque
type.
1.61.0
·
Source
pub fn
is_finished
(&self) ->
bool
Checks if the associated thread has finished running its main function.
is_finished
supports implementing a non-blocking join operation, by checking
is_finished
, and calling
join
if it returns
true
. This function does not block. To
block while waiting on the thread to finish, use
join
.
This might return
true
for a brief moment after the thread’s main
function has returned, but before the thread itself has stopped running.
However, once this returns
true
,
join
can be expected
to return quickly, without blocking for any significant amount of time.
Trait Implementations
§
1.63.0
·
Source
§
impl<T>
AsHandle
for
JoinHandle
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
1.9.0
·
Source
§
impl<T>
AsRawHandle
for
JoinHandle
<T>
Available on
Windows
only.
Source
§
fn
as_raw_handle
(&self) ->
RawHandle
Extracts the raw handle.
Read more
1.16.0
·
Source
§
impl<T>
Debug
for
JoinHandle
<T>
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
1.63.0
·
Source
§
impl<T>
From
<
JoinHandle
<T>> for
OwnedHandle
Available on
Windows
only.
Source
§
fn
from
(join_handle:
JoinHandle
<T>) ->
OwnedHandle
Converts to this type from the input type.
1.9.0
·
Source
§
impl<T>
IntoRawHandle
for
JoinHandle
<T>
Available on
Windows
only.
Source
§
fn
into_raw_handle
(self) ->
RawHandle
Consumes this object, returning the raw underlying handle.
Read more
1.9.0
·
Source
§
impl<T>
JoinHandleExt
for
JoinHandle
<T>
Available on
Unix
only.
Source
§
fn
as_pthread_t
(&self) ->
RawPthread
Extracts the raw pthread_t without taking ownership
Source
§
fn
into_pthread_t
(self) ->
RawPthread
Consumes the thread, returning the raw pthread_t
Read more
1.29.0
·
Source
§
impl<T>
Send
for
JoinHandle
<T>
1.29.0
·
Source
§
impl<T>
Sync
for
JoinHandle
<T>
Auto Trait Implementations
§
§
impl<T>
Freeze
for
JoinHandle
<T>
§
impl<T> !
RefUnwindSafe
for
JoinHandle
<T>
§
impl<T>
Unpin
for
JoinHandle
<T>
§
impl<T> !
UnwindSafe
for
JoinHandle
<T>
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