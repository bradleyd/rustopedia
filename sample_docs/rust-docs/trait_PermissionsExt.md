PermissionsExt in std::os::unix::fs - Rust
std
::
os
::
unix
::
fs
Trait
PermissionsExt
Copy item path
1.1.0
·
Source
pub trait PermissionsExt {
    // Required methods
    fn
mode
(&self) ->
u32
;
fn
set_mode
(&mut self, mode:
u32
);
fn
from_mode
(mode:
u32
) -> Self;
}
Available on
Unix
only.
Expand description
Unix-specific extensions to
fs::Permissions
.
§
Examples
use
std::fs::{File, Permissions};
use
std::io::{ErrorKind,
Result
as
IoResult};
use
std::os::unix::fs::PermissionsExt;
fn
main() -> IoResult<()> {
let
name =
"test_file_for_permissions"
;
// make sure file does not exist
let _
= std::fs::remove_file(name);
assert_eq!
(
        File::open(name).unwrap_err().kind(),
        ErrorKind::NotFound,
"file already exists"
);
// full read/write/execute mode bits for owner of file
    // that we want to add to existing mode bits
let
my_mode =
0o700
;
// create new file with specified permissions
{
let
file = File::create(name)
?
;
let
mut
permissions = file.metadata()
?
.permissions();
eprintln!
(
"Current permissions: {:o}"
, permissions.mode());
// make sure new permissions are not already set
assert!
(
            permissions.mode() & my_mode != my_mode,
"permissions already set"
);
// either use `set_mode` to change an existing Permissions struct
permissions.set_mode(permissions.mode() | my_mode);
// or use `from_mode` to construct a new Permissions struct
permissions = Permissions::from_mode(permissions.mode() | my_mode);
// write new permissions to file
file.set_permissions(permissions)
?
;
    }
let
permissions = File::open(name)
?
.metadata()
?
.permissions();
eprintln!
(
"New permissions: {:o}"
, permissions.mode());
// assert new permissions were set
assert_eq!
(
        permissions.mode() & my_mode,
        my_mode,
"new permissions not set"
);
Ok
(())
}
use
std::fs::Permissions;
use
std::os::unix::fs::PermissionsExt;
// read/write for owner and read for others
let
my_mode =
0o644
;
let
mut
permissions = Permissions::from_mode(my_mode);
assert_eq!
(permissions.mode(), my_mode);
// read/write/execute for owner
let
other_mode =
0o700
;
permissions.set_mode(other_mode);
assert_eq!
(permissions.mode(), other_mode);
Required Methods
§
1.1.0
·
Source
fn
mode
(&self) ->
u32
Returns the mode permission bits
1.1.0
·
Source
fn
set_mode
(&mut self, mode:
u32
)
Sets the mode permission bits.
1.1.0
·
Source
fn
from_mode
(mode:
u32
) -> Self
Creates a new instance from the given mode permission bits.
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
PermissionsExt
for
Permissions