temp_dir in std::env - Rust
std
::
env
Function
temp_dir
Copy item path
1.0.0
·
Source
pub fn temp_dir() ->
PathBuf
Expand description
Returns the path of a temporary directory.
The temporary directory may be shared among users, or between processes
with different privileges; thus, the creation of any files or directories
in the temporary directory must use a secure method to create a uniquely
named file. Creating a file or directory with a fixed or predictable name
may result in “insecure temporary file” security vulnerabilities. Consider
using a crate that securely creates temporary files or directories.
Note that the returned value may be a symbolic link, not a directory.
§
Platform-specific behavior
On Unix, returns the value of the
TMPDIR
environment variable if it is
set, otherwise the value is OS-specific:
On Android, there is no global temporary folder (it is usually allocated
per-app), it will return the application’s cache dir if the program runs
in application’s namespace and system version is Android 13 (or above), or
/data/local/tmp
otherwise.
On Darwin-based OSes (macOS, iOS, etc) it returns the directory provided
by
confstr(_CS_DARWIN_USER_TEMP_DIR, ...)
, as recommended by
Apple’s
security guidelines
.
On all other unix-based OSes, it returns
/tmp
.
On Windows, the behavior is equivalent to that of
GetTempPath2
/
GetTempPath
, which this function uses internally.
Note that, this
may change in the future
.
use
std::env;
fn
main() {
let
dir = env::temp_dir();
println!
(
"Temporary directory: {}"
, dir.display());
}