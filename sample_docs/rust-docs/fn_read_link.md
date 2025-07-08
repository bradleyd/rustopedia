read_link in std::fs - Rust
std
::
fs
Function
read_link
Copy item path
1.0.0
·
Source
pub fn read_link<P:
AsRef
<
Path
>>(path: P) ->
Result
<
PathBuf
>
Expand description
Reads a symbolic link, returning the file that the link points to.
§
Platform-specific behavior
This function currently corresponds to the
readlink
function on Unix
and the
CreateFile
function with
FILE_FLAG_OPEN_REPARSE_POINT
and
FILE_FLAG_BACKUP_SEMANTICS
flags on Windows.
Note that, this
may change in the future
.
§
Errors
This function will return an error in the following situations, but is not
limited to just these cases:
path
is not a symbolic link.
path
does not exist.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
let
path = fs::read_link(
"a.txt"
)
?
;
Ok
(())
}