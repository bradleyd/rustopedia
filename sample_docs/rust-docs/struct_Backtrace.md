Backtrace in std::backtrace - Rust
std
::
backtrace
Struct
Backtrace
Copy item path
1.65.0
·
Source
pub struct Backtrace {
/* private fields */
}
Expand description
A captured OS thread stack backtrace.
This type represents a stack backtrace for an OS thread captured at a
previous point in time. In some instances the
Backtrace
type may
internally be empty due to configuration. For more information see
Backtrace::capture
.
Implementations
§
Source
§
impl
Backtrace
1.65.0
·
Source
pub fn
capture
() ->
Backtrace
Captures a stack backtrace of the current thread.
This function will capture a stack backtrace of the current OS thread of
execution, returning a
Backtrace
type which can be later used to print
the entire stack trace or render it to a string.
This function will be a noop if the
RUST_BACKTRACE
or
RUST_LIB_BACKTRACE
backtrace variables are both not set. If either
environment variable is set and enabled then this function will actually
capture a backtrace. Capturing a backtrace can be both memory intensive
and slow, so these environment variables allow liberally using
Backtrace::capture
and only incurring a slowdown when the environment
variables are set.
To forcibly capture a backtrace regardless of environment variables, use
the
Backtrace::force_capture
function.
1.65.0
·
Source
pub fn
force_capture
() ->
Backtrace
Forcibly captures a full backtrace, regardless of environment variable
configuration.
This function behaves the same as
capture
except that it ignores the
values of the
RUST_BACKTRACE
and
RUST_LIB_BACKTRACE
environment
variables, always capturing a backtrace.
Note that capturing a backtrace can be an expensive operation on some
platforms, so this should be used with caution in performance-sensitive
parts of code.
1.65.0 (const: 1.65.0)
·
Source
pub const fn
disabled
() ->
Backtrace
Forcibly captures a disabled backtrace, regardless of environment
variable configuration.
1.65.0
·
Source
pub fn
status
(&self) ->
BacktraceStatus
Returns the status of this backtrace, indicating whether this backtrace
request was unsupported, disabled, or a stack trace was actually
captured.
Source
§
impl<'a>
Backtrace
Source
pub fn
frames
(&'a self) -> &'a [
BacktraceFrame
]
🔬
This is a nightly-only experimental API. (
backtrace_frames
#79676
)
Returns an iterator over the backtrace frames.
Trait Implementations
§
1.65.0
·
Source
§
impl
Debug
for
Backtrace
Source
§
fn
fmt
(&self, fmt: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
1.65.0
·
Source
§
impl
Display
for
Backtrace
Source
§
fn
fmt
(&self, fmt: &mut
Formatter
<'_>) ->
Result
Formats the value using the given formatter.
Read more
Auto Trait Implementations
§
§
impl !
Freeze
for
Backtrace
§
impl
RefUnwindSafe
for
Backtrace
§
impl
Send
for
Backtrace
§
impl
Sync
for
Backtrace
§
impl
Unpin
for
Backtrace
§
impl
UnwindSafe
for
Backtrace
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