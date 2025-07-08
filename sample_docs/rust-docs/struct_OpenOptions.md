OpenOptions in std::fs - Rust
std
::
fs
Struct
OpenOptions
Copy item path
1.0.0
·
Source
pub struct OpenOptions(
/* private fields */
);
Expand description
Options and flags which can be used to configure how a file is opened.
This builder exposes the ability to configure how a
File
is opened and
what operations are permitted on the open file. The
File::open
and
File::create
methods are aliases for commonly used options using this
builder.
Generally speaking, when using
OpenOptions
, you’ll first call
OpenOptions::new
, then chain calls to methods to set each option, then
call
OpenOptions::open
, passing the path of the file you’re trying to
open. This will give you a
io::Result
with a
File
inside that you
can further operate on.
§
Examples
Opening a file to read:
use
std::fs::OpenOptions;
let
file = OpenOptions::new().read(
true
).open(
"foo.txt"
);
Opening a file for both reading and writing, as well as creating it if it
doesn’t exist:
use
std::fs::OpenOptions;
let
file = OpenOptions::new()
            .read(
true
)
            .write(
true
)
            .create(
true
)
            .open(
"foo.txt"
);
Implementations
§
Source
§
impl
OpenOptions
1.0.0
·
Source
pub fn
new
() -> Self
Creates a blank new set of options ready for configuration.
All options are initially set to
false
.
§
Examples
use
std::fs::OpenOptions;
let
mut
options = OpenOptions::new();
let
file = options.read(
true
).open(
"foo.txt"
);
1.0.0
·
Source
pub fn
read
(&mut self, read:
bool
) -> &mut Self
Sets the option for read access.
This option, when true, will indicate that the file should be
read
-able if opened.
§
Examples
use
std::fs::OpenOptions;
let
file = OpenOptions::new().read(
true
).open(
"foo.txt"
);
1.0.0
·
Source
pub fn
write
(&mut self, write:
bool
) -> &mut Self
Sets the option for write access.
This option, when true, will indicate that the file should be
write
-able if opened.
If the file already exists, any write calls on it will overwrite its
contents, without truncating it.
§
Examples
use
std::fs::OpenOptions;
let
file = OpenOptions::new().write(
true
).open(
"foo.txt"
);
1.0.0
·
Source
pub fn
append
(&mut self, append:
bool
) -> &mut Self
Sets the option for the append mode.
This option, when true, means that writes will append to a file instead
of overwriting previous contents.
Note that setting
.write(true).append(true)
has the same effect as
setting only
.append(true)
.
Append mode guarantees that writes will be positioned at the current end of file,
even when there are other processes or threads appending to the same file. This is
unlike
seek
(
SeekFrom
::
End
(0))
followed by
write()
, which
has a race between seeking and writing during which another writer can write, with
our
write()
overwriting their data.
Keep in mind that this does not necessarily guarantee that data appended by
different processes or threads does not interleave. The amount of data accepted a
single
write()
call depends on the operating system and file system. A
successful
write()
is allowed to write only part of the given data, so even if
you’re careful to provide the whole message in a single call to
write()
, there
is no guarantee that it will be written out in full. If you rely on the filesystem
accepting the message in a single write, make sure that all data that belongs
together is written in one operation. This can be done by concatenating strings
before passing them to
write()
.
If a file is opened with both read and append access, beware that after
opening, and after every write, the position for reading may be set at the
end of the file. So, before writing, save the current position (using
Seek
::
stream_position
), and restore it before the next read.
§
Note
This function doesn’t create the file if it doesn’t exist. Use the
OpenOptions::create
method to do so.
§
Examples
use
std::fs::OpenOptions;
let
file = OpenOptions::new().append(
true
).open(
"foo.txt"
);
1.0.0
·
Source
pub fn
truncate
(&mut self, truncate:
bool
) -> &mut Self
Sets the option for truncating a previous file.
If a file is successfully opened with this option set to true, it will truncate
the file to 0 length if it already exists.
The file must be opened with write access for truncate to work.
§
Examples
use
std::fs::OpenOptions;
let
file = OpenOptions::new().write(
true
).truncate(
true
).open(
"foo.txt"
);
1.0.0
·
Source
pub fn
create
(&mut self, create:
bool
) -> &mut Self
Sets the option to create a new file, or open it if it already exists.
In order for the file to be created,
OpenOptions::write
or
OpenOptions::append
access must be used.
See also
std::fs::write()
for a simple function to
create a file with some given data.
§
Examples
use
std::fs::OpenOptions;
let
file = OpenOptions::new().write(
true
).create(
true
).open(
"foo.txt"
);
1.9.0
·
Source
pub fn
create_new
(&mut self, create_new:
bool
) -> &mut Self
Sets the option to create a new file, failing if it already exists.
No file is allowed to exist at the target location, also no (dangling) symlink. In this
way, if the call succeeds, the file returned is guaranteed to be new.
If a file exists at the target location, creating a new file will fail with
AlreadyExists
or another error based on the situation. See
OpenOptions::open
for a
non-exhaustive list of likely errors.
This option is useful because it is atomic. Otherwise between checking
whether a file exists and creating a new one, the file may have been
created by another process (a TOCTOU race condition / attack).
If
.create_new(true)
is set,
.create()
and
.truncate()
are
ignored.
The file must be opened with write or append access in order to create
a new file.
§
Examples
use
std::fs::OpenOptions;
let
file = OpenOptions::new().write(
true
)
                             .create_new(
true
)
                             .open(
"foo.txt"
);
1.0.0
·
Source
pub fn
open
<P:
AsRef
<
Path
>>(&self, path: P) ->
Result
<
File
>
Opens a file at
path
with the options specified by
self
.
§
Errors
This function will return an error under a number of different
circumstances. Some of these error conditions are listed here, together
with their
io::ErrorKind
. The mapping to
io::ErrorKind
s is not
part of the compatibility contract of the function.
NotFound
: The specified file does not exist and neither
create
or
create_new
is set.
NotFound
: One of the directory components of the file path does
not exist.
PermissionDenied
: The user lacks permission to get the specified
access rights for the file.
PermissionDenied
: The user lacks permission to open one of the
directory components of the specified path.
AlreadyExists
:
create_new
was specified and the file already
exists.
InvalidInput
: Invalid combinations of open options (truncate
without write access, no access mode set, etc.).
The following errors don’t match any existing
io::ErrorKind
at the moment:
One of the directory components of the specified file path
was not, in fact, a directory.
Filesystem-level errors: full disk, write permission
requested on a read-only file system, exceeded disk quota, too many
open files, too long filename, too many symbolic links in the
specified path (Unix-like systems only), etc.
§
Examples
use
std::fs::OpenOptions;
let
file = OpenOptions::new().read(
true
).open(
"foo.txt"
);
Trait Implementations
§
1.0.0
·
Source
§
impl
Clone
for
OpenOptions
Source
§
fn
clone
(&self) ->
OpenOptions
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
OpenOptions
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
1.1.0
·
Source
§
impl
OpenOptionsExt
for
OpenOptions
Available on
Unix
only.
Source
§
fn
mode
(&mut self, mode:
u32
) -> &mut
OpenOptions
Sets the mode bits that a new file will be created with.
Read more
Source
§
fn
custom_flags
(&mut self, flags:
i32
) -> &mut
OpenOptions
Pass custom flags to the
flags
argument of
open
.
Read more
Source
§
impl
OpenOptionsExt
for
OpenOptions
Available on
WASI
only.
Source
§
fn
lookup_flags
(&mut self, flags:
u32
) -> &mut
OpenOptions
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Pass custom
dirflags
argument to
path_open
.
Read more
Source
§
fn
directory
(&mut self, dir:
bool
) -> &mut
OpenOptions
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Indicates whether
OpenOptions
must open a directory or not.
Read more
Source
§
fn
dsync
(&mut self, enabled:
bool
) -> &mut
OpenOptions
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Indicates whether
__WASI_FDFLAG_DSYNC
is passed in the
fs_flags
field of
path_open
.
Read more
Source
§
fn
nonblock
(&mut self, enabled:
bool
) -> &mut
OpenOptions
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Indicates whether
__WASI_FDFLAG_NONBLOCK
is passed in the
fs_flags
field of
path_open
.
Read more
Source
§
fn
rsync
(&mut self, enabled:
bool
) -> &mut
OpenOptions
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Indicates whether
__WASI_FDFLAG_RSYNC
is passed in the
fs_flags
field of
path_open
.
Read more
Source
§
fn
sync
(&mut self, enabled:
bool
) -> &mut
OpenOptions
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Indicates whether
__WASI_FDFLAG_SYNC
is passed in the
fs_flags
field of
path_open
.
Read more
Source
§
fn
fs_rights_base
(&mut self, rights:
u64
) -> &mut
OpenOptions
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Indicates the value that should be passed in for the
fs_rights_base
parameter of
path_open
.
Read more
Source
§
fn
fs_rights_inheriting
(&mut self, rights:
u64
) -> &mut
OpenOptions
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Indicates the value that should be passed in for the
fs_rights_inheriting
parameter of
path_open
.
Read more
Source
§
fn
open_at
<P:
AsRef
<
Path
>>(&self, file: &
File
, path: P) ->
Result
<
File
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Open a file or directory.
Read more
1.10.0
·
Source
§
impl
OpenOptionsExt
for
OpenOptions
Available on
Windows
only.
Source
§
fn
access_mode
(&mut self, access:
u32
) -> &mut
OpenOptions
Overrides the
dwDesiredAccess
argument to the call to
CreateFile
with the specified value.
Read more
Source
§
fn
share_mode
(&mut self, share:
u32
) -> &mut
OpenOptions
Overrides the
dwShareMode
argument to the call to
CreateFile
with
the specified value.
Read more
Source
§
fn
custom_flags
(&mut self, flags:
u32
) -> &mut
OpenOptions
Sets extra flags for the
dwFileFlags
argument to the call to
CreateFile2
to the specified value (or combines it with
attributes
and
security_qos_flags
to set the
dwFlagsAndAttributes
for
CreateFile
).
Read more
Source
§
fn
attributes
(&mut self, attributes:
u32
) -> &mut
OpenOptions
Sets the
dwFileAttributes
argument to the call to
CreateFile2
to
the specified value (or combines it with
custom_flags
and
security_qos_flags
to set the
dwFlagsAndAttributes
for
CreateFile
).
Read more
Source
§
fn
security_qos_flags
(&mut self, flags:
u32
) -> &mut
OpenOptions
Sets the
dwSecurityQosFlags
argument to the call to
CreateFile2
to
the specified value (or combines it with
custom_flags
and
attributes
to set the
dwFlagsAndAttributes
for
CreateFile
).
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
OpenOptions
§
impl
RefUnwindSafe
for
OpenOptions
§
impl
Send
for
OpenOptions
§
impl
Sync
for
OpenOptions
§
impl
Unpin
for
OpenOptions
§
impl
UnwindSafe
for
OpenOptions
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