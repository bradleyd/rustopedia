join_paths in std::env - Rust
std
::
env
Function
join_paths
Copy item path
1.0.0
·
Source
pub fn join_paths<I, T>(paths: I) ->
Result
<
OsString
,
JoinPathsError
>
where
    I:
IntoIterator
<Item = T>,
    T:
AsRef
<
OsStr
>,
Expand description
Joins a collection of
Path
s appropriately for the
PATH
environment variable.
§
Errors
Returns an
Err
(containing an error message) if one of the input
Path
s contains an invalid character for constructing the
PATH
variable (a double quote on Windows or a colon on Unix), or if the system
does not have a
PATH
-like variable (e.g. UEFI or WASI).
§
Examples
Joining paths on a Unix-like platform:
use
std::env;
use
std::ffi::OsString;
use
std::path::Path;
fn
main() ->
Result
<(), env::JoinPathsError> {
let
paths = [Path::new(
"/bin"
), Path::new(
"/usr/bin"
)];
let
path_os_string = env::join_paths(paths.iter())
?
;
assert_eq!
(path_os_string, OsString::from(
"/bin:/usr/bin"
));
Ok
(())
}
Joining a path containing a colon on a Unix-like platform results in an
error:
use
std::env;
use
std::path::Path;
let
paths = [Path::new(
"/bin"
), Path::new(
"/usr/bi:n"
)];
assert!
(env::join_paths(paths.iter()).is_err());
Using
env::join_paths()
with
env::split_paths()
to append an item to
the
PATH
environment variable:
use
std::env;
use
std::path::PathBuf;
fn
main() ->
Result
<(), env::JoinPathsError> {
if let
Some
(path) = env::var_os(
"PATH"
) {
let
mut
paths = env::split_paths(
&
path).collect::<Vec<
_
>>();
        paths.push(PathBuf::from(
"/home/xyz/bin"
));
let
new_path = env::join_paths(paths)
?
;
unsafe
{ env::set_var(
"PATH"
,
&
new_path); }
    }
Ok
(())
}