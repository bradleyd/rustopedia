SystemTimeError in std::time - Rust
std
::
time
Struct
SystemTimeError
Copy item path
1.8.0
·
Source
pub struct SystemTimeError(
/* private fields */
);
Expand description
An error returned from the
duration_since
and
elapsed
methods on
SystemTime
, used to learn how far in the opposite direction a system time
lies.
§
Examples
use
std::thread::sleep;
use
std::time::{Duration, SystemTime};
let
sys_time = SystemTime::now();
sleep(Duration::from_secs(
1
));
let
new_sys_time = SystemTime::now();
match
sys_time.duration_since(new_sys_time) {
Ok
(
_
) => {}
Err
(e) =>
println!
(
"SystemTimeError difference: {:?}"
, e.duration()),
}
Implementations
§
Source
§
impl
SystemTimeError
1.8.0
·
Source
pub fn
duration
(&self) ->
Duration
Returns the positive duration which represents how far forward the
second system time was from the first.
A
SystemTimeError
is returned from the
SystemTime::duration_since
and
SystemTime::elapsed
methods whenever the second system time
represents a point later in time than the
self
of the method call.
§
Examples
use
std::thread::sleep;
use
std::time::{Duration, SystemTime};
let
sys_time = SystemTime::now();
sleep(Duration::from_secs(
1
));
let
new_sys_time = SystemTime::now();
match
sys_time.duration_since(new_sys_time) {
Ok
(
_
) => {}
Err
(e) =>
println!
(
"SystemTimeError difference: {:?}"
, e.duration()),
}
Trait Implementations
§
1.8.0
·
Source
§
impl
Clone
for
SystemTimeError
Source
§
fn
clone
(&self) ->
SystemTimeError
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
1.8.0
·
Source
§
impl
Debug
for
SystemTimeError
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
1.8.0
·
Source
§
impl
Display
for
SystemTimeError
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
1.8.0
·
Source
§
impl
Error
for
SystemTimeError
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
Auto Trait Implementations
§
§
impl
Freeze
for
SystemTimeError
§
impl
RefUnwindSafe
for
SystemTimeError
§
impl
Send
for
SystemTimeError
§
impl
Sync
for
SystemTimeError
§
impl
Unpin
for
SystemTimeError
§
impl
UnwindSafe
for
SystemTimeError
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