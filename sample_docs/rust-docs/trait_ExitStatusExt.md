ExitStatusExt in std::os::unix::process - Rust
std
::
os
::
unix
::
process
Trait
ExitStatusExt
Copy item path
1.0.0
·
Source
pub trait ExitStatusExt: Sealed {
    // Required methods
    fn
from_raw
(raw:
i32
) -> Self;
fn
signal
(&self) ->
Option
<
i32
>;
fn
core_dumped
(&self) ->
bool
;
fn
stopped_signal
(&self) ->
Option
<
i32
>;
fn
continued
(&self) ->
bool
;
fn
into_raw
(self) ->
i32
;
}
Available on
Unix
only.
Expand description
Unix-specific extensions to
process::ExitStatus
and
ExitStatusError
.
On Unix,
ExitStatus
does not necessarily represent an exit status
, as
passed to the
_exit
system call or returned by
ExitStatus::code()
.  It represents
any wait status
as returned by one of the
wait
family of system
calls.
A Unix wait status (a Rust
ExitStatus
) can represent a Unix exit status, but can also
represent other kinds of process event.
This trait is sealed: it cannot be implemented outside the standard library.
This is so that future additional methods are not breaking changes.
Required Methods
§
1.12.0
·
Source
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
The value should be a
wait status, not an exit status
.
§
Panics
Panics on an attempt to make an
ExitStatusError
from a wait status of
0
.
Making an
ExitStatus
always succeeds and never panics.
1.0.0
·
Source
fn
signal
(&self) ->
Option
<
i32
>
If the process was terminated by a signal, returns that signal.
In other words, if
WIFSIGNALED
, this returns
WTERMSIG
.
1.58.0
·
Source
fn
core_dumped
(&self) ->
bool
If the process was terminated by a signal, says whether it dumped core.
1.58.0
·
Source
fn
stopped_signal
(&self) ->
Option
<
i32
>
If the process was stopped by a signal, returns that signal.
In other words, if
WIFSTOPPED
, this returns
WSTOPSIG
.  This is only possible if the status came from
a
wait
system call which was passed
WUNTRACED
, and was then converted into an
ExitStatus
.
1.58.0
·
Source
fn
continued
(&self) ->
bool
Whether the process was continued from a stopped status.
Ie,
WIFCONTINUED
.  This is only possible if the status came from a
wait
system call
which was passed
WCONTINUED
, and was then converted into an
ExitStatus
.
1.58.0
·
Source
fn
into_raw
(self) ->
i32
Returns the underlying raw
wait
status.
The returned integer is a
wait status, not an exit status
.
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.0.0
·
Source
§
impl
ExitStatusExt
for
ExitStatus
Source
§
impl
ExitStatusExt
for
ExitStatusError