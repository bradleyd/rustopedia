create_dir in std::fs - Rust
std
::
fs
Function
create_dir
Copy item path
1.0.0
·
Source
pub fn create_dir<P:
AsRef
<
Path
>>(path: P) ->
Result
<
()
>
Expand description
Creates a new, empty directory at the provided path
§
Platform-specific behavior
This function currently corresponds to the
mkdir
function on Unix
and the
CreateDirectoryW
function on Windows.
Note that, this
may change in the future
.
NOTE
: If a parent of the given path doesn’t exist, this function will
return an error. To create a directory and all its missing parents at the
same time, use the
create_dir_all
function.
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
User lacks permissions to create directory at
path
.
A parent of the given path doesn’t exist. (To create a directory and all
its missing parents at the same time, use the
create_dir_all
function.)
path
already exists.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
    fs::create_dir(
"/some/dir"
)
?
;
Ok
(())
}