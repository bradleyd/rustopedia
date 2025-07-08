SystemTime in std::time - Rust
std
::
time
Struct
SystemTime
Copy item path
1.8.0
·
Source
pub struct SystemTime(
/* private fields */
);
Expand description
A measurement of the system clock, useful for talking to
external entities like the file system or other processes.
Distinct from the
Instant
type, this time measurement
is not
monotonic
. This means that you can save a file to the file system, then
save another file to the file system,
and the second file has a
SystemTime
measurement earlier than the first
. In other words, an
operation that happens after another operation in real time may have an
earlier
SystemTime
!
Consequently, comparing two
SystemTime
instances to learn about the
duration between them returns a
Result
instead of an infallible
Duration
to indicate that this sort of time drift may happen and needs to be handled.
Although a
SystemTime
cannot be directly inspected, the
UNIX_EPOCH
constant is provided in this module as an anchor in time to learn
information about a
SystemTime
. By calculating the duration from this
fixed point in time, a
SystemTime
can be converted to a human-readable time,
or perhaps some other string representation.
The size of a
SystemTime
struct may vary depending on the target operating
system.
A
SystemTime
does not count leap seconds.
SystemTime::now()
’s behavior around a leap second
is the same as the operating system’s wall clock.
The precise behavior near a leap second
(e.g. whether the clock appears to run slow or fast, or stop, or jump)
depends on platform and configuration,
so should not be relied on.
Example:
use
std::time::{Duration, SystemTime};
use
std::thread::sleep;
fn
main() {
let
now = SystemTime::now();
// we sleep for 2 seconds
sleep(Duration::new(
2
,
0
));
match
now.elapsed() {
Ok
(elapsed) => {
// it prints '2'
println!
(
"{}"
, elapsed.as_secs());
       }
Err
(e) => {
// an error occurred!
println!
(
"Error: {e:?}"
);
       }
   }
}
§
Platform-specific behavior
The precision of
SystemTime
can depend on the underlying OS-specific time format.
For example, on Windows the time is represented in 100 nanosecond intervals whereas Linux
can represent nanosecond intervals.
The following system calls are
currently
being used by
now()
to find out
the current time:
Platform
System call
SGX
insecure_time
usercall
. More information on
timekeeping in SGX
UNIX
clock_gettime (Realtime Clock)
Darwin
clock_gettime (Realtime Clock)
VXWorks
clock_gettime (Realtime Clock)
SOLID
SOLID_RTC_ReadTime
WASI
__wasi_clock_time_get (Realtime Clock)
Windows
GetSystemTimePreciseAsFileTime
/
GetSystemTimeAsFileTime
Disclaimer:
These system calls might change over time.
Note: mathematical operations like
add
may panic if the underlying
structure cannot represent the new point in time.
Implementations
§
Source
§
impl
SystemTime
1.28.0
·
Source
pub const
UNIX_EPOCH
:
SystemTime
= UNIX_EPOCH
An anchor in time which can be used to create new
SystemTime
instances or
learn about where in time a
SystemTime
lies.
This constant is defined to be “1970-01-01 00:00:00 UTC” on all systems with
respect to the system clock. Using
duration_since
on an existing
SystemTime
instance can tell how far away from this point in time a
measurement lies, and using
UNIX_EPOCH + duration
can be used to create a
SystemTime
instance to represent another fixed point in time.
duration_since(UNIX_EPOCH).unwrap().as_secs()
returns
the number of non-leap seconds since the start of 1970 UTC.
This is a POSIX
time_t
(as a
u64
),
and is the same time representation as used in many Internet protocols.
§
Examples
use
std::time::SystemTime;
match
SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
Ok
(n) =>
println!
(
"1970-01-01 00:00:00 UTC was {} seconds ago!"
, n.as_secs()),
Err
(
_
) =>
panic!
(
"SystemTime before UNIX EPOCH!"
),
}
1.8.0
·
Source
pub fn
now
() ->
SystemTime
Returns the system time corresponding to “now”.
§
Examples
use
std::time::SystemTime;
let
sys_time = SystemTime::now();
1.8.0
·
Source
pub fn
duration_since
(
    &self,
    earlier:
SystemTime
,
) ->
Result
<
Duration
,
SystemTimeError
>
Returns the amount of time elapsed from an earlier point in time.
This function may fail because measurements taken earlier are not
guaranteed to always be before later measurements (due to anomalies such
as the system clock being adjusted either forwards or backwards).
Instant
can be used to measure elapsed time without this risk of failure.
If successful,
Ok
(
Duration
)
is returned where the duration represents
the amount of time elapsed from the specified measurement to this one.
Returns an
Err
if
earlier
is later than
self
, and the error
contains how far from
self
the time is.
§
Examples
use
std::time::SystemTime;
let
sys_time = SystemTime::now();
let
new_sys_time = SystemTime::now();
let
difference = new_sys_time.duration_since(sys_time)
    .expect(
"Clock may have gone backwards"
);
println!
(
"{difference:?}"
);
1.8.0
·
Source
pub fn
elapsed
(&self) ->
Result
<
Duration
,
SystemTimeError
>
Returns the difference from this system time to the
current clock time.
This function may fail as the underlying system clock is susceptible to
drift and updates (e.g., the system clock could go backwards), so this
function might not always succeed. If successful,
Ok
(
Duration
)
is
returned where the duration represents the amount of time elapsed from
this time measurement to the current time.
To measure elapsed time reliably, use
Instant
instead.
Returns an
Err
if
self
is later than the current system time, and
the error contains how far from the current system time
self
is.
§
Examples
use
std::thread::sleep;
use
std::time::{Duration, SystemTime};
let
sys_time = SystemTime::now();
let
one_sec = Duration::from_secs(
1
);
sleep(one_sec);
assert!
(sys_time.elapsed().unwrap() >= one_sec);
1.34.0
·
Source
pub fn
checked_add
(&self, duration:
Duration
) ->
Option
<
SystemTime
>
Returns
Some(t)
where
t
is the time
self + duration
if
t
can be represented as
SystemTime
(which means it’s inside the bounds of the underlying data structure),
None
otherwise.
1.34.0
·
Source
pub fn
checked_sub
(&self, duration:
Duration
) ->
Option
<
SystemTime
>
Returns
Some(t)
where
t
is the time
self - duration
if
t
can be represented as
SystemTime
(which means it’s inside the bounds of the underlying data structure),
None
otherwise.
Trait Implementations
§
1.8.0
·
Source
§
impl
Add
<
Duration
> for
SystemTime
Source
§
fn
add
(self, dur:
Duration
) ->
SystemTime
§
Panics
This function may panic if the resulting point in time cannot be represented by the
underlying data structure. See
SystemTime::checked_add
for a version without panic.
Source
§
type
Output
=
SystemTime
The resulting type after applying the
+
operator.
1.9.0
·
Source
§
impl
AddAssign
<
Duration
> for
SystemTime
Source
§
fn
add_assign
(&mut self, other:
Duration
)
Performs the
+=
operation.
Read more
1.8.0
·
Source
§
impl
Clone
for
SystemTime
Source
§
fn
clone
(&self) ->
SystemTime
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
SystemTime
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
Hash
for
SystemTime
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
1.8.0
·
Source
§
impl
Ord
for
SystemTime
Source
§
fn
cmp
(&self, other: &
SystemTime
) ->
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
1.8.0
·
Source
§
impl
PartialEq
for
SystemTime
Source
§
fn
eq
(&self, other: &
SystemTime
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
1.8.0
·
Source
§
impl
PartialOrd
for
SystemTime
Source
§
fn
partial_cmp
(&self, other: &
SystemTime
) ->
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
1.0.0
·
Source
§
fn
lt
(&self, other:
&Rhs
) ->
bool
Tests less than (for
self
and
other
) and is used by the
<
operator.
Read more
1.0.0
·
Source
§
fn
le
(&self, other:
&Rhs
) ->
bool
Tests less than or equal to (for
self
and
other
) and is used by the
<=
operator.
Read more
1.0.0
·
Source
§
fn
gt
(&self, other:
&Rhs
) ->
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
fn
ge
(&self, other:
&Rhs
) ->
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
1.8.0
·
Source
§
impl
Sub
<
Duration
> for
SystemTime
Source
§
type
Output
=
SystemTime
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, dur:
Duration
) ->
SystemTime
Performs the
-
operation.
Read more
1.9.0
·
Source
§
impl
SubAssign
<
Duration
> for
SystemTime
Source
§
fn
sub_assign
(&mut self, other:
Duration
)
Performs the
-=
operation.
Read more
1.8.0
·
Source
§
impl
Copy
for
SystemTime
1.8.0
·
Source
§
impl
Eq
for
SystemTime
1.8.0
·
Source
§
impl
StructuralPartialEq
for
SystemTime
Auto Trait Implementations
§
§
impl
Freeze
for
SystemTime
§
impl
RefUnwindSafe
for
SystemTime
§
impl
Send
for
SystemTime
§
impl
Sync
for
SystemTime
§
impl
Unpin
for
SystemTime
§
impl
UnwindSafe
for
SystemTime
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