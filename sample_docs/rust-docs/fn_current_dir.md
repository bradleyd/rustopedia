current_dir in std::env - Rust
std
::
env
Function
current_dir
Copy item path
1.0.0
·
Source
pub fn current_dir() ->
Result
<
PathBuf
>
Expand description
Returns the current working directory as a
PathBuf
.
§
Platform-specific behavior
This function
currently
corresponds to the
getcwd
function on Unix
and the
GetCurrentDirectoryW
function on Windows.
§
Errors
Returns an
Err
if the current working directory value is invalid.
Possible cases:
Current directory does not exist.
There are insufficient permissions to access the current directory.
§
Examples
use
std::env;
fn
main() -> std::io::Result<()> {
let
path = env::current_dir()
?
;
println!
(
"The current directory is {}"
, path.display());
Ok
(())
}