set_current_dir in std::env - Rust
std
::
env
Function
set_current_dir
Copy item path
1.0.0
·
Source
pub fn set_current_dir<P:
AsRef
<
Path
>>(path: P) ->
Result
<
()
>
Expand description
Changes the current working directory to the specified path.
§
Platform-specific behavior
This function
currently
corresponds to the
chdir
function on Unix
and the
SetCurrentDirectoryW
function on Windows.
Returns an
Err
if the operation fails.
§
Examples
use
std::env;
use
std::path::Path;
let
root = Path::new(
"/"
);
assert!
(env::set_current_dir(
&
root).is_ok());
println!
(
"Successfully changed working directory to {}!"
, root.display());