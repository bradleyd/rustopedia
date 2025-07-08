DirEntry in std::fs - Rust
std
::
fs
Struct
DirEntry
Copy item path
1.0.0
·
Source
pub struct DirEntry(
/* private fields */
);
Expand description
Entries returned by the
ReadDir
iterator.
An instance of
DirEntry
represents an entry inside of a directory on the
filesystem. Each entry can be inspected via methods to learn about the full
path or possibly other metadata through per-platform extension traits.
§
Platform-specific behavior
On Unix, the
DirEntry
struct contains an internal reference to the open
directory. Holding
DirEntry
objects will consume a file handle even
after the
ReadDir
iterator is dropped.
Note that this
may change in the future
.
Implementations
§
Source
§
impl
DirEntry
1.0.0
·
Source
pub fn
path
(&self) ->
PathBuf
Returns the full path to the file that this entry represents.
The full path is created by joining the original path to
read_dir
with the filename of this entry.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
for
entry
in
fs::read_dir(
"."
)
?
{
let
dir = entry
?
;
println!
(
"{:?}"
, dir.path());
    }
Ok
(())
}
This prints output like:
"./whatever.txt"
"./foo.html"
"./hello_world.rs"
The exact text, of course, depends on what files you have in
.
.
1.1.0
·
Source
pub fn
metadata
(&self) ->
Result
<
Metadata
>
Returns the metadata for the file that this entry points at.
This function will not traverse symlinks if this entry points at a
symlink. To traverse symlinks use
fs::metadata
or
fs::File::metadata
.
§
Platform-specific behavior
On Windows this function is cheap to call (no extra system calls
needed), but on Unix platforms this function is the equivalent of
calling
symlink_metadata
on the path.
§
Examples
use
std::fs;
if let
Ok
(entries) = fs::read_dir(
"."
) {
for
entry
in
entries {
if let
Ok
(entry) = entry {
// Here, `entry` is a `DirEntry`.
if let
Ok
(metadata) = entry.metadata() {
// Now let's show our entry's permissions!
println!
(
"{:?}: {:?}"
, entry.path(), metadata.permissions());
            }
else
{
println!
(
"Couldn't get metadata for {:?}"
, entry.path());
            }
        }
    }
}
1.1.0
·
Source
pub fn
file_type
(&self) ->
Result
<
FileType
>
Returns the file type for the file that this entry points at.
This function will not traverse symlinks if this entry points at a
symlink.
§
Platform-specific behavior
On Windows and most Unix platforms this function is free (no extra
system calls needed), but some Unix platforms may require the equivalent
call to
symlink_metadata
to learn about the target file type.
§
Examples
use
std::fs;
if let
Ok
(entries) = fs::read_dir(
"."
) {
for
entry
in
entries {
if let
Ok
(entry) = entry {
// Here, `entry` is a `DirEntry`.
if let
Ok
(file_type) = entry.file_type() {
// Now let's show our entry's file type!
println!
(
"{:?}: {:?}"
, entry.path(), file_type);
            }
else
{
println!
(
"Couldn't get file type for {:?}"
, entry.path());
            }
        }
    }
}
1.1.0
·
Source
pub fn
file_name
(&self) ->
OsString
Returns the file name of this directory entry without any
leading path component(s).
As an example,
the output of the function will result in “foo” for all the following paths:
“./foo”
“/the/foo”
“../../foo”
§
Examples
use
std::fs;
if let
Ok
(entries) = fs::read_dir(
"."
) {
for
entry
in
entries {
if let
Ok
(entry) = entry {
// Here, `entry` is a `DirEntry`.
println!
(
"{:?}"
, entry.file_name());
        }
    }
}
Trait Implementations
§
1.13.0
·
Source
§
impl
Debug
for
DirEntry
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
DirEntryExt
for
DirEntry
Available on
Unix
only.
Source
§
fn
ino
(&self) ->
u64
Returns the underlying
d_ino
field in the contained
dirent
structure.
Read more
Source
§
impl
DirEntryExt
for
DirEntry
Available on
WASI
only.
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
Returns the underlying
d_ino
field of the
dirent_t
Source
§
impl
DirEntryExt2
for
DirEntry
Available on
Unix
only.
Source
§
fn
file_name_ref
(&self) -> &
OsStr
🔬
This is a nightly-only experimental API. (
dir_entry_ext2
#85573
)
Returns a reference to the underlying
OsStr
of this entry’s filename.
Read more
Auto Trait Implementations
§
§
impl
Freeze
for
DirEntry
§
impl
RefUnwindSafe
for
DirEntry
§
impl
Send
for
DirEntry
§
impl
Sync
for
DirEntry
§
impl
Unpin
for
DirEntry
§
impl
UnwindSafe
for
DirEntry
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