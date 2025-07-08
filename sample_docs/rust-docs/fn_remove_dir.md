remove_dir in std::fs - Rust
std
::
fs
Function
remove_dir
Copy item path
1.0.0
·
Source
pub fn remove_dir<P:
AsRef
<
Path
>>(path: P) ->
Result
<
()
>
Expand description
Removes an empty directory.
If you want to remove a directory that is not empty, as well as all
of its contents recursively, consider using
remove_dir_all
instead.
§
Platform-specific behavior
This function currently corresponds to the
rmdir
function on Unix
and the
RemoveDirectory
function on Windows.
Note that, this
may change in the future
.
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
path
doesn’t exist.
path
isn’t a directory.
The user lacks permissions to remove the directory at the provided
path
.
The directory isn’t empty.
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
    fs::remove_dir(
"/some/dir"
)
?
;
Ok
(())
}