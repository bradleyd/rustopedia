home_dir in std::env - Rust
std
::
env
Function
home_dir
Copy item path
1.0.0
·
Source
pub fn home_dir() ->
Option
<
PathBuf
>
Expand description
Returns the path of the current user’s home directory if known.
This may return
None
if getting the directory fails or if the platform does not have user home directories.
For storing user data and configuration it is often preferable to use more specific directories.
For example,
XDG Base Directories
on Unix or the
LOCALAPPDATA
and
APPDATA
environment variables on Windows.
§
Unix
Returns the value of the ‘HOME’ environment variable if it is set
(including to an empty string).
Otherwise, it tries to determine the home directory by invoking the
getpwuid_r
function
using the UID of the current user. An empty home directory field returned from the
getpwuid_r
function is considered to be a valid value.
Returns
None
if the current user has no entry in the /etc/passwd file.
§
Windows
Returns the value of the ‘USERPROFILE’ environment variable if it is set, and is not an empty string.
Otherwise,
GetUserProfileDirectory
is used to return the path. This may change in the future.
In UWP (Universal Windows Platform) targets this function is unimplemented and always returns
None
.
Before Rust 1.85.0, this function used to return the value of the ‘HOME’ environment variable
on Windows, which in Cygwin or Mingw environments could return non-standard paths like
/home/you
instead of
C:\Users\you
.
§
Examples
use
std::env;
match
env::home_dir() {
Some
(path) =>
println!
(
"Your home directory, probably: {}"
, path.display()),
None
=>
println!
(
"Impossible to get your home dir!"
),
}