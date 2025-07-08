ExitCode in std::process - Rust
std
::
process
Struct
ExitCode
Copy item path
1.61.0
·
Source
pub struct ExitCode(
/* private fields */
);
Expand description
This type represents the status code the current process can return
to its parent under normal termination.
ExitCode
is intended to be consumed only by the standard library (via
Termination::report()
). For forwards compatibility with potentially
unusual targets, this type currently does not provide
Eq
,
Hash
, or
access to the raw value. This type does provide
PartialEq
for
comparison, but note that there may potentially be multiple failure
codes, some of which will
not
compare equal to
ExitCode::FAILURE
.
The standard library provides the canonical
SUCCESS
and
FAILURE
exit codes as well as
From<u8> for ExitCode
for constructing other
arbitrary exit codes.
§
Portability
Numeric values used in this type don’t have portable meanings, and
different platforms may mask different amounts of them.
For the platform’s canonical successful and unsuccessful codes, see
the
SUCCESS
and
FAILURE
associated items.
§
Differences from
ExitStatus
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
§
Examples
ExitCode
can be returned from the
main
function of a crate, as it implements
Termination
:
use
std::process::ExitCode;
fn
main() -> ExitCode {
if
!check_foo() {
return
ExitCode::from(
42
);
    }

    ExitCode::SUCCESS
}
Implementations
§
1.61.0
·
Source
§
impl
ExitCode
1.61.0
·
Source
pub const
SUCCESS
:
ExitCode
The canonical
ExitCode
for successful termination on this platform.
Note that a
()
-returning
main
implicitly results in a successful
termination, so there’s no need to return this from
main
unless
you’re also returning other possible codes.
1.61.0
·
Source
pub const
FAILURE
:
ExitCode
The canonical
ExitCode
for unsuccessful termination on this platform.
If you’re only returning this and
SUCCESS
from
main
, consider
instead returning
Err(_)
and
Ok(())
respectively, which will
return the same codes (but will also
eprintln!
the error).
Source
pub fn
exit_process
(self) ->
!
🔬
This is a nightly-only experimental API. (
exitcode_exit_method
#97100
)
Exit the current process with the given
ExitCode
.
Note that this has the same caveats as
process::exit()
, namely that this function
terminates the process immediately, so no destructors on the current stack or any other
thread’s stack will be run. Also see those docs for some important notes on interop with C
code. If a clean shutdown is needed, it is recommended to simply return this ExitCode from
the
main
function, as demonstrated in the
type documentation
.
§
Differences from
process::exit()
process::exit()
accepts any
i32
value as the exit code for the process; however, there
are platforms that only use a subset of that value (see
process::exit
platform-specific
behavior
).
ExitCode
exists because of this; only
ExitCode
s that are supported by a majority of our platforms can be created, so those
problems don’t exist (as much) with this method.
§
Examples
#![feature(exitcode_exit_method)]
// there's no way to gracefully recover from an UhOhError, so we just
// print a message and exit
fn
handle_unrecoverable_error(err: UhOhError) -> ! {
eprintln!
(
"UH OH! {err}"
);
let
code =
match
err {
        UhOhError::GenericProblem => ExitCode::FAILURE,
        UhOhError::Specific => ExitCode::from(
3
),
        UhOhError::WithCode { exit_code, .. } => exit_code,
    };
    code.exit_process()
}
Trait Implementations
§
1.61.0
·
Source
§
impl
Clone
for
ExitCode
Source
§
fn
clone
(&self) ->
ExitCode
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
1.61.0
·
Source
§
impl
Debug
for
ExitCode
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
1.75.0
·
Source
§
impl
Default
for
ExitCode
The default value is
ExitCode::SUCCESS
Source
§
fn
default
() -> Self
Returns the “default value” for a type.
Read more
Source
§
impl
ExitCodeExt
for
ExitCode
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
🔬
This is a nightly-only experimental API. (
windows_process_exit_code_from
#111688
)
Creates a new
ExitCode
from the raw underlying
u32
return value of
a process.
Read more
1.61.0
·
Source
§
impl
From
<
u8
> for
ExitCode
Source
§
fn
from
(code:
u8
) -> Self
Constructs an
ExitCode
from an arbitrary u8 value.
1.61.0
·
Source
§
impl
PartialEq
for
ExitCode
Source
§
fn
eq
(&self, other: &
ExitCode
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
1.61.0
·
Source
§
impl
Termination
for
ExitCode
Source
§
fn
report
(self) ->
ExitCode
Is called to get the representation of the value as status code.
This status code is returned to the operating system.
1.61.0
·
Source
§
impl
Copy
for
ExitCode
1.61.0
·
Source
§
impl
StructuralPartialEq
for
ExitCode
Auto Trait Implementations
§
§
impl
Freeze
for
ExitCode
§
impl
RefUnwindSafe
for
ExitCode
§
impl
Send
for
ExitCode
§
impl
Sync
for
ExitCode
§
impl
Unpin
for
ExitCode
§
impl
UnwindSafe
for
ExitCode
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