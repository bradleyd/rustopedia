metadata in std::fs - Rust
std
::
fs
Function
metadata
Copy item path
1.0.0
·
Source
pub fn metadata<P:
AsRef
<
Path
>>(path: P) ->
Result
<
Metadata
>
Expand description
Given a path, queries the file system to get information about a file,
directory, etc.
This function will traverse symbolic links to query information about the
destination file.
§
Platform-specific behavior
This function currently corresponds to the
stat
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
attr = fs::metadata(
"/some/file/path.txt"
)
?
;
// inspect attr ...
Ok
(())
}