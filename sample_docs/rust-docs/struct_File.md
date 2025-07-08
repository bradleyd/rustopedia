File in std::fs - Rust
std
::
fs
Struct
File
Copy item path
1.0.0
·
Source
pub struct File {
/* private fields */
}
Expand description
An object providing access to an open file on the filesystem.
An instance of a
File
can be read and/or written depending on what options
it was opened with. Files also implement
Seek
to alter the logical cursor
that the file contains internally.
Files are automatically closed when they go out of scope.  Errors detected
on closing are ignored by the implementation of
Drop
.  Use the method
sync_all
if these errors must be manually handled.
File
does not buffer reads and writes. For efficiency, consider wrapping the
file in a
BufReader
or
BufWriter
when performing many small
read
or
write
calls, unless unbuffered reads and writes are required.
§
Examples
Creates a new file and write bytes to it (you can also use
write
):
use
std::fs::File;
use
std::io::prelude::
*
;
fn
main() -> std::io::Result<()> {
let
mut
file = File::create(
"foo.txt"
)
?
;
    file.write_all(
b"Hello, world!"
)
?
;
Ok
(())
}
Reads the contents of a file into a
String
(you can also use
read
):
use
std::fs::File;
use
std::io::prelude::
*
;
fn
main() -> std::io::Result<()> {
let
mut
file = File::open(
"foo.txt"
)
?
;
let
mut
contents = String::new();
    file.read_to_string(
&mut
contents)
?
;
assert_eq!
(contents,
"Hello, world!"
);
Ok
(())
}
Using a buffered
Read
er:
use
std::fs::File;
use
std::io::BufReader;
use
std::io::prelude::
*
;
fn
main() -> std::io::Result<()> {
let
file = File::open(
"foo.txt"
)
?
;
let
mut
buf_reader = BufReader::new(file);
let
mut
contents = String::new();
    buf_reader.read_to_string(
&mut
contents)
?
;
assert_eq!
(contents,
"Hello, world!"
);
Ok
(())
}
Note that, although read and write methods require a
&mut File
, because
of the interfaces for
Read
and
Write
, the holder of a
&File
can
still modify the file, either through methods that take
&File
or by
retrieving the underlying OS object and modifying the file that way.
Additionally, many operating systems allow concurrent modification of files
by different processes. Avoid assuming that holding a
&File
means that the
file will not change.
§
Platform-specific behavior
On Windows, the implementation of
Read
and
Write
traits for
File
perform synchronous I/O operations. Therefore the underlying file must not
have been opened for asynchronous I/O (e.g. by using
FILE_FLAG_OVERLAPPED
).
Implementations
§
Source
§
impl
File
1.0.0
·
Source
pub fn
open
<P:
AsRef
<
Path
>>(path: P) ->
Result
<
File
>
Attempts to open a file in read-only mode.
See the
OpenOptions::open
method for more details.
If you only need to read the entire file contents,
consider
std::fs::read()
or
std::fs::read_to_string()
instead.
§
Errors
This function will return an error if
path
does not already exist.
Other errors may also be returned according to
OpenOptions::open
.
§
Examples
use
std::fs::File;
use
std::io::Read;
fn
main() -> std::io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
let
mut
data =
vec!
[];
    f.read_to_end(
&mut
data)
?
;
Ok
(())
}
Source
pub fn
open_buffered
<P:
AsRef
<
Path
>>(path: P) ->
Result
<
BufReader
<
File
>>
🔬
This is a nightly-only experimental API. (
file_buffered
#130804
)
Attempts to open a file in read-only mode with buffering.
See the
OpenOptions::open
method, the
BufReader
type,
and the
BufRead
trait for more details.
If you only need to read the entire file contents,
consider
std::fs::read()
or
std::fs::read_to_string()
instead.
§
Errors
This function will return an error if
path
does not already exist,
or if memory allocation fails for the new buffer.
Other errors may also be returned according to
OpenOptions::open
.
§
Examples
#![feature(file_buffered)]
use
std::fs::File;
use
std::io::BufRead;
fn
main() -> std::io::Result<()> {
let
mut
f = File::open_buffered(
"foo.txt"
)
?
;
assert!
(f.capacity() >
0
);
for
(line, i)
in
f.lines().zip(
1
..) {
println!
(
"{i:6}: {}"
, line
?
);
    }
Ok
(())
}
1.0.0
·
Source
pub fn
create
<P:
AsRef
<
Path
>>(path: P) ->
Result
<
File
>
Opens a file in write-only mode.
This function will create a file if it does not exist,
and will truncate it if it does.
Depending on the platform, this function may fail if the
full directory path does not exist.
See the
OpenOptions::open
function for more details.
See also
std::fs::write()
for a simple function to
create a file with some given data.
§
Examples
use
std::fs::File;
use
std::io::Write;
fn
main() -> std::io::Result<()> {
let
mut
f = File::create(
"foo.txt"
)
?
;
    f.write_all(
&
1234_u32
.to_be_bytes())
?
;
Ok
(())
}
Source
pub fn
create_buffered
<P:
AsRef
<
Path
>>(path: P) ->
Result
<
BufWriter
<
File
>>
🔬
This is a nightly-only experimental API. (
file_buffered
#130804
)
Opens a file in write-only mode with buffering.
This function will create a file if it does not exist,
and will truncate it if it does.
Depending on the platform, this function may fail if the
full directory path does not exist.
See the
OpenOptions::open
method and the
BufWriter
type for more details.
See also
std::fs::write()
for a simple function to
create a file with some given data.
§
Examples
#![feature(file_buffered)]
use
std::fs::File;
use
std::io::Write;
fn
main() -> std::io::Result<()> {
let
mut
f = File::create_buffered(
"foo.txt"
)
?
;
assert!
(f.capacity() >
0
);
for
i
in
0
..
100
{
writeln!
(
&mut
f,
"{i}"
)
?
;
    }
    f.flush()
?
;
Ok
(())
}
1.77.0
·
Source
pub fn
create_new
<P:
AsRef
<
Path
>>(path: P) ->
Result
<
File
>
Creates a new file in read-write mode; error if the file exists.
This function will create a file if it does not exist, or return an error if it does. This
way, if the call succeeds, the file returned is guaranteed to be new.
If a file exists at the target location, creating a new file will fail with
AlreadyExists
or another error based on the situation. See
OpenOptions::open
for a
non-exhaustive list of likely errors.
This option is useful because it is atomic. Otherwise between checking whether a file
exists and creating a new one, the file may have been created by another process (a TOCTOU
race condition / attack).
This can also be written using
File::options().read(true).write(true).create_new(true).open(...)
.
§
Examples
use
std::fs::File;
use
std::io::Write;
fn
main() -> std::io::Result<()> {
let
mut
f = File::create_new(
"foo.txt"
)
?
;
    f.write_all(
"Hello, world!"
.as_bytes())
?
;
Ok
(())
}
1.58.0
·
Source
pub fn
options
() ->
OpenOptions
Returns a new OpenOptions object.
This function returns a new OpenOptions object that you can use to
open or create a file with specific options if
open()
or
create()
are not appropriate.
It is equivalent to
OpenOptions::new()
, but allows you to write more
readable code. Instead of
OpenOptions::new().append(true).open("example.log")
,
you can write
File::options().append(true).open("example.log")
. This
also avoids the need to import
OpenOptions
.
See the
OpenOptions::new
function for more details.
§
Examples
use
std::fs::File;
use
std::io::Write;
fn
main() -> std::io::Result<()> {
let
mut
f = File::options().append(
true
).open(
"example.log"
)
?
;
writeln!
(
&mut
f,
"new line"
)
?
;
Ok
(())
}
1.0.0
·
Source
pub fn
sync_all
(&self) ->
Result
<
()
>
Attempts to sync all OS-internal file content and metadata to disk.
This function will attempt to ensure that all in-memory data reaches the
filesystem before returning.
This can be used to handle errors that would otherwise only be caught
when the
File
is closed, as dropping a
File
will ignore all errors.
Note, however, that
sync_all
is generally more expensive than closing
a file by dropping it, because the latter is not required to block until
the data has been written to the filesystem.
If synchronizing the metadata is not required, use
sync_data
instead.
§
Examples
use
std::fs::File;
use
std::io::prelude::
*
;
fn
main() -> std::io::Result<()> {
let
mut
f = File::create(
"foo.txt"
)
?
;
    f.write_all(
b"Hello, world!"
)
?
;

    f.sync_all()
?
;
Ok
(())
}
1.0.0
·
Source
pub fn
sync_data
(&self) ->
Result
<
()
>
This function is similar to
sync_all
, except that it might not
synchronize file metadata to the filesystem.
This is intended for use cases that must synchronize content, but don’t
need the metadata on disk. The goal of this method is to reduce disk
operations.
Note that some platforms may simply implement this in terms of
sync_all
.
§
Examples
use
std::fs::File;
use
std::io::prelude::
*
;
fn
main() -> std::io::Result<()> {
let
mut
f = File::create(
"foo.txt"
)
?
;
    f.write_all(
b"Hello, world!"
)
?
;

    f.sync_data()
?
;
Ok
(())
}
Source
pub fn
lock
(&self) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
file_lock
#130994
)
Acquire an exclusive lock on the file. Blocks until the lock can be acquired.
This acquires an exclusive lock; no other file handle to this file may acquire another lock.
This lock may be advisory or mandatory. This lock is meant to interact with
lock
,
try_lock
,
lock_shared
,
try_lock_shared
, and
unlock
. Its interactions with
other methods, such as
read
and
write
are platform specific, and it may or may not
cause non-lockholders to block.
If this file handle/descriptor, or a clone of it, already holds an lock the exact behavior
is unspecified and platform dependent, including the possibility that it will deadlock.
However, if this method returns, then an exclusive lock is held.
If the file not open for writing, it is unspecified whether this function returns an error.
The lock will be released when this file (along with any other file descriptors/handles
duplicated or inherited from it) is closed, or if the
unlock
method is called.
§
Platform-specific behavior
This function currently corresponds to the
flock
function on Unix with the
LOCK_EX
flag,
and the
LockFileEx
function on Windows with the
LOCKFILE_EXCLUSIVE_LOCK
flag. Note that,
this
may change in the future
.
On Windows, locking a file will fail if the file is opened only for append. To lock a file,
open it with one of
.read(true)
,
.read(true).append(true)
, or
.write(true)
.
§
Examples
#![feature(file_lock)]
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
f = File::create(
"foo.txt"
)
?
;
    f.lock()
?
;
Ok
(())
}
Source
pub fn
lock_shared
(&self) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
file_lock
#130994
)
Acquire a shared (non-exclusive) lock on the file. Blocks until the lock can be acquired.
This acquires a shared lock; more than one file handle may hold a shared lock, but none may
hold an exclusive lock at the same time.
This lock may be advisory or mandatory. This lock is meant to interact with
lock
,
try_lock
,
lock_shared
,
try_lock_shared
, and
unlock
. Its interactions with
other methods, such as
read
and
write
are platform specific, and it may or may not
cause non-lockholders to block.
If this file handle/descriptor, or a clone of it, already holds an lock, the exact behavior
is unspecified and platform dependent, including the possibility that it will deadlock.
However, if this method returns, then a shared lock is held.
The lock will be released when this file (along with any other file descriptors/handles
duplicated or inherited from it) is closed, or if the
unlock
method is called.
§
Platform-specific behavior
This function currently corresponds to the
flock
function on Unix with the
LOCK_SH
flag,
and the
LockFileEx
function on Windows. Note that, this
may change in the future
.
On Windows, locking a file will fail if the file is opened only for append. To lock a file,
open it with one of
.read(true)
,
.read(true).append(true)
, or
.write(true)
.
§
Examples
#![feature(file_lock)]
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
f = File::open(
"foo.txt"
)
?
;
    f.lock_shared()
?
;
Ok
(())
}
Source
pub fn
try_lock
(&self) ->
Result
<
bool
>
🔬
This is a nightly-only experimental API. (
file_lock
#130994
)
Try to acquire an exclusive lock on the file.
Returns
Ok(false)
if a different lock is already held on this file (via another
handle/descriptor).
This acquires an exclusive lock; no other file handle to this file may acquire another lock.
This lock may be advisory or mandatory. This lock is meant to interact with
lock
,
try_lock
,
lock_shared
,
try_lock_shared
, and
unlock
. Its interactions with
other methods, such as
read
and
write
are platform specific, and it may or may not
cause non-lockholders to block.
If this file handle/descriptor, or a clone of it, already holds an lock, the exact behavior
is unspecified and platform dependent, including the possibility that it will deadlock.
However, if this method returns
Ok(true)
, then it has acquired an exclusive lock.
If the file not open for writing, it is unspecified whether this function returns an error.
The lock will be released when this file (along with any other file descriptors/handles
duplicated or inherited from it) is closed, or if the
unlock
method is called.
§
Platform-specific behavior
This function currently corresponds to the
flock
function on Unix with the
LOCK_EX
and
LOCK_NB
flags, and the
LockFileEx
function on Windows with the
LOCKFILE_EXCLUSIVE_LOCK
and
LOCKFILE_FAIL_IMMEDIATELY
flags. Note that, this
may change in the future
.
On Windows, locking a file will fail if the file is opened only for append. To lock a file,
open it with one of
.read(true)
,
.read(true).append(true)
, or
.write(true)
.
§
Examples
#![feature(file_lock)]
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
f = File::create(
"foo.txt"
)
?
;
    f.try_lock()
?
;
Ok
(())
}
Source
pub fn
try_lock_shared
(&self) ->
Result
<
bool
>
🔬
This is a nightly-only experimental API. (
file_lock
#130994
)
Try to acquire a shared (non-exclusive) lock on the file.
Returns
Ok(false)
if an exclusive lock is already held on this file (via another
handle/descriptor).
This acquires a shared lock; more than one file handle may hold a shared lock, but none may
hold an exclusive lock at the same time.
This lock may be advisory or mandatory. This lock is meant to interact with
lock
,
try_lock
,
lock_shared
,
try_lock_shared
, and
unlock
. Its interactions with
other methods, such as
read
and
write
are platform specific, and it may or may not
cause non-lockholders to block.
If this file handle, or a clone of it, already holds an lock, the exact behavior is
unspecified and platform dependent, including the possibility that it will deadlock.
However, if this method returns
Ok(true)
, then it has acquired a shared lock.
The lock will be released when this file (along with any other file descriptors/handles
duplicated or inherited from it) is closed, or if the
unlock
method is called.
§
Platform-specific behavior
This function currently corresponds to the
flock
function on Unix with the
LOCK_SH
and
LOCK_NB
flags, and the
LockFileEx
function on Windows with the
LOCKFILE_FAIL_IMMEDIATELY
flag. Note that, this
may change in the future
.
On Windows, locking a file will fail if the file is opened only for append. To lock a file,
open it with one of
.read(true)
,
.read(true).append(true)
, or
.write(true)
.
§
Examples
#![feature(file_lock)]
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
f = File::open(
"foo.txt"
)
?
;
    f.try_lock_shared()
?
;
Ok
(())
}
Source
pub fn
unlock
(&self) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
file_lock
#130994
)
Release all locks on the file.
All locks are released when the file (along with any other file descriptors/handles
duplicated or inherited from it) is closed. This method allows releasing locks without
closing the file.
If no lock is currently held via this file descriptor/handle, this method may return an
error, or may return successfully without taking any action.
§
Platform-specific behavior
This function currently corresponds to the
flock
function on Unix with the
LOCK_UN
flag,
and the
UnlockFile
function on Windows. Note that, this
may change in the future
.
On Windows, locking a file will fail if the file is opened only for append. To lock a file,
open it with one of
.read(true)
,
.read(true).append(true)
, or
.write(true)
.
§
Examples
#![feature(file_lock)]
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
f = File::open(
"foo.txt"
)
?
;
    f.lock()
?
;
    f.unlock()
?
;
Ok
(())
}
1.0.0
·
Source
pub fn
set_len
(&self, size:
u64
) ->
Result
<
()
>
Truncates or extends the underlying file, updating the size of
this file to become
size
.
If the
size
is less than the current file’s size, then the file will
be shrunk. If it is greater than the current file’s size, then the file
will be extended to
size
and have all of the intermediate data filled
in with 0s.
The file’s cursor isn’t changed. In particular, if the cursor was at the
end and the file is shrunk using this operation, the cursor will now be
past the end.
§
Errors
This function will return an error if the file is not opened for writing.
Also,
std::io::ErrorKind::InvalidInput
will be returned if the desired length would cause an overflow due to
the implementation specifics.
§
Examples
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
mut
f = File::create(
"foo.txt"
)
?
;
    f.set_len(
10
)
?
;
Ok
(())
}
Note that this method alters the content of the underlying file, even
though it takes
&self
rather than
&mut self
.
1.0.0
·
Source
pub fn
metadata
(&self) ->
Result
<
Metadata
>
Queries metadata about the underlying file.
§
Examples
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
mut
f = File::open(
"foo.txt"
)
?
;
let
metadata = f.metadata()
?
;
Ok
(())
}
1.9.0
·
Source
pub fn
try_clone
(&self) ->
Result
<
File
>
Creates a new
File
instance that shares the same underlying file handle
as the existing
File
instance. Reads, writes, and seeks will affect
both
File
instances simultaneously.
§
Examples
Creates two handles for a file named
foo.txt
:
use
std::fs::File;
fn
main() -> std::io::Result<()> {
let
mut
file = File::open(
"foo.txt"
)
?
;
let
file_copy = file.try_clone()
?
;
Ok
(())
}
Assuming there’s a file named
foo.txt
with contents
abcdef\n
, create
two handles, seek one of them, and read the remaining bytes from the
other handle:
use
std::fs::File;
use
std::io::SeekFrom;
use
std::io::prelude::
*
;
fn
main() -> std::io::Result<()> {
let
mut
file = File::open(
"foo.txt"
)
?
;
let
mut
file_copy = file.try_clone()
?
;

    file.seek(SeekFrom::Start(
3
))
?
;
let
mut
contents =
vec!
[];
    file_copy.read_to_end(
&mut
contents)
?
;
assert_eq!
(contents,
b"def\n"
);
Ok
(())
}
1.16.0
·
Source
pub fn
set_permissions
(&self, perm:
Permissions
) ->
Result
<
()
>
Changes the permissions on the underlying file.
§
Platform-specific behavior
This function currently corresponds to the
fchmod
function on Unix and
the
SetFileInformationByHandle
function on Windows. Note that, this
may change in the future
.
§
Errors
This function will return an error if the user lacks permission change
attributes on the underlying file. It may also return an error in other
os-specific unspecified cases.
§
Examples
fn
main() -> std::io::Result<()> {
use
std::fs::File;
let
file = File::open(
"foo.txt"
)
?
;
let
mut
perms = file.metadata()
?
.permissions();
    perms.set_readonly(
true
);
    file.set_permissions(perms)
?
;
Ok
(())
}
Note that this method alters the permissions of the underlying file,
even though it takes
&self
rather than
&mut self
.
1.75.0
·
Source
pub fn
set_times
(&self, times:
FileTimes
) ->
Result
<
()
>
Changes the timestamps of the underlying file.
§
Platform-specific behavior
This function currently corresponds to the
futimens
function on Unix (falling back to
futimes
on macOS before 10.13) and the
SetFileTime
function on Windows. Note that this
may change in the future
.
§
Errors
This function will return an error if the user lacks permission to change timestamps on the
underlying file. It may also return an error in other os-specific unspecified cases.
This function may return an error if the operating system lacks support to change one or
more of the timestamps set in the
FileTimes
structure.
§
Examples
fn
main() -> std::io::Result<()> {
use
std::fs::{
self
, File, FileTimes};
let
src = fs::metadata(
"src"
)
?
;
let
dest = File::options().write(
true
).open(
"dest"
)
?
;
let
times = FileTimes::new()
        .set_accessed(src.accessed()
?
)
        .set_modified(src.modified()
?
);
    dest.set_times(times)
?
;
Ok
(())
}
1.75.0
·
Source
pub fn
set_modified
(&self, time:
SystemTime
) ->
Result
<
()
>
Changes the modification time of the underlying file.
This is an alias for
set_times(FileTimes::new().set_modified(time))
.
Trait Implementations
§
1.63.0
·
Source
§
impl
AsFd
for
File
Source
§
fn
as_fd
(&self) ->
BorrowedFd
<'_>
Borrows the file descriptor.
Read more
1.63.0
·
Source
§
impl
AsHandle
for
File
Available on
Windows
only.
Source
§
fn
as_handle
(&self) ->
BorrowedHandle
<'_>
Borrows the handle.
Read more
1.0.0
·
Source
§
impl
AsRawFd
for
File
Source
§
fn
as_raw_fd
(&self) ->
RawFd
Extracts the raw file descriptor.
Read more
1.0.0
·
Source
§
impl
AsRawHandle
for
File
Available on
Windows
only.
Source
§
fn
as_raw_handle
(&self) ->
RawHandle
Extracts the raw handle.
Read more
1.0.0
·
Source
§
impl
Debug
for
File
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
1.15.0
·
Source
§
impl
FileExt
for
File
Available on
Unix
only.
Source
§
fn
read_at
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
usize
>
Reads a number of bytes starting from a given offset.
Read more
Source
§
fn
read_vectored_at
(
    &self,
    bufs: &mut [
IoSliceMut
<'_>],
    offset:
u64
,
) ->
Result
<
usize
>
🔬
This is a nightly-only experimental API. (
unix_file_vectored_at
#89517
)
Like
read_at
, except that it reads into a slice of buffers.
Read more
Source
§
fn
write_at
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
usize
>
Writes a number of bytes starting from a given offset.
Read more
Source
§
fn
write_vectored_at
(&self, bufs: &[
IoSlice
<'_>], offset:
u64
) ->
Result
<
usize
>
🔬
This is a nightly-only experimental API. (
unix_file_vectored_at
#89517
)
Like
write_at
, except that it writes from a slice of buffers.
Read more
1.33.0
·
Source
§
fn
read_exact_at
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
()
>
Reads the exact number of bytes required to fill
buf
from the given offset.
Read more
1.33.0
·
Source
§
fn
write_all_at
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
()
>
Attempts to write an entire buffer starting from a given offset.
Read more
Source
§
impl
FileExt
for
File
Available on
WASI
only.
Source
§
fn
read_vectored_at
(
    &self,
    bufs: &mut [
IoSliceMut
<'_>],
    offset:
u64
,
) ->
Result
<
usize
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Reads a number of bytes starting from a given offset.
Read more
Source
§
fn
write_vectored_at
(&self, bufs: &[
IoSlice
<'_>], offset:
u64
) ->
Result
<
usize
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Writes a number of bytes starting from a given offset.
Read more
Source
§
fn
fdstat_set_flags
(&self, flags:
u16
) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Adjusts the flags associated with this file.
Read more
Source
§
fn
fdstat_set_rights
(&self, rights:
u64
, inheriting:
u64
) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Adjusts the rights associated with this file.
Read more
Source
§
fn
advise
(&self, offset:
u64
, len:
u64
, advice:
u8
) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Provides file advisory information on a file descriptor.
Read more
Source
§
fn
allocate
(&self, offset:
u64
, len:
u64
) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Forces the allocation of space in a file.
Read more
Source
§
fn
create_directory
<P:
AsRef
<
Path
>>(&self, dir: P) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Creates a directory.
Read more
Source
§
fn
read_link
<P:
AsRef
<
Path
>>(&self, path: P) ->
Result
<
PathBuf
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Reads the contents of a symbolic link.
Read more
Source
§
fn
metadata_at
<P:
AsRef
<
Path
>>(
    &self,
    lookup_flags:
u32
,
    path: P,
) ->
Result
<
Metadata
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns the attributes of a file or directory.
Read more
Source
§
fn
remove_file
<P:
AsRef
<
Path
>>(&self, path: P) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Unlinks a file.
Read more
Source
§
fn
remove_directory
<P:
AsRef
<
Path
>>(&self, path: P) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Removes a directory.
Read more
Source
§
fn
read_at
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
usize
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Reads a number of bytes starting from a given offset.
Read more
1.33.0
·
Source
§
fn
read_exact_at
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
()
>
Reads the exact number of byte required to fill
buf
from the given offset.
Read more
Source
§
fn
write_at
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
usize
>
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Writes a number of bytes starting from a given offset.
Read more
1.33.0
·
Source
§
fn
write_all_at
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
()
>
Attempts to write an entire buffer starting from a given offset.
Read more
1.15.0
·
Source
§
impl
FileExt
for
File
Available on
Windows
only.
Source
§
fn
seek_read
(&self, buf: &mut [
u8
], offset:
u64
) ->
Result
<
usize
>
Seeks to a given position and reads a number of bytes.
Read more
Source
§
fn
seek_write
(&self, buf: &[
u8
], offset:
u64
) ->
Result
<
usize
>
Seeks to a given position and writes a number of bytes.
Read more
1.63.0
·
Source
§
impl
From
<
File
> for
OwnedFd
Source
§
fn
from
(file:
File
) ->
OwnedFd
Takes ownership of a
File
’s underlying file descriptor.
1.63.0
·
Source
§
impl
From
<
File
> for
OwnedHandle
Available on
Windows
only.
Source
§
fn
from
(file:
File
) ->
OwnedHandle
Takes ownership of a
File
’s underlying file handle.
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
File
Source
§
fn
from
(owned_fd:
OwnedFd
) -> Self
Returns a
File
that takes ownership of the given
file descriptor.
1.63.0
·
Source
§
impl
From
<
OwnedHandle
> for
File
Available on
Windows
only.
Source
§
fn
from
(owned:
OwnedHandle
) -> Self
Returns a
File
that takes ownership of the given handle.
1.1.0
·
Source
§
impl
FromRawFd
for
File
Source
§
unsafe fn
from_raw_fd
(fd:
RawFd
) ->
File
ⓘ
Constructs a new instance of
Self
from the given raw file
descriptor.
Read more
1.1.0
·
Source
§
impl
FromRawHandle
for
File
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
File
ⓘ
Constructs a new I/O object from the specified raw handle.
Read more
1.4.0
·
Source
§
impl
IntoRawFd
for
File
Source
§
fn
into_raw_fd
(self) ->
RawFd
Consumes this object, returning the raw underlying file descriptor.
Read more
1.4.0
·
Source
§
impl
IntoRawHandle
for
File
Available on
Windows
only.
Source
§
fn
into_raw_handle
(self) ->
RawHandle
Consumes this object, returning the raw underlying handle.
Read more
1.70.0
·
Source
§
impl
IsTerminal
for
File
Source
§
fn
is_terminal
(&self) ->
bool
Returns
true
if the descriptor/handle refers to a terminal/tty.
Read more
1.0.0
·
Source
§
impl
Read
for &
File
Source
§
fn
read
(&mut self, buf: &mut [
u8
]) ->
Result
<
usize
>
Reads some bytes from the file.
See
Read::read
docs for more info.
§
Platform-specific behavior
This function currently corresponds to the
read
function on Unix and
the
NtReadFile
function on Windows. Note that this
may change in
the future
.
Source
§
fn
read_vectored
(&mut self, bufs: &mut [
IoSliceMut
<'_>]) ->
Result
<
usize
>
Like
read
, except that it reads into a slice of buffers.
See
Read::read_vectored
docs for more info.
§
Platform-specific behavior
This function currently corresponds to the
readv
function on Unix and
falls back to the
read
implementation on Windows. Note that this
may change in the future
.
Source
§
fn
is_read_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if
File
has an efficient
read_vectored
implementation.
See
Read::is_read_vectored
docs for more info.
§
Platform-specific behavior
This function currently returns
true
on Unix an
false
on Windows.
Note that this
may change in the future
.
Source
§
fn
read_buf
(&mut self, cursor:
BorrowedCursor
<'_>) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
read_buf
#78485
)
Pull some bytes from this source into the specified buffer.
Read more
Source
§
fn
read_to_end
(&mut self, buf: &mut
Vec
<
u8
>) ->
Result
<
usize
>
Reads all bytes until EOF in this source, placing them into
buf
.
Read more
Source
§
fn
read_to_string
(&mut self, buf: &mut
String
) ->
Result
<
usize
>
Reads all bytes until EOF in this source, appending them to
buf
.
Read more
1.6.0
·
Source
§
fn
read_exact
(&mut self, buf: &mut [
u8
]) ->
Result
<
()
>
Reads the exact number of bytes required to fill
buf
.
Read more
Source
§
fn
read_buf_exact
(&mut self, cursor:
BorrowedCursor
<'_>) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
read_buf
#78485
)
Reads the exact number of bytes required to fill
cursor
.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adaptor for this instance of
Read
.
Read more
1.0.0
·
Source
§
fn
bytes
(self) ->
Bytes
<Self>
ⓘ
where
    Self:
Sized
,
Transforms this
Read
instance to an
Iterator
over its bytes.
Read more
1.0.0
·
Source
§
fn
chain
<R:
Read
>(self, next: R) ->
Chain
<Self, R>
ⓘ
where
    Self:
Sized
,
Creates an adapter which will chain this stream with another.
Read more
1.0.0
·
Source
§
fn
take
(self, limit:
u64
) ->
Take
<Self>
ⓘ
where
    Self:
Sized
,
Creates an adapter which will read at most
limit
bytes from it.
Read more
1.0.0
·
Source
§
impl
Read
for
File
Source
§
fn
read
(&mut self, buf: &mut [
u8
]) ->
Result
<
usize
>
Pull some bytes from this source into the specified buffer, returning
how many bytes were read.
Read more
Source
§
fn
read_vectored
(&mut self, bufs: &mut [
IoSliceMut
<'_>]) ->
Result
<
usize
>
Like
read
, except that it reads into a slice of buffers.
Read more
Source
§
fn
read_buf
(&mut self, cursor:
BorrowedCursor
<'_>) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
read_buf
#78485
)
Pull some bytes from this source into the specified buffer.
Read more
Source
§
fn
is_read_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if this
Read
er has an efficient
read_vectored
implementation.
Read more
Source
§
fn
read_to_end
(&mut self, buf: &mut
Vec
<
u8
>) ->
Result
<
usize
>
Reads all bytes until EOF in this source, placing them into
buf
.
Read more
Source
§
fn
read_to_string
(&mut self, buf: &mut
String
) ->
Result
<
usize
>
Reads all bytes until EOF in this source, appending them to
buf
.
Read more
1.6.0
·
Source
§
fn
read_exact
(&mut self, buf: &mut [
u8
]) ->
Result
<
()
>
Reads the exact number of bytes required to fill
buf
.
Read more
Source
§
fn
read_buf_exact
(&mut self, cursor:
BorrowedCursor
<'_>) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
read_buf
#78485
)
Reads the exact number of bytes required to fill
cursor
.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adaptor for this instance of
Read
.
Read more
1.0.0
·
Source
§
fn
bytes
(self) ->
Bytes
<Self>
ⓘ
where
    Self:
Sized
,
Transforms this
Read
instance to an
Iterator
over its bytes.
Read more
1.0.0
·
Source
§
fn
chain
<R:
Read
>(self, next: R) ->
Chain
<Self, R>
ⓘ
where
    Self:
Sized
,
Creates an adapter which will chain this stream with another.
Read more
1.0.0
·
Source
§
fn
take
(self, limit:
u64
) ->
Take
<Self>
ⓘ
where
    Self:
Sized
,
Creates an adapter which will read at most
limit
bytes from it.
Read more
1.0.0
·
Source
§
impl
Seek
for &
File
Source
§
fn
seek
(&mut self, pos:
SeekFrom
) ->
Result
<
u64
>
Seek to an offset, in bytes, in a stream.
Read more
Source
§
fn
stream_position
(&mut self) ->
Result
<
u64
>
Returns the current seek position from the start of the stream.
Read more
1.55.0
·
Source
§
fn
rewind
(&mut self) ->
Result
<
()
>
Rewind to the beginning of a stream.
Read more
Source
§
fn
stream_len
(&mut self) ->
Result
<
u64
>
🔬
This is a nightly-only experimental API. (
seek_stream_len
#59359
)
Returns the length of this stream (in bytes).
Read more
1.80.0
·
Source
§
fn
seek_relative
(&mut self, offset:
i64
) ->
Result
<
()
>
Seeks relative to the current position.
Read more
1.0.0
·
Source
§
impl
Seek
for
File
Source
§
fn
seek
(&mut self, pos:
SeekFrom
) ->
Result
<
u64
>
Seek to an offset, in bytes, in a stream.
Read more
Source
§
fn
stream_position
(&mut self) ->
Result
<
u64
>
Returns the current seek position from the start of the stream.
Read more
1.55.0
·
Source
§
fn
rewind
(&mut self) ->
Result
<
()
>
Rewind to the beginning of a stream.
Read more
Source
§
fn
stream_len
(&mut self) ->
Result
<
u64
>
🔬
This is a nightly-only experimental API. (
seek_stream_len
#59359
)
Returns the length of this stream (in bytes).
Read more
1.80.0
·
Source
§
fn
seek_relative
(&mut self, offset:
i64
) ->
Result
<
()
>
Seeks relative to the current position.
Read more
1.0.0
·
Source
§
impl
Write
for &
File
Source
§
fn
write
(&mut self, buf: &[
u8
]) ->
Result
<
usize
>
Writes some bytes to the file.
See
Write::write
docs for more info.
§
Platform-specific behavior
This function currently corresponds to the
write
function on Unix and
the
NtWriteFile
function on Windows. Note that this
may change in
the future
.
Source
§
fn
write_vectored
(&mut self, bufs: &[
IoSlice
<'_>]) ->
Result
<
usize
>
Like
write
, except that it writes into a slice of buffers.
See
Write::write_vectored
docs for more info.
§
Platform-specific behavior
This function currently corresponds to the
writev
function on Unix
and falls back to the
write
implementation on Windows. Note that this
may change in the future
.
Source
§
fn
is_write_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if
File
has an efficient
write_vectored
implementation.
See
Write::is_write_vectored
docs for more info.
§
Platform-specific behavior
This function currently returns
true
on Unix an
false
on Windows.
Note that this
may change in the future
.
Source
§
fn
flush
(&mut self) ->
Result
<
()
>
Flushes the file, ensuring that all intermediately buffered contents
reach their destination.
See
Write::flush
docs for more info.
§
Platform-specific behavior
Since a
File
structure doesn’t contain any buffers, this function is
currently a no-op on Unix and Windows. Note that this
may change in
the future
.
1.0.0
·
Source
§
fn
write_all
(&mut self, buf: &[
u8
]) ->
Result
<
()
>
Attempts to write an entire buffer into this writer.
Read more
Source
§
fn
write_all_vectored
(&mut self, bufs: &mut [
IoSlice
<'_>]) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
write_all_vectored
#70436
)
Attempts to write multiple buffers into this writer.
Read more
1.0.0
·
Source
§
fn
write_fmt
(&mut self, args:
Arguments
<'_>) ->
Result
<
()
>
Writes a formatted string into this writer, returning any error
encountered.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adapter for this instance of
Write
.
Read more
1.0.0
·
Source
§
impl
Write
for
File
Source
§
fn
write
(&mut self, buf: &[
u8
]) ->
Result
<
usize
>
Writes a buffer into this writer, returning how many bytes were written.
Read more
Source
§
fn
write_vectored
(&mut self, bufs: &[
IoSlice
<'_>]) ->
Result
<
usize
>
Like
write
, except that it writes from a slice of buffers.
Read more
Source
§
fn
is_write_vectored
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
can_vector
#69941
)
Determines if this
Write
r has an efficient
write_vectored
implementation.
Read more
Source
§
fn
flush
(&mut self) ->
Result
<
()
>
Flushes this output stream, ensuring that all intermediately buffered
contents reach their destination.
Read more
1.0.0
·
Source
§
fn
write_all
(&mut self, buf: &[
u8
]) ->
Result
<
()
>
Attempts to write an entire buffer into this writer.
Read more
Source
§
fn
write_all_vectored
(&mut self, bufs: &mut [
IoSlice
<'_>]) ->
Result
<
()
>
🔬
This is a nightly-only experimental API. (
write_all_vectored
#70436
)
Attempts to write multiple buffers into this writer.
Read more
1.0.0
·
Source
§
fn
write_fmt
(&mut self, args:
Arguments
<'_>) ->
Result
<
()
>
Writes a formatted string into this writer, returning any error
encountered.
Read more
1.0.0
·
Source
§
fn
by_ref
(&mut self) -> &mut Self
where
    Self:
Sized
,
Creates a “by reference” adapter for this instance of
Write
.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
File
§
impl
RefUnwindSafe
for
File
§
impl
Send
for
File
§
impl
Sync
for
File
§
impl
Unpin
for
File
§
impl
UnwindSafe
for
File
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