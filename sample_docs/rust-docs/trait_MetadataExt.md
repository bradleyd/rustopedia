MetadataExt in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Trait
MetadataExt
Copy item path
1.1.0
·
Source
pub trait MetadataExt {
Show 16 methods
// Required methods
    fn
dev
(&self) ->
u64
;
fn
ino
(&self) ->
u64
;
fn
mode
(&self) ->
u32
;
fn
nlink
(&self) ->
u64
;
fn
uid
(&self) ->
u32
;
fn
gid
(&self) ->
u32
;
fn
rdev
(&self) ->
u64
;
fn
size
(&self) ->
u64
;
fn
atime
(&self) ->
i64
;
fn
atime_nsec
(&self) ->
i64
;
fn
mtime
(&self) ->
i64
;
fn
mtime_nsec
(&self) ->
i64
;
fn
ctime
(&self) ->
i64
;
fn
ctime_nsec
(&self) ->
i64
;
fn
blksize
(&self) ->
u64
;
fn
blocks
(&self) ->
u64
;
}
Available on
Unix
only.
Expand description
Unix-specific extensions to
fs::Metadata
.
Required Methods
§
1.1.0
·
Source
fn
dev
(&self) ->
u64
Returns the ID of the device containing the file.
§
Examples
use
std::io;
use
std::fs;
use
std::os::unix::fs::MetadataExt;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
dev_id = meta.dev();
Ok
(())
}
1.1.0
·
Source
fn
ino
(&self) ->
u64
Returns the inode number.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
inode = meta.ino();
Ok
(())
}
1.1.0
·
Source
fn
mode
(&self) ->
u32
Returns the rights applied to this file.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
mode = meta.mode();
let
user_has_write_access      = mode &
0o200
;
let
user_has_read_write_access = mode &
0o600
;
let
group_has_read_access      = mode &
0o040
;
let
others_have_exec_access    = mode &
0o001
;
Ok
(())
}
1.1.0
·
Source
fn
nlink
(&self) ->
u64
Returns the number of hard links pointing to this file.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
nb_hard_links = meta.nlink();
Ok
(())
}
1.1.0
·
Source
fn
uid
(&self) ->
u32
Returns the user ID of the owner of this file.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
user_id = meta.uid();
Ok
(())
}
1.1.0
·
Source
fn
gid
(&self) ->
u32
Returns the group ID of the owner of this file.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
group_id = meta.gid();
Ok
(())
}
1.1.0
·
Source
fn
rdev
(&self) ->
u64
Returns the device ID of this file (if it is a special one).
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
device_id = meta.rdev();
Ok
(())
}
1.1.0
·
Source
fn
size
(&self) ->
u64
Returns the total size of this file in bytes.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
file_size = meta.size();
Ok
(())
}
1.1.0
·
Source
fn
atime
(&self) ->
i64
Returns the last access time of the file, in seconds since Unix Epoch.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
last_access_time = meta.atime();
Ok
(())
}
1.1.0
·
Source
fn
atime_nsec
(&self) ->
i64
Returns the last access time of the file, in nanoseconds since
atime
.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
nano_last_access_time = meta.atime_nsec();
Ok
(())
}
1.1.0
·
Source
fn
mtime
(&self) ->
i64
Returns the last modification time of the file, in seconds since Unix Epoch.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
last_modification_time = meta.mtime();
Ok
(())
}
1.1.0
·
Source
fn
mtime_nsec
(&self) ->
i64
Returns the last modification time of the file, in nanoseconds since
mtime
.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
nano_last_modification_time = meta.mtime_nsec();
Ok
(())
}
1.1.0
·
Source
fn
ctime
(&self) ->
i64
Returns the last status change time of the file, in seconds since Unix Epoch.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
last_status_change_time = meta.ctime();
Ok
(())
}
1.1.0
·
Source
fn
ctime_nsec
(&self) ->
i64
Returns the last status change time of the file, in nanoseconds since
ctime
.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
nano_last_status_change_time = meta.ctime_nsec();
Ok
(())
}
1.1.0
·
Source
fn
blksize
(&self) ->
u64
Returns the block size for filesystem I/O.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
block_size = meta.blksize();
Ok
(())
}
1.1.0
·
Source
fn
blocks
(&self) ->
u64
Returns the number of blocks allocated to the file, in 512-byte units.
Please note that this may be smaller than
st_size / 512
when the file has holes.
§
Examples
use
std::fs;
use
std::os::unix::fs::MetadataExt;
use
std::io;
fn
main() -> io::Result<()> {
let
meta = fs::metadata(
"some_file"
)
?
;
let
blocks = meta.blocks();
Ok
(())
}
Implementors
§
1.1.0
·
Source
§
impl
MetadataExt
for
Metadata