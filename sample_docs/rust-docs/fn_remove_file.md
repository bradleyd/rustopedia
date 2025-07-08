remove_file in std::fs - Rust
std
::
fs
Function
remove_file
Copy item path
1.0.0
·
Source
pub fn remove_file<P:
AsRef
<
Path
>>(path: P) ->
Result
<
()
>
Expand description
Removes a file from the filesystem.
Note that there is no
guarantee that the file is immediately deleted (e.g., depending on
platform, other open file descriptors may prevent immediate removal).
§
Platform-specific behavior
This function currently corresponds to the
unlink
function on Unix.
On Windows,
DeleteFile
is used or
CreateFileW
and
SetInformationByHandle
for readonly files.
Note that, this
may change in the future
.
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
path
points to a directory.
The file doesn’t exist.
The user lacks permissions to remove the file.
This function will only ever return an error of kind
NotFound
if the given
path does not exist. Note that the inverse is not true,
ie. if a path does not exist, its removal may fail for a number of reasons,
such as insufficient permissions.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
    fs::remove_file(
"a.txt"
)
?
;
Ok
(())
}