PoisonError in std::sync::poison - Rust
std
::
sync
::
poison
Struct
PoisonError
Copy item path
Source
pub struct PoisonError<T> {
/* private fields */
}
🔬
This is a nightly-only experimental API. (
sync_poison_mod
#134646
)
Expand description
A type of error which can be returned whenever a lock is acquired.
Both
Mutex
es and
RwLock
s are poisoned whenever a thread fails while the lock
is held. The precise semantics for when a lock is poisoned is documented on
each lock. For a lock in the poisoned state, unless the state is cleared manually,
all future acquisitions will return this error.
§
Examples
use
std::sync::{Arc, Mutex};
use
std::thread;
let
mutex = Arc::new(Mutex::new(
1
));
// poison the mutex
let
c_mutex = Arc::clone(
&
mutex);
let _
= thread::spawn(
move
|| {
let
mut
data = c_mutex.lock().unwrap();
*
data =
2
;
panic!
();
}).join();
match
mutex.lock() {
Ok
(
_
) =>
unreachable!
(),
Err
(p_err) => {
let
data = p_err.get_ref();
println!
(
"recovered: {data}"
);
    }
};
Implementations
§
Source
§
impl<T>
PoisonError
<T>
1.2.0
·
Source
pub fn
new
(data: T) ->
PoisonError
<T>
Creates a
PoisonError
.
This is generally created by methods like
Mutex::lock
or
RwLock::read
.
This method may panic if std was built with
panic="abort"
.
1.2.0
·
Source
pub fn
into_inner
(self) -> T
Consumes this error indicating that a lock is poisoned, returning the
associated data.
§
Examples
use
std::collections::HashSet;
use
std::sync::{Arc, Mutex};
use
std::thread;
let
mutex = Arc::new(Mutex::new(HashSet::new()));
// poison the mutex
let
c_mutex = Arc::clone(
&
mutex);
let _
= thread::spawn(
move
|| {
let
mut
data = c_mutex.lock().unwrap();
    data.insert(
10
);
panic!
();
}).join();
let
p_err = mutex.lock().unwrap_err();
let
data = p_err.into_inner();
println!
(
"recovered {} items"
, data.len());
1.2.0
·
Source
pub fn
get_ref
(&self) ->
&T
Reaches into this error indicating that a lock is poisoned, returning a
reference to the associated data.
1.2.0
·
Source
pub fn
get_mut
(&mut self) ->
&mut T
Reaches into this error indicating that a lock is poisoned, returning a
mutable reference to the associated data.
Trait Implementations
§
1.0.0
·
Source
§
impl<T>
Debug
for
PoisonError
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
1.0.0
·
Source
§
impl<T>
Display
for
PoisonError
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
1.0.0
·
Source
§
impl<T>
Error
for
PoisonError
<T>
Source
§
fn
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
Read more
1.30.0
·
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
1.0.0
·
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
provide
<'a>(&'a self, request: &mut
Request
<'a>)
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
impl<T>
From
<
PoisonError
<T>> for
TryLockError
<T>
Source
§
fn
from
(err:
PoisonError
<T>) ->
TryLockError
<T>
Converts to this type from the input type.
Auto Trait Implementations
§
§
impl<T>
Freeze
for
PoisonError
<T>
where
    T:
Freeze
,
§
impl<T>
RefUnwindSafe
for
PoisonError
<T>
where
    T:
RefUnwindSafe
,
§
impl<T>
Send
for
PoisonError
<T>
where
    T:
Send
,
§
impl<T>
Sync
for
PoisonError
<T>
where
    T:
Sync
,
§
impl<T>
Unpin
for
PoisonError
<T>
where
    T:
Unpin
,
§
impl<T>
UnwindSafe
for
PoisonError
<T>
where
    T:
UnwindSafe
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