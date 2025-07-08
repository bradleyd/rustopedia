BacktraceStatus in std::backtrace - Rust
std
::
backtrace
Enum
BacktraceStatus
Copy item path
1.65.0
·
Source
#[non_exhaustive]
pub enum BacktraceStatus {
    Unsupported,
    Disabled,
    Captured,
}
Expand description
The current status of a backtrace, indicating whether it was captured or
whether it is empty for some other reason.
Variants (Non-exhaustive)
§
This enum is marked as non-exhaustive
Non-exhaustive enums could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm must be added to account for any future variants.
§
1.65.0
Unsupported
Capturing a backtrace is not supported, likely because it’s not
implemented for the current platform.
§
1.65.0
Disabled
Capturing a backtrace has been disabled through either the
RUST_LIB_BACKTRACE
or
RUST_BACKTRACE
environment variables.
§
1.65.0
Captured
A backtrace has been captured and the
Backtrace
should print
reasonable information when rendered.
Trait Implementations
§
1.65.0
·
Source
§
impl
Debug
for
BacktraceStatus
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
1.65.0
·
Source
§
impl
PartialEq
for
BacktraceStatus
Source
§
fn
eq
(&self, other: &
BacktraceStatus
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
1.65.0
·
Source
§
impl
Eq
for
BacktraceStatus
1.65.0
·
Source
§
impl
StructuralPartialEq
for
BacktraceStatus
Auto Trait Implementations
§
§
impl
Freeze
for
BacktraceStatus
§
impl
RefUnwindSafe
for
BacktraceStatus
§
impl
Send
for
BacktraceStatus
§
impl
Sync
for
BacktraceStatus
§
impl
Unpin
for
BacktraceStatus
§
impl
UnwindSafe
for
BacktraceStatus
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