Metadata in std::fs - Rust
std
::
fs
Struct
Metadata
Copy item path
1.0.0
·
Source
pub struct Metadata(
/* private fields */
);
Expand description
Metadata information about a file.
This structure is returned from the
metadata
or
symlink_metadata
function or method and represents known
metadata about a file such as its permissions, size, modification
times, etc.
Implementations
§
Source
§
impl
Metadata
1.1.0
·
Source
pub fn
file_type
(&self) ->
FileType
Returns the file type for this metadata.
§
Examples
fn
main() -> std::io::Result<()> {
use
std::fs;
let
metadata = fs::metadata(
"foo.txt"
)
?
;
println!
(
"{:?}"
, metadata.file_type());
Ok
(())
}
1.0.0
·
Source
pub fn
is_dir
(&self) ->
bool
Returns
true
if this metadata is for a directory. The
result is mutually exclusive to the result of
Metadata::is_file
, and will be false for symlink metadata
obtained from
symlink_metadata
.
§
Examples
fn
main() -> std::io::Result<()> {
use
std::fs;
let
metadata = fs::metadata(
"foo.txt"
)
?
;
assert!
(!metadata.is_dir());
Ok
(())
}
1.0.0
·
Source
pub fn
is_file
(&self) ->
bool
Returns
true
if this metadata is for a regular file. The
result is mutually exclusive to the result of
Metadata::is_dir
, and will be false for symlink metadata
obtained from
symlink_metadata
.
When the goal is simply to read from (or write to) the source, the most
reliable way to test the source can be read (or written to) is to open
it. Only using
is_file
can break workflows like
diff <( prog_a )
on
a Unix-like system for example. See
File::open
or
OpenOptions::open
for more information.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
metadata = fs::metadata(
"foo.txt"
)
?
;
assert!
(metadata.is_file());
Ok
(())
}
1.58.0
·
Source
pub fn
is_symlink
(&self) ->
bool
Returns
true
if this metadata is for a symbolic link.
§
Examples
use
std::fs;
use
std::path::Path;
use
std::os::unix::fs::symlink;
fn
main() -> std::io::Result<()> {
let
link_path = Path::new(
"link"
);
    symlink(
"/origin_does_not_exist/"
, link_path)
?
;
let
metadata = fs::symlink_metadata(link_path)
?
;
assert!
(metadata.is_symlink());
Ok
(())
}
1.0.0
·
Source
pub fn
len
(&self) ->
u64
Returns the size of the file, in bytes, this metadata is for.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
metadata = fs::metadata(
"foo.txt"
)
?
;
assert_eq!
(
0
, metadata.len());
Ok
(())
}
1.0.0
·
Source
pub fn
permissions
(&self) ->
Permissions
Returns the permissions of the file this metadata is for.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
metadata = fs::metadata(
"foo.txt"
)
?
;
assert!
(!metadata.permissions().readonly());
Ok
(())
}
1.10.0
·
Source
pub fn
modified
(&self) ->
Result
<
SystemTime
>
Returns the last modification time listed in this metadata.
The returned value corresponds to the
mtime
field of
stat
on Unix
platforms and the
ftLastWriteTime
field on Windows platforms.
§
Errors
This field might not be available on all platforms, and will return an
Err
on platforms where it is not available.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
metadata = fs::metadata(
"foo.txt"
)
?
;
if let
Ok
(time) = metadata.modified() {
println!
(
"{time:?}"
);
    }
else
{
println!
(
"Not supported on this platform"
);
    }
Ok
(())
}
1.10.0
·
Source
pub fn
accessed
(&self) ->
Result
<
SystemTime
>
Returns the last access time of this metadata.
The returned value corresponds to the
atime
field of
stat
on Unix
platforms and the
ftLastAccessTime
field on Windows platforms.
Note that not all platforms will keep this field update in a file’s
metadata, for example Windows has an option to disable updating this
time when files are accessed and Linux similarly has
noatime
.
§
Errors
This field might not be available on all platforms, and will return an
Err
on platforms where it is not available.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
metadata = fs::metadata(
"foo.txt"
)
?
;
if let
Ok
(time) = metadata.accessed() {
println!
(
"{time:?}"
);
    }
else
{
println!
(
"Not supported on this platform"
);
    }
Ok
(())
}
1.10.0
·
Source
pub fn
created
(&self) ->
Result
<
SystemTime
>
Returns the creation time listed in this metadata.
The returned value corresponds to the
btime
field of
statx
on
Linux kernel starting from to 4.11, the
birthtime
field of
stat
on other
Unix platforms, and the
ftCreationTime
field on Windows platforms.
§
Errors
This field might not be available on all platforms, and will return an
Err
on platforms or filesystems where it is not available.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
metadata = fs::metadata(
"foo.txt"
)
?
;
if let
Ok
(time) = metadata.created() {
println!
(
"{time:?}"
);
    }
else
{
println!
(
"Not supported on this platform or filesystem"
);
    }
Ok
(())
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
Metadata
Source
§
fn
clone
(&self) ->
Metadata
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
1.16.0
·
Source
§
impl
Debug
for
Metadata
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
MetadataExt
for
Metadata
Available on
Apple
only.
Source
§
fn
as_raw_stat
(&self) -> &
stat
👎
Deprecated since 1.8.0: deprecated in favor of the accessor methods of this trait
Available on
macOS or iOS
only.
Gain a reference to the underlying
stat
structure which contains
the raw information returned by the OS.
Read more
Source
§
fn
st_dev
(&self) ->
u64
Source
§
fn
st_ino
(&self) ->
u64
Source
§
fn
st_mode
(&self) ->
u32
Source
§
fn
st_nlink
(&self) ->
u64
Source
§
fn
st_uid
(&self) ->
u32
Source
§
fn
st_gid
(&self) ->
u32
Source
§
fn
st_rdev
(&self) ->
u64
Source
§
fn
st_size
(&self) ->
u64
Source
§
fn
st_atime
(&self) ->
i64
Source
§
fn
st_atime_nsec
(&self) ->
i64
Source
§
fn
st_mtime
(&self) ->
i64
Source
§
fn
st_mtime_nsec
(&self) ->
i64
Source
§
fn
st_ctime
(&self) ->
i64
Source
§
fn
st_ctime_nsec
(&self) ->
i64
Source
§
fn
st_birthtime
(&self) ->
i64
Source
§
fn
st_birthtime_nsec
(&self) ->
i64
Source
§
fn
st_blksize
(&self) ->
u64
Source
§
fn
st_blocks
(&self) ->
u64
Source
§
fn
st_gen
(&self) ->
u32
Source
§
fn
st_flags
(&self) ->
u32
Source
§
fn
st_lspare
(&self) ->
u32
Source
§
fn
st_qspare
(&self) -> [
u64
;
2
]
1.1.0
·
Source
§
impl
MetadataExt
for
Metadata
Available on
Unix
only.
Source
§
fn
dev
(&self) ->
u64
Returns the ID of the device containing the file.
Read more
Source
§
fn
ino
(&self) ->
u64
Returns the inode number.
Read more
Source
§
fn
mode
(&self) ->
u32
Returns the rights applied to this file.
Read more
Source
§
fn
nlink
(&self) ->
u64
Returns the number of hard links pointing to this file.
Read more
Source
§
fn
uid
(&self) ->
u32
Returns the user ID of the owner of this file.
Read more
Source
§
fn
gid
(&self) ->
u32
Returns the group ID of the owner of this file.
Read more
Source
§
fn
rdev
(&self) ->
u64
Returns the device ID of this file (if it is a special one).
Read more
Source
§
fn
size
(&self) ->
u64
Returns the total size of this file in bytes.
Read more
Source
§
fn
atime
(&self) ->
i64
Returns the last access time of the file, in seconds since Unix Epoch.
Read more
Source
§
fn
atime_nsec
(&self) ->
i64
Returns the last access time of the file, in nanoseconds since
atime
.
Read more
Source
§
fn
mtime
(&self) ->
i64
Returns the last modification time of the file, in seconds since Unix Epoch.
Read more
Source
§
fn
mtime_nsec
(&self) ->
i64
Returns the last modification time of the file, in nanoseconds since
mtime
.
Read more
Source
§
fn
ctime
(&self) ->
i64
Returns the last status change time of the file, in seconds since Unix Epoch.
Read more
Source
§
fn
ctime_nsec
(&self) ->
i64
Returns the last status change time of the file, in nanoseconds since
ctime
.
Read more
Source
§
fn
blksize
(&self) ->
u64
Returns the block size for filesystem I/O.
Read more
Source
§
fn
blocks
(&self) ->
u64
Returns the number of blocks allocated to the file, in 512-byte units.
Read more
1.1.0
·
Source
§
impl
MetadataExt
for
Metadata
Available on
Linux
only.
Source
§
fn
as_raw_stat
(&self) -> &
stat
👎
Deprecated since 1.8.0: other methods of this trait are now preferred
Gain a reference to the underlying
stat
structure which contains
the raw information returned by the OS.
Read more
Source
§
fn
st_dev
(&self) ->
u64
Returns the device ID on which this file resides.
Read more
Source
§
fn
st_ino
(&self) ->
u64
Returns the inode number.
Read more
Source
§
fn
st_mode
(&self) ->
u32
Returns the file type and mode.
Read more
Source
§
fn
st_nlink
(&self) ->
u64
Returns the number of hard links to file.
Read more
Source
§
fn
st_uid
(&self) ->
u32
Returns the user ID of the file owner.
Read more
Source
§
fn
st_gid
(&self) ->
u32
Returns the group ID of the file owner.
Read more
Source
§
fn
st_rdev
(&self) ->
u64
Returns the device ID that this file represents. Only relevant for special file.
Read more
Source
§
fn
st_size
(&self) ->
u64
Returns the size of the file (if it is a regular file or a symbolic link) in bytes.
Read more
Source
§
fn
st_atime
(&self) ->
i64
Returns the last access time of the file, in seconds since Unix Epoch.
Read more
Source
§
fn
st_atime_nsec
(&self) ->
i64
Returns the last access time of the file, in nanoseconds since
st_atime
.
Read more
Source
§
fn
st_mtime
(&self) ->
i64
Returns the last modification time of the file, in seconds since Unix Epoch.
Read more
Source
§
fn
st_mtime_nsec
(&self) ->
i64
Returns the last modification time of the file, in nanoseconds since
st_mtime
.
Read more
Source
§
fn
st_ctime
(&self) ->
i64
Returns the last status change time of the file, in seconds since Unix Epoch.
Read more
Source
§
fn
st_ctime_nsec
(&self) ->
i64
Returns the last status change time of the file, in nanoseconds since
st_ctime
.
Read more
Source
§
fn
st_blksize
(&self) ->
u64
Returns the “preferred” block size for efficient filesystem I/O.
Read more
Source
§
fn
st_blocks
(&self) ->
u64
Returns the number of blocks allocated to the file, 512-byte units.
Read more
Source
§
impl
MetadataExt
for
Metadata
Available on
WASI
only.
Source
§
fn
dev
(&self) ->
u64
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns the
st_dev
field of the internal
filestat_t
Source
§
fn
ino
(&self) ->
u64
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns the
st_ino
field of the internal
filestat_t
Source
§
fn
nlink
(&self) ->
u64
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns the
st_nlink
field of the internal
filestat_t
Source
§
fn
size
(&self) ->
u64
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns the
st_size
field of the internal
filestat_t
Source
§
fn
atim
(&self) ->
u64
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns the
st_atim
field of the internal
filestat_t
Source
§
fn
mtim
(&self) ->
u64
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns the
st_mtim
field of the internal
filestat_t
Source
§
fn
ctim
(&self) ->
u64
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns the
st_ctim
field of the internal
filestat_t
1.1.0
·
Source
§
impl
MetadataExt
for
Metadata
Available on
Windows
only.
Source
§
fn
file_attributes
(&self) ->
u32
Returns the value of the
dwFileAttributes
field of this metadata.
Read more
Source
§
fn
creation_time
(&self) ->
u64
Returns the value of the
ftCreationTime
field of this metadata.
Read more
Source
§
fn
last_access_time
(&self) ->
u64
Returns the value of the
ftLastAccessTime
field of this metadata.
Read more
Source
§
fn
last_write_time
(&self) ->
u64
Returns the value of the
ftLastWriteTime
field of this metadata.
Read more
Source
§
fn
file_size
(&self) ->
u64
Returns the value of the
nFileSize
fields of this
metadata.
Read more
Source
§
fn
volume_serial_number
(&self) ->
Option
<
u32
>
🔬
This is a nightly-only experimental API. (
windows_by_handle
#63010
)
Returns the value of the
dwVolumeSerialNumber
field of this
metadata.
Read more
Source
§
fn
number_of_links
(&self) ->
Option
<
u32
>
🔬
This is a nightly-only experimental API. (
windows_by_handle
#63010
)
Returns the value of the
nNumberOfLinks
field of this
metadata.
Read more
Source
§
fn
file_index
(&self) ->
Option
<
u64
>
🔬
This is a nightly-only experimental API. (
windows_by_handle
#63010
)
Returns the value of the
nFileIndex
fields of this
metadata.
Read more
Source
§
fn
change_time
(&self) ->
Option
<
u64
>
🔬
This is a nightly-only experimental API. (
windows_change_time
#121478
)
Returns the value of the
ChangeTime
fields of this metadata.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
Metadata
§
impl
RefUnwindSafe
for
Metadata
§
impl
Send
for
Metadata
§
impl
Sync
for
Metadata
§
impl
Unpin
for
Metadata
§
impl
UnwindSafe
for
Metadata
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