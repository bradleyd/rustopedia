Permissions in std::fs - Rust
std
::
fs
Struct
Permissions
Copy item path
1.0.0
·
Source
pub struct Permissions(
/* private fields */
);
Expand description
Representation of the various permissions on a file.
This module only currently provides one bit of information,
Permissions::readonly
, which is exposed on all currently supported
platforms. Unix-specific functionality, such as mode bits, is available
through the
PermissionsExt
trait.
Implementations
§
Source
§
impl
Permissions
1.0.0
·
Source
pub fn
readonly
(&self) ->
bool
Returns
true
if these permissions describe a readonly (unwritable) file.
§
Note
This function does not take Access Control Lists (ACLs), Unix group
membership and other nuances into account.
Therefore the return value of this function cannot be relied upon
to predict whether attempts to read or write the file will actually succeed.
§
Windows
On Windows this returns
FILE_ATTRIBUTE_READONLY
.
If
FILE_ATTRIBUTE_READONLY
is set then writes to the file will fail
but the user may still have permission to change this flag. If
FILE_ATTRIBUTE_READONLY
is
not
set then writes may still fail due
to lack of write permission.
The behavior of this attribute for directories depends on the Windows
version.
§
Unix (including macOS)
On Unix-based platforms this checks if
any
of the owner, group or others
write permission bits are set. It does not consider anything else, including:
Whether the current user is in the file’s assigned group.
Permissions granted by ACL.
That
root
user can write to files that do not have any write bits set.
Writable files on a filesystem that is mounted read-only.
The
PermissionsExt
trait gives direct access to the permission bits but
also does not read ACLs.
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
let
metadata = f.metadata()
?
;
assert_eq!
(
false
, metadata.permissions().readonly());
Ok
(())
}
1.0.0
·
Source
pub fn
set_readonly
(&mut self, readonly:
bool
)
Modifies the readonly flag for this set of permissions. If the
readonly
argument is
true
, using the resulting
Permission
will
update file permissions to forbid writing. Conversely, if it’s
false
,
using the resulting
Permission
will update file permissions to allow
writing.
This operation does
not
modify the files attributes. This only
changes the in-memory value of these attributes for this
Permissions
instance. To modify the files attributes use the
set_permissions
function which commits these attribute changes to the file.
§
Note
set_readonly(false)
makes the file
world-writable
on Unix.
You can use the
PermissionsExt
trait on Unix to avoid this issue.
It also does not take Access Control Lists (ACLs) or Unix group
membership into account.
§
Windows
On Windows this sets or clears
FILE_ATTRIBUTE_READONLY
.
If
FILE_ATTRIBUTE_READONLY
is set then writes to the file will fail
but the user may still have permission to change this flag. If
FILE_ATTRIBUTE_READONLY
is
not
set then the write may still fail if
the user does not have permission to write to the file.
In Windows 7 and earlier this attribute prevents deleting empty
directories. It does not prevent modifying the directory contents.
On later versions of Windows this attribute is ignored for directories.
§
Unix (including macOS)
On Unix-based platforms this sets or clears the write access bit for
the owner, group
and
others, equivalent to
chmod a+w <file>
or
chmod a-w <file>
respectively. The latter will grant write access
to all users! You can use the
PermissionsExt
trait on Unix
to avoid this issue.
§
Examples
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
let
metadata = f.metadata()
?
;
let
mut
permissions = metadata.permissions();

    permissions.set_readonly(
true
);
// filesystem doesn't change, only the in memory state of the
    // readonly permission
assert_eq!
(
false
, metadata.permissions().readonly());
// just this particular `permissions`.
assert_eq!
(
true
, permissions.readonly());
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
Permissions
Source
§
fn
clone
(&self) ->
Permissions
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
Permissions
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
PartialEq
for
Permissions
Source
§
fn
eq
(&self, other: &
Permissions
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
1.1.0
·
Source
§
impl
PermissionsExt
for
Permissions
Available on
Unix
only.
Source
§
fn
mode
(&self) ->
u32
Returns the mode permission bits
Source
§
fn
set_mode
(&mut self, mode:
u32
)
Sets the mode permission bits.
Source
§
fn
from_mode
(mode:
u32
) ->
Permissions
Creates a new instance from the given mode permission bits.
1.0.0
·
Source
§
impl
Eq
for
Permissions
1.0.0
·
Source
§
impl
StructuralPartialEq
for
Permissions
Auto Trait Implementations
§
§
impl
Freeze
for
Permissions
§
impl
RefUnwindSafe
for
Permissions
§
impl
Send
for
Permissions
§
impl
Sync
for
Permissions
§
impl
Unpin
for
Permissions
§
impl
UnwindSafe
for
Permissions
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