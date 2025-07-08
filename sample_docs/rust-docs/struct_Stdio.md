Stdio in std::process - Rust
std
::
process
Struct
Stdio
Copy item path
1.0.0
·
Source
pub struct Stdio(
/* private fields */
);
Expand description
Describes what to do with a standard I/O stream for a child process when
passed to the
stdin
,
stdout
, and
stderr
methods of
Command
.
Implementations
§
Source
§
impl
Stdio
1.0.0
·
Source
pub fn
piped
() ->
Stdio
A new pipe should be arranged to connect the parent and child processes.
§
Examples
With stdout:
use
std::process::{Command, Stdio};
let
output = Command::new(
"echo"
)
    .arg(
"Hello, world!"
)
    .stdout(Stdio::piped())
    .output()
    .expect(
"Failed to execute command"
);
assert_eq!
(String::from_utf8_lossy(
&
output.stdout),
"Hello, world!\n"
);
// Nothing echoed to console
With stdin:
use
std::io::Write;
use
std::process::{Command, Stdio};
let
mut
child = Command::new(
"rev"
)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect(
"Failed to spawn child process"
);
let
mut
stdin = child.stdin.take().expect(
"Failed to open stdin"
);
std::thread::spawn(
move
|| {
    stdin.write_all(
"Hello, world!"
.as_bytes()).expect(
"Failed to write to stdin"
);
});
let
output = child.wait_with_output().expect(
"Failed to read stdout"
);
assert_eq!
(String::from_utf8_lossy(
&
output.stdout),
"!dlrow ,olleH"
);
Writing more than a pipe buffer’s worth of input to stdin without also reading
stdout and stderr at the same time may cause a deadlock.
This is an issue when running any program that doesn’t guarantee that it reads
its entire stdin before writing more than a pipe buffer’s worth of output.
The size of a pipe buffer varies on different targets.
1.0.0
·
Source
pub fn
inherit
() ->
Stdio
The child inherits from the corresponding parent descriptor.
§
Examples
With stdout:
use
std::process::{Command, Stdio};
let
output = Command::new(
"echo"
)
    .arg(
"Hello, world!"
)
    .stdout(Stdio::inherit())
    .output()
    .expect(
"Failed to execute command"
);
assert_eq!
(String::from_utf8_lossy(
&
output.stdout),
""
);
// "Hello, world!" echoed to console
With stdin:
use
std::process::{Command, Stdio};
use
std::io::{
self
, Write};
let
output = Command::new(
"rev"
)
    .stdin(Stdio::inherit())
    .stdout(Stdio::piped())
    .output()
?
;
print!
(
"You piped in the reverse of: "
);
io::stdout().write_all(
&
output.stdout)
?
;
1.0.0
·
Source
pub fn
null
() ->
Stdio
This stream will be ignored. This is the equivalent of attaching the
stream to
/dev/null
.
§
Examples
With stdout:
use
std::process::{Command, Stdio};
let
output = Command::new(
"echo"
)
    .arg(
"Hello, world!"
)
    .stdout(Stdio::null())
    .output()
    .expect(
"Failed to execute command"
);
assert_eq!
(String::from_utf8_lossy(
&
output.stdout),
""
);
// Nothing echoed to console
With stdin:
use
std::process::{Command, Stdio};
let
output = Command::new(
"rev"
)
    .stdin(Stdio::null())
    .stdout(Stdio::piped())
    .output()
    .expect(
"Failed to execute command"
);
assert_eq!
(String::from_utf8_lossy(
&
output.stdout),
""
);
// Ignores any piped-in input
Source
pub fn
makes_pipe
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
stdio_makes_pipe
#98288
)
Returns
true
if this requires
Command
to create a new pipe.
§
Example
#![feature(stdio_makes_pipe)]
use
std::process::Stdio;
let
io = Stdio::piped();
assert_eq!
(io.makes_pipe(),
true
);
Trait Implementations
§
1.16.0
·
Source
§
impl
Debug
for
Stdio
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
1.20.0
·
Source
§
impl
From
<
ChildStderr
> for
Stdio
Source
§
fn
from
(child:
ChildStderr
) ->
Stdio
Converts a
ChildStderr
into a
Stdio
.
§
Examples
use
std::process::{Command, Stdio};
let
reverse = Command::new(
"rev"
)
    .arg(
"non_existing_file.txt"
)
    .stderr(Stdio::piped())
    .spawn()
    .expect(
"failed reverse command"
);
let
cat = Command::new(
"cat"
)
    .arg(
"-"
)
    .stdin(reverse.stderr.unwrap())
// Converted into a Stdio here
.output()
    .expect(
"failed echo command"
);
assert_eq!
(
    String::from_utf8_lossy(
&
cat.stdout),
"rev: cannot open non_existing_file.txt: No such file or directory\n"
);
1.20.0
·
Source
§
impl
From
<
ChildStdin
> for
Stdio
Source
§
fn
from
(child:
ChildStdin
) ->
Stdio
Converts a
ChildStdin
into a
Stdio
.
§
Examples
ChildStdin
will be converted to
Stdio
using
Stdio::from
under the hood.
use
std::process::{Command, Stdio};
let
reverse = Command::new(
"rev"
)
    .stdin(Stdio::piped())
    .spawn()
    .expect(
"failed reverse command"
);
let
_echo = Command::new(
"echo"
)
    .arg(
"Hello, world!"
)
    .stdout(reverse.stdin.unwrap())
// Converted into a Stdio here
.output()
    .expect(
"failed echo command"
);
// "!dlrow ,olleH" echoed to console
1.20.0
·
Source
§
impl
From
<
ChildStdout
> for
Stdio
Source
§
fn
from
(child:
ChildStdout
) ->
Stdio
Converts a
ChildStdout
into a
Stdio
.
§
Examples
ChildStdout
will be converted to
Stdio
using
Stdio::from
under the hood.
use
std::process::{Command, Stdio};
let
hello = Command::new(
"echo"
)
    .arg(
"Hello, world!"
)
    .stdout(Stdio::piped())
    .spawn()
    .expect(
"failed echo command"
);
let
reverse = Command::new(
"rev"
)
    .stdin(hello.stdout.unwrap())
// Converted into a Stdio here
.output()
    .expect(
"failed reverse command"
);
assert_eq!
(reverse.stdout,
b"!dlrow ,olleH\n"
);
1.20.0
·
Source
§
impl
From
<
File
> for
Stdio
Source
§
fn
from
(file:
File
) ->
Stdio
Converts a
File
into a
Stdio
.
§
Examples
File
will be converted to
Stdio
using
Stdio::from
under the hood.
use
std::fs::File;
use
std::process::Command;
// With the `foo.txt` file containing "Hello, world!"
let
file = File::open(
"foo.txt"
)
?
;
let
reverse = Command::new(
"rev"
)
    .stdin(file)
// Implicit File conversion into a Stdio
.output()
?
;
assert_eq!
(reverse.stdout,
b"!dlrow ,olleH"
);
1.63.0
·
Source
§
impl
From
<
OwnedFd
> for
Stdio
Available on
Unix
only.
Source
§
fn
from
(fd:
OwnedFd
) ->
Stdio
Takes ownership of a file descriptor and returns a
Stdio
that can attach a stream to it.
1.63.0
·
Source
§
impl
From
<
OwnedHandle
> for
Stdio
Available on
Windows
only.
Source
§
fn
from
(handle:
OwnedHandle
) ->
Stdio
Takes ownership of a handle and returns a
Stdio
that can attach a stream to it.
1.87.0
·
Source
§
impl
From
<
PipeReader
> for
Stdio
Source
§
fn
from
(pipe:
PipeReader
) -> Self
Converts to this type from the input type.
1.87.0
·
Source
§
impl
From
<
PipeWriter
> for
Stdio
Source
§
fn
from
(pipe:
PipeWriter
) -> Self
Converts to this type from the input type.
1.74.0
·
Source
§
impl
From
<
Stderr
> for
Stdio
Source
§
fn
from
(inherit:
Stderr
) ->
Stdio
Redirect command stdout/stderr to our stderr
§
Examples
#![feature(exit_status_error)]
use
std::io;
use
std::process::Command;
let
output = Command::new(
"whoami"
)
    .stdout(io::stderr())
    .output()
?
;
output.status.exit_ok()
?
;
assert!
(output.stdout.is_empty());
1.74.0
·
Source
§
impl
From
<
Stdout
> for
Stdio
Source
§
fn
from
(inherit:
Stdout
) ->
Stdio
Redirect command stdout/stderr to our stdout
§
Examples
#![feature(exit_status_error)]
use
std::io;
use
std::process::Command;
let
output = Command::new(
"whoami"
)
    .stdout(io::stdout())
    .output()
?
;
output.status.exit_ok()
?
;
assert!
(output.stdout.is_empty());
1.2.0
·
Source
§
impl
FromRawFd
for
Stdio
Available on
Unix
only.
Source
§
unsafe fn
from_raw_fd
(fd:
RawFd
) ->
Stdio
Constructs a new instance of
Self
from the given raw file
descriptor.
Read more
1.2.0
·
Source
§
impl
FromRawHandle
for
Stdio
Available on
Windows
only.
Source
§
unsafe fn
from_raw_handle
(handle:
RawHandle
) ->
Stdio
Constructs a new I/O object from the specified raw handle.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
Stdio
§
impl
RefUnwindSafe
for
Stdio
§
impl
Send
for
Stdio
§
impl
Sync
for
Stdio
§
impl
Unpin
for
Stdio
§
impl
UnwindSafe
for
Stdio
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