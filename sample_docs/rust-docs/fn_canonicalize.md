canonicalize in std::fs - Rust
std
::
fs
Function
canonicalize
Copy item path
1.5.0
·
Source
pub fn canonicalize<P:
AsRef
<
Path
>>(path: P) ->
Result
<
PathBuf
>
Expand description
Returns the canonical, absolute form of a path with all intermediate
components normalized and symbolic links resolved.
§
Platform-specific behavior
This function currently corresponds to the
realpath
function on Unix
and the
CreateFile
and
GetFinalPathNameByHandle
functions on Windows.
Note that this
may change in the future
.
On Windows, this converts the path to use
extended length path
syntax, which allows your program to use longer path names, but means you
can only join backslash-delimited paths to it, and it may be incompatible
with other applications (if passed to the application on the command-line,
or written to a file another application may read).
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
path
does not exist.
A non-final component in path is not a directory.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
path = fs::canonicalize(
"../a/../foo.txt"
)
?
;
Ok
(())
}