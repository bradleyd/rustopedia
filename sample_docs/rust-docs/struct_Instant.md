Instant in std::time - Rust
std
::
time
Struct
Instant
Copy item path
1.8.0
·
Source
pub struct Instant(
/* private fields */
);
Expand description
A measurement of a monotonically nondecreasing clock.
Opaque and useful only with
Duration
.
Instants are always guaranteed, barring
platform bugs
, to be no less than any previously
measured instant when created, and are often useful for tasks such as measuring
benchmarks or timing how long an operation takes.
Note, however, that instants are
not
guaranteed to be
steady
. In other
words, each tick of the underlying clock might not be the same length (e.g.
some seconds may be longer than others). An instant may jump forwards or
experience time dilation (slow down or speed up), but it will never go
backwards.
As part of this non-guarantee it is also not specified whether system suspends count as
elapsed time or not. The behavior varies across platforms and Rust versions.
Instants are opaque types that can only be compared to one another. There is
no method to get “the number of seconds” from an instant. Instead, it only
allows measuring the duration between two instants (or comparing two
instants).
The size of an
Instant
struct may vary depending on the target operating
system.
Example:
use
std::time::{Duration, Instant};
use
std::thread::sleep;
fn
main() {
let
now = Instant::now();
// we sleep for 2 seconds
sleep(Duration::new(
2
,
0
));
// it prints '2'
println!
(
"{}"
, now.elapsed().as_secs());
}
§
OS-specific behaviors
An
Instant
is a wrapper around system-specific types and it may behave
differently depending on the underlying operating system. For example,
the following snippet is fine on Linux but panics on macOS:
use
std::time::{Instant, Duration};
let
now = Instant::now();
let
days_per_10_millennia =
365_2425
;
let
solar_seconds_per_day =
60
*
60
*
24
;
let
millenium_in_solar_seconds =
31_556_952_000
;
assert_eq!
(millenium_in_solar_seconds, days_per_10_millennia * solar_seconds_per_day /
10
);
let
duration = Duration::new(millenium_in_solar_seconds,
0
);
println!
(
"{:?}"
, now + duration);
For cross-platform code, you can comfortably use durations of up to around one hundred years.
§
Underlying System calls
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
clock_gettime (Monotonic Clock)
Darwin
clock_gettime (Monotonic Clock)
VXWorks
clock_gettime (Monotonic Clock)
SOLID
get_tim
WASI
__wasi_clock_time_get (Monotonic Clock)
Windows
QueryPerformanceCounter
Disclaimer:
These system calls might change over time.
Note: mathematical operations like
add
may panic if the underlying
structure cannot represent the new point in time.
§
Monotonicity
On all platforms
Instant
will try to use an OS API that guarantees monotonic behavior
if available, which is the case for all
tier 1
platforms.
In practice such guarantees are – under rare circumstances – broken by hardware, virtualization
or operating system bugs. To work around these bugs and platforms not offering monotonic clocks
duration_since
,
elapsed
and
sub
saturate to zero. In older Rust versions this
lead to a panic instead.
checked_duration_since
can be used to detect and handle situations
where monotonicity is violated, or
Instant
s are subtracted in the wrong order.
This workaround obscures programming errors where earlier and later instants are accidentally
swapped. For this reason future Rust versions may reintroduce panics.
Implementations
§
Source
§
impl
Instant
1.8.0
·
Source
pub fn
now
() ->
Instant
Returns an instant corresponding to “now”.
§
Examples
use
std::time::Instant;
let
now = Instant::now();
1.8.0
·
Source
pub fn
duration_since
(&self, earlier:
Instant
) ->
Duration
Returns the amount of time elapsed from another instant to this one,
or zero duration if that instant is later than this one.
§
Panics
Previous Rust versions panicked when
earlier
was later than
self
. Currently this
method saturates. Future versions may reintroduce the panic in some circumstances.
See
Monotonicity
.
§
Examples
use
std::time::{Duration, Instant};
use
std::thread::sleep;
let
now = Instant::now();
sleep(Duration::new(
1
,
0
));
let
new_now = Instant::now();
println!
(
"{:?}"
, new_now.duration_since(now));
println!
(
"{:?}"
, now.duration_since(new_now));
// 0ns
1.39.0
·
Source
pub fn
checked_duration_since
(&self, earlier:
Instant
) ->
Option
<
Duration
>
Returns the amount of time elapsed from another instant to this one,
or None if that instant is later than this one.
Due to
monotonicity bugs
, even under correct logical ordering of the passed
Instant
s,
this method can return
None
.
§
Examples
use
std::time::{Duration, Instant};
use
std::thread::sleep;
let
now = Instant::now();
sleep(Duration::new(
1
,
0
));
let
new_now = Instant::now();
println!
(
"{:?}"
, new_now.checked_duration_since(now));
println!
(
"{:?}"
, now.checked_duration_since(new_now));
// None
1.39.0
·
Source
pub fn
saturating_duration_since
(&self, earlier:
Instant
) ->
Duration
Returns the amount of time elapsed from another instant to this one,
or zero duration if that instant is later than this one.
§
Examples
use
std::time::{Duration, Instant};
use
std::thread::sleep;
let
now = Instant::now();
sleep(Duration::new(
1
,
0
));
let
new_now = Instant::now();
println!
(
"{:?}"
, new_now.saturating_duration_since(now));
println!
(
"{:?}"
, now.saturating_duration_since(new_now));
// 0ns
1.8.0
·
Source
pub fn
elapsed
(&self) ->
Duration
Returns the amount of time elapsed since this instant.
§
Panics
Previous Rust versions panicked when the current time was earlier than self. Currently this
method returns a Duration of zero in that case. Future versions may reintroduce the panic.
See
Monotonicity
.
§
Examples
use
std::thread::sleep;
use
std::time::{Duration, Instant};
let
instant = Instant::now();
let
three_secs = Duration::from_secs(
3
);
sleep(three_secs);
assert!
(instant.elapsed() >= three_secs);
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
Instant
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
Instant
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
Instant
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
Instant
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
Instant
Source
§
fn
add
(self, other:
Duration
) ->
Instant
§
Panics
This function may panic if the resulting point in time cannot be represented by the
underlying data structure. See
Instant::checked_add
for a version without panic.
Source
§
type
Output
=
Instant
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
Instant
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
Instant
Source
§
fn
clone
(&self) ->
Instant
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
Instant
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
Instant
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
Instant
Source
§
fn
cmp
(&self, other: &
Instant
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
Instant
Source
§
fn
eq
(&self, other: &
Instant
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
Instant
Source
§
fn
partial_cmp
(&self, other: &
Instant
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
Instant
Source
§
type
Output
=
Instant
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, other:
Duration
) ->
Instant
Performs the
-
operation.
Read more
1.8.0
·
Source
§
impl
Sub
for
Instant
Source
§
fn
sub
(self, other:
Instant
) ->
Duration
Returns the amount of time elapsed from another instant to this one,
or zero duration if that instant is later than this one.
§
Panics
Previous Rust versions panicked when
other
was later than
self
. Currently this
method saturates. Future versions may reintroduce the panic in some circumstances.
See
Monotonicity
.
Source
§
type
Output
=
Duration
The resulting type after applying the
-
operator.
1.9.0
·
Source
§
impl
SubAssign
<
Duration
> for
Instant
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
Instant
1.8.0
·
Source
§
impl
Eq
for
Instant
1.8.0
·
Source
§
impl
StructuralPartialEq
for
Instant
Auto Trait Implementations
§
§
impl
Freeze
for
Instant
§
impl
RefUnwindSafe
for
Instant
§
impl
Send
for
Instant
§
impl
Sync
for
Instant
§
impl
Unpin
for
Instant
§
impl
UnwindSafe
for
Instant
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