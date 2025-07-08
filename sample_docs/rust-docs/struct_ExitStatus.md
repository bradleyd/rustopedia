ExitStatus in std::process - Rust
std
::
process
Struct
ExitStatus
Copy item path
1.0.0
·
Source
pub struct ExitStatus(
/* private fields */
);
Expand description
Describes the result of a process after it has terminated.
This
struct
is used to represent the exit status or other termination of a child process.
Child processes are created via the
Command
struct and their exit
status is exposed through the
status
method, or the
wait
method
of a
Child
process.
An
ExitStatus
represents every possible disposition of a process.  On Unix this
is the
wait status
.  It is
not
simply an
exit status
(a value passed to
exit
).
For proper error reporting of failed processes, print the value of
ExitStatus
or
ExitStatusError
using their implementations of
Display
.
§
Differences from
ExitCode
ExitCode
is intended for terminating the currently running process, via
the
Termination
trait, in contrast to
ExitStatus
, which represents the
termination of a child process. These APIs are separate due to platform
compatibility differences and their expected usage; it is not generally
possible to exactly reproduce an
ExitStatus
from a child for the current
process after the fact.
Implementations
§
Source
§
impl
ExitStatus
Source
pub fn
exit_ok
(&self) ->
Result
<
()
,
ExitStatusError
>
🔬
This is a nightly-only experimental API. (
exit_status_error
#84908
)
Was termination successful?  Returns a
Result
.
§
Examples
#![feature(exit_status_error)]
use
std::process::Command;
let
status = Command::new(
"ls"
)
    .arg(
"/dev/nonexistent"
)
    .status()
    .expect(
"ls could not be executed"
);
println!
(
"ls: {status}"
);
status.exit_ok().expect_err(
"/dev/nonexistent could be listed!"
);
1.0.0
·
Source
pub fn
success
(&self) ->
bool
Was termination successful? Signal termination is not considered a
success, and success is defined as a zero exit status.
§
Examples
use
std::process::Command;
let
status = Command::new(
"mkdir"
)
    .arg(
"projects"
)
    .status()
    .expect(
"failed to execute mkdir"
);
if
status.success() {
println!
(
"'projects/' directory created"
);
}
else
{
println!
(
"failed to create 'projects/' directory: {status}"
);
}
1.0.0
·
Source
pub fn
code
(&self) ->
Option
<
i32
>
Returns the exit code of the process, if any.
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
if the process was terminated by a signal.
ExitStatusExt
is an
extension trait for extracting any such signal, and other details, from the
ExitStatus
.
§
Examples
use
std::process::Command;
let
status = Command::new(
"mkdir"
)
    .arg(
"projects"
)
    .status()
    .expect(
"failed to execute mkdir"
);
match
status.code() {
Some
(code) =>
println!
(
"Exited with status code: {code}"
),
None
=>
println!
(
"Process terminated by signal"
)
}
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
ExitStatus
Source
§
fn
clone
(&self) ->
ExitStatus
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
1.0.0
·
Source
§
impl
Debug
for
ExitStatus
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
1.73.0
·
Source
§
impl
Default
for
ExitStatus
The default value is one which indicates successful completion.
Source
§
fn
default
() -> Self
Returns the “default value” for a type.
Read more
1.0.0
·
Source
§
impl
Display
for
ExitStatus
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
impl
ExitStatusExt
for
ExitStatus
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
1.12.0
·
Source
§
impl
ExitStatusExt
for
ExitStatus
Available on
Windows
only.
Source
§
fn
from_raw
(raw:
u32
) -> Self
Creates a new
ExitStatus
from the raw underlying
u32
return value of
a process.
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
1.0.0
·
Source
§
impl
PartialEq
for
ExitStatus
Source
§
fn
eq
(&self, other: &
ExitStatus
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
1.0.0
·
Source
§
impl
Copy
for
ExitStatus
1.0.0
·
Source
§
impl
Eq
for
ExitStatus
1.0.0
·
Source
§
impl
StructuralPartialEq
for
ExitStatus
Auto Trait Implementations
§
§
impl
Freeze
for
ExitStatus
§
impl
RefUnwindSafe
for
ExitStatus
§
impl
Send
for
ExitStatus
§
impl
Sync
for
ExitStatus
§
impl
Unpin
for
ExitStatus
§
impl
UnwindSafe
for
ExitStatus
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