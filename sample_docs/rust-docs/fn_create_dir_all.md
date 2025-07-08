create_dir_all in std::fs - Rust
std
::
fs
Function
create_dir_all
Copy item path
1.0.0
·
Source
pub fn create_dir_all<P:
AsRef
<
Path
>>(path: P) ->
Result
<
()
>
Expand description
Recursively create a directory and all of its parent components if they
are missing.
If this function returns an error, some of the parent components might have
been created already.
If the empty path is passed to this function, it always succeeds without
creating any directories.
§
Platform-specific behavior
This function currently corresponds to multiple calls to the
mkdir
function on Unix and the
CreateDirectoryW
function on Windows.
Note that, this
may change in the future
.
§
Errors
The function will return an error if any directory specified in path does not exist and
could not be created. There may be other error conditions; see
fs::create_dir
for specifics.
Notable exception is made for situations where any of the directories
specified in the
path
could not be created as it was being created concurrently.
Such cases are considered to be successful. That is, calling
create_dir_all
concurrently from multiple threads or processes is guaranteed not to fail
due to a race condition with itself.
§
Examples
use
std::fs;
fn
main() -> std::io::Result<()> {
    fs::create_dir_all(
"/some/dir"
)
?
;
Ok
(())
}