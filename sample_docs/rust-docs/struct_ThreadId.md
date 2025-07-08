ThreadId in std::thread - Rust
std
::
thread
Struct
ThreadId
Copy item path
1.19.0
·
Source
pub struct ThreadId(
/* private fields */
);
Expand description
A unique identifier for a running thread.
A
ThreadId
is an opaque object that uniquely identifies each thread
created during the lifetime of a process.
ThreadId
s are guaranteed not to
be reused, even when a thread terminates.
ThreadId
s are under the control
of Rust’s standard library and there may not be any relationship between
ThreadId
and the underlying platform’s notion of a thread identifier –
the two concepts cannot, therefore, be used interchangeably. A
ThreadId
can be retrieved from the
id
method on a
Thread
.
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
Implementations
§
Source
§
impl
ThreadId
Source
pub fn
as_u64
(&self) ->
NonZero
<
u64
>
🔬
This is a nightly-only experimental API. (
thread_id_value
#67939
)
This returns a numeric identifier for the thread identified by this
ThreadId
.
As noted in the documentation for the type itself, it is essentially an
opaque ID, but is guaranteed to be unique for each thread. The returned
value is entirely opaque – only equality testing is stable. Note that
it is not guaranteed which values new threads will return, and this may
change across Rust versions.
Trait Implementations
§
1.19.0
·
Source
§
impl
Clone
for
ThreadId
Source
§
fn
clone
(&self) ->
ThreadId
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
1.19.0
·
Source
§
impl
Debug
for
ThreadId
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
1.19.0
·
Source
§
impl
Hash
for
ThreadId
Source
§
fn
hash
<__H:
Hasher
>(&self, state:
&mut __H
)
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
1.19.0
·
Source
§
impl
PartialEq
for
ThreadId
Source
§
fn
eq
(&self, other: &
ThreadId
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
1.19.0
·
Source
§
impl
Copy
for
ThreadId
1.19.0
·
Source
§
impl
Eq
for
ThreadId
1.19.0
·
Source
§
impl
StructuralPartialEq
for
ThreadId
Auto Trait Implementations
§
§
impl
Freeze
for
ThreadId
§
impl
RefUnwindSafe
for
ThreadId
§
impl
Send
for
ThreadId
§
impl
Sync
for
ThreadId
§
impl
Unpin
for
ThreadId
§
impl
UnwindSafe
for
ThreadId
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