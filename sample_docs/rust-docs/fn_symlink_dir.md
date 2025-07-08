symlink_dir in std::os::windows::fs - Rust
std
::
os
::
windows
::
fs
Function
symlink_dir
Copy item path
1.1.0
·
Source
pub fn symlink_dir<P:
AsRef
<
Path
>, Q:
AsRef
<
Path
>>(
    original: P,
    link: Q,
) ->
Result
<
()
>
Available on
Windows
only.
Expand description
Creates a new symlink to a directory on the filesystem.
The
link
path will be a directory symbolic link pointing to the
original
path.
The
original
path must be a directory or a symlink to a directory,
otherwise the symlink will be broken. Use
symlink_file
for other files.
This function currently corresponds to
CreateSymbolicLinkW
.
Note that this
may change in the future
.
§
Examples
use
std::os::windows::fs;
fn
main() -> std::io::Result<()> {
    fs::symlink_dir(
"a"
,
"b"
)
?
;
Ok
(())
}
§
Limitations
Windows treats symlink creation as a
privileged action
,
therefore this function is likely to fail unless the user makes changes to
their system to permit symlink creation. Users can try enabling Developer
Mode, granting the
SeCreateSymbolicLinkPrivilege
privilege, or running
the process as an administrator.