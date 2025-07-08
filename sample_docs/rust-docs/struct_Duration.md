Duration in std::time - Rust
std
::
time
Struct
Duration
Copy item path
1.3.0
·
Source
pub struct Duration {
/* private fields */
}
Expand description
A
Duration
type to represent a span of time, typically used for system
timeouts.
Each
Duration
is composed of a whole number of seconds and a fractional part
represented in nanoseconds. If the underlying system does not support
nanosecond-level precision, APIs binding a system timeout will typically round up
the number of nanoseconds.
Duration
s implement many common traits, including
Add
,
Sub
, and other
ops
traits. It implements
Default
by returning a zero-length
Duration
.
§
Examples
use
std::time::Duration;
let
five_seconds = Duration::new(
5
,
0
);
let
five_seconds_and_five_nanos = five_seconds + Duration::new(
0
,
5
);
assert_eq!
(five_seconds_and_five_nanos.as_secs(),
5
);
assert_eq!
(five_seconds_and_five_nanos.subsec_nanos(),
5
);
let
ten_millis = Duration::from_millis(
10
);
§
Formatting
Duration
values
Duration
intentionally does not have a
Display
impl, as there are a
variety of ways to format spans of time for human readability.
Duration
provides a
Debug
impl that shows the full precision of the value.
The
Debug
output uses the non-ASCII “µs” suffix for microseconds. If your
program output may appear in contexts that cannot rely on full Unicode
compatibility, you may wish to format
Duration
objects yourself or use a
crate to do so.
Implementations
§
Source
§
impl
Duration
Source
pub const
SECOND
:
Duration
🔬
This is a nightly-only experimental API. (
duration_constants
#57391
)
The duration of one second.
§
Examples
#![feature(duration_constants)]
use
std::time::Duration;
assert_eq!
(Duration::SECOND, Duration::from_secs(
1
));
Source
pub const
MILLISECOND
:
Duration
🔬
This is a nightly-only experimental API. (
duration_constants
#57391
)
The duration of one millisecond.
§
Examples
#![feature(duration_constants)]
use
std::time::Duration;
assert_eq!
(Duration::MILLISECOND, Duration::from_millis(
1
));
Source
pub const
MICROSECOND
:
Duration
🔬
This is a nightly-only experimental API. (
duration_constants
#57391
)
The duration of one microsecond.
§
Examples
#![feature(duration_constants)]
use
std::time::Duration;
assert_eq!
(Duration::MICROSECOND, Duration::from_micros(
1
));
Source
pub const
NANOSECOND
:
Duration
🔬
This is a nightly-only experimental API. (
duration_constants
#57391
)
The duration of one nanosecond.
§
Examples
#![feature(duration_constants)]
use
std::time::Duration;
assert_eq!
(Duration::NANOSECOND, Duration::from_nanos(
1
));
1.53.0
·
Source
pub const
ZERO
:
Duration
A duration of zero time.
§
Examples
use
std::time::Duration;
let
duration = Duration::ZERO;
assert!
(duration.is_zero());
assert_eq!
(duration.as_nanos(),
0
);
1.53.0
·
Source
pub const
MAX
:
Duration
The maximum duration.
May vary by platform as necessary. Must be able to contain the difference between
two instances of
Instant
or two instances of
SystemTime
.
This constraint gives it a value of about 584,942,417,355 years in practice,
which is currently used on all platforms.
§
Examples
use
std::time::Duration;
assert_eq!
(Duration::MAX, Duration::new(u64::MAX,
1_000_000_000
-
1
));
1.3.0 (const: 1.58.0)
·
Source
pub const fn
new
(secs:
u64
, nanos:
u32
) ->
Duration
Creates a new
Duration
from the specified number of whole seconds and
additional nanoseconds.
If the number of nanoseconds is greater than 1 billion (the number of
nanoseconds in a second), then it will carry over into the seconds provided.
§
Panics
This constructor will panic if the carry from the nanoseconds overflows
the seconds counter.
§
Examples
use
std::time::Duration;
let
five_seconds = Duration::new(
5
,
0
);
1.3.0 (const: 1.32.0)
·
Source
pub const fn
from_secs
(secs:
u64
) ->
Duration
Creates a new
Duration
from the specified number of whole seconds.
§
Examples
use
std::time::Duration;
let
duration = Duration::from_secs(
5
);
assert_eq!
(
5
, duration.as_secs());
assert_eq!
(
0
, duration.subsec_nanos());
1.3.0 (const: 1.32.0)
·
Source
pub const fn
from_millis
(millis:
u64
) ->
Duration
Creates a new
Duration
from the specified number of milliseconds.
§
Examples
use
std::time::Duration;
let
duration = Duration::from_millis(
2_569
);
assert_eq!
(
2
, duration.as_secs());
assert_eq!
(
569_000_000
, duration.subsec_nanos());
1.27.0 (const: 1.32.0)
·
Source
pub const fn
from_micros
(micros:
u64
) ->
Duration
Creates a new
Duration
from the specified number of microseconds.
§
Examples
use
std::time::Duration;
let
duration = Duration::from_micros(
1_000_002
);
assert_eq!
(
1
, duration.as_secs());
assert_eq!
(
2_000
, duration.subsec_nanos());
1.27.0 (const: 1.32.0)
·
Source
pub const fn
from_nanos
(nanos:
u64
) ->
Duration
Creates a new
Duration
from the specified number of nanoseconds.
Note: Using this on the return value of
as_nanos()
might cause unexpected behavior:
as_nanos()
returns a u128, and can return values that do not fit in u64, e.g. 585 years.
Instead, consider using the pattern
Duration::new(d.as_secs(), d.subsec_nanos())
if you cannot copy/clone the Duration directly.
§
Examples
use
std::time::Duration;
let
duration = Duration::from_nanos(
1_000_000_123
);
assert_eq!
(
1
, duration.as_secs());
assert_eq!
(
123
, duration.subsec_nanos());
Source
pub const fn
from_weeks
(weeks:
u64
) ->
Duration
🔬
This is a nightly-only experimental API. (
duration_constructors
#120301
)
Creates a new
Duration
from the specified number of weeks.
§
Panics
Panics if the given number of weeks overflows the
Duration
size.
§
Examples
#![feature(duration_constructors)]
use
std::time::Duration;
let
duration = Duration::from_weeks(
4
);
assert_eq!
(
4
*
7
*
24
*
60
*
60
, duration.as_secs());
assert_eq!
(
0
, duration.subsec_nanos());
Source
pub const fn
from_days
(days:
u64
) ->
Duration
🔬
This is a nightly-only experimental API. (
duration_constructors
#120301
)
Creates a new
Duration
from the specified number of days.
§
Panics
Panics if the given number of days overflows the
Duration
size.
§
Examples
#![feature(duration_constructors)]
use
std::time::Duration;
let
duration = Duration::from_days(
7
);
assert_eq!
(
7
*
24
*
60
*
60
, duration.as_secs());
assert_eq!
(
0
, duration.subsec_nanos());
Source
pub const fn
from_hours
(hours:
u64
) ->
Duration
🔬
This is a nightly-only experimental API. (
duration_constructors
#120301
)
Creates a new
Duration
from the specified number of hours.
§
Panics
Panics if the given number of hours overflows the
Duration
size.
§
Examples
#![feature(duration_constructors)]
use
std::time::Duration;
let
duration = Duration::from_hours(
6
);
assert_eq!
(
6
*
60
*
60
, duration.as_secs());
assert_eq!
(
0
, duration.subsec_nanos());
Source
pub const fn
from_mins
(mins:
u64
) ->
Duration
🔬
This is a nightly-only experimental API. (
duration_constructors
#120301
)
Creates a new
Duration
from the specified number of minutes.
§
Panics
Panics if the given number of minutes overflows the
Duration
size.
§
Examples
#![feature(duration_constructors)]
use
std::time::Duration;
let
duration = Duration::from_mins(
10
);
assert_eq!
(
10
*
60
, duration.as_secs());
assert_eq!
(
0
, duration.subsec_nanos());
1.53.0 (const: 1.53.0)
·
Source
pub const fn
is_zero
(&self) ->
bool
Returns true if this
Duration
spans no time.
§
Examples
use
std::time::Duration;
assert!
(Duration::ZERO.is_zero());
assert!
(Duration::new(
0
,
0
).is_zero());
assert!
(Duration::from_nanos(
0
).is_zero());
assert!
(Duration::from_secs(
0
).is_zero());
assert!
(!Duration::new(
1
,
1
).is_zero());
assert!
(!Duration::from_nanos(
1
).is_zero());
assert!
(!Duration::from_secs(
1
).is_zero());
1.3.0 (const: 1.32.0)
·
Source
pub const fn
as_secs
(&self) ->
u64
Returns the number of
whole
seconds contained by this
Duration
.
The returned value does not include the fractional (nanosecond) part of the
duration, which can be obtained using
subsec_nanos
.
§
Examples
use
std::time::Duration;
let
duration = Duration::new(
5
,
730_023_852
);
assert_eq!
(duration.as_secs(),
5
);
To determine the total number of seconds represented by the
Duration
including the fractional part, use
as_secs_f64
or
as_secs_f32
1.27.0 (const: 1.32.0)
·
Source
pub const fn
subsec_millis
(&self) ->
u32
Returns the fractional part of this
Duration
, in whole milliseconds.
This method does
not
return the length of the duration when
represented by milliseconds. The returned number always represents a
fractional portion of a second (i.e., it is less than one thousand).
§
Examples
use
std::time::Duration;
let
duration = Duration::from_millis(
5_432
);
assert_eq!
(duration.as_secs(),
5
);
assert_eq!
(duration.subsec_millis(),
432
);
1.27.0 (const: 1.32.0)
·
Source
pub const fn
subsec_micros
(&self) ->
u32
Returns the fractional part of this
Duration
, in whole microseconds.
This method does
not
return the length of the duration when
represented by microseconds. The returned number always represents a
fractional portion of a second (i.e., it is less than one million).
§
Examples
use
std::time::Duration;
let
duration = Duration::from_micros(
1_234_567
);
assert_eq!
(duration.as_secs(),
1
);
assert_eq!
(duration.subsec_micros(),
234_567
);
1.3.0 (const: 1.32.0)
·
Source
pub const fn
subsec_nanos
(&self) ->
u32
Returns the fractional part of this
Duration
, in nanoseconds.
This method does
not
return the length of the duration when
represented by nanoseconds. The returned number always represents a
fractional portion of a second (i.e., it is less than one billion).
§
Examples
use
std::time::Duration;
let
duration = Duration::from_millis(
5_010
);
assert_eq!
(duration.as_secs(),
5
);
assert_eq!
(duration.subsec_nanos(),
10_000_000
);
1.33.0 (const: 1.33.0)
·
Source
pub const fn
as_millis
(&self) ->
u128
Returns the total number of whole milliseconds contained by this
Duration
.
§
Examples
use
std::time::Duration;
let
duration = Duration::new(
5
,
730_023_852
);
assert_eq!
(duration.as_millis(),
5_730
);
1.33.0 (const: 1.33.0)
·
Source
pub const fn
as_micros
(&self) ->
u128
Returns the total number of whole microseconds contained by this
Duration
.
§
Examples
use
std::time::Duration;
let
duration = Duration::new(
5
,
730_023_852
);
assert_eq!
(duration.as_micros(),
5_730_023
);
1.33.0 (const: 1.33.0)
·
Source
pub const fn
as_nanos
(&self) ->
u128
Returns the total number of nanoseconds contained by this
Duration
.
§
Examples
use
std::time::Duration;
let
duration = Duration::new(
5
,
730_023_852
);
assert_eq!
(duration.as_nanos(),
5_730_023_852
);
1.81.0 (const: 1.81.0)
·
Source
pub const fn
abs_diff
(self, other:
Duration
) ->
Duration
Computes the absolute difference between
self
and
other
.
§
Examples
use
std::time::Duration;
assert_eq!
(Duration::new(
100
,
0
).abs_diff(Duration::new(
80
,
0
)), Duration::new(
20
,
0
));
assert_eq!
(Duration::new(
100
,
400_000_000
).abs_diff(Duration::new(
110
,
0
)), Duration::new(
9
,
600_000_000
));
1.16.0 (const: 1.58.0)
·
Source
pub const fn
checked_add
(self, rhs:
Duration
) ->
Option
<
Duration
>
Checked
Duration
addition. Computes
self + other
, returning
None
if overflow occurred.
§
Examples
use
std::time::Duration;
assert_eq!
(Duration::new(
0
,
0
).checked_add(Duration::new(
0
,
1
)),
Some
(Duration::new(
0
,
1
)));
assert_eq!
(Duration::new(
1
,
0
).checked_add(Duration::new(u64::MAX,
0
)),
None
);
1.53.0 (const: 1.58.0)
·
Source
pub const fn
saturating_add
(self, rhs:
Duration
) ->
Duration
Saturating
Duration
addition. Computes
self + other
, returning
Duration::MAX
if overflow occurred.
§
Examples
#![feature(duration_constants)]
use
std::time::Duration;
assert_eq!
(Duration::new(
0
,
0
).saturating_add(Duration::new(
0
,
1
)), Duration::new(
0
,
1
));
assert_eq!
(Duration::new(
1
,
0
).saturating_add(Duration::new(u64::MAX,
0
)), Duration::MAX);
1.16.0 (const: 1.58.0)
·
Source
pub const fn
checked_sub
(self, rhs:
Duration
) ->
Option
<
Duration
>
Checked
Duration
subtraction. Computes
self - other
, returning
None
if the result would be negative or if overflow occurred.
§
Examples
use
std::time::Duration;
assert_eq!
(Duration::new(
0
,
1
).checked_sub(Duration::new(
0
,
0
)),
Some
(Duration::new(
0
,
1
)));
assert_eq!
(Duration::new(
0
,
0
).checked_sub(Duration::new(
0
,
1
)),
None
);
1.53.0 (const: 1.58.0)
·
Source
pub const fn
saturating_sub
(self, rhs:
Duration
) ->
Duration
Saturating
Duration
subtraction. Computes
self - other
, returning
Duration::ZERO
if the result would be negative or if overflow occurred.
§
Examples
use
std::time::Duration;
assert_eq!
(Duration::new(
0
,
1
).saturating_sub(Duration::new(
0
,
0
)), Duration::new(
0
,
1
));
assert_eq!
(Duration::new(
0
,
0
).saturating_sub(Duration::new(
0
,
1
)), Duration::ZERO);
1.16.0 (const: 1.58.0)
·
Source
pub const fn
checked_mul
(self, rhs:
u32
) ->
Option
<
Duration
>
Checked
Duration
multiplication. Computes
self * other
, returning
None
if overflow occurred.
§
Examples
use
std::time::Duration;
assert_eq!
(Duration::new(
0
,
500_000_001
).checked_mul(
2
),
Some
(Duration::new(
1
,
2
)));
assert_eq!
(Duration::new(u64::MAX -
1
,
0
).checked_mul(
2
),
None
);
1.53.0 (const: 1.58.0)
·
Source
pub const fn
saturating_mul
(self, rhs:
u32
) ->
Duration
Saturating
Duration
multiplication. Computes
self * other
, returning
Duration::MAX
if overflow occurred.
§
Examples
#![feature(duration_constants)]
use
std::time::Duration;
assert_eq!
(Duration::new(
0
,
500_000_001
).saturating_mul(
2
), Duration::new(
1
,
2
));
assert_eq!
(Duration::new(u64::MAX -
1
,
0
).saturating_mul(
2
), Duration::MAX);
1.16.0 (const: 1.58.0)
·
Source
pub const fn
checked_div
(self, rhs:
u32
) ->
Option
<
Duration
>
Checked
Duration
division. Computes
self / other
, returning
None
if
other == 0
.
§
Examples
use
std::time::Duration;
assert_eq!
(Duration::new(
2
,
0
).checked_div(
2
),
Some
(Duration::new(
1
,
0
)));
assert_eq!
(Duration::new(
1
,
0
).checked_div(
2
),
Some
(Duration::new(
0
,
500_000_000
)));
assert_eq!
(Duration::new(
2
,
0
).checked_div(
0
),
None
);
1.38.0 (const: 1.83.0)
·
Source
pub const fn
as_secs_f64
(&self) ->
f64
Returns the number of seconds contained by this
Duration
as
f64
.
The returned value includes the fractional (nanosecond) part of the duration.
§
Examples
use
std::time::Duration;
let
dur = Duration::new(
2
,
700_000_000
);
assert_eq!
(dur.as_secs_f64(),
2.7
);
1.38.0 (const: 1.83.0)
·
Source
pub const fn
as_secs_f32
(&self) ->
f32
Returns the number of seconds contained by this
Duration
as
f32
.
The returned value includes the fractional (nanosecond) part of the duration.
§
Examples
use
std::time::Duration;
let
dur = Duration::new(
2
,
700_000_000
);
assert_eq!
(dur.as_secs_f32(),
2.7
);
Source
pub const fn
as_millis_f64
(&self) ->
f64
🔬
This is a nightly-only experimental API. (
duration_millis_float
#122451
)
Returns the number of milliseconds contained by this
Duration
as
f64
.
The returned value includes the fractional (nanosecond) part of the duration.
§
Examples
#![feature(duration_millis_float)]
use
std::time::Duration;
let
dur = Duration::new(
2
,
345_678_000
);
assert_eq!
(dur.as_millis_f64(),
2_345.678
);
Source
pub const fn
as_millis_f32
(&self) ->
f32
🔬
This is a nightly-only experimental API. (
duration_millis_float
#122451
)
Returns the number of milliseconds contained by this
Duration
as
f32
.
The returned value includes the fractional (nanosecond) part of the duration.
§
Examples
#![feature(duration_millis_float)]
use
std::time::Duration;
let
dur = Duration::new(
2
,
345_678_000
);
assert_eq!
(dur.as_millis_f32(),
2_345.678
);
1.38.0
·
Source
pub fn
from_secs_f64
(secs:
f64
) ->
Duration
Creates a new
Duration
from the specified number of seconds represented
as
f64
.
§
Panics
This constructor will panic if
secs
is negative, overflows
Duration
or not finite.
§
Examples
use
std::time::Duration;
let
res = Duration::from_secs_f64(
0.0
);
assert_eq!
(res, Duration::new(
0
,
0
));
let
res = Duration::from_secs_f64(
1e-20
);
assert_eq!
(res, Duration::new(
0
,
0
));
let
res = Duration::from_secs_f64(
4.2e-7
);
assert_eq!
(res, Duration::new(
0
,
420
));
let
res = Duration::from_secs_f64(
2.7
);
assert_eq!
(res, Duration::new(
2
,
700_000_000
));
let
res = Duration::from_secs_f64(
3e10
);
assert_eq!
(res, Duration::new(
30_000_000_000
,
0
));
// subnormal float
let
res = Duration::from_secs_f64(f64::from_bits(
1
));
assert_eq!
(res, Duration::new(
0
,
0
));
// conversion uses rounding
let
res = Duration::from_secs_f64(
0.999e-9
);
assert_eq!
(res, Duration::new(
0
,
1
));
1.38.0
·
Source
pub fn
from_secs_f32
(secs:
f32
) ->
Duration
Creates a new
Duration
from the specified number of seconds represented
as
f32
.
§
Panics
This constructor will panic if
secs
is negative, overflows
Duration
or not finite.
§
Examples
use
std::time::Duration;
let
res = Duration::from_secs_f32(
0.0
);
assert_eq!
(res, Duration::new(
0
,
0
));
let
res = Duration::from_secs_f32(
1e-20
);
assert_eq!
(res, Duration::new(
0
,
0
));
let
res = Duration::from_secs_f32(
4.2e-7
);
assert_eq!
(res, Duration::new(
0
,
420
));
let
res = Duration::from_secs_f32(
2.7
);
assert_eq!
(res, Duration::new(
2
,
700_000_048
));
let
res = Duration::from_secs_f32(
3e10
);
assert_eq!
(res, Duration::new(
30_000_001_024
,
0
));
// subnormal float
let
res = Duration::from_secs_f32(f32::from_bits(
1
));
assert_eq!
(res, Duration::new(
0
,
0
));
// conversion uses rounding
let
res = Duration::from_secs_f32(
0.999e-9
);
assert_eq!
(res, Duration::new(
0
,
1
));
1.38.0
·
Source
pub fn
mul_f64
(self, rhs:
f64
) ->
Duration
Multiplies
Duration
by
f64
.
§
Panics
This method will panic if result is negative, overflows
Duration
or not finite.
§
Examples
use
std::time::Duration;
let
dur = Duration::new(
2
,
700_000_000
);
assert_eq!
(dur.mul_f64(
3.14
), Duration::new(
8
,
478_000_000
));
assert_eq!
(dur.mul_f64(
3.14e5
), Duration::new(
847_800
,
0
));
1.38.0
·
Source
pub fn
mul_f32
(self, rhs:
f32
) ->
Duration
Multiplies
Duration
by
f32
.
§
Panics
This method will panic if result is negative, overflows
Duration
or not finite.
§
Examples
use
std::time::Duration;
let
dur = Duration::new(
2
,
700_000_000
);
assert_eq!
(dur.mul_f32(
3.14
), Duration::new(
8
,
478_000_641
));
assert_eq!
(dur.mul_f32(
3.14e5
), Duration::new(
847_800
,
0
));
1.38.0
·
Source
pub fn
div_f64
(self, rhs:
f64
) ->
Duration
Divides
Duration
by
f64
.
§
Panics
This method will panic if result is negative, overflows
Duration
or not finite.
§
Examples
use
std::time::Duration;
let
dur = Duration::new(
2
,
700_000_000
);
assert_eq!
(dur.div_f64(
3.14
), Duration::new(
0
,
859_872_611
));
assert_eq!
(dur.div_f64(
3.14e5
), Duration::new(
0
,
8_599
));
1.38.0
·
Source
pub fn
div_f32
(self, rhs:
f32
) ->
Duration
Divides
Duration
by
f32
.
§
Panics
This method will panic if result is negative, overflows
Duration
or not finite.
§
Examples
use
std::time::Duration;
let
dur = Duration::new(
2
,
700_000_000
);
// note that due to rounding errors result is slightly
// different from 0.859_872_611
assert_eq!
(dur.div_f32(
3.14
), Duration::new(
0
,
859_872_580
));
assert_eq!
(dur.div_f32(
3.14e5
), Duration::new(
0
,
8_599
));
1.80.0 (const: 1.83.0)
·
Source
pub const fn
div_duration_f64
(self, rhs:
Duration
) ->
f64
Divides
Duration
by
Duration
and returns
f64
.
§
Examples
use
std::time::Duration;
let
dur1 = Duration::new(
2
,
700_000_000
);
let
dur2 = Duration::new(
5
,
400_000_000
);
assert_eq!
(dur1.div_duration_f64(dur2),
0.5
);
1.80.0 (const: 1.83.0)
·
Source
pub const fn
div_duration_f32
(self, rhs:
Duration
) ->
f32
Divides
Duration
by
Duration
and returns
f32
.
§
Examples
use
std::time::Duration;
let
dur1 = Duration::new(
2
,
700_000_000
);
let
dur2 = Duration::new(
5
,
400_000_000
);
assert_eq!
(dur1.div_duration_f32(dur2),
0.5
);
Source
§
impl
Duration
1.66.0
·
Source
pub fn
try_from_secs_f32
(secs:
f32
) ->
Result
<
Duration
,
TryFromFloatSecsError
>
The checked version of
from_secs_f32
.
This constructor will return an
Err
if
secs
is negative, overflows
Duration
or not finite.
§
Examples
use
std::time::Duration;
let
res = Duration::try_from_secs_f32(
0.0
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
0
)));
let
res = Duration::try_from_secs_f32(
1e-20
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
0
)));
let
res = Duration::try_from_secs_f32(
4.2e-7
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
420
)));
let
res = Duration::try_from_secs_f32(
2.7
);
assert_eq!
(res,
Ok
(Duration::new(
2
,
700_000_048
)));
let
res = Duration::try_from_secs_f32(
3e10
);
assert_eq!
(res,
Ok
(Duration::new(
30_000_001_024
,
0
)));
// subnormal float:
let
res = Duration::try_from_secs_f32(f32::from_bits(
1
));
assert_eq!
(res,
Ok
(Duration::new(
0
,
0
)));
let
res = Duration::try_from_secs_f32(-
5.0
);
assert!
(res.is_err());
let
res = Duration::try_from_secs_f32(f32::NAN);
assert!
(res.is_err());
let
res = Duration::try_from_secs_f32(
2e19
);
assert!
(res.is_err());
// the conversion uses rounding with tie resolution to even
let
res = Duration::try_from_secs_f32(
0.999e-9
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
1
)));
// this float represents exactly 976562.5e-9
let
val = f32::from_bits(
0x3A80_0000
);
let
res = Duration::try_from_secs_f32(val);
assert_eq!
(res,
Ok
(Duration::new(
0
,
976_562
)));
// this float represents exactly 2929687.5e-9
let
val = f32::from_bits(
0x3B40_0000
);
let
res = Duration::try_from_secs_f32(val);
assert_eq!
(res,
Ok
(Duration::new(
0
,
2_929_688
)));
// this float represents exactly 1.000_976_562_5
let
val = f32::from_bits(
0x3F802000
);
let
res = Duration::try_from_secs_f32(val);
assert_eq!
(res,
Ok
(Duration::new(
1
,
976_562
)));
// this float represents exactly 1.002_929_687_5
let
val = f32::from_bits(
0x3F806000
);
let
res = Duration::try_from_secs_f32(val);
assert_eq!
(res,
Ok
(Duration::new(
1
,
2_929_688
)));
1.66.0
·
Source
pub fn
try_from_secs_f64
(secs:
f64
) ->
Result
<
Duration
,
TryFromFloatSecsError
>
The checked version of
from_secs_f64
.
This constructor will return an
Err
if
secs
is negative, overflows
Duration
or not finite.
§
Examples
use
std::time::Duration;
let
res = Duration::try_from_secs_f64(
0.0
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
0
)));
let
res = Duration::try_from_secs_f64(
1e-20
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
0
)));
let
res = Duration::try_from_secs_f64(
4.2e-7
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
420
)));
let
res = Duration::try_from_secs_f64(
2.7
);
assert_eq!
(res,
Ok
(Duration::new(
2
,
700_000_000
)));
let
res = Duration::try_from_secs_f64(
3e10
);
assert_eq!
(res,
Ok
(Duration::new(
30_000_000_000
,
0
)));
// subnormal float
let
res = Duration::try_from_secs_f64(f64::from_bits(
1
));
assert_eq!
(res,
Ok
(Duration::new(
0
,
0
)));
let
res = Duration::try_from_secs_f64(-
5.0
);
assert!
(res.is_err());
let
res = Duration::try_from_secs_f64(f64::NAN);
assert!
(res.is_err());
let
res = Duration::try_from_secs_f64(
2e19
);
assert!
(res.is_err());
// the conversion uses rounding with tie resolution to even
let
res = Duration::try_from_secs_f64(
0.999e-9
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
1
)));
let
res = Duration::try_from_secs_f64(
0.999_999_999_499
);
assert_eq!
(res,
Ok
(Duration::new(
0
,
999_999_999
)));
let
res = Duration::try_from_secs_f64(
0.999_999_999_501
);
assert_eq!
(res,
Ok
(Duration::new(
1
,
0
)));
let
res = Duration::try_from_secs_f64(
42.999_999_999_499
);
assert_eq!
(res,
Ok
(Duration::new(
42
,
999_999_999
)));
let
res = Duration::try_from_secs_f64(
42.999_999_999_501
);
assert_eq!
(res,
Ok
(Duration::new(
43
,
0
)));
// this float represents exactly 976562.5e-9
let
val = f64::from_bits(
0x3F50_0000_0000_0000
);
let
res = Duration::try_from_secs_f64(val);
assert_eq!
(res,
Ok
(Duration::new(
0
,
976_562
)));
// this float represents exactly 2929687.5e-9
let
val = f64::from_bits(
0x3F68_0000_0000_0000
);
let
res = Duration::try_from_secs_f64(val);
assert_eq!
(res,
Ok
(Duration::new(
0
,
2_929_688
)));
// this float represents exactly 1.000_976_562_5
let
val = f64::from_bits(
0x3FF0_0400_0000_0000
);
let
res = Duration::try_from_secs_f64(val);
assert_eq!
(res,
Ok
(Duration::new(
1
,
976_562
)));
// this float represents exactly 1.002_929_687_5
let
val = f64::from_bits(
0x3_FF00_C000_0000_000
);
let
res = Duration::try_from_secs_f64(val);
assert_eq!
(res,
Ok
(Duration::new(
1
,
2_929_688
)));
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
1.3.0
·
Source
§
impl
Add
for
Duration
Source
§
type
Output
=
Duration
The resulting type after applying the
+
operator.
Source
§
fn
add
(self, rhs:
Duration
) ->
Duration
Performs the
+
operation.
Read more
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
1.9.0
·
Source
§
impl
AddAssign
for
Duration
Source
§
fn
add_assign
(&mut self, rhs:
Duration
)
Performs the
+=
operation.
Read more
1.3.0
·
Source
§
impl
Clone
for
Duration
Source
§
fn
clone
(&self) ->
Duration
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
1.27.0
·
Source
§
impl
Debug
for
Duration
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
1.3.0
·
Source
§
impl
Default
for
Duration
Source
§
fn
default
() ->
Duration
Returns the “default value” for a type.
Read more
1.3.0
·
Source
§
impl
Div
<
u32
> for
Duration
Source
§
type
Output
=
Duration
The resulting type after applying the
/
operator.
Source
§
fn
div
(self, rhs:
u32
) ->
Duration
Performs the
/
operation.
Read more
1.9.0
·
Source
§
impl
DivAssign
<
u32
> for
Duration
Source
§
fn
div_assign
(&mut self, rhs:
u32
)
Performs the
/=
operation.
Read more
1.3.0
·
Source
§
impl
Hash
for
Duration
Source
§
fn
hash
<__H>(&self, state:
&mut __H
)
where
    __H:
Hasher
,
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
1.31.0
·
Source
§
impl
Mul
<
Duration
> for
u32
Source
§
type
Output
=
Duration
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
Duration
) ->
Duration
Performs the
*
operation.
Read more
1.3.0
·
Source
§
impl
Mul
<
u32
> for
Duration
Source
§
type
Output
=
Duration
The resulting type after applying the
*
operator.
Source
§
fn
mul
(self, rhs:
u32
) ->
Duration
Performs the
*
operation.
Read more
1.9.0
·
Source
§
impl
MulAssign
<
u32
> for
Duration
Source
§
fn
mul_assign
(&mut self, rhs:
u32
)
Performs the
*=
operation.
Read more
1.3.0
·
Source
§
impl
Ord
for
Duration
Source
§
fn
cmp
(&self, other: &
Duration
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
1.3.0
·
Source
§
impl
PartialEq
for
Duration
Source
§
fn
eq
(&self, other: &
Duration
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
1.3.0
·
Source
§
impl
PartialOrd
for
Duration
Source
§
fn
partial_cmp
(&self, other: &
Duration
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
1.3.0
·
Source
§
impl
Sub
for
Duration
Source
§
type
Output
=
Duration
The resulting type after applying the
-
operator.
Source
§
fn
sub
(self, rhs:
Duration
) ->
Duration
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
1.9.0
·
Source
§
impl
SubAssign
for
Duration
Source
§
fn
sub_assign
(&mut self, rhs:
Duration
)
Performs the
-=
operation.
Read more
1.16.0
·
Source
§
impl<'a>
Sum
<&'a
Duration
> for
Duration
Source
§
fn
sum
<I>(iter: I) ->
Duration
where
    I:
Iterator
<Item = &'a
Duration
>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.16.0
·
Source
§
impl
Sum
for
Duration
Source
§
fn
sum
<I>(iter: I) ->
Duration
where
    I:
Iterator
<Item =
Duration
>,
Takes an iterator and generates
Self
from the elements by “summing up”
the items.
1.3.0
·
Source
§
impl
Copy
for
Duration
1.3.0
·
Source
§
impl
Eq
for
Duration
1.3.0
·
Source
§
impl
StructuralPartialEq
for
Duration
Auto Trait Implementations
§
§
impl
Freeze
for
Duration
§
impl
RefUnwindSafe
for
Duration
§
impl
Send
for
Duration
§
impl
Sync
for
Duration
§
impl
Unpin
for
Duration
§
impl
UnwindSafe
for
Duration
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