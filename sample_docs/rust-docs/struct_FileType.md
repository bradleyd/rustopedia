FileType in std::fs - Rust
std
::
fs
Struct
FileType
Copy item path
1.1.0
·
Source
pub struct FileType(
/* private fields */
);
Expand description
A structure representing a type of file with accessors for each file type.
It is returned by
Metadata::file_type
method.
Implementations
§
Source
§
impl
FileType
1.1.0
·
Source
pub fn
is_dir
(&self) ->
bool
Tests whether this file type represents a directory. The
result is mutually exclusive to the results of
is_file
and
is_symlink
; only zero or one of these
tests may pass.
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
let
file_type = metadata.file_type();
assert_eq!
(file_type.is_dir(),
false
);
Ok
(())
}
1.1.0
·
Source
pub fn
is_file
(&self) ->
bool
Tests whether this file type represents a regular file.
The result is mutually exclusive to the results of
is_dir
and
is_symlink
; only zero or one of these
tests may pass.
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
let
file_type = metadata.file_type();
assert_eq!
(file_type.is_file(),
true
);
Ok
(())
}
1.1.0
·
Source
pub fn
is_symlink
(&self) ->
bool
Tests whether this file type represents a symbolic link.
The result is mutually exclusive to the results of
is_dir
and
is_file
; only zero or one of these
tests may pass.
The underlying
Metadata
struct needs to be retrieved
with the
fs::symlink_metadata
function and not the
fs::metadata
function. The
fs::metadata
function
follows symbolic links, so
is_symlink
would always
return
false
for the target file.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
metadata = fs::symlink_metadata(
"foo.txt"
)
?
;
let
file_type = metadata.file_type();
assert_eq!
(file_type.is_symlink(),
false
);
Ok
(())
}
Trait Implementations
§
1.1.0
·
Source
§
impl
Clone
for
FileType
Source
§
fn
clone
(&self) ->
FileType
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
FileType
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
1.5.0
·
Source
§
impl
FileTypeExt
for
FileType
Available on
Unix
only.
Source
§
fn
is_block_device
(&self) ->
bool
Returns
true
if this file type is a block device.
Read more
Source
§
fn
is_char_device
(&self) ->
bool
Returns
true
if this file type is a char device.
Read more
Source
§
fn
is_fifo
(&self) ->
bool
Returns
true
if this file type is a fifo.
Read more
Source
§
fn
is_socket
(&self) ->
bool
Returns
true
if this file type is a socket.
Read more
Source
§
impl
FileTypeExt
for
FileType
Available on
WASI
only.
Source
§
fn
is_block_device
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns
true
if this file type is a block device.
Source
§
fn
is_char_device
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns
true
if this file type is a character device.
Source
§
fn
is_socket_dgram
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns
true
if this file type is a socket datagram.
Source
§
fn
is_socket_stream
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns
true
if this file type is a socket stream.
Source
§
fn
is_socket
(&self) ->
bool
🔬
This is a nightly-only experimental API. (
wasi_ext
#71213
)
Returns
true
if this file type is any type of socket.
1.64.0
·
Source
§
impl
FileTypeExt
for
FileType
Available on
Windows
only.
Source
§
fn
is_symlink_dir
(&self) ->
bool
Returns
true
if this file type is a symbolic link that is also a directory.
Source
§
fn
is_symlink_file
(&self) ->
bool
Returns
true
if this file type is a symbolic link that is also a file.
1.1.0
·
Source
§
impl
Hash
for
FileType
Source
§
fn
hash
<__H:
Hasher
>(&self, state:
&mut __H
)
Feeds this value into the given
Hasher
.
Read more
1.3.0
·
Source
§
fn
hash_slice
<H>(data: &[Self], state:
&mut H
)
where
    H:
Hasher
,
    Self:
Sized
,
Feeds a slice of this type into the given
Hasher
.
Read more
1.1.0
·
Source
§
impl
PartialEq
for
FileType
Source
§
fn
eq
(&self, other: &
FileType
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
Copy
for
FileType
1.1.0
·
Source
§
impl
Eq
for
FileType
1.1.0
·
Source
§
impl
StructuralPartialEq
for
FileType
Auto Trait Implementations
§
§
impl
Freeze
for
FileType
§
impl
RefUnwindSafe
for
FileType
§
impl
Send
for
FileType
§
impl
Sync
for
FileType
§
impl
Unpin
for
FileType
§
impl
UnwindSafe
for
FileType
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