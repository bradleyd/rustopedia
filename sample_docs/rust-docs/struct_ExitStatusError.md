ExitStatusError in std::process - Rust
std
::
process
Struct
ExitStatusError
Copy item path
Source
pub struct ExitStatusError(
/* private fields */
);
🔬
This is a nightly-only experimental API. (
exit_status_error
#84908
)
Expand description
Describes the result of a process after it has failed
Produced by the
.exit_ok
method on
ExitStatus
.
§
Examples
#![feature(exit_status_error)]
use
std::process::{Command, ExitStatusError};
fn
run(cmd:
&
str) ->
Result
<(), ExitStatusError> {
    Command::new(cmd).status().unwrap().exit_ok()
?
;
Ok
(())
}

run(
"true"
).unwrap();
run(
"false"
).unwrap_err();
Implementations
§
Source
§
impl
ExitStatusError
Source
pub fn
code
(&self) ->
Option
<
i32
>
🔬
This is a nightly-only experimental API. (
exit_status_error
#84908
)
Reports the exit code, if applicable, from an
ExitStatusError
.
In Unix terms the return value is the
exit status
: the value passed to
exit
, if the
process finished by calling
exit
.  Note that on Unix the exit status is truncated to 8
bits, and that values that didn’t come from a program’s call to
exit
may be invented by the
runtime system (often, for example, 255, 254, 127 or 126).
On Unix, this will return
None
if the process was terminated by a signal.  If you want to
handle such situations specially, consider using methods from
ExitStatusExt
.
If the process finished by calling
exit
with a nonzero value, this will return
that exit status.
If the error was something else, it will return
None
.
If the process exited successfully (ie, by calling
exit(0)
), there is no
ExitStatusError
.  So the return value from
ExitStatusError::code()
is always nonzero.
§
Examples
#![feature(exit_status_error)]
use
std::process::Command;
let
bad = Command::new(
"false"
).status().unwrap().exit_ok().unwrap_err();
assert_eq!
(bad.code(),
Some
(
1
));
Source
pub fn
code_nonzero
(&self) ->
Option
<
NonZero
<
i32
>>
🔬
This is a nightly-only experimental API. (
exit_status_error
#84908
)
Reports the exit code, if applicable, from an
ExitStatusError
, as a
NonZero
.
This is exactly like
code()
, except that it returns a
NonZero
<
i32
>
.
Plain
code
, returning a plain integer, is provided because it is often more convenient.
The returned value from
code()
is indeed also nonzero; use
code_nonzero()
when you want
a type-level guarantee of nonzeroness.
§
Examples
#![feature(exit_status_error)]
use
std::num::NonZero;
use
std::process::Command;
let
bad = Command::new(
"false"
).status().unwrap().exit_ok().unwrap_err();
assert_eq!
(bad.code_nonzero().unwrap(), NonZero::new(
1
).unwrap());
Source
pub fn
into_status
(&self) ->
ExitStatus
🔬
This is a nightly-only experimental API. (
exit_status_error
#84908
)
Converts an
ExitStatusError
(back) to an
ExitStatus
.
Trait Implementations
§
Source
§
impl
Clone
for
ExitStatusError
Source
§
fn
clone
(&self) ->
ExitStatusError
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
Source
§
impl
Debug
for
ExitStatusError
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
Source
§
impl
Display
for
ExitStatusError
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
Source
§
impl
Error
for
ExitStatusError
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
description
(&self) -> &
str
👎
Deprecated since 1.42.0: use the Display impl or to_string()
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
Source
§
impl
ExitStatusExt
for
ExitStatusError
Available on
Unix
only.
Source
§
fn
from_raw
(raw:
i32
) -> Self
Creates a new
ExitStatus
or
ExitStatusError
from the raw underlying integer status
value from
wait
Read more
Source
§
fn
signal
(&self) ->
Option
<
i32
>
If the process was terminated by a signal, returns that signal.
Read more
Source
§
fn
core_dumped
(&self) ->
bool
If the process was terminated by a signal, says whether it dumped core.
Source
§
fn
stopped_signal
(&self) ->
Option
<
i32
>
If the process was stopped by a signal, returns that signal.
Read more
Source
§
fn
continued
(&self) ->
bool
Whether the process was continued from a stopped status.
Read more
Source
§
fn
into_raw
(self) ->
i32
Returns the underlying raw
wait
status.
Read more
Source
§
impl
From
<
ExitStatusError
> for
ExitStatus
Source
§
fn
from
(error:
ExitStatusError
) -> Self
Converts to this type from the input type.
Source
§
impl
PartialEq
for
ExitStatusError
Source
§
fn
eq
(&self, other: &
ExitStatusError
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
Source
§
impl
Copy
for
ExitStatusError
Source
§
impl
Eq
for
ExitStatusError
Source
§
impl
StructuralPartialEq
for
ExitStatusError
Auto Trait Implementations
§
§
impl
Freeze
for
ExitStatusError
§
impl
RefUnwindSafe
for
ExitStatusError
§
impl
Send
for
ExitStatusError
§
impl
Sync
for
ExitStatusError
§
impl
Unpin
for
ExitStatusError
§
impl
UnwindSafe
for
ExitStatusError
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