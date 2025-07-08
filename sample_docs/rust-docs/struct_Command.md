Command in std::process - Rust
std
::
process
Struct
Command
Copy item path
1.0.0
·
Source
pub struct Command {
/* private fields */
}
Expand description
A process builder, providing fine-grained control
over how a new process should be spawned.
A default configuration can be
generated using
Command::new(program)
, where
program
gives a path to the
program to be executed. Additional builder methods allow the configuration
to be changed (for example, by adding arguments) prior to spawning:
use
std::process::Command;
let
output =
if
cfg!
(target_os =
"windows"
) {
    Command::new(
"cmd"
)
        .args([
"/C"
,
"echo hello"
])
        .output()
        .expect(
"failed to execute process"
)
}
else
{
    Command::new(
"sh"
)
        .arg(
"-c"
)
        .arg(
"echo hello"
)
        .output()
        .expect(
"failed to execute process"
)
};
let
hello = output.stdout;
Command
can be reused to spawn multiple processes. The builder methods
change the command without needing to immediately spawn the process.
use
std::process::Command;
let
mut
echo_hello = Command::new(
"sh"
);
echo_hello.arg(
"-c"
).arg(
"echo hello"
);
let
hello_1 = echo_hello.output().expect(
"failed to execute process"
);
let
hello_2 = echo_hello.output().expect(
"failed to execute process"
);
Similarly, you can call builder methods after spawning a process and then
spawn a new process with the modified settings.
use
std::process::Command;
let
mut
list_dir = Command::new(
"ls"
);
// Execute `ls` in the current directory of the program.
list_dir.status().expect(
"process failed to execute"
);
println!
();
// Change `ls` to execute in the root directory.
list_dir.current_dir(
"/"
);
// And then execute `ls` again but in the root directory.
list_dir.status().expect(
"process failed to execute"
);
Implementations
§
Source
§
impl
Command
1.0.0
·
Source
pub fn
new
<S:
AsRef
<
OsStr
>>(program: S) ->
Command
Constructs a new
Command
for launching the program at
path
program
, with the following default configuration:
No arguments to the program
Inherit the current process’s environment
Inherit the current process’s working directory
Inherit stdin/stdout/stderr for
spawn
or
status
, but create pipes for
output
Builder methods are provided to change these defaults and
otherwise configure the process.
If
program
is not an absolute path, the
PATH
will be searched in
an OS-defined way.
The search path to be used may be controlled by setting the
PATH
environment variable on the Command,
but this has some implementation limitations on Windows
(see issue #37519).
§
Platform-specific behavior
Note on Windows: For executable files with the .exe extension,
it can be omitted when specifying the program for this Command.
However, if the file has a different extension,
a filename including the extension needs to be provided,
otherwise the file won’t be found.
§
Examples
use
std::process::Command;

Command::new(
"sh"
)
    .spawn()
    .expect(
"sh command failed to start"
);
§
Caveats
Command::new
is only intended to accept the path of the program. If you pass a program
path along with arguments like
Command::new("ls -l").spawn()
, it will try to search for
ls -l
literally. The arguments need to be passed separately, such as via
arg
or
args
.
use
std::process::Command;

Command::new(
"ls"
)
    .arg(
"-l"
)
// arg passed separately
.spawn()
    .expect(
"ls command failed to start"
);
1.0.0
·
Source
pub fn
arg
<S:
AsRef
<
OsStr
>>(&mut self, arg: S) -> &mut
Command
Adds an argument to pass to the program.
Only one argument can be passed per use. So instead of:
.arg(
"-C /path/to/repo"
)
usage would be:
.arg(
"-C"
)
.arg(
"/path/to/repo"
)
To pass multiple arguments see
args
.
Note that the argument is not passed through a shell, but given
literally to the program. This means that shell syntax like quotes,
escaped characters, word splitting, glob patterns, variable substitution,
etc. have no effect.
On Windows, use caution with untrusted inputs. Most applications use the
standard convention for decoding arguments passed to them. These are safe to
use with
arg
. However, some applications such as
cmd.exe
and
.bat
files
use a non-standard way of decoding arguments. They are therefore vulnerable
to malicious input.
In the case of
cmd.exe
this is especially important because a malicious
argument can potentially run arbitrary shell commands.
See
Windows argument splitting
for more details
or
raw_arg
for manually implementing non-standard argument encoding.
§
Examples
use
std::process::Command;

Command::new(
"ls"
)
    .arg(
"-l"
)
    .arg(
"-a"
)
    .spawn()
    .expect(
"ls command failed to start"
);
1.0.0
·
Source
pub fn
args
<I, S>(&mut self, args: I) -> &mut
Command
where
    I:
IntoIterator
<Item = S>,
    S:
AsRef
<
OsStr
>,
Adds multiple arguments to pass to the program.
To pass a single argument see
arg
.
Note that the arguments are not passed through a shell, but given
literally to the program. This means that shell syntax like quotes,
escaped characters, word splitting, glob patterns, variable substitution, etc.
have no effect.
On Windows, use caution with untrusted inputs. Most applications use the
standard convention for decoding arguments passed to them. These are safe to
use with
arg
. However, some applications such as
cmd.exe
and
.bat
files
use a non-standard way of decoding arguments. They are therefore vulnerable
to malicious input.
In the case of
cmd.exe
this is especially important because a malicious
argument can potentially run arbitrary shell commands.
See
Windows argument splitting
for more details
or
raw_arg
for manually implementing non-standard argument encoding.
§
Examples
use
std::process::Command;

Command::new(
"ls"
)
    .args([
"-l"
,
"-a"
])
    .spawn()
    .expect(
"ls command failed to start"
);
1.0.0
·
Source
pub fn
env
<K, V>(&mut self, key: K, val: V) -> &mut
Command
where
    K:
AsRef
<
OsStr
>,
    V:
AsRef
<
OsStr
>,
Inserts or updates an explicit environment variable mapping.
This method allows you to add an environment variable mapping to the spawned process or
overwrite a previously set value. You can use
Command::envs
to set multiple environment
variables simultaneously.
Child processes will inherit environment variables from their parent process by default.
Environment variables explicitly set using
Command::env
take precedence over inherited
variables. You can disable environment variable inheritance entirely using
Command::env_clear
or for a single key using
Command::env_remove
.
Note that environment variable names are case-insensitive (but
case-preserving) on Windows and case-sensitive on all other platforms.
§
Examples
use
std::process::Command;

Command::new(
"ls"
)
    .env(
"PATH"
,
"/bin"
)
    .spawn()
    .expect(
"ls command failed to start"
);
1.19.0
·
Source
pub fn
envs
<I, K, V>(&mut self, vars: I) -> &mut
Command
where
    I:
IntoIterator
<Item =
(K, V)
>,
    K:
AsRef
<
OsStr
>,
    V:
AsRef
<
OsStr
>,
Inserts or updates multiple explicit environment variable mappings.
This method allows you to add multiple environment variable mappings to the spawned process
or overwrite previously set values. You can use
Command::env
to set a single environment
variable.
Child processes will inherit environment variables from their parent process by default.
Environment variables explicitly set using
Command::envs
take precedence over inherited
variables. You can disable environment variable inheritance entirely using
Command::env_clear
or for a single key using
Command::env_remove
.
Note that environment variable names are case-insensitive (but case-preserving) on Windows
and case-sensitive on all other platforms.
§
Examples
use
std::process::{Command, Stdio};
use
std::env;
use
std::collections::HashMap;
let
filtered_env : HashMap<String, String> =
    env::vars().filter(|
&
(
ref
k,
_
)|
        k ==
"TERM"
|| k ==
"TZ"
|| k ==
"LANG"
|| k ==
"PATH"
).collect();

Command::new(
"printenv"
)
    .stdin(Stdio::null())
    .stdout(Stdio::inherit())
    .env_clear()
    .envs(
&
filtered_env)
    .spawn()
    .expect(
"printenv failed to start"
);
1.0.0
·
Source
pub fn
env_remove
<K:
AsRef
<
OsStr
>>(&mut self, key: K) -> &mut
Command
Removes an explicitly set environment variable and prevents inheriting it from a parent
process.
This method will remove the explicit value of an environment variable set via
Command::env
or
Command::envs
. In addition, it will prevent the spawned child
process from inheriting that environment variable from its parent process.
After calling
Command::env_remove
, the value associated with its key from
Command::get_envs
will be
None
.
To clear all explicitly set environment variables and disable all environment variable
inheritance, you can use
Command::env_clear
.
§
Examples
Prevent any inherited
GIT_DIR
variable from changing the target of the
git
command,
while allowing all other variables, like
GIT_AUTHOR_NAME
.
use
std::process::Command;

Command::new(
"git"
)
    .arg(
"commit"
)
    .env_remove(
"GIT_DIR"
)
    .spawn()
?
;
1.0.0
·
Source
pub fn
env_clear
(&mut self) -> &mut
Command
Clears all explicitly set environment variables and prevents inheriting any parent process
environment variables.
This method will remove all explicitly added environment variables set via
Command::env
or
Command::envs
. In addition, it will prevent the spawned child process from inheriting
any environment variable from its parent process.
After calling
Command::env_clear
, the iterator from
Command::get_envs
will be
empty.
You can use
Command::env_remove
to clear a single mapping.
§
Examples
The behavior of
sort
is affected by
LANG
and
LC_*
environment variables.
Clearing the environment makes
sort
‘s behavior independent of the parent processes’ language.
use
std::process::Command;

Command::new(
"sort"
)
    .arg(
"file.txt"
)
    .env_clear()
    .spawn()
?
;
1.0.0
·
Source
pub fn
current_dir
<P:
AsRef
<
Path
>>(&mut self, dir: P) -> &mut
Command
Sets the working directory for the child process.
§
Platform-specific behavior
If the program path is relative (e.g.,
"./script.sh"
), it’s ambiguous
whether it should be interpreted relative to the parent’s working
directory or relative to
current_dir
. The behavior in this case is
platform specific and unstable, and it’s recommended to use
canonicalize
to get an absolute program path instead.
§
Examples
use
std::process::Command;

Command::new(
"ls"
)
    .current_dir(
"/bin"
)
    .spawn()
    .expect(
"ls command failed to start"
);
1.0.0
·
Source
pub fn
stdin
<T:
Into
<
Stdio
>>(&mut self, cfg: T) -> &mut
Command
Configuration for the child process’s standard input (stdin) handle.
Defaults to
inherit
when used with
spawn
or
status
, and
defaults to
piped
when used with
output
.
§
Examples
use
std::process::{Command, Stdio};

Command::new(
"ls"
)
    .stdin(Stdio::null())
    .spawn()
    .expect(
"ls command failed to start"
);
1.0.0
·
Source
pub fn
stdout
<T:
Into
<
Stdio
>>(&mut self, cfg: T) -> &mut
Command
Configuration for the child process’s standard output (stdout) handle.
Defaults to
inherit
when used with
spawn
or
status
, and
defaults to
piped
when used with
output
.
§
Examples
use
std::process::{Command, Stdio};

Command::new(
"ls"
)
    .stdout(Stdio::null())
    .spawn()
    .expect(
"ls command failed to start"
);
1.0.0
·
Source
pub fn
stderr
<T:
Into
<
Stdio
>>(&mut self, cfg: T) -> &mut
Command
Configuration for the child process’s standard error (stderr) handle.
Defaults to
inherit
when used with
spawn
or
status
, and
defaults to
piped
when used with
output
.
§
Examples
use
std::process::{Command, Stdio};

Command::new(
"ls"
)
    .stderr(Stdio::null())
    .spawn()
    .expect(
"ls command failed to start"
);
1.0.0
·
Source
pub fn
spawn
(&mut self) ->
Result
<
Child
>
Executes the command as a child process, returning a handle to it.
By default, stdin, stdout and stderr are inherited from the parent.
§
Examples
use
std::process::Command;

Command::new(
"ls"
)
    .spawn()
    .expect(
"ls command failed to start"
);
1.0.0
·
Source
pub fn
output
(&mut self) ->
Result
<
Output
>
Executes the command as a child process, waiting for it to finish and
collecting all of its output.
By default, stdout and stderr are captured (and used to provide the
resulting output). Stdin is not inherited from the parent and any
attempt by the child process to read from the stdin stream will result
in the stream immediately closing.
§
Examples
ⓘ
use
std::process::Command;
use
std::io::{
self
, Write};
let
output = Command::new(
"/bin/cat"
)
    .arg(
"file.txt"
)
    .output()
?
;
println!
(
"status: {}"
, output.status);
io::stdout().write_all(
&
output.stdout)
?
;
io::stderr().write_all(
&
output.stderr)
?
;
assert!
(output.status.success());
1.0.0
·
Source
pub fn
status
(&mut self) ->
Result
<
ExitStatus
>
Executes a command as a child process, waiting for it to finish and
collecting its status.
By default, stdin, stdout and stderr are inherited from the parent.
§
Examples
ⓘ
use
std::process::Command;
let
status = Command::new(
"/bin/cat"
)
    .arg(
"file.txt"
)
    .status()
    .expect(
"failed to execute process"
);
println!
(
"process finished with: {status}"
);
assert!
(status.success());
1.57.0
·
Source
pub fn
get_program
(&self) -> &
OsStr
Returns the path to the program that was given to
Command::new
.
§
Examples
use
std::process::Command;
let
cmd = Command::new(
"echo"
);
assert_eq!
(cmd.get_program(),
"echo"
);
1.57.0
·
Source
pub fn
get_args
(&self) ->
CommandArgs
<'_>
ⓘ
Returns an iterator of the arguments that will be passed to the program.
This does not include the path to the program as the first argument;
it only includes the arguments specified with
Command::arg
and
Command::args
.
§
Examples
use
std::ffi::OsStr;
use
std::process::Command;
let
mut
cmd = Command::new(
"echo"
);
cmd.arg(
"first"
).arg(
"second"
);
let
args: Vec<
&
OsStr> = cmd.get_args().collect();
assert_eq!
(args,
&
[
"first"
,
"second"
]);
1.57.0
·
Source
pub fn
get_envs
(&self) ->
CommandEnvs
<'_>
ⓘ
Returns an iterator of the environment variables explicitly set for the child process.
Environment variables explicitly set using
Command::env
,
Command::envs
, and
Command::env_remove
can be retrieved with this method.
Note that this output does not include environment variables inherited from the parent
process.
Each element is a tuple key/value pair
(&OsStr, Option<&OsStr>)
. A
None
value
indicates its key was explicitly removed via
Command::env_remove
. The associated key for
the
None
value will no longer inherit from its parent process.
An empty iterator can indicate that no explicit mappings were added or that
Command::env_clear
was called. After calling
Command::env_clear
, the child process
will not inherit any environment variables from its parent process.
§
Examples
use
std::ffi::OsStr;
use
std::process::Command;
let
mut
cmd = Command::new(
"ls"
);
cmd.env(
"TERM"
,
"dumb"
).env_remove(
"TZ"
);
let
envs: Vec<(
&
OsStr,
Option
<
&
OsStr>)> = cmd.get_envs().collect();
assert_eq!
(envs,
&
[
    (OsStr::new(
"TERM"
),
Some
(OsStr::new(
"dumb"
))),
    (OsStr::new(
"TZ"
),
None
)
]);
1.57.0
·
Source
pub fn
get_current_dir
(&self) ->
Option
<&
Path
>
Returns the working directory for the child process.
This returns
None
if the working directory will not be changed.
§
Examples
use
std::path::Path;
use
std::process::Command;
let
mut
cmd = Command::new(
"ls"
);
assert_eq!
(cmd.get_current_dir(),
None
);
cmd.current_dir(
"/bin"
);
assert_eq!
(cmd.get_current_dir(),
Some
(Path::new(
"/bin"
)));
Trait Implementations
§
1.0.0
·
Source
§
impl
CommandExt
for
Command
Available on
Unix
only.
Source
§
fn
uid
(&mut self, id:
u32
) -> &mut
Command
Sets the child process’s user ID. This translates to a
setuid
call in the child process. Failure in the
setuid
call will cause the spawn to fail.
Read more
Source
§
fn
gid
(&mut self, id:
u32
) -> &mut
Command
Similar to
uid
, but sets the group ID of the child process. This has
the same semantics as the
uid
field.
Source
§
fn
groups
(&mut self, groups: &[
u32
]) -> &mut
Command
🔬
This is a nightly-only experimental API. (
setgroups
#90747
)
Sets the supplementary group IDs for the calling process. Translates to
a
setgroups
call in the child process.
Source
§
unsafe fn
pre_exec
<F>(&mut self, f: F) -> &mut
Command
where
    F:
FnMut
() ->
Result
<
()
> +
Send
+
Sync
+ 'static,
Schedules a closure to be run just before the
exec
function is
invoked.
Read more
Source
§
fn
exec
(&mut self) ->
Error
Performs all the required setup by this
Command
, followed by calling
the
execvp
syscall.
Read more
Source
§
fn
arg0
<S>(&mut self, arg: S) -> &mut
Command
where
    S:
AsRef
<
OsStr
>,
Set executable argument
Read more
Source
§
fn
process_group
(&mut self, pgroup:
i32
) -> &mut
Command
Sets the process group ID (PGID) of the child process. Equivalent to a
setpgid
call in the child process, but may be more efficient.
Read more
1.15.0
·
Source
§
unsafe fn
before_exec
<F>(&mut self, f: F) -> &mut
Command
where
    F:
FnMut
() ->
Result
<
()
> +
Send
+
Sync
+ 'static,
👎
Deprecated since 1.37.0: should be unsafe, use
pre_exec
instead
Schedules a closure to be run just before the
exec
function is
invoked.
Read more
Source
§
impl
CommandExt
for
Command
Available on
Linux
only.
Source
§
fn
create_pidfd
(&mut self, val:
bool
) -> &mut
Command
🔬
This is a nightly-only experimental API. (
linux_pidfd
#82971
)
Sets whether a
PidFd
should be created for the
Child
spawned by this
Command
.
By default, no pidfd will be created.
Read more
1.16.0
·
Source
§
impl
CommandExt
for
Command
Available on
Windows
only.
Source
§
fn
creation_flags
(&mut self, flags:
u32
) -> &mut
Command
Sets the
process creation flags
to be passed to
CreateProcess
.
Read more
Source
§
fn
show_window
(&mut self, cmd_show:
u16
) -> &mut
Command
🔬
This is a nightly-only experimental API. (
windows_process_extensions_show_window
#127544
)
Sets the field
wShowWindow
of
STARTUPINFO
that is passed to
CreateProcess
.
Allowed values are the ones listed in
https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
Source
§
fn
force_quotes
(&mut self, enabled:
bool
) -> &mut
Command
🔬
This is a nightly-only experimental API. (
windows_process_extensions_force_quotes
#82227
)
Forces all arguments to be wrapped in quote (
"
) characters.
Read more
Source
§
fn
raw_arg
<S:
AsRef
<
OsStr
>>(&mut self, raw_text: S) -> &mut
Command
Append literal text to the command line without any quoting or escaping.
Read more
Source
§
fn
async_pipes
(&mut self, always_async:
bool
) -> &mut
Command
🔬
This is a nightly-only experimental API. (
windows_process_extensions_async_pipes
#98289
)
When
process::Command
creates pipes, request that our side is always async.
Read more
Source
§
fn
spawn_with_attributes
(
    &mut self,
    attribute_list: &
ProcThreadAttributeList
<'_>,
) ->
Result
<
Child
>
🔬
This is a nightly-only experimental API. (
windows_process_extensions_raw_attribute
#114854
)
Executes the command as a child process with the given
ProcThreadAttributeList
, returning a handle to it.
Read more
1.0.0
·
Source
§
impl
Debug
for
Command
Source
§
fn
fmt
(&self, f: &mut
Formatter
<'_>) ->
Result
Format the program and arguments of a Command for display. Any
non-utf8 data is lossily converted using the utf8 replacement
character.
The default format approximates a shell invocation of the program along with its
arguments. It does not include most of the other command properties. The output is not guaranteed to work
(e.g. due to lack of shell-escaping or differences in path resolution).
On some platforms you can use
the alternate syntax
to show more fields.
Note that the debug implementation is platform-specific.
Auto Trait Implementations
§
§
impl
Freeze
for
Command
§
impl !
RefUnwindSafe
for
Command
§
impl
Send
for
Command
§
impl
Sync
for
Command
§
impl
Unpin
for
Command
§
impl !
UnwindSafe
for
Command
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