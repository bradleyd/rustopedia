symlink_metadata in std::fs - Rust
std
::
fs
Function
symlink_metadata
Copy item path
1.1.0
·
Source
pub fn symlink_metadata<P:
AsRef
<
Path
>>(path: P) ->
Result
<
Metadata
>
Expand description
Queries the metadata about a file without following symlinks.
§
Platform-specific behavior
This function currently corresponds to the
lstat
function on Unix
and the
GetFileInformationByHandle
function on Windows.
Note that, this
may change in the future
.
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
The user lacks permissions to perform
metadata
call on
path
.
path
does not exist.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
attr = fs::symlink_metadata(
"/some/file/path.txt"
)
?
;
// inspect attr ...
Ok
(())
}