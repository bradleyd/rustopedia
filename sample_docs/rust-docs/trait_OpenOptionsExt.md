OpenOptionsExt in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Trait
OpenOptionsExt
Copy item path
1.1.0
·
Source
pub trait OpenOptionsExt {
    // Required methods
    fn
mode
(&mut self, mode:
u32
) -> &mut Self;
fn
custom_flags
(&mut self, flags:
i32
) -> &mut Self;
}
Available on
Unix
only.
Expand description
Unix-specific extensions to
fs::OpenOptions
.
Required Methods
§
1.1.0
·
Source
fn
mode
(&mut self, mode:
u32
) -> &mut Self
Sets the mode bits that a new file will be created with.
If a new file is created as part of an
OpenOptions::open
call then this
specified
mode
will be used as the permission bits for the new file.
If no
mode
is set, the default of
0o666
will be used.
The operating system masks out bits with the system’s
umask
, to produce
the final permissions.
§
Examples
use
std::fs::OpenOptions;
use
std::os::unix::fs::OpenOptionsExt;
let
mut
options = OpenOptions::new();
options.mode(
0o644
);
// Give read/write for owner and read for others.
let
file = options.open(
"foo.txt"
);
1.10.0
·
Source
fn
custom_flags
(&mut self, flags:
i32
) -> &mut Self
Pass custom flags to the
flags
argument of
open
.
The bits that define the access mode are masked out with
O_ACCMODE
, to
ensure they do not interfere with the access mode set by Rusts options.
Custom flags can only set flags, not remove flags set by Rusts options.
This options overwrites any previously set custom flags.
§
Examples
use
std::fs::OpenOptions;
use
std::os::unix::fs::OpenOptionsExt;
let
mut
options = OpenOptions::new();
options.write(
true
);
if
cfg!
(unix) {
    options.custom_flags(libc::O_NOFOLLOW);
}
let
file = options.open(
"foo.txt"
);
Dyn Compatibility
§
This trait is
not
dyn compatible
.
In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.
Implementors
§
1.1.0
·
Source
§
impl
OpenOptionsExt
for
OpenOptions